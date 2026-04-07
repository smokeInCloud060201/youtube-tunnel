# Architecture – Worker Service

## Executive Summary

The `worker` service is a **background Rust process** responsible for the heavy lifting of the YouTube Tunnel platform: downloading YouTube videos with `yt-dlp`, transcoding them to HLS format with `ffmpeg`, and uploading the resulting segments progressively to MinIO while tracking job state in Redis.

It is designed as a **concurrent job consumer**: 4 parallel Tokio workers process jobs simultaneously, each blocking on the Redis `job-queue` list.

---

## Technology Stack

| Category | Technology | Version |
|----------|-----------|---------|
| Language | Rust | 2024 edition |
| Async Runtime | Tokio | 1.48 (full features) |
| Web Framework | Actix-web | 4.11 (lightweight HTTP for health check) |
| Queue (consumer) | Redis via deadpool-redis | 0.22 |
| Object Storage | MinIO via aws-sdk-s3 | 1.108 |
| Serialization | Serde + serde_json | 1.x |
| Error Handling | anyhow | 1.x |
| Video Download | yt-dlp | 2025.09.26 |
| Transcoding | FFmpeg | 6.1 (custom-built with x264, x265, libfdk-aac) |
| Logging | tracing + tracing-subscriber | 0.1 / 0.3 |
| Env Config | dotenvy | 0.15 |

---

## Architecture Pattern

**Producer-Consumer with Pipeline Streaming:**

```
Redis job-queue (BRPOP, blocking)
       │
       ▼
 JobConsumer (4 parallel Tokio workers)
       │
       ├── Fetch YouTube cookie from MinIO
       │
       ├── Spawn yt-dlp process (stdout pipe)
       │        │
       │        ▼ (piped stdout → stdin)
       ├── Spawn ffmpeg process (HLS output to tmp dir)
       │        │
       │        ▼ (async file watch)
       └── Upload loop (tokio::spawn):
               - Watch tmp dir for .ts segments
               - Upload each stable segment to MinIO
               - Upload updated playlist.m3u8 periodically
               - Update Redis progress key
       │
       ▼
  Final upload: remaining .ts + final playlist.m3u8
  Cleanup: tmp dir
  Set status: "completed" / "failed"
```

---

## Module Structure

```
src/
├── main.rs                  # Calls server::start()
├── server.rs                # Bootstrap:
│                            #   - Init Redis pool (with retry loop)
│                            #   - Init MinIO client (with retry loop)
│                            #   - Init YoutubeCookie credential helper
│                            #   - Spawn JobConsumer in background tokio::spawn
│                            #   - Start HTTP server (health check)
├── config/
│   ├── logger.rs            # tracing_subscriber setup
│   ├── minio.rs             # aws-sdk-s3 Client factory
│   ├── redis_pool.rs        # deadpool-redis Pool factory
│   └── cookie_credential.rs # YoutubeCookie: fetches cookie.txt from MinIO on demand
├── model/
│   └── video_player.rs      # VideoJob { job_id, video_url } – deserialized from Redis queue
└── services/
    └── job_consumer.rs      # Core consumer – see breakdown below
```

---

## JobConsumer Deep Dive (`services/job_consumer.rs`)

### Concurrency Model

```rust
pub async fn start(self: Arc<Self>) -> anyhow::Result<()> {
    for i in 0..4 {
        let worker = self.clone();
        tokio::spawn(async move { worker.run_loop().await });
    }
}
```

Spawns **4 independent async tasks**, each with its own Redis connection doing `BRPOP job-queue 0` (blocking indefinitely).

### Job Lifecycle

```
run_loop()
  └── BRPOP "job-queue" (blocks)
      └── handle_job(VideoJob)
          ├── Check if already completed (MinIO object_exists check)
          ├── set_job_status("processing")
          ├── set_job_progress(0.0)
          └── consume_data(&job)
              ├── Create temp dir /tmp/video-{job_id}/
              ├── Fetch cookie from MinIO → write to temp dir
              ├── Spawn yt-dlp (stdout pipe, format: bv*[vcodec^=avc1]+ba/b)
              ├── Spawn ffmpeg (stdin = yt-dlp stdout, output = HLS files in temp dir)
              ├── tokio::spawn: upload loop
              │   ├── Poll temp dir every 1 second
              │   ├── For each .ts segment:
              │   │   - Wait for file stability (size unchanged after 100-300ms)
              │   │   - Upload to MinIO: yt-videos/{job_id}/{segment}.ts
              │   │   - Update progress: (segments_count * 0.05).min(0.95)
              │   │   - Upload updated playlist.m3u8
              │   │   - Delete local .ts after upload
              │   └── For playlist.m3u8:
              │       - Upload every 5s or when new segment uploaded
              ├── Await yt-dlp exit
              ├── Await ffmpeg exit
              ├── Final upload: any remaining .ts + final playlist.m3u8
              └── Cleanup: delete temp dir, cookie file
```

