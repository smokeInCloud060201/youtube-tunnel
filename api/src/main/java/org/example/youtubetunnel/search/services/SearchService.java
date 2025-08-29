package org.example.youtubetunnel.search.services;

import org.example.youtubetunnel.search.dto.VideoDTO;

import java.util.List;

public interface SearchService {

    List<VideoDTO> searchVideo(String part, String type, String query, int maxResults);

}
