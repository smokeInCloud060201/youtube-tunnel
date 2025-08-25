package org.example.youtubetunnel.services.impl;

import lombok.extern.slf4j.Slf4j;
import org.apache.commons.exec.CommandLine;
import org.apache.commons.exec.DefaultExecutor;
import org.apache.commons.exec.PumpStreamHandler;
import org.example.youtubetunnel.services.YoutubeService;
import org.springframework.stereotype.Service;

import java.io.ByteArrayOutputStream;

@Service
@Slf4j
public class YoutubeServiceImpl implements YoutubeService {

    public String getAudioStreamUrl(String youtubeUrl) throws Exception {

        CommandLine cmd = new CommandLine("python");
        cmd.addArgument("-m");
        cmd.addArgument("yt_dlp");
        cmd.addArgument("-f");
        cmd.addArgument("bestaudio");
        cmd.addArgument("-g");
        cmd.addArgument(youtubeUrl, false);


        ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
        DefaultExecutor executor = new DefaultExecutor();
        executor.setStreamHandler(new PumpStreamHandler(outputStream));
        int exitCode = executor.execute(cmd);
        if (exitCode != 0) {
            throw new IllegalStateException("yt-dlp failed with exit code: " + exitCode);
        }
        String result = outputStream.toString().trim();
        String first = result.contains("\n") ? result.split("\n")[0].trim() : result;
        if (first.isEmpty()) {
            throw new IllegalStateException("Empty URL returned by yt-dlp");
        }
        return first;
    }
}