### FFmpeg Configuration

| Parameter | Value | Purpose |
|-----------|-------|---------|
| `-c:v libx264` | H.264 | Wide browser compatibility |
| `-preset fast` | Speed/quality | Balance for streaming |
| `-crf 23` | Quality | Good quality, ~23 is visually lossless |
| `-g 60` `-keyint_min 60` | Keyframe interval | 60-frame GOP for HLS seeking |
| `-sc_threshold 0` | Scene detect off | Consistent segment boundaries |
| `-c:a aac -b:a 128k` | AAC 128kbps | Audio codec |
| `-hls_time 6` | 6-second segments | Standard HLS segment duration |
| `-hls_flags independent_segments` | IDR per segment | Enables seeking without dependency |
| `-f hls` | HLS muxer | Outputs `.m3u8` + `.ts` files |

### yt-dlp Configuration

```
yt-dlp --no-playlist --cookies {cookie_path} -f "bv*[vcodec^=avc1]+ba/b" -o - {youtube_url}
```

- `--no-playlist` – download single video only
- `--cookies` – use saved YouTube cookie for authenticated access
- `-f "bv*[vcodec^=avc1]+ba/b"` – prefer AVC1 (H.264) video + any audio; fallback to best combined
- `-o -` – pipe video stream to stdout

---

## Redis Keys

| Key | Type | TTL | Value |
|-----|------|-----|-------|
| `job-queue` | List | None | JSON: `{"jobId": "...", "videoUrl": "..."}` |
| `job:{id}:status` | String | 1 hour | `pending` / `processing` / `completed` / `failed` |
| `job:{id}:progress` | String | 1 hour | Float `0.0–1.0` as string |

---

## MinIO Storage Layout

```
yt-videos/
├── cookie.txt              # Active YouTube auth cookie
└── {job_id}/
    ├── playlist.m3u8       # HLS master playlist (updated progressively)
    ├── segment0.ts         # HLS segment 0
    ├── segment1.ts         # HLS segment 1
    └── segmentN.ts         # Additional segments
```

---

## Configuration (Environment Variables)

| Variable | Description |
|----------|-------------|
| `SERVER_HOST` | HTTP health server bind host |
| `SERVER_PORT` | HTTP health server bind port (8081) |
| `REDIS_HOST`, `REDIS_PORT`, `REDIS_USERNAME`, `REDIS_PASSWORD` | Redis connection |
| `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION` | MinIO auth |
| `MINIO_ENDPOINT` | MinIO API URL (internal: `http://minio:9000`) |

---

## Dockerfile Notes

The worker uses a **3-stage Docker build**:

1. **`rust:alpine` build stage** – compiles the Rust binary
2. **`alpine` ffmpeg-build stage** – compiles FFmpeg from source with GPL codecs (x264, x265, fdk-aac, opus, libass)
3. **`alpine` final stage** – installs yt-dlp via pip into a venv, copies binary + FFmpeg, runs as non-root `appuser`

The FFmpeg source tarball (`tools/ffmpeg-6.1.tar.gz`) is bundled in the repo to enable reproducible offline builds.

---

## Key Trade-offs and Constraints

| Concern | Current Approach | Notes |
|---------|-----------------|-------|
| Parallelism | 4 hardcoded workers | Consider making configurable via env var |
| Idempotency | Checks `object_exists` before re-processing | Prevents duplicate downloads |
| Error recovery | `Err` from `consume_data` → `set_status("failed")`, error logged | Jobs are not retried automatically |
| Temp storage | Uses system `/tmp` dir | Ensure sufficient disk space for concurrent jobs |
| Cookie freshness | Cookie fetched per-job from MinIO | Fresh but adds MinIO round-trip |
| Progress accuracy | Segment count × 0.05, capped at 0.95 | Approximate; final completion sets 1.0 |
