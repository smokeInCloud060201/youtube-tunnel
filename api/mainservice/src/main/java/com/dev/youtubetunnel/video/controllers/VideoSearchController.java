package com.dev.youtubetunnel.video.controllers;


import com.dev.youtubetunnel.video.dto.VideoSearchDTO;
import com.dev.youtubetunnel.video.services.SearchService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequiredArgsConstructor
@RequestMapping("/v1/search")
public class VideoSearchController {

    private final SearchService searchService;

    @GetMapping
    public List<VideoSearchDTO> getVideo(@RequestParam(name = "part", defaultValue = "snippet") String part,
                                         @RequestParam(name = "type", defaultValue = "video") String type, @RequestParam(name = "q") String query,
                                         @RequestParam(name = "maxResults", defaultValue = "50") int maxResults) {
        return searchService.searchVideo(part, type, query, maxResults);
    }
}
