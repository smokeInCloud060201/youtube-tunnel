package com.dev.youtubetunnel.video.services.impl;

import com.dev.youtubetunnel.common.dto.VideoJobRequest;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;

@RequiredArgsConstructor
@Slf4j
@Service
public class VideoJobProducer {

    private final RedisTemplate<String, VideoJobRequest> redisTemplate;

    public void produceJob(VideoJobRequest message) {
        redisTemplate.opsForList().leftPush("job-queue", message);
        log.info("Job pushed: {}", message);
    }
}
