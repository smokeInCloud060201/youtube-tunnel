package org.example.youtubetunnel.stream.services;

import org.example.youtubetunnel.stream.dtos.VideoQuality;

public interface YoutubeStreamService {

	String getAudioStreamUrl(String youtubeUrl, boolean showVideo, VideoQuality quality) throws Exception;

}
