package org.example.youtubetunnel.stream.services;

public interface YoutubeStreamService {

	String getAudioStreamUrl(String youtubeUrl, boolean showVideo) throws Exception;

}
