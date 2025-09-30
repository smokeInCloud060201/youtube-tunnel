package com.dev.youtubetunnel.config;

import com.dev.youtubetunnel.common.dto.VideoJobRequest;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.redis.connection.RedisConnectionFactory;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.data.redis.serializer.Jackson2JsonRedisSerializer;
import org.springframework.data.redis.serializer.StringRedisSerializer;

@Configuration
public class RedisConfig {

    @Bean
    public RedisTemplate<String, VideoJobRequest> videoJobRedisTemplate(
            RedisConnectionFactory connectionFactory, ObjectMapper objectMapper) {

        RedisTemplate<String, VideoJobRequest> template = new RedisTemplate<>();
        template.setConnectionFactory(connectionFactory);

        template.setKeySerializer(new StringRedisSerializer());

        Jackson2JsonRedisSerializer<VideoJobRequest> serializer =
                new Jackson2JsonRedisSerializer<>(objectMapper, VideoJobRequest.class);

        template.setValueSerializer(serializer);
        template.setHashValueSerializer(serializer);

        template.afterPropertiesSet();
        return template;
    }
}
