package org.example.youtubetunnel.stream.services.impl;

import jakarta.servlet.http.HttpServletResponse;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.exec.CommandLine;
import org.apache.commons.exec.DefaultExecutor;
import org.apache.commons.exec.PumpStreamHandler;
import org.example.youtubetunnel.exceptions.ApplicationException;
import org.example.youtubetunnel.stream.dtos.VideoQuality;
import org.example.youtubetunnel.stream.services.YoutubeStreamService;
import org.springframework.stereotype.Service;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

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

    @Override
    public void streamHls(String youtubeUrl, String quality, HttpServletResponse response) throws ApplicationException, IOException {
        String format = String.format("bestvideo[height<=%s]+bestaudio/best", quality.replace("p", ""));

        // Build command
        String[] cmd = {
                "bash", "-c",
                String.format("yt-dlp -f \"%s\" -o - \"%s\" | " +
                        "ffmpeg -i pipe:0 -c:v copy -c:a copy -f hls -hls_time 6 -hls_list_size 0 -", format, youtubeUrl)
        };

        ProcessBuilder pb = new ProcessBuilder(cmd);
        Process process = pb.start();

        // Send headers
        response.setContentType("application/vnd.apple.mpegurl");

        try (InputStream in = process.getInputStream();
             OutputStream out = response.getOutputStream()) {
            byte[] buffer = new byte[8192];
            int len;
            while ((len = in.read(buffer)) != -1) {
                out.write(buffer, 0, len);
                out.flush();
            }
        } catch (Exception e) {
            log.error("Error while streaming HLS", e);
            throw new ApplicationException("Error while streaming HLS");
        }
    }

}
