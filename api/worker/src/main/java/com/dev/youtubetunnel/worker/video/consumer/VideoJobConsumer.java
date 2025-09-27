package com.dev.youtubetunnel.worker.video.consumer;

import com.dev.youtubetunnel.common.kafka.dto.VideoJobRequest;
import io.minio.MinioClient;
import io.minio.StatObjectArgs;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.lang3.StringUtils;
import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.kafka.support.Acknowledgment;
import org.springframework.stereotype.Component;

@Slf4j
@Component
@RequiredArgsConstructor
public class VideoJobConsumer {

    private final MinioClient minioClient;
    private final KafkaTemplate<String, VideoJobRequest> kafkaTemplate;
    private final VideoJobConsumerProxy videoJobConsumerProxy;

    @KafkaListener(topics = "video-jobs", groupId = "worker-group")
    public void handleJob(VideoJobRequest request, Acknowledgment ack) {
        log.info("Received VideoJobRequest: {}", request);
        if (StringUtils.isEmpty(request.jobId()) || objectExists("videos", request.jobId() + "/playlist.m3u8")) {
            ack.acknowledge();
            return;
        }

        try {
            videoJobConsumerProxy.consumeData(request);
        } catch (Exception e) {
            log.error("Failed to process job {}: {}", request.jobId(), e.getMessage(), e);
            kafkaTemplate.send("video-jobs-dlt", request.jobId(), request);
        } finally {
            ack.acknowledge();
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
