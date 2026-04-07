# YouTube Tunnel – Project Overview

## What Is This Project?

**YouTube Tunnel** is a self-hosted YouTube video proxy and streaming platform that allows users to search for and stream YouTube videos through their own infrastructure. The platform bypasses YouTube's native player by downloading video streams, transcoding them to HLS (HTTP Live Streaming) format, and hosting the resulting segments in object storage that can be served directly to a custom web player.

## Architecture Type

**Multi-part / Microservice-style monorepo** with three distinct services that share a Redis queue and MinIO object store.

| Part | Role | Language / Framework |
|------|------|---------------------|
| `backend/api` | REST API gateway – accepts client requests, manages jobs | Rust · Actix-web 4 |
| `backend/worker` | Async job consumer – downloads and transcodes video | Rust · Tokio |
| `web` | React SPA – search UI and HLS video player | TypeScript · React 18 · Vite 7 |

## Key Capabilities

- **YouTube video search** via YouTube Data API v3
- **On-demand video download** using `yt-dlp` (cookie-authenticated)
- **Real-time HLS transcoding** with `ffmpeg` (H.264 + AAC → `.m3u8`)
- **Progressive segment upload** to MinIO as transcoding proceeds (low-latency playback start)
- **Job status tracking** via Redis (pending → processing → completed / failed)
- **Cookie management** – upload a YouTube cookie file through the UI for authenticated downloads
- **In-browser HLS playback** powered by `hls.js`

## Technology Summary

### Backend (both services)
| Category | Technology | Version |
|----------|-----------|---------|
| Language | Rust | 2024 edition |
| Async Runtime | Tokio | 1.48 |
| Web Framework | Actix-web | 4.11 |
| Queue | Redis (RPUSH/BRPOP) | 7.2 |
| Object Storage | MinIO (S3-compatible) | Latest |
| Serialization | Serde / serde_json | 1.x |
| HTTP Client | Reqwest | 0.12 |
| Video Download | yt-dlp | 2025.09.26 |
| Transcoding | FFmpeg | 6.1 (custom build) |
| Env Config | dotenvy | 0.15 |

### Frontend (web)
| Category | Technology | Version |
|----------|-----------|---------|
| Language | TypeScript | ~5.8 |
| UI Framework | React | 18.3 |
| Build Tool | Vite | 7.1 |
| Styling | TailwindCSS | 3 |
| Component Library | shadcn/ui (Radix UI) | Latest |
| HLS Player | hls.js | 1.6 |
| HTTP Client | Axios | 0.14 |
| Routing | React Router | 6 |

## Repository Structure

```
youtube-tunnel/
├── backend/
│   ├── api/          # REST API service (Actix-web)
│   └── worker/       # Video processing worker (Tokio)
├── web/              # React SPA frontend
├── deploy/
│   ├── docker/       # Dockerfiles for all 3 services
│   └── compose/      # docker-compose files (local infra + production)
├── docs/             # ← This documentation
└── .agent/           # BMad agent configuration
```

## High-Level Flow

```
Browser (web)
  │  1. Search YouTube         HTTP GET /v1/search
  ▼
API (backend/api)              Calls YouTube Data API → returns results
  │
  │  2. Request video          HTTP POST /v1/video-player?youtubeUrl=...
  ▼
API creates job    ─┐
  │                 │  RPUSH job-queue {jobId, videoUrl}
  │                 ▼
  │            Redis job-queue
  │                 │  BRPOP
  ▼                 ▼
                Worker (backend/worker)
                  │  yt-dlp | ffmpeg → .ts segments + .m3u8
                  │  Progressive upload to MinIO
                  │  Sets job:{id}:status = processing/completed
                  ▼
               MinIO (yt-videos bucket)
  │
  │  3. Poll status             HTTP GET /v1/video-player/{jobId}/status
  ▼
API reads Redis → returns {status, progress}
  │
  │  4. Get playlist            HTTP GET /v1/video-player/{jobId}/playlist
  ▼
API reads MinIO playlist.m3u8 → returns M3U8 content
  │
  ▼
Browser plays via hls.js (fetches .ts segments from MinIO directly)
```

## Getting Started

See [Development Guide](./development-guide.md) for local setup instructions.

## Documentation Index

- [Architecture – API](./architecture-api.md)
- [Architecture – Worker](./architecture-worker.md)
- [Architecture – Web](./architecture-web.md)
- [API Contracts](./api-contracts-api.md)
- [Integration Architecture](./integration-architecture.md)
- [Source Tree Analysis](./source-tree-analysis.md)
- [Component Inventory – Web](./component-inventory-web.md)
- [Development Guide](./development-guide.md)
- [Deployment Guide](./deployment-guide.md)
