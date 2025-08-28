package org.example.youtubetunnel.youtube.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/private/search/v1")
public class YoutubeController {

	@GetMapping()
	public String getVideo() {
		return "Youtube";
	}

}
