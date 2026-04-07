# Source Tree Analysis

## Repository Overview

**youtube-tunnel** is a multi-part monorepo with three independently deployable services and a shared deployment layer.

```
youtube-tunnel/                        ← Repository root
├── backend/                           ← All server-side Rust services
│   ├── api/                           ← REST API service (actix-web)
│   │   ├── Cargo.toml                 ← Package manifest – actix-web, aws-sdk-s3, deadpool-redis, reqwest
│   │   ├── Cargo.lock                 ← Locked dependency tree
│   │   ├── .env.local                 ← Local environment variables (git-ignored)
│   │   └── src/
│   │       ├── main.rs                ← Entry point – calls server::start()
│   │       ├── server.rs              ← HTTP server setup, route registration, dependency wiring
│   │       ├── config/                ← Infrastructure configuration modules
│   │       │   ├── logger.rs          ← tracing-subscriber initialization
│   │       │   ├── minio.rs           ← MinIO/S3 client factory (aws-sdk-s3)
│   │       │   └── redis_pool.rs      ← deadpool-redis connection pool factory
│   │       ├── handlers/              ← Actix-web route handlers (HTTP layer)
│   │       │   ├── video.rs           ← POST /v1/video/clean, POST /v1/video/cookie
│   │       │   ├── video_player.rs    ← POST /v1/video-player, GET …/playlist, GET …/status, DELETE …/jobs
│   │       │   └── video_search.rs    ← GET /v1/search
│   │       ├── services/              ← Business logic layer
│   │       │   ├── video.rs           ← MinIO storage management + cron cleanup
│   │       │   ├── video_job_producer.rs ← Redis job queue producer (RPUSH job-queue)
│   │       │   ├── video_player.rs    ← Job submission + status/playlist retrieval
│   │       │   └── video_search.rs    ← YouTube Data API v3 search client
│   │       ├── model/                 ← Serde data models (request/response types)
│   │       │   ├── video_player.rs    ← VideoJob, JobStatus, VideoPlayerResponse, JobStatusResponse
│   │       │   └── video_search.rs    ← YouTube search request/response DTOs
│   │       └── errors/                ← Custom error types (thiserror)
│   │
│   └── worker/                        ← Async job consumer service
│       ├── Cargo.toml                 ← Package manifest – actix-web, aws-sdk-s3, deadpool-redis
│       ├── Cargo.lock
│       ├── .env.local                 ← Local environment variables (git-ignored)
│       ├── tools/                     ← Bundled binary assets (ffmpeg tar.gz for Docker build)
│       └── src/
│           ├── main.rs                ← Entry point – calls server::start()
│           ├── server.rs              ← Spawns JobConsumer + starts HTTP server (health endpoint)
│           ├── config/                ← Infrastructure configuration (mirrors api/config)
│           │   ├── cookie_credential.rs ← Downloads YouTube cookie from MinIO
│           │   ├── logger.rs
│           │   ├── minio.rs
│           │   └── redis_pool.rs
│           ├── model/                 ← Shared data models
│           │   └── video_player.rs    ← VideoJob struct (deserialized from Redis)
│           └── services/
│               └── job_consumer.rs    ← Core worker: BRPOP loop → yt-dlp | ffmpeg | MinIO upload
│
├── web/                               ← React SPA (TypeScript + Vite)
│   ├── package.json                   ← Node dependencies + scripts (dev, build, lint, format)
│   ├── vite.config.ts                 ← Vite build configuration
│   ├── tsconfig.json / tsconfig.app.json / tsconfig.node.json
│   ├── tailwind.config.js             ← TailwindCSS configuration
│   ├── components.json                ← shadcn/ui component registry config
│   ├── eslint.config.js / prettier.config.js
│   ├── index.html                     ← SPA entry HTML
│   ├── nginx/                         ← Nginx config for serving SPA in Docker
│   ├── public/                        ← Static assets served as-is
│   └── src/
│       ├── main.tsx                   ← SPA entry – ReactDOM.createRoot, BrowserRouter, ThemeProvider
│       ├── App.tsx                    ← Route configuration (/, /video/:id, /search, /*)
│       ├── ApiContext.tsx             ← React context providing API base URL
│       ├── vite-env.d.ts              ← Vite env type declarations
│       ├── assets/                    ← Static image/SVG assets (bundled by Vite)
│       ├── components/
│       │   ├── ui/                    ← shadcn/ui primitives (Button, Select, Avatar, DropdownMenu)
│       │   ├── video/
│       │   │   └── VideoPlayer.tsx    ← hls.js-based HLS video player component
│       │   ├── navigation/            ← Top navigation bar component
│       │   ├── mode_toggle/           ← Dark/light theme toggle
│       │   ├── icon/                  ← Custom SVG icon wrappers
│       │   └── theme-provider.tsx     ← CSS variable-based theme provider
│       ├── pages/
│       │   ├── home/                  ← Home page (video player + search bar)
│       │   ├── video-search/          ← Search results page
│       │   ├── layouts/               ← BaseLayout (wraps all pages)
│       │   └── 404_not_found/         ← Not-found page
│       ├── services/
│       │   ├── api.base.ts            ← Axios instance (base URL, interceptors)
│       │   ├── video.ts               ← loadVideo, getVideoStatus, getVideoPlaylist
│       │   └── search_service.ts      ← searchVideos
│       ├── hooks/                     ← Custom React hooks
│       ├── hook/                      ← Additional hooks (legacy naming – may be consolidated)
│       ├── lib/                       ← Utility functions (e.g., cn() for className merging)
│       ├── types/                     ← TypeScript type definitions (video.type.ts, etc.)
│       ├── utils/
│       │   └── app.config.ts          ← VITE_* env var accessors (URL_BASE_HOST)
│       └── styles/                    ← Global CSS / Tailwind base styles
│
├── deploy/                            ← All deployment configuration
│   ├── Jenkinsfile.groovy             ← CI/CD pipeline (build images in parallel + deploy)
│   ├── docker/                        ← Service Dockerfiles
│   │   ├── api.Dockerfile             ← Rust multi-stage build for API
│   │   ├── worker.Dockerfile          ← Rust + FFmpeg + yt-dlp multi-stage build for Worker
│   │   └── web.Dockerfile             ← Vite build + Nginx serve for Web
│   └── compose/
│       ├── app-docker-compose.yml     ← Production services (api, worker, web) + Traefik labels
│       ├── base-docker-compose.local.yml ← Local infra (Redis + MinIO) for development
│       └── redis-entrypoint.sh        ← Redis startup script with auth config
│
├── docs/                              ← ← This directory – all project documentation
├── design-artifacts/                  ← Design files / mockups
└── .agent/                            ← BMad agent skills and workflows
```

## Critical Entry Points

| Service | Entry Point | Description |
|---------|------------|-------------|
| API | `backend/api/src/main.rs` → `server::start()` | Boots Actix-web HTTP server on `$SERVER_HOST:$SERVER_PORT` |
| Worker | `backend/worker/src/main.rs` → `server::start()` | Spawns `JobConsumer` + HTTP health server on port 8081 |
| Web | `web/src/main.tsx` | Mounts React into `#root`, wraps with `BrowserRouter` and `ThemeProvider` |

## Integration Points

| From → To | Mechanism |
|-----------|----------|
| Web → API | Axios HTTP calls to `$VITE_API_BASE_URL/v1/*` |
| API → Worker | Redis `RPUSH job-queue` (job payloads as JSON) |
| Worker → Redis | Writes `job:{id}:status` and `job:{id}:progress` keys |
| API → Redis | Reads `job:{id}:status` and `job:{id}:progress` for status responses |
| API + Worker → MinIO | Shared `yt-videos` bucket – worker writes, API reads |
