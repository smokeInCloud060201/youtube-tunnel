package com.dev.youtubetunnel.video.services.impl;

import com.dev.youtubetunnel.common.dto.VideoJobRequest;
import com.dev.youtubetunnel.video.services.VideoPlayerService;
import io.minio.GetObjectArgs;
import io.minio.GetPresignedObjectUrlArgs;
import io.minio.MinioClient;
import io.minio.StatObjectArgs;
import io.minio.errors.ErrorResponseException;
import io.minio.errors.InsufficientDataException;
import io.minio.errors.InternalException;
import io.minio.errors.InvalidResponseException;
import io.minio.errors.ServerException;
import io.minio.errors.XmlParserException;
import io.minio.http.Method;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.io.IOException;
import java.io.InputStream;
import java.net.URI;
import java.net.URLDecoder;
import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Collectors;

@Service
@Slf4j
@RequiredArgsConstructor
public class VideoPlayerServiceImpl implements VideoPlayerService {

    private final MinioClient minioClient;
    private final VideoJobProducer videoJobProducer;
    private static final String BUCKET = "videos";

    @Override
    public String submitVideoJob(String videoSourceUrl) {
        String jobId = getJobId(videoSourceUrl);
        VideoJobRequest job = new VideoJobRequest(jobId, videoSourceUrl);
        videoJobProducer.produceJob(job);

        log.info("Video Job Request: {} {}", job, videoSourceUrl);
        return jobId;
    }

    @Override
    public String getPlayList(String jobId) throws ServerException, InsufficientDataException, ErrorResponseException, IOException, NoSuchAlgorithmException, InvalidKeyException, InvalidResponseException, XmlParserException, InternalException {
        String playlistObject = jobId + "/playlist.m3u8";

        minioClient.statObject(
                StatObjectArgs.builder().bucket(BUCKET).object(playlistObject).build()
        );

        try (InputStream stream = minioClient.getObject(
                GetObjectArgs.builder().bucket(BUCKET).object(playlistObject).build()
        )) {
            String playlistContent = new String(stream.readAllBytes(), StandardCharsets.UTF_8);

            return Arrays.stream(playlistContent.split("\n"))
                    .map(line -> {
                        if (line.endsWith(".ts")) {
                            try {
                                return minioClient.getPresignedObjectUrl(
                                        GetPresignedObjectUrlArgs.builder()
                                                .bucket(BUCKET)
                                                .object(jobId + "/" + line)
                                                .method(Method.GET)
                                                .expiry(86400)
                                                .build()
                                );
                            } catch (Exception e) {
                                log.error("Failed to presign {}", line, e);
                                return line;
                            }
                        }
                        return line;
                    })
                    .collect(Collectors.joining("\n"));
        }
    }


    private String getJobId(String linkURL) {
        URI url = URI.create(linkURL);
        String query = url.getQuery();
        if (query != null) {
            Map<String, String> queryParams = parseQuery(query);
            String videoId = queryParams.get("v");
            if (videoId != null) {
                return URLDecoder.decode(videoId, StandardCharsets.UTF_8);
            }
        }
        throw new IllegalArgumentException("Invalid URL.");
    }

    private static Map<String, String> parseQuery(String query) {
        Map<String, String> queryParams = new HashMap<>();
        for (String pair : query.split("&")) {
            int idx = pair.indexOf("=");
            if (idx > 0) {
                queryParams.put(pair.substring(0, idx), pair.substring(idx + 1));
            }
        }
        return queryParams;
    }
}
