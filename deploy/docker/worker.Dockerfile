################################################################################
FROM gradle:8.10.2-jdk21 AS build

WORKDIR /build

RUN apt-get update && apt-get install -y dos2unix && rm -rf /var/lib/apt/lists/*

COPY --chmod=0755 gradlew gradlew
RUN chmod +x gradlew && dos2unix ./gradlew

COPY gradle gradle
COPY gradle/wrapper gradle/wrapper
COPY build.gradle settings.gradle ./
COPY common/src common/src
COPY common/build.gradle common/build.gradle

COPY worker/src worker/src
COPY worker/gradle worker/gradle
COPY worker/gradle/wrapper worker/gradle/wrapper
COPY worker/build.gradle worker/build.gradle

RUN --mount=type=cache,target=/home/gradle/.gradle \
    ./gradlew --no-daemon dependencies

COPY . .

RUN chmod +x gradlew && dos2unix ./gradlew

RUN --mount=type=cache,target=/home/gradle/.gradle \
    ./gradlew --no-daemon bootJar -x test

################################################################################
FROM eclipse-temurin:21-jre AS final

WORKDIR /app

COPY --from=build /build/worker/build/libs/*-SNAPSHOT.jar app.jar

RUN apt-get update && apt-get install -y \
        python3 python3-pip python3-venv bash wget tar \
        autoconf automake build-essential cmake git \
        pkg-config yasm nasm libx264-dev libx265-dev \
        libvpx-dev libfdk-aac-dev libopus-dev libass-dev \
        libfreetype6-dev libvorbis-dev libmp3lame-dev \
        libssl-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*


ENV YT_DLP_VERSION=2025.09.26

RUN python3 -m venv /opt/venv \
    && /opt/venv/bin/pip install --no-cache-dir yt-dlp==$YT_DLP_VERSION \
    && ln -s /opt/venv/bin/yt-dlp /usr/local/bin/yt-dlp \
    && ln -s /opt/venv/bin/python3 /usr/local/bin/python3

ENV FFMPEG_VERSION=6.1
RUN mkdir -p /opt/ffmpeg \
    && cd /opt/ffmpeg \
    && wget https://ffmpeg.org/releases/ffmpeg-$FFMPEG_VERSION.tar.gz \
    && tar xzf ffmpeg-$FFMPEG_VERSION.tar.gz \
    && cd ffmpeg-$FFMPEG_VERSION \
    && ./configure \
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
    && make -j$(nproc) \
    && make install \
    && make clean \
    && rm -rf /opt/ffmpeg

USER 1000

EXPOSE 8081

ENV JAVA_OPTS="-Xms128m -Xmx256m"

ENTRYPOINT ["sh", "-c", "exec java $JAVA_OPTS -jar app.jar"]
