package org.example.youtubetunnel.stream.controller;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.example.youtubetunnel.stream.dtos.VideoQuality;
import org.example.youtubetunnel.stream.services.ProxyStreamService;
import org.example.youtubetunnel.stream.services.YoutubeStreamService;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequiredArgsConstructor
@Slf4j
@RequestMapping("/api/stream")
public class StreamController {

	private final YoutubeStreamService youtubeStreamService;

	private final ProxyStreamService proxyStreamService;

	@GetMapping(produces = MediaType.APPLICATION_OCTET_STREAM_VALUE)
	public void streamAudio(@RequestParam(name = "enableVideo", defaultValue = "true") boolean enableVideo,
			@RequestParam("url") String youtubeUrl,
			@RequestParam(name = "quality", defaultValue = "1080p") VideoQuality quality, HttpServletRequest request,
			HttpServletResponse response) {
		try {
			log.info("#StreamController.streamAudio(enableVideo={}, url={})", enableVideo, youtubeUrl);
			String audioUrl = youtubeStreamService.getAudioStreamUrl(youtubeUrl, enableVideo, quality);
			proxyStreamService.proxyWithRange(audioUrl, request, response);
		}
		catch (Exception e) {
			response.setStatus(500);
		}
	}

}
