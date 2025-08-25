package org.example.youtubetunnel.controller;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.RequiredArgsConstructor;
import org.example.youtubetunnel.services.ProxyStreamService;
import org.example.youtubetunnel.services.YoutubeService;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequiredArgsConstructor
@RequestMapping("/api/stream")
public class StreamController {

    private final YoutubeService youtubeService;
    private final ProxyStreamService proxyStreamService;


    @GetMapping(value = "/audio", produces = MediaType.APPLICATION_OCTET_STREAM_VALUE)
    public void streamAudio(@RequestParam("url") String youtubeUrl,
                            HttpServletRequest request,
                            HttpServletResponse response) {
        try {
            String audioUrl = youtubeService.getAudioStreamUrl(youtubeUrl);
            proxyStreamService.proxyWithRange(audioUrl, request, response);
        } catch (Exception e) {
            response.setStatus(500);
        }
    }
}
