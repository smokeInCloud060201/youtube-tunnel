package com.dev.youtubetunnel.utils;

import com.dev.youtubetunnel.exception.ApplicationException;
import io.netty.handler.timeout.TimeoutException;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatusCode;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Component;
import org.springframework.util.MultiValueMap;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.ClientResponse;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.server.ServerErrorException;
import reactor.core.publisher.Mono;
import reactor.util.retry.Retry;

import java.net.ConnectException;
import java.time.Duration;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

@Component
@RequiredArgsConstructor
@Slf4j
public class WebClientUtil {

    private final WebClient webClient;

    private static final Duration REQUEST_TIMEOUT = Duration.ofSeconds(10);

    private static final Retry RETRY_POLICY = Retry.backoff(3, Duration.ofMillis(500))
            .jitter(0.5)
            .maxBackoff(Duration.ofSeconds(5))
            .filter(throwable -> throwable instanceof TimeoutException
                    || throwable instanceof java.util.concurrent.TimeoutException || throwable instanceof ConnectException
                    || throwable instanceof ServerErrorException
                    || (throwable.getMessage() != null && throwable.getMessage().toLowerCase().contains("timed out")))
            .doBeforeRetry(retrySignal -> log.info("Retry attempt {} due to {}", retrySignal.totalRetriesInARow() + 1,
                    retrySignal.failure().getClass().getSimpleName()));

    private WebClient.ResponseSpec handleErrors(WebClient.RequestHeadersSpec<?> request) {
        return request.retrieve()
                .onStatus(HttpStatusCode::is4xxClientError, this::mapClientError)
                .onStatus(HttpStatusCode::is5xxServerError, this::mapServerError);
    }

    private Mono<Throwable> mapClientError(ClientResponse response) {
        return response.bodyToMono(String.class)
                .flatMap(body -> Mono.error(new RuntimeException("Client Error: " + body)));
    }

    private Mono<Throwable> mapServerError(ClientResponse response) {
        return response.bodyToMono(String.class)
                .flatMap(body -> Mono.error(new ApplicationException("Server Error: " + body)));
    }

    private <R> CompletableFuture<R> execute(Mono<R> mono, String uri) {
        return mono.timeout(REQUEST_TIMEOUT).retryWhen(RETRY_POLICY).doOnError(throwable -> {
            if (!(throwable instanceof ApplicationException)) {
                log.error("Unexpected error on [{}]: {}", uri, throwable.toString(), throwable);
            }
        }).onErrorResume(TimeoutException.class, ex -> {
            log.warn("Timeout while calling [{}]: {}", uri, ex.getMessage());
            return Mono.error(new ApplicationException("External service timed out"));
        }).onErrorMap(throwable -> {
            if (throwable instanceof ApplicationException)
                return throwable;
            return new ApplicationException(throwable.getMessage());
        }).toFuture();
    }

    private <T extends WebClient.RequestHeadersSpec<?>> T applyHeaders(T spec, Map<String, String> headers) {
        if (headers != null) {
            headers.forEach(spec::header);
        }
        return spec;
    }

    public <R> CompletableFuture<R> get(String uri, Map<String, String> headers, Class<R> responseType) {
        WebClient.RequestHeadersSpec<?> request = applyHeaders(webClient.get().uri(uri), headers);

        return execute(handleErrors(request).bodyToMono(responseType), uri);
    }

    public <R> CompletableFuture<R> get(String uri, Class<R> responseType) {
        return get(uri, null, responseType);
    }

    public <R> CompletableFuture<R> getFullUrl(String url, Class<R> responseType) {
        return execute(webClient.get()
                .uri(url) // pass full URL directly
                .retrieve()
                .bodyToMono(responseType), url);
    }

}