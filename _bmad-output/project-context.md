---
project_name: 'youtube-tunnel'
user_name: 'Khanh'
date: '2026-04-07T09:08:21+07:00'
sections_completed: ['technology_stack', 'critical_implementation_rules']
existing_patterns_found: 8
---

# Project Context for AI Agents

_This file contains critical rules and patterns that AI agents must follow when implementing code in this project. Focus on unobvious details that agents might otherwise miss._

---

## Technology Stack & Versions

The project consists of three services operating as a multi-part architecture.

### API Service (`backend/api/`)
*   **Language:** Rust 2024
*   **Framework:** Actix-web 4.11.0
*   **Async Runtime:** Tokio 1.48.0
*   **Clients:** `reqwest` 0.12.24 (HTTP), `aws-sdk-s3` 1.108.0 (MinIO), `deadpool-redis` 0.22.0 (Redis)
*   **Logging:** `tracing` 0.1.41, `tracing-subscriber` 0.3.20

### Worker Service (`backend/worker/`)
*   **Language:** Rust 2024
*   **Async Runtime:** Tokio 1.48.0
*   **Framework:** Actix-web 4.11.0 (for health checks)
*   **Clients:** `aws-sdk-s3` 1.108.0 (MinIO), `deadpool-redis` 0.22.0 (Redis)
*   **External Tools:** `yt-dlp` 2025.09.26, `ffmpeg` 6.1 (with H.264/AAC support)

### Web Frontend (`web/`)
*   **Language:** TypeScript ~5.8
*   **Framework:** React 18.3.1
*   **Build Tool:** Vite 7.1.2
*   **Styling:** TailwindCSS 3 (with `@tailwindcss/vite` v4 plugin structure in dev dependencies but version 3 in runtime)
*   **Components:** `shadcn/ui` (Radix UI primitives - versions 1.x-2.x)
*   **Video Playback:** `hls.js` 1.6.13
*   **Client:** Axios 0.14.4
*   **Routing:** React Router DOM 6

---

## Critical Implementation Rules

### 1. Inter-Service Communication (Redis)
*   **Queue Structure:** API enqueues job payloads as JSON strings to the `job-queue` Redis list via `RPUSH`. Example: `{"jobId": "<uuid>", "videoUrl": "..."}`.
*   **Worker Consumption:** Workers consume from the queue using blocking pop (`BRPOP job-queue 0`).
*   **State Management:** Workers write status and progress to Redis keys with a **1-hour TTL**:
    *   `job:{id}:status` -> `"pending"`, `"processing"`, `"completed"`, `"failed"`
    *   `job:{id}:progress` -> Float as string (`"0.0"` to `"1.0"`)
*   **DO NOT** rely on the API to update job progress. The worker is the source of truth.

### 2. File Storage and MinIO
*   **Bucket Configuration:** `yt-videos` bucket must have a public read policy assigned in production because the web UI (`hls.js`) fetches `.ts` streaming segments **directly from MinIO**, bypassing the API.
*   **Youtube Cookies:** The worker does not read cookies from the local filesystem directly; it reads `cookie.txt` from the MinIO `yt-videos` bucket. When downloading a video via yt-dlp, the worker pulls the cookie from MinIO, writes it to the local `/tmp/video-{job_id}/` dir, and tells yt-dlp to use it.
*   **API Responsibilities:** API reads the `playlist.m3u8` from MinIO and serves it to clients. API is also responsible for managing cookie file uploads to MinIO via `/v1/video/cookie`.

### 3. Rust Backend Conventions
*   **Dependency Injection (Actix):** Services are wrapped in `Arc` and injected into Actix state using `web::Data<Arc<T>>`. Example: `.app_data(web::Data::new(Arc::new(service)))`.
*   **Infrastructure Resiliency:** Connection pools for Redis and MinIO **MUST** implement a retry loop at startup (exponential backoff, max 10 attempts, 3s delay). Do not just unwrap/panic on immediate connection failure, especially for docker-compose booting sequence.
*   **Error Handling:** Use `thiserror` (v2.x) for custom service error domains and `anyhow` for top-level application errors.
*   **Logging:** Use the `tracing` ecosystem. Ensure `tracing_subscriber` is initialized to read the `RUST_LOG` env variable.

### 4. Worker ffmpeg & yt-dlp execution
*   **yt-dlp Format:** Enforce downloading an H.264 video codec format if possible: `-f "bv*[vcodec^=avc1]+ba/b"`.
*   **Piping:** yt-dlp streams output straight to stdout (`-o -`), which is piped into `ffmpeg`'s stdin.
*   **Segment Upload:** The worker polls the generated `.ts` files. It considers a file "stable" and ready for MinIO upload if the file size remains unchanged after 100-300ms.

### 5. Frontend & React Patterns
*   **API Configuration:** Do not hardcode API bounds. The API base URL is provided through the `VITE_API_BASE_URL` env variable and mapped into an `ApiContext` via `services/api.base.ts`.
*   **Caching Workaround:** Axios GET requests in `services/api.base.ts` have an interceptor that appends `_={timestamp}` to the query params. Maintain this pattern to bypass browser caching when polling job status.
*   **Styling (Tailwind + shadcn-ui):** Use the `cn()` utility function (defined in `src/lib/`) for merging conditional tailwind classes using `clsx` and `tailwind-merge`.
*   **Theming:** Colors are CSS-variable based. `ThemeProvider` toggles a `"dark"` HTML class.
*   **Lazy Loading:** Large or less frequent routes (like the 404 page) should be lazy-loaded using `React.lazy()` and wrapped in `<Suspense>`.
