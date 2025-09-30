package com.dev.youtubetunnel.worker.video.subscriber;

import com.dev.youtubetunnel.common.dto.VideoJobRequest;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.minio.MinioClient;
import io.minio.StatObjectArgs;
import io.minio.UploadObjectArgs;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.lang3.StringUtils;
import org.springframework.core.io.ClassPathResource;
import org.springframework.dao.QueryTimeoutException;
import org.springframework.data.redis.core.RedisCallback;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Component;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.nio.file.StandardWatchEventKinds;
import java.nio.file.WatchEvent;
import java.nio.file.WatchKey;
import java.nio.file.WatchService;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

@Slf4j
@Component
@RequiredArgsConstructor
public class VideoJobConsumer implements Runnable {

    private final MinioClient minioClient;
    private final RedisTemplate<String, VideoJobRequest> redisTemplate;
    private ExecutorService executor;

    @PostConstruct
    public void startWorker() {
        executor = Executors.newVirtualThreadPerTaskExecutor();
        for (int i = 0; i < 9; i++) {
            executor.submit(this);
        }
    }

    @PreDestroy
    public void stopWorker() {
        if (executor != null) {
            executor.shutdownNow();
            log.info("Shutting down worker...");
        }
    }

    @Override
    public void run() {
        while (!Thread.currentThread().isInterrupted()) {
            try {
                List<byte[]> result = redisTemplate.execute(
                        (RedisCallback<List<byte[]>>) connection ->
                                connection.listCommands().bRPop(0, "job-queue".getBytes(StandardCharsets.UTF_8))
                );

                if (result != null && result.size() == 2) {
                    byte[] payload = result.get(1);
                    VideoJobRequest job = (VideoJobRequest) redisTemplate.getValueSerializer().deserialize(payload);

                    if (job != null) {
                        log.info("Received Job Request: {}", job);
                        handleJob(job);
                    } else {
                        log.warn("Failed to deserialize job payload: {}", new String(payload, StandardCharsets.UTF_8));
                    }
                }
            }
            catch (QueryTimeoutException ignored) {

            }
            catch (Exception e) {
                log.error("Error consuming job", e);
            }
        }
    }

    private void handleJob(VideoJobRequest videoJobMessage) {
        try {
            log.info("Received VideoJobRequest: {}", videoJobMessage);

            if (StringUtils.isEmpty(videoJobMessage.jobId()) || objectExists("videos", videoJobMessage.jobId() + "/playlist.m3u8")) {
                log.info("Job is invalid or already exists {}", videoJobMessage.jobId());
                return;
            }

            consumeData(videoJobMessage);
        } catch (IOException | InterruptedException e) {
            log.error(e.getMessage(), e);
            throw new RuntimeException(e);
        }
    }

    private boolean objectExists(String bucket, String object) {
        try {
            minioClient.statObject(
                    StatObjectArgs.builder()
                            .bucket(bucket)
                            .object(object)
                            .build()
            );
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    public void consumeData(VideoJobRequest request) throws IOException, InterruptedException {
        String jobId = request.jobId();
        Path workDir = Files.createTempDirectory("video-" + jobId);
        Path playlist = workDir.resolve("playlist.m3u8");

        Path cookiePath = Files.createTempFile("youtube-cookies", ".txt");
        Files.copy(
                new ClassPathResource("static/cookie.txt").getInputStream(),
                cookiePath,
                StandardCopyOption.REPLACE_EXISTING
        );

        ProcessBuilder ytPb = new ProcessBuilder(
                "/home/ngockhanh/yt-dlp-venv/bin/yt-dlp",
                "--no-playlist",
                "--cookies", cookiePath.toAbsolutePath().toString(),
                "-f", "bv*+ba/b",
                "-o", "-",
                request.youtubeUrl()
        );
        ytPb.redirectError(ProcessBuilder.Redirect.INHERIT);
        Process ytProcess = ytPb.start();

        ProcessBuilder ffPb = new ProcessBuilder(
                "ffmpeg",
                "-i", "pipe:0",
                "-c:v", "libx264",
                "-preset", "fast",
                "-crf", "23",
                "-g", "60",
                "-keyint_min", "60",
                "-sc_threshold", "0",
                "-c:a", "aac",
                "-b:a", "128k",
                "-ac", "2",
                "-ar", "44100",
                "-af", "aresample=async=1",
                "-hls_time", "6",
                "-hls_list_size", "5",
                "-hls_flags", "delete_segments+independent_segments",
                "-start_number", "0",
                "-f", "hls",
                playlist.toString()
        );
        ffPb.directory(workDir.toFile());
        ffPb.redirectError(ProcessBuilder.Redirect.INHERIT);
        Process ffProcess = ffPb.start();

        ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor();

        executor.submit(() -> {
            try (var in = ytProcess.getInputStream();
                 var out = ffProcess.getOutputStream()) {
                in.transferTo(out);
            } catch (IOException e) {
                log.error("Pipe failed", e);
            }
        });

        executor.submit(() -> {
            long lastPlaylistUpdateTime = 0L;

            try (WatchService watcher = FileSystems.getDefault().newWatchService()) {
                workDir.register(
                        watcher,
                        StandardWatchEventKinds.ENTRY_CREATE,
                        StandardWatchEventKinds.ENTRY_MODIFY
                );

                while (ffProcess.isAlive()) {
                    WatchKey key = watcher.take();
                    for (WatchEvent<?> event : key.pollEvents()) {
                        Path file = workDir.resolve((Path) event.context());
                        String filename = file.getFileName().toString();

                        try {
                            if (filename.endsWith(".ts")) {
                                minioClient.uploadObject(
                                        UploadObjectArgs.builder()
                                                .bucket("videos")
                                                .object(jobId + "/" + filename)
                                                .filename(file.toString())
                                                .build()
                                );
                                log.info("Uploaded segment {}", filename);

                            } else if (filename.endsWith(".m3u8")) {
                                long lastModified = Files.getLastModifiedTime(file).toMillis();

                                if (lastModified > lastPlaylistUpdateTime) {
                                    lastPlaylistUpdateTime = lastModified;

                                    minioClient.uploadObject(
                                            UploadObjectArgs.builder()
                                                    .bucket("videos")
                                                    .object(jobId + "/" + filename)
                                                    .filename(file.toString())
                                                    .build()
                                    );
                                    log.info("Uploaded updated playlist {}", filename);
                                }
                            }
                        } catch (Exception e) {
                            log.error("Failed upload {}", filename, e);
                        }
                    }
                    key.reset();
                }
            } catch (Exception e) {
                log.error("Watcher failed", e);
            }
        });

        ytProcess.waitFor();
        ffProcess.waitFor();

        try {
            if (Files.exists(playlist)) {
                minioClient.uploadObject(
                        UploadObjectArgs.builder()
                                .bucket("videos")
                                .object(jobId + "/playlist.m3u8")
                                .filename(playlist.toString())
                                .build()
                );
                log.info("Final upload of playlist.m3u8 for job {}", jobId);
            }
        } catch (Exception e) {
            log.error("Failed final upload playlist.m3u8 for job {}", jobId, e);
        }

        log.info("Video processing completed for job {}", jobId);
        executor.shutdown();
    }

}
