package org.example.youtubetunnel.stream.dtos;

import lombok.AllArgsConstructor;
import lombok.Getter;

@Getter
@AllArgsConstructor
public enum VideoQuality {

	P144("144p"), P240("240p"), P360("360p"), P480("480p"), P720("720p"), P1080("1080p"), P_BEST("best");

	private final String code;

	public static VideoQuality fromCode(String code) {
		for (VideoQuality q : values()) {
			if (q.code.equalsIgnoreCase(code)) {
				return q;
			}
		}
		throw new IllegalArgumentException("Unknown video quality: " + code);
	}

}
