package org.example.youtubetunnel.search.controller;

import lombok.RequiredArgsConstructor;
import org.example.youtubetunnel.search.dto.VideoDTO;
import org.example.youtubetunnel.search.services.SearchService;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequiredArgsConstructor
@RequestMapping("/api/private/search/v1")
public class YoutubeController {

	private final SearchService searchService;

	@GetMapping
	public List<VideoDTO> getVideo(@RequestParam(name = "part", defaultValue = "snippet") String part,
			@RequestParam(name = "type", defaultValue = "video") String type, @RequestParam(name = "q") String query,
			@RequestParam(name = "maxResults", defaultValue = "50") int maxResults) {
		return searchService.searchVideo(part, type, query, maxResults);
	}

	@GetMapping("/credential")
	public String getCredential() {
		return searchService.getCredential();
	}

}
