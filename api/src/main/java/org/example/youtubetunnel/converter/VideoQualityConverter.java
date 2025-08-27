package org.example.youtubetunnel.converter;

import org.example.youtubetunnel.stream.dtos.VideoQuality;
import org.jetbrains.annotations.NotNull;
import org.springframework.core.convert.converter.Converter;
import org.springframework.stereotype.Component;

@Component
public class VideoQualityConverter implements Converter<String, VideoQuality> {

	@Override
	public VideoQuality convert(@NotNull String source) {
		return VideoQuality.fromCode(source);
	}

}