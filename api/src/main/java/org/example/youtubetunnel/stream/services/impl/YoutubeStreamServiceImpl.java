package org.example.youtubetunnel.stream.services.impl;

import lombok.extern.slf4j.Slf4j;
import org.apache.commons.exec.CommandLine;
import org.apache.commons.exec.DefaultExecutor;
import org.apache.commons.exec.PumpStreamHandler;
import org.example.youtubetunnel.exceptions.ApplicationException;
import org.example.youtubetunnel.stream.services.YoutubeStreamService;
import org.springframework.stereotype.Service;

import java.io.ByteArrayOutputStream;

@Service
@Slf4j
public class YoutubeStreamServiceImpl implements YoutubeStreamService {

	public String getAudioStreamUrl(String youtubeUrl, boolean showVideo) throws Exception {

//		CommandLine cmd = new CommandLine("python");
//		cmd.addArgument("-m");
//		cmd.addArgument("yt_dlp");
//		cmd.addArgument("--no-playlist");
//		cmd.addArgument("-f");

		CommandLine cmd = new CommandLine("yt-dlp");
		cmd.addArgument("--no-playlist");
		cmd.addArgument("-f");

		if (showVideo) {
			// Progressive MP4 (contains both audio + video)
			cmd.addArgument("best[ext=mp4]");
		}
		else {
			// Audio only
			cmd.addArgument("bestaudio");
		}

		cmd.addArgument("-g");
		cmd.addArgument(youtubeUrl, false);

		ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
		DefaultExecutor executor = new DefaultExecutor();
		executor.setStreamHandler(new PumpStreamHandler(outputStream));
		int exitCode = executor.execute(cmd);
		log.info("#YoutubeStreamService.getAudioStreamUrl: URl {} isShowVideo: {} with code : {}", youtubeUrl, showVideo, exitCode);
		if (exitCode != 0) {
			throw new ApplicationException("yt-dlp failed with exit code: " + exitCode);
		}

		return outputStream.toString().trim().split("\\R")[0];
	}

}
