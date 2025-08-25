package org.example.youtubetunnel.config;

import org.springframework.boot.context.properties.ConfigurationProperties;

@ConfigurationProperties(prefix = "config")
public record ConfigProperties(String key) {
}
