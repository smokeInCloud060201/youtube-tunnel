package org.example.youtubetunnel.search.services.impl;

import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.example.youtubetunnel.config.ConfigProperties;
import org.example.youtubetunnel.search.dto.SearchVideoResponseDTO;
import org.example.youtubetunnel.search.dto.VideoDTO;
import org.example.youtubetunnel.search.services.SearchService;
import org.example.youtubetunnel.utils.WebClientUtil;
import org.springframework.stereotype.Service;

import java.util.Base64;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

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
		queryParams.put("query", query);
		queryParams.put("maxResults", String.valueOf(maxResults));
		queryParams.put("key", configProperties.key());

		 SearchVideoResponseDTO searchVideoResponseDTO = webClientUtil.get(VIDEO_URL,
		 queryParams, Collections.emptyMap(), SearchVideoResponseDTO.class).join();

		return searchVideoResponseDTO.getItems().stream().map(this::mapItemToVideoDTO).toList();
	}

	private VideoDTO mapItemToVideoDTO(SearchVideoResponseDTO.Item item) {
		if (item == null) {
			return null;
		}

		SearchVideoResponseDTO.Thumbnails thumbnails = item.getSnippet().getThumbnails();

		CompletableFuture<String> defaultThumb = webClientUtil
			.getFullUrl(thumbnails.getDefaultThumbnail().getUrl(), byte[].class)
			.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes));

		CompletableFuture<String> mediumThumb = webClientUtil.getFullUrl(thumbnails.getMedium().getUrl(), byte[].class)
			.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes));

		CompletableFuture<String> highThumb = webClientUtil.getFullUrl(thumbnails.getHigh().getUrl(), byte[].class)
			.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes));

		List<String> base64Thumbnails = CompletableFuture.allOf(defaultThumb, mediumThumb, highThumb)
			.thenApply(v -> List.of(defaultThumb.join(), mediumThumb.join(), highThumb.join()))
			.join();

		return VideoDTO.builder()
			.publishTime(item.getSnippet().getPublishTime())
			.publishedAt(item.getSnippet().getPublishedAt())
			.title(item.getSnippet().getTitle())
			.id(item.getId().getVideoId())
			.description(item.getSnippet().getDescription())
			.channelTitle(item.getSnippet().getChannelTitle())
			.thumbnails(base64Thumbnails)
			.build();
	}

}
