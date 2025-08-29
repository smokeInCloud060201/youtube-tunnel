package org.example.youtubetunnel.search.dto;

import lombok.Builder;
import lombok.Getter;

import java.util.List;

@Getter
@Builder
public class VideoDTO {
    private String id;
    private String title;
    private String description;
    private List<String> thumbnails;
    private String channelTitle;
    private String publishTime;
    private String publishedAt;
}
