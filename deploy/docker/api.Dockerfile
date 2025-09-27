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

COPY mainservice/src mainservice/src
COPY mainservice/gradle mainservice/gradle
COPY mainservice/gradle/wrapper mainservice/gradle/wrapper
COPY mainservice/build.gradle mainservice/build.gradle

RUN --mount=type=cache,target=/home/gradle/.gradle \
    ./gradlew --no-daemon dependencies

COPY . .

RUN chmod +x gradlew

RUN --mount=type=cache,target=/home/gradle/.gradle \
    ./gradlew --no-daemon bootJar -x test

################################################################################
FROM eclipse-temurin:21-jre AS final

WORKDIR /app

COPY --from=build /build/worker/build/libs/*-SNAPSHOT.jar app.jar

USER app:app

EXPOSE 8080

ENV JAVA_OPTS="-Xms128m -Xmx256m"

ENTRYPOINT ["sh", "-c", "exec java $JAVA_OPTS -jar app.jar"]
