package org.example.youtubetunnel.services;

import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;

import java.io.IOException;

public interface ProxyStreamService {

    void proxyWithRange(String upstreamUrl, HttpServletRequest request, HttpServletResponse response) throws IOException;
}
