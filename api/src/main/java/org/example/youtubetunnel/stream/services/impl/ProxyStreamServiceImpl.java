package org.example.youtubetunnel.stream.services.impl;

import jakarta.servlet.ServletOutputStream;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.Response;
import okhttp3.ResponseBody;
import org.example.youtubetunnel.stream.services.ProxyStreamService;
import org.springframework.http.HttpStatus;
import org.springframework.stereotype.Service;

import java.io.IOException;
import java.util.Optional;

@Service
public class ProxyStreamServiceImpl implements ProxyStreamService {

	private static final OkHttpClient client = new OkHttpClient.Builder().followRedirects(true)
		.followSslRedirects(true)
		.build();

	public void proxyWithRange(String upstreamUrl, HttpServletRequest request, HttpServletResponse response)
			throws IOException {
		String rangeHeader = request.getHeader("Range");

		String contentType = null;
		Long contentLength = null;
		Request headReq = new Request.Builder().url(upstreamUrl).head().build();
		try (Response headRes = client.newCall(headReq).execute()) {
			if (headRes.isSuccessful()) {
				contentType = headRes.header("Content-Type");
				contentLength = parseLong(headRes.header("Content-Length"));
			}
		}
		catch (Exception ignored) {
		}
		Request.Builder getBuilder = new Request.Builder().url(upstreamUrl);
		if (rangeHeader != null && !rangeHeader.isBlank()) {
			getBuilder.addHeader("Range", rangeHeader);
		}
		getBuilder.header("User-Agent", "Mozilla/5.0 (compatible; yt-audio-proxy/1.0)");

		try (Response upstream = client.newCall(getBuilder.build()).execute()) {
			int upstreamCode = upstream.code();
			ResponseBody body = upstream.body();
			if (body == null) {
				response.setStatus(HttpStatus.BAD_GATEWAY.value());
				return;
			}

			boolean partial = upstreamCode == 206 || (rangeHeader != null && upstream.header("Content-Range") != null);
			response.setStatus(partial ? HttpStatus.PARTIAL_CONTENT.value() : HttpStatus.OK.value());

			String finalType = Optional.ofNullable(upstream.header("Content-Type")).orElse(contentType);
			if (finalType == null) {
				finalType = "audio/mp4"; // many YouTube audio streams are mp4/aac or
											// webm/opus
			}
			response.setContentType(finalType);

			response.setHeader("Accept-Ranges", "bytes");

			String upstreamCL = upstream.header("Content-Length");
			if (upstreamCL != null) {
				response.setHeader("Content-Length", upstreamCL);
			}
			else if (contentLength != null && !partial) {
				response.setHeader("Content-Length", String.valueOf(contentLength));
			}

			String contentRange = upstream.header("Content-Range");
			if (contentRange != null) {
				response.setHeader("Content-Range", contentRange);
			}

			response.setHeader("Cache-Control", "no-store");

			try (ServletOutputStream out = response.getOutputStream()) {
				body.source().readAll(okio.Okio.sink(out));
				out.flush();
			}
		}
	}

	private static Long parseLong(String v) {
		try {
			return v == null ? null : Long.parseLong(v);
		}
		catch (Exception e) {
			return null;
		}
	}

}
