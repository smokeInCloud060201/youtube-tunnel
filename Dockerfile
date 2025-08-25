################################################################################
# Stage 1: Build the Spring Boot application with Gradle
FROM gradle:8.10.2-jdk21 AS build

WORKDIR /build

# Install dos2unix for Windows gradle wrapper files
RUN apt-get update && apt-get install -y dos2unix && rm -rf /var/lib/apt/lists/*

COPY --chmod=0755 gradlew gradlew
RUN dos2unix ./gradlew

# Copy Gradle wrapper and config first (to leverage cache)
COPY gradle gradle
COPY gradle/wrapper gradle/wrapper
COPY build.gradle settings.gradle ./

# Download Gradle dependencies (cached between builds)
RUN --mount=type=cache,target=/home/gradle/.gradle \
    ./gradlew --no-daemon dependencies

# Copy source code
COPY . .

# Build the fat jar (skip tests for speed)
RUN --mount=type=cache,target=/home/gradle/.gradle \
    ./gradlew --no-daemon bootJar -x test

################################################################################
# Stage 2: Minimal runtime with JRE + Python + yt-dlp + ffmpeg
FROM eclipse-temurin:21-jre-alpine AS final

WORKDIR /app

# Copy Spring Boot jar
COPY --from=build /build/build/libs/*-SNAPSHOT.jar app.jar

# Install Python, pip, ffmpeg, bash
RUN apk add --no-cache python3 py3-pip ffmpeg bash \
    && python3 -m venv /opt/venv \
    && /opt/venv/bin/pip install --no-cache-dir yt-dlp \
    && ln -s /opt/venv/bin/yt-dlp /usr/local/bin/yt-dlp \
    && ln -s /opt/venv/bin/python3 /usr/local/bin/python3 \
    && addgroup -S app && adduser -S app -G app

# Switch to non-root user
USER app:app

# Expose port
EXPOSE 8001

# Java memory options
ENV JAVA_OPTS="-Xms128m -Xmx256m"

# Entry point
ENTRYPOINT ["sh", "-c", "exec java $JAVA_OPTS -jar app.jar"]
