package org.example.youtubetunnel.utils;

import lombok.experimental.UtilityClass;

import java.util.Optional;

@UtilityClass
public class RangeUtils {

    public static Optional<String> normalizeSingleRange(String rangeHeader) {
        if (rangeHeader == null || rangeHeader.isBlank()) return Optional.empty();
        String r = rangeHeader.trim().toLowerCase();
        if (!r.startsWith("bytes=")) return Optional.empty();
        return Optional.of(r);
    }
}
