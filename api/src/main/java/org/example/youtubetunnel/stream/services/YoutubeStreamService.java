package org.example.youtubetunnel.stream.services;

import jakarta.servlet.http.HttpServletResponse;
import org.example.youtubetunnel.exceptions.ApplicationException;
import org.example.youtubetunnel.stream.dtos.VideoQuality;

import java.io.IOException;

public interface YoutubeStreamService {

	String getAudioStreamUrl(String youtubeUrl, boolean showVideo, VideoQuality quality) throws Exception;

    void streamHls(String youtubeUrl, String quality, HttpServletResponse response) throws ApplicationException, IOException;
}
