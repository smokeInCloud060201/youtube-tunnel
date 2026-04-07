# API Contracts – API Service

Base URL: `http(s)://<host>/` (configured via `SERVER_HOST:SERVER_PORT` env vars)  
In production the API is reverse-proxied by Traefik at `https://yt.sonbn.xyz/api/` with `/api` prefix stripped.

All responses use `Content-Type: application/json` unless noted otherwise.

---

## Video Player

### POST /v1/video-player

Submit a YouTube URL for download and transcoding. Creates a new job and returns a job ID for polling.

**Query Parameters**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `youtubeUrl` | `string` | ✅ | Full YouTube video URL (percent-encoded) |

**Response 200 OK**

```json
{
  "jobId": "abc123uuid",
  "status": "pending"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `jobId` | `string` | Unique job identifier (UUID) |
| `status` | `string` | Initial status: `"pending"` \| `"unknown"` |

**Notes**
- The API pushes a `VideoJob` to the Redis `job-queue` and immediately returns the job ID
- Worker picks up the job asynchronously
- If a video for the same URL already exists in MinIO, the worker will mark it completed immediately

---

### GET /v1/video-player/{job_id}/status

Poll the current status of a video processing job.

**Path Parameters**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `job_id` | `string` | ✅ | Job ID returned from POST /v1/video-player |

**Response 200 OK**

```json
{
  "status": "processing",
  "progress": 0.35
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | `string` | `"pending"` \| `"processing"` \| `"completed"` \| `"failed"` \| `"unknown"` |
| `progress` | `number \| null` | `0.0–1.0` float representing transcoding progress (null when pending/completed) |

**Notes**
- Status data stored in Redis with 1-hour TTL: `job:{id}:status`, `job:{id}:progress`
- Returns `{ "status": "unknown" }` if keys have expired or job not found

---

### GET /v1/video-player/{job_id}/playlist

Retrieve the HLS M3U8 playlist for a completed job.

**Path Parameters**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `job_id` | `string` | ✅ | Job ID |

**Response 200 OK**

```
Content-Type: application/json
Body: M3U8 playlist content (text/plain content served as JSON body string)
```

The response body is the raw M3U8 playlist text fetched from MinIO (`yt-videos/{job_id}/playlist.m3u8`).

**Notes**
- Only available after job status is `"completed"`
- Segment URLs in the playlist point directly to MinIO

---

### DELETE /v1/video-player/jobs/{job_id}

Delete all Redis keys associated with a specific job.

**Path Parameters**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `job_id` | `string` | ✅ | Job ID |

**Response 200 OK**

```json
{
  "message": "Job abc123uuid cleaned successfully",
  "deleted_keys": 2
}
```

**Error 500**

```json
{
  "error": "Failed to clean job: <error message>"
}
```

---

### DELETE /v1/video-player/jobs

Delete all job-related Redis keys (bulk cleanup).

**Response 200 OK**

```json
{
  "message": "All jobs cleaned successfully",
  "deleted_keys": 14
}
```

**Error 500**

```json
{
  "error": "Failed to clean all jobs: <error message>"
}
```

---

## Video Search

### GET /v1/search

Search YouTube videos via the YouTube Data API v3.

**Query Parameters**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | `string` | ✅ | Search query string |
| *(additional params)* | – | – | See `model/video_search.rs` for full `QueryParams` struct |

**Response 200 OK**

Returns the raw YouTube Data API v3 search response (proxied through).

```json
{
  "kind": "youtube#searchListResponse",
  "items": [
    {
      "id": { "videoId": "dQw4w9WgXcQ" },
      "snippet": {
        "title": "...",
        "description": "...",
        "thumbnails": { ... }
      }
    }
  ]
}
```

**Notes**
- Requires `YOUTUBE_API_KEY` environment variable to be set
- Proxied to avoid CORS issues and to protect API key

---

## Video / Storage Management

### POST /v1/video/clean

Trigger a cleanup of all stored video files in MinIO (`yt-videos` bucket). This is also executed automatically via a cron job registered at API startup.

**Response 200 OK**

```
Body: "Storage cleaned successfully" (plain text)
```

---

### POST /v1/video/cookie

Upload a YouTube authentication cookie file (Netscape cookie format) to be used by `yt-dlp` for authenticated downloads. The file is stored in MinIO for the worker to retrieve.

**Request**
- Content-Type: `multipart/form-data`
- Body: single file field containing the cookie file (max 10 MB)

**Response 200 OK**

```
Body: "Cookie file uploaded successfully!" (plain text)
```

**Error 400 Bad Request**

```
Body: "Uploaded cookie file is empty. Received 0 fields, wrote 0 bytes"
```

**Notes**
- Cookie is stored to MinIO and overrides any previous cookie
- Required for downloading age-restricted or login-required videos
- Cookie file should be in Netscape format (exported from browser)

---

## Redis Data Schema

| Key Pattern | Type | TTL | Description |
|-------------|------|-----|-------------|
| `job-queue` | Redis List | None | Job queue (RPUSH/BRPOP) |
| `job:{id}:status` | String | 1 hour | Job status string |
| `job:{id}:progress` | String | 1 hour | Progress float as string (0.0–1.0) |

## MinIO Bucket Schema

**Bucket:** `yt-videos`

| Key Pattern | Description |
|-------------|-------------|
| `{job_id}/playlist.m3u8` | HLS master playlist |
| `{job_id}/segment0.ts` | HLS video segment (0-indexed) |
| `{job_id}/segmentN.ts` | Subsequent HLS segments |
| `cookie.txt` | Active YouTube cookie file |
