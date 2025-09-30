package com.dev.youtubetunnel.video.services.impl;

import com.dev.youtubetunnel.common.dto.VideoJobRequest;
import com.dev.youtubetunnel.common.publisher.MessagePublisher;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.data.redis.listener.ChannelTopic;
import org.springframework.stereotype.Service;

@RequiredArgsConstructor
@Slf4j
@Service
public class VideoJobPublisher implements MessagePublisher<VideoJobRequest> {

    private final RedisTemplate<String, Object> redisTemplate;

    private final ChannelTopic topic;

    @Override
    public void publish(VideoJobRequest message) {
        log.info("Publishing video message to topic {} {}", topic.getTopic(), message.jobId());
        redisTemplate.convertAndSend(topic.getTopic(), message);
    }
}
