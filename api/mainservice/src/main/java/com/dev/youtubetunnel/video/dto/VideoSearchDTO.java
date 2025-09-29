package com.dev.youtubetunnel.video.dto;

import lombok.Builder;
import lombok.Getter;

import java.util.List;

@Getter
@Builder
public class VideoSearchDTO {

    private String id;

    private String title;

    private String description;

    private List<String> thumbnails;

    private String channelTitle;

    private String publishTime;

    private String publishedAt;
}
