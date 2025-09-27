package com.dev.youtubetunnel.worker.base.config;


import io.minio.BucketExistsArgs;
import io.minio.MakeBucketArgs;
import io.minio.MinioClient;
import jakarta.annotation.PostConstruct;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Component;

@Component
@RequiredArgsConstructor
public class MinioInitializer {

    private final MinioClient minioClient;

    @PostConstruct
    public void init() throws Exception {
        boolean exists = minioClient.bucketExists(
                BucketExistsArgs.builder().bucket("videos").build()
        );
        if (!exists) {
            minioClient.makeBucket(MakeBucketArgs.builder().bucket("videos").build());
            System.out.println("Created bucket: videos");
        } else {
            System.out.println("Bucket already exists: videos");
        }
    }
}