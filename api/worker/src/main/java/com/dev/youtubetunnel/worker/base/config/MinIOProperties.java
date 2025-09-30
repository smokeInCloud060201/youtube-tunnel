package com.dev.youtubetunnel.worker.base.config;

import org.springframework.boot.context.properties.ConfigurationProperties;

@ConfigurationProperties(prefix = "minio")
public record MinIOProperties(String internalUrl, String username, String password) {
}
