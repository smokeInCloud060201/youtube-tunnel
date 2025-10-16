package com.dev.youtubetunnel.video.services.impl;

import com.dev.youtubetunnel.video.services.VideoService;
import io.minio.ListObjectsArgs;
import io.minio.MinioClient;
import io.minio.RemoveObjectsArgs;
import io.minio.Result;
import io.minio.messages.DeleteError;
import io.minio.messages.DeleteObject;
import io.minio.messages.Item;
import lombok.RequiredArgsConstructor;
import lombok.SneakyThrows;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.stream.StreamSupport;

@Service
@RequiredArgsConstructor
@Slf4j
public class VideoServiceImpl implements VideoService {

    private final MinioClient minioClient;

    @Override
    @SneakyThrows
    public String cleanStorage() {
        String bucketName = "videos";

        Iterable<Result<Item>> results = minioClient.listObjects(
                ListObjectsArgs.builder()
                        .bucket(bucketName)
                        .recursive(true)
                        .build()
        );

        List<DeleteObject> toDelete = StreamSupport.stream(results.spliterator(), false)
                .map(r -> {
                    try {
                        return new DeleteObject(r.get().objectName());
                    } catch (Exception e) {
                        log.warn("Error listing object: {}", e.getMessage());
                        return null;
                    }
                })
                .filter(Objects::nonNull)
                .toList();

        if (toDelete.isEmpty()) {
            log.info("Bucket '{}' is already empty.", bucketName);
            return "No objects to delete.";
        }

        log.info("Deleting {} objects from bucket '{}'", toDelete.size(), bucketName);

        Iterable<Result<DeleteError>> errors = minioClient.removeObjects(
                RemoveObjectsArgs.builder()
                        .bucket(bucketName)
                        .objects(toDelete)
                        .build()
        );

        List<String> failed = new ArrayList<>();
        for (Result<DeleteError> errorResult : errors) {
            try {
                DeleteError error = errorResult.get();
                failed.add(error.objectName() + " -> " + error.message());
            } catch (Exception e) {
                log.warn("Error deleting object: {}", e.getMessage());
            }
        }

        if (failed.isEmpty()) {
            log.info("Successfully deleted all {} objects from '{}'", toDelete.size(), bucketName);
            return String.format("Successfully deleted %d objects", toDelete.size());
        } else {
            log.error("Failed to delete {} objects: {}", failed.size(), failed);
            return String.format("Deleted %d objects, but %d failed: %s",
                    toDelete.size() - failed.size(), failed.size(), failed);
        }
    }
}
