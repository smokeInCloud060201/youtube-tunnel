package com.dev.youtubetunnel.worker.base.config;


import io.minio.BucketExistsArgs;
import io.minio.MakeBucketArgs;
import io.minio.MinioClient;
import jakarta.annotation.PostConstruct;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Component;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@Component
@RequiredArgsConstructor
public class MinioInitializer {

    private final MinioClient minioClient;

    @PostConstruct
    public void init() throws Exception {
        for (int i = 0; i < 5; i++) {
            try {
                boolean exists = minioClient.bucketExists(
                        BucketExistsArgs.builder().bucket("videos").build()
                );
                if (!exists) {
                    minioClient.makeBucket(
                            MakeBucketArgs.builder().bucket("videos").build()
                    );
                    log.info("Created bucket: {}", "videos");
                } else {
                    log.info("Bucket already exists: {}", "videos");
                }
                return;
            } catch (Exception e) {
                System.out.println("MinIO not ready yet, retrying in 3s...");
                Thread.sleep(3000);
            }
        }
        throw new IllegalStateException("Could not connect to MinIO after 5 retries");
    }
}