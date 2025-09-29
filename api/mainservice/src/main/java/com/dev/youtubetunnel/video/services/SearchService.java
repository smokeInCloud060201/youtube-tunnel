package com.dev.youtubetunnel.video.services;

import com.dev.youtubetunnel.video.dto.VideoSearchDTO;

import java.util.List;

public interface SearchService {
    List<VideoSearchDTO> searchVideo(String part, String type, String query, int maxResults);
}
