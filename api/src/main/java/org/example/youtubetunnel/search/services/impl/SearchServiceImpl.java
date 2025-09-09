package org.example.youtubetunnel.search.services.impl;

import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.example.youtubetunnel.config.ConfigProperties;
import org.example.youtubetunnel.search.dto.SearchVideoResponseDTO;
import org.example.youtubetunnel.search.dto.VideoDTO;
import org.example.youtubetunnel.search.services.SearchService;
import org.example.youtubetunnel.utils.WebClientUtil;
import org.springframework.stereotype.Service;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.Base64;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import java.util.stream.Collectors;

@Service
@RequiredArgsConstructor
@Slf4j
public class SearchServiceImpl implements SearchService {

	private final ConfigProperties configProperties;

	private final WebClientUtil webClientUtil;

	private static final String VIDEO_URL = "/youtube/v3/search";

	@Override
	public List<VideoDTO> searchVideo(String part, String type, String query, int maxResults) {

		Map<String, String> queryParams = new HashMap<>();
		queryParams.put("part", part);
		queryParams.put("type", type);
		queryParams.put("q", query);
		queryParams.put("maxResults", String.valueOf(maxResults));
		queryParams.put("key", configProperties.key());

		String uri = configProperties.host() + VIDEO_URL + "?"
				+ queryParams.entrySet()
					.stream()
					.map(e -> e.getKey() + "=" + URLEncoder.encode(e.getValue(), StandardCharsets.UTF_8))
					.collect(Collectors.joining("&"));

		SearchVideoResponseDTO searchVideoResponseDTO = webClientUtil.get(uri, SearchVideoResponseDTO.class).join();

		return searchVideoResponseDTO.getItems().stream().map(this::mapItemToVideoDTO).toList();
	}

	@Override
	public String getCredential() {
		return configProperties.key();
	}

	private VideoDTO mapItemToVideoDTO(SearchVideoResponseDTO.Item item) {
		if (item == null) {
			return null;
		}

		SearchVideoResponseDTO.Thumbnails thumbnails = item.getSnippet().getThumbnails();

		CompletableFuture<String> defaultThumb = Optional.ofNullable(thumbnails.getDefaultThumbnail())
			.map(t -> webClientUtil.getFullUrl(t.getUrl(), byte[].class)
				.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes)))
			.orElse(CompletableFuture.completedFuture(null));

		CompletableFuture<String> mediumThumb = Optional.ofNullable(thumbnails.getMedium())
			.map(t -> webClientUtil.getFullUrl(t.getUrl(), byte[].class)
				.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes)))
			.orElse(CompletableFuture.completedFuture(null));

		CompletableFuture<String> highThumb = Optional.ofNullable(thumbnails.getHigh())
			.map(t -> webClientUtil.getFullUrl(t.getUrl(), byte[].class)
				.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes)))
			.orElse(CompletableFuture.completedFuture(null));

		String bestThumb = CompletableFuture.allOf(defaultThumb, mediumThumb, highThumb).thenApply(v -> {
			if (defaultThumb.join() != null)
				return defaultThumb.join();
			if (mediumThumb.join() != null)
				return mediumThumb.join();
			return highThumb.join();
		}).join();

		return VideoDTO.builder()
			.publishTime(item.getSnippet().getPublishTime())
			.publishedAt(item.getSnippet().getPublishedAt())
			.title(item.getSnippet().getTitle())
			.id(item.getId().getVideoId())
			.description(item.getSnippet().getDescription())
			.channelTitle(item.getSnippet().getChannelTitle())
			.thumbnails(List.of(bestThumb))
			.build();
	}

}
