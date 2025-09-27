package com.dev.youtubetunnel.config;


import io.minio.BucketExistsArgs;
import io.minio.MakeBucketArgs;
import io.minio.MinioClient;
import io.minio.errors.MinioException;
import io.minio.messages.AccessControlList;
import jakarta.annotation.PostConstruct;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Component;
import org.springframework.web.cors.CorsConfiguration;
import java.util.Arrays;

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