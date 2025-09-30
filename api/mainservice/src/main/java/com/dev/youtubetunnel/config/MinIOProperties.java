package com.dev.youtubetunnel.config;

import org.springframework.boot.context.properties.ConfigurationProperties;

@ConfigurationProperties(prefix = "minio")
public record MinIOProperties(String externalUrl, String username, String password) {
}
