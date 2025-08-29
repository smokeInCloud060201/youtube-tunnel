package org.example.youtubetunnel.stream.services.impl;

import lombok.extern.slf4j.Slf4j;
import org.apache.commons.exec.CommandLine;
import org.apache.commons.exec.DefaultExecutor;
import org.apache.commons.exec.PumpStreamHandler;
import org.example.youtubetunnel.exceptions.ApplicationException;
import org.example.youtubetunnel.stream.dtos.VideoQuality;
import org.example.youtubetunnel.stream.services.YoutubeStreamService;
import org.springframework.stereotype.Service;

import java.io.ByteArrayOutputStream;

@Service
@Slf4j
public class YoutubeStreamServiceImpl implements YoutubeStreamService {

	public String getAudioStreamUrl(String youtubeUrl, boolean showVideo, VideoQuality quality) throws Exception {

		// CommandLine cmd = new CommandLine("python");
		// cmd.addArgument("-m");
		// cmd.addArgument("yt_dlp");
		// cmd.addArgument("--no-playlist");
		// cmd.addArgument("-f");

		CommandLine cmd = new CommandLine("yt-dlp");
		cmd.addArgument("--no-playlist");
		cmd.addArgument("-f");

		if (showVideo) {
			if (VideoQuality.P_BEST.equals(quality)) {
                cmd.addArgument("best");
			}
			else {
				// Limit video height to requested quality
                String height = quality.getCode().replace("p", "");
                cmd.addArgument(String.format("best[height<=%s]", height));
			}
		}
		else {
			cmd.addArgument("bestaudio");
		}

		cmd.addArgument("-g");
		cmd.addArgument(youtubeUrl, false);

		ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
		DefaultExecutor executor = new DefaultExecutor();
		executor.setStreamHandler(new PumpStreamHandler(outputStream));
		int exitCode = executor.execute(cmd);
		log.info("#YoutubeStreamService.getAudioStreamUrl: URL {} showVideo={} quality={} exitCode={}", youtubeUrl,
				showVideo, quality, exitCode);
		if (exitCode != 0) {
			throw new ApplicationException("yt-dlp failed with exit code: " + exitCode);
		}

		return outputStream.toString().trim().split("\\R")[0];
	}

}
