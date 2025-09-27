package com.dev.youtubetunnel.common.kafka.dto;

import java.io.Serializable;

public record VideoJobRequest(
        String jobId,
        String youtubeUrl
) implements Serializable {}
