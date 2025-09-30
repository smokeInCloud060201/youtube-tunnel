package com.dev.youtubetunnel.worker.base.config;

import io.minio.MinioClient;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Slf4j
@Configuration
@RequiredArgsConstructor
public class ApplicationConfiguration {

    private final MinIOProperties minIOProperties;

    @Bean
    public MinioClient minioClient() {
        log.info("Initial MinIO at: {}", minIOProperties.internalUrl());
        return MinioClient.builder()
                .endpoint(minIOProperties.internalUrl())
                .credentials(minIOProperties.username(), minIOProperties.password())
                .build();
    }
}
