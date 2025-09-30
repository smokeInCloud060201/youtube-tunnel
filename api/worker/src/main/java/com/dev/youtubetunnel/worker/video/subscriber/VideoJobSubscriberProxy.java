package com.dev.youtubetunnel.worker.video.subscriber;

import com.dev.youtubetunnel.common.dto.VideoJobRequest;
import io.minio.MinioClient;
import io.minio.UploadObjectArgs;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.core.io.ClassPathResource;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Service;

import java.io.IOException;
import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.nio.file.StandardWatchEventKinds;
import java.nio.file.WatchEvent;
import java.nio.file.WatchKey;
import java.nio.file.WatchService;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

@Service
@RequiredArgsConstructor
@Slf4j
public class VideoJobSubscriberProxy {

    private final MinioClient minioClient;


    @Async("workerConsumeThreadPool")
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
                "-c:a", "aac",
                "-preset", "fast",
                "-crf", "23",
                "-hls_time", "10",
                "-hls_list_size", "0",
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
