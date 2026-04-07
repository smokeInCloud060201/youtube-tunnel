# Architecture – API Service

## Executive Summary

The `api` service is a **Rust/Actix-web 4 REST API** that acts as the gateway for the YouTube Tunnel platform. It handles:

1. **Video search** – proxies YouTube Data API v3 queries
2. **Video job submission** – enqueues download/transcode jobs via Redis
3. **Job status & playlist retrieval** – reads job state from Redis + MinIO
4. **Storage management** – cookie upload and cron-triggered storage cleanup

The service is stateless (aside from shared Redis and MinIO) and can be horizontally scaled.

---

## Technology Stack

| Category | Technology | Version |
|----------|-----------|---------|
| Language | Rust | 2024 edition |
| Web Framework | Actix-web | 4.11 |
| Async Runtime | Tokio | 1.48 (full features) |
| HTTP Client | Reqwest | 0.12 |
| Queue (producer) | Redis via deadpool-redis | 0.22 |
| Object Storage | MinIO via aws-sdk-s3 | 1.108 |
| Serialization | Serde + serde_json | 1.x |
| Error Handling | anyhow + thiserror | 1.x / 2.x |
| CORS | actix-cors | 0.7 |
| Multipart | actix-multipart | 0.7 |
| Logging | tracing + tracing-subscriber | 0.1 / 0.3 |
| Env Config | dotenvy | 0.15 |
| Cron | tokio-cron-scheduler | 0.15 |
| Graceful Reload | listenfd | 1.0 |

---

## Architecture Pattern

**Layered / Handler-Service-Model** (classic MVC without views):

```
HTTP Request
     │
     ▼
[Handler Layer] (src/handlers/)
   Parses HTTP params, delegates to service, maps response
     │
     ▼
[Service Layer] (src/services/)
   Business logic – interacts with Redis, MinIO, YouTube API
     │
     ▼
[Infrastructure] (redis_pool, minio, logger)
   Connection pools and external clients
```

---

## Module Structure

```
src/
├── main.rs                  # Calls server::start()
├── server.rs                # Server bootstrap:
│                            #   - Init Redis pool (with retry loop)
│                            #   - Init MinIO client (with retry loop)
│                            #   - Wire all services as Actix Data<>
│                            #   - Register all routes
│                            #   - Register cron cleanup job
├── config/
│   ├── logger.rs            # tracing_subscriber setup (env-filter)
│   ├── minio.rs             # aws-sdk-s3 Client factory (reads AWS_* + MINIO_ENDPOINT env)
│   └── redis_pool.rs        # deadpool-redis Pool factory (reads REDIS_HOST/PORT/USERNAME/PASSWORD)
├── handlers/                # Thin HTTP layer – param extraction + response mapping
│   ├── video.rs             # clean_storage, upload_cookie
│   ├── video_player.rs      # request_video, get_video_playlist, get_video_status, clean_job, clean_all_jobs
│   └── video_search.rs      # get_video
├── services/                # Business logic
│   ├── video.rs             # Video (MinIO operations): save_cookie, clean_storage (+ cron)
│   ├── video_job_producer.rs # VideoJobProducer: submit_job (RPUSH), get_job_status (Redis read), clean_job, clean_all_jobs
│   ├── video_player.rs      # VideoPlayer: submit_video_job (generate UUID, produce to queue), get_playlist (MinIO read), get_job_status
│   └── video_search.rs      # VideoSearchService: search() → calls YouTube Data API v3
├── model/                   # Serde input/output types
│   ├── video_player.rs      # QueryParams, VideoJob, JobStatus (enum), VideoPlayerResponse, JobStatusResponse
│   └── video_search.rs      # YouTube search QueryParams + response DTOs
└── errors/                  # Custom error types (thiserror)
```

---

## Dependency Injection

Services are injected as `web::Data<Arc<T>>` into Actix-web's app state:

```rust
App::new()
    .app_data(web::Data::new(Arc::new(video_search)))
    .app_data(web::Data::new(video.clone()))
    .app_data(web::Data::new(Arc::new(video_player.clone())))
    .app_data(web::Data::new(Arc::new(video_job_producer.clone())))
```

Handlers receive services via Actix's extractor system.

---

## Job Lifecycle (API perspective)

1. Client calls `POST /v1/video-player?youtubeUrl=...`
2. `VideoPlayer::submit_video_job(url)`:
   - Generates a UUID as `job_id`
   - Pushes `VideoJob { job_id, video_url }` JSON to `job-queue` Redis list via `VideoJobProducer`
   - Reads initial status from Redis (usually `"unknown"` or `"pending"`)
3. Returns `{ jobId, status }` immediately
4. Client polls `GET /v1/video-player/{job_id}/status`
5. Worker updates Redis keys; API reads and returns them
6. When `status == "completed"`, client calls `GET /v1/video-player/{job_id}/playlist`
7. API fetches `{job_id}/playlist.m3u8` from MinIO and streams it back

---

## Startup Resilience

Both Redis and MinIO connections use a **retry loop with exponential backoff**:
- Up to 10 attempts, 3-second delay between retries
- `panic!()` if connection not established after max attempts
- Ensures the service waits for infrastructure to become available (important in Docker environments)

---

## Configuration (Environment Variables)

| Variable | Description | Example |
|----------|-------------|---------|
| `SERVER_HOST` | Bind host | `0.0.0.0` |
| `SERVER_PORT` | Bind port | `8080` |
| `YOUTUBE_API_KEY` | YouTube Data API v3 key | `AIza...` |
| `YOUTUBE_HOST` | YouTube base URL (for proxying) | `https://www.youtube.com` |
| `REDIS_HOST` | Redis hostname | `localhost` |
| `REDIS_PORT` | Redis port | `6379` |
| `REDIS_USERNAME` | Redis username | `default` |
| `REDIS_PASSWORD` | Redis password | `...` |
| `AWS_ACCESS_KEY_ID` | MinIO access key | `minioadmin` |
| `AWS_SECRET_ACCESS_KEY` | MinIO secret key | `...` |
| `AWS_REGION` | S3 region (dummy for MinIO) | `us-east-1` |
| `MINIO_ENDPOINT` | MinIO API endpoint URL | `http://minio:9000` |

---

## CORS Policy

Currently configured to allow:
- Any origin
- Any method
- Any header

> ⚠️ This is permissive – tighten in production if needed.

---

## API Route Summary

See [API Contracts](./api-contracts-api.md) for full request/response documentation.

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| POST | `/v1/video-player` | `request_video` | Submit YouTube URL for processing |
| GET | `/v1/video-player/{id}/playlist` | `get_video_playlist` | Fetch HLS M3U8 playlist |
| GET | `/v1/video-player/{id}/status` | `get_video_status` | Poll job status |
| DELETE | `/v1/video-player/jobs/{id}` | `clean_job` | Remove single job from Redis |
| DELETE | `/v1/video-player/jobs` | `clean_all_jobs` | Remove all jobs from Redis |
| GET | `/v1/search` | `get_video` | YouTube search proxy |
| POST | `/v1/video/clean` | `clean_storage` | Clean MinIO storage |
| POST | `/v1/video/cookie` | `upload_cookie` | Upload YouTube cookie |
