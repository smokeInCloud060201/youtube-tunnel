package com.dev.youtubetunnel.video.controllers;

import com.dev.youtubetunnel.video.dto.VideoPlayerResponse;
import com.dev.youtubetunnel.video.services.VideoPlayerService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@Slf4j
@RestController
@RequestMapping("/api/v1/video-player")
@CrossOrigin
@RequiredArgsConstructor
public class VideoPlayerController {

    private final VideoPlayerService videoPlayerService;

    @PostMapping
    public ResponseEntity<VideoPlayerResponse> requestVideo(@RequestParam String youtubeUrl) {
        final String jobId = videoPlayerService.submitVideoJob(youtubeUrl);
        return ResponseEntity.ok(new VideoPlayerResponse(jobId));
    }

    @GetMapping("/{jobId}/playlist")
    public ResponseEntity<String> getPlaylist(@PathVariable String jobId) throws Exception {
        final String updatedPlaylist = videoPlayerService.getPlayList(jobId);
        return ResponseEntity.ok()
                .header("Content-Type", "application/vnd.apple.mpegurl")
                .body(updatedPlaylist);
    }
}
