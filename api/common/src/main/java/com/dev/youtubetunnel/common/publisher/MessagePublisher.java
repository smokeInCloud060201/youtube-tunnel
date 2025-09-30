package com.dev.youtubetunnel.common.publisher;

public interface MessagePublisher<T> {
    void publish(T message);
}
