package com.dev.youtubetunnel.video.controllers;

import com.dev.youtubetunnel.video.services.VideoService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequiredArgsConstructor
@RequestMapping("/v1/video")
public class VideoController {

    private final VideoService videoService;


    @DeleteMapping("/clean")
    public String postMapping() {
       return videoService.cleanStorage();
    }
}
