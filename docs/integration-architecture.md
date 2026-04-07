# Integration Architecture

## Overview

YouTube Tunnel is a **three-service distributed system** where components communicate through HTTP, a Redis list-based job queue, and shared MinIO object storage.

```
┌─────────────────────────────────────────────────────────────────┐
│                        Browser (web)                            │
│  React SPA · hls.js · Axios                                     │
└──────────────┬───────────────────────────────────────┬──────────┘
               │ HTTP REST (Axios)                     │ HLS segments
               │ VITE_API_BASE_URL/v1/*                │ (direct from MinIO)
               ▼                                       │
┌──────────────────────────┐                           │
│   API Service (api)      │                           │
│   Rust · Actix-web 4     │                           │
│   Port: 8080             │                           │
└───┬──────────────────────┘                           │
    │                                                  │
    │ RPUSH job-queue                  ┌───────────────┘
    │ (JSON payload)                   │
    ▼                                  ▼
┌──────────┐              ┌─────────────────────────┐
│  Redis   │◄─────────────│   MinIO Object Store    │
│  7.2     │    status/   │   (S3-compatible)       │
│          │    progress  │   Bucket: yt-videos     │
└──────────┘              └────────────┬────────────┘
    │ BRPOP job-queue                  │ PUT objects
    ▼                                  │
┌──────────────────────────┐           │
│   Worker Service         │───────────┘
│   Rust · Tokio           │
│   Port: 8081             │
│   [yt-dlp | ffmpeg]      │
└──────────────────────────┘
```

---

## Integration Points

### 1. Web → API (REST HTTP)

| Aspect | Detail |
|--------|--------|
| Protocol | HTTP/HTTPS |
| Client | Axios (30s timeout, cache-busting for GET) |
| Base URL | `VITE_API_BASE_URL` (env var, default `http://localhost:8080`) |
| Auth | None (open) |
| CORS | API allows any origin/method/header |

**Endpoints consumed by frontend:**

| Call | Endpoint | Purpose |
|------|----------|---------|
| Search | `GET /v1/search?q=...` | YouTube proxy search |
| Submit Video | `POST /v1/video-player?youtubeUrl=...` | Queue video job |
| Poll Status | `GET /v1/video-player/{id}/status` | Check job progress |
| Get Playlist | `GET /v1/video-player/{id}/playlist` | Fetch M3U8 |

---

### 2. API → Worker (Redis Job Queue)

| Aspect | Detail |
|--------|--------|
| Protocol | Redis List (RPUSH / BRPOP) |
| Queue Name | `job-queue` |
| Direction | API produces, Worker consumes |
| Message Format | JSON: `{"jobId": "<uuid>", "videoUrl": "<youtube_url>"}` |
| Delivery | At-least-once (no acknowledgment mechanism) |
| Concurrency | Worker runs 4 parallel BRPOP consumers |

**Job message schema:**

```json
{
  "jobId": "550e8400-e29b-41d4-a716-446655440000",
  "videoUrl": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}
```

---

### 3. Worker → API (Shared Redis State)

| Aspect | Detail |
|--------|--------|
| Protocol | Redis key reads/writes |
| Direction | Worker writes → API reads |
| Key TTL | 1 hour |

**Shared Redis keys:**

| Key | Writer | Reader | Value |
|-----|--------|--------|-------|
| `job:{id}:status` | Worker | API | `"pending"` / `"processing"` / `"completed"` / `"failed"` |
| `job:{id}:progress` | Worker | API | Float string `"0.0"` – `"1.0"` |

---

### 4. Worker → MinIO (Video Storage)

| Aspect | Detail |
|--------|--------|
| Protocol | S3-compatible API (aws-sdk-s3) |
| Direction | Worker writes HLS segments progressively; API reads playlist |
| Bucket | `yt-videos` |
| Cookie | Worker reads `cookie.txt` from MinIO; API writes it via `/v1/video/cookie` |

