ARG RUST_VERSION=1.90.0
ARG APP_NAME=youtube-tunnel-worker
ARG FFMPEG_VERSION=6.1
ARG YT_DLP_VERSION=2025.09.26

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git pkgconfig openssl-dev openssl-libs-static
ENV OPENSSL_STATIC=1

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY shared shared
COPY service service
COPY web web

RUN cargo build --locked --release && cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.18 AS ffmpeg-build
ARG FFMPEG_VERSION
WORKDIR /build

RUN apk add --no-cache build-base yasm nasm wget tar \
    x264-dev x265-dev libvpx-dev fdk-aac-dev opus-dev freetype-dev libass-dev openssl-dev

COPY ./tools/ffmpeg-$FFMPEG_VERSION.tar.gz /build/
RUN tar xzf ffmpeg-$FFMPEG_VERSION.tar.gz && \
    cd ffmpeg-$FFMPEG_VERSION && \
    ./configure \
        --prefix=/usr/local \
        --enable-gpl \
        --enable-nonfree \
        --enable-openssl \
        --enable-libx264 \
        --enable-libx265 \
        --enable-libvpx \
        --enable-libfdk-aac \
        --enable-libopus \
        --enable-libass \
        --enable-libfreetype \
        --disable-debug \
        --disable-doc \
    && make -j$(nproc) && make install && strip /usr/local/bin/ffmpeg /usr/local/bin/ffprobe

FROM alpine:3.18 AS final
WORKDIR /app

ARG YT_DLP_VERSION=2025.10.22

RUN apk add --no-cache \
    python3 py3-pip libstdc++ libssl3 ca-certificates \
    libass freetype libvpx x264-libs x265-libs fdk-aac opus \
    libxcb libx11 libxext

ENV YT_DLP_VERSION=${YT_DLP_VERSION}
ENV VENV_PATH=/opt/venv
RUN python3 -m venv $VENV_PATH && \
    $VENV_PATH/bin/pip install --no-cache-dir yt-dlp==$YT_DLP_VERSION && \
    ln -s $VENV_PATH/bin/yt-dlp /usr/local/bin/yt-dlp && \
    ln -s $VENV_PATH/bin/python3 /usr/local/bin/python3

COPY --from=build /bin/server /bin/server
COPY --from=ffmpeg-build /usr/local/bin/ffmpeg /usr/local/bin/ffmpeg
COPY --from=ffmpeg-build /usr/local/bin/ffprobe /usr/local/bin/ffprobe

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN mkdir -p /app/downloads && chown -R appuser:appuser /app

USER appuser
EXPOSE 8081
CMD ["/bin/server"]
