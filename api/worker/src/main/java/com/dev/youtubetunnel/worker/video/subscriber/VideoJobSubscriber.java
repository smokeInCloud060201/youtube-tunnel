package com.dev.youtubetunnel.worker.video.subscriber;

import com.dev.youtubetunnel.common.dto.VideoJobRequest;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.minio.MinioClient;
import io.minio.StatObjectArgs;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.lang3.StringUtils;
import org.springframework.data.redis.connection.Message;
import org.springframework.data.redis.connection.MessageListener;
import org.springframework.stereotype.Component;

import java.io.IOException;

@Slf4j
@Component
@RequiredArgsConstructor
public class VideoJobSubscriber implements MessageListener {

    private final MinioClient minioClient;
    private final VideoJobSubscriberProxy videoJobSubscriberProxy;
    private final ObjectMapper objectMapper;

    @Override
    public void onMessage(Message message, byte[] pattern) {
        String body = new String(message.getBody());
        try {
            VideoJobRequest videoJobMessage = objectMapper.readValue(body, VideoJobRequest.class);
            log.info("Received VideoJobRequest: {}", videoJobMessage);

            if (StringUtils.isEmpty(videoJobMessage.jobId()) || objectExists("videos", videoJobMessage.jobId() + "/playlist.m3u8")) {
                log.info("Job is invalid or already exists {}", videoJobMessage.jobId());
                return;
            }

            videoJobSubscriberProxy.consumeData(videoJobMessage);
        } catch (IOException | InterruptedException e) {
            log.error(e.getMessage(), e);
            throw new RuntimeException(e);
        }
    }


    private boolean objectExists(String bucket, String object) {
        try {
            minioClient.statObject(
                    StatObjectArgs.builder()
                            .bucket(bucket)
                            .object(object)
                            .build()
            );
            return true;
        } catch (Exception e) {
            return false;
        }
    }
}