**MinIO object structure:**

```
yt-videos/
├── cookie.txt                  ← Written by API, read by Worker
└── {job_id}/
    ├── playlist.m3u8           ← Written by Worker, read by API (/v1/video-player/{id}/playlist)
    ├── segment0.ts             ← Written by Worker, read by browser via hls.js (direct URLs)
    └── segment{N}.ts
```

---

### 5. Browser → MinIO (Direct HLS Streaming)

After receiving the playlist from the API:
- hls.js parses the `.m3u8` to extract segment URLs
- Segments are fetched **directly from MinIO** (bypassing the API)
- MinIO must be publicly accessible or the M3U8 segment URLs must be pre-signed

> ⚠️ **Note:** In production, CORS must be configured on MinIO for browser direct access to `.ts` segments.

---

## Data Flow – Full Video Request

```
Step 1: User enters YouTube URL in browser
        │
        ▼
Step 2: Web → POST /v1/video-player?youtubeUrl=...
        │     Returns: { jobId, status: "pending" }
        │
        ▼
Step 3: API → RPUSH job-queue { jobId, videoUrl }
        │     API sets Redis status = (left to worker)
        │
        ▼
Step 4: Worker → BRPOP job-queue (one of 4 workers picks up)
        │     Worker sets: job:{id}:status = "processing"
        │     Worker sets: job:{id}:progress = 0.0
        │
        ▼
Step 5: Worker → yt-dlp | ffmpeg → tmp dir
        │     As segments complete:
        │       Upload segment{N}.ts to MinIO
        │       Upload playlist.m3u8 to MinIO
        │       Update Redis progress: (n * 0.05).min(0.95)
        │
        ▼
Step 6: Web → GET /v1/video-player/{id}/status (polling)
        │     API reads: Redis job:{id}:status + job:{id}:progress
        │     Returns: { status: "processing", progress: 0.35 }
        │
Step 7: Worker finishes:
        │     Final upload of all remaining segments + playlist
        │     Cleanup temp dir
        │     Set Redis status = "completed"
        │     Set Redis progress = 1.0
        │
        ▼
Step 8: Web → GET /v1/video-player/{id}/playlist (when status = "completed")
        │     API fetches yt-videos/{id}/playlist.m3u8 from MinIO
        │     Returns M3U8 text
        │
        ▼
Step 9: hls.js initializes with M3U8
        Browser fetches .ts segments directly from MinIO
        Video plays
```

---

## Network Configuration (Production)

| Service | Internal Network | External Access |
|---------|-----------------|-----------------|
| API | `yt-network`, `proxy_network`, `sunflower_data` | Via Traefik: `yt.sonbn.xyz/api` |
| Worker | `yt-network`, `sunflower_data` | None (internal only) |
| Web | `yt-network`, `proxy_network`, `sunflower_data` | Via Traefik: `yt.sonbn.xyz` |
| Redis | `yt-network` (external) | Internal only |
| MinIO | `sunflower_data` (external) | Separate domain (`MINIO_API_DOMAIN`) |
| Traefik | `proxy_network` | SSL termination (Let's Encrypt) |

---

## Failure Scenarios

| Failure | Impact | Current Handling |
|---------|--------|-----------------|
| Redis unavailable at startup | Service won't start | Retry loop (10 attempts, 3s delay) |
| MinIO unavailable at startup | Service won't start | Retry loop (10 attempts, 3s delay) |
| Worker crashes mid-job | Job stuck in "processing" | Status remains "processing" until TTL expires (1h) |
| yt-dlp fails | Job marked "failed" | Error logged, job not retried |
| ffmpeg fails | Job marked "failed" | Error logged, job not retried |
| Redis key expired | Status returns "unknown" | API returns `{ status: "unknown" }` |
| MinIO segment upload fails | Retry 3x with 2s delays | If all fail, continues to next segment |
