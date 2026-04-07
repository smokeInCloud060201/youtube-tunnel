# Development Guide

## Prerequisites

### All Services
- **Git**
- **Docker** + **Docker Compose** (for local infrastructure: Redis + MinIO)
- **Make** (optional, for Makefile targets)

### API Service (`backend/api`)
- **Rust** toolchain (2024 edition) – install via [rustup.rs](https://rustup.rs)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update stable
  ```

### Worker Service (`backend/worker`)
- **Rust** toolchain (as above)
- **yt-dlp** (for local development without Docker)
  ```bash
  pip install yt-dlp
  # Or: brew install yt-dlp
  ```
- **FFmpeg** with H.264 support (for local development)
  ```bash
  brew install ffmpeg         # macOS
  sudo apt install ffmpeg     # Ubuntu/Debian
  ```

### Web Frontend (`web`)
- **Node.js** 18+ (LTS recommended)
- **Yarn** (classic or berry)
  ```bash
  npm install -g yarn
  ```

---

## 1. Start Local Infrastructure

The local infrastructure (Redis + MinIO) is managed via Docker Compose.

```bash
# Create the Docker network first (one-time setup)
docker network create yt-network

# Start Redis and MinIO
docker-compose -f deploy/compose/base-docker-compose.local.yml up -d

# Verify services are running
docker ps
# Should show: yt-redis, minio
```

**Default local credentials:**
- **Redis**: `localhost:6379`, user `default`, password `redispassword123`
- **MinIO**: `http://localhost:9000`, user `minioadmin`, password `minioadmin123`
- **MinIO Console**: `http://localhost:9001`

**One-time MinIO setup:** Create the `yt-videos` bucket in the MinIO Console at `http://localhost:9001`.

---

## 2. API Service Development

### Environment Setup

```bash
cd backend/api
cp .env.local.example .env.local  # Create from template (if it exists)
# Or create .env.local manually:
```

**`backend/api/.env.local`:**
```env
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
YOUTUBE_API_KEY=your_youtube_data_api_v3_key_here
YOUTUBE_HOST=https://www.youtube.com
REDIS_HOST=127.0.0.1
REDIS_PORT=6379
REDIS_USERNAME=default
REDIS_PASSWORD=redispassword123
AWS_ACCESS_KEY_ID=minioadmin
AWS_SECRET_ACCESS_KEY=minioadmin123
AWS_REGION=us-east-1
MINIO_ENDPOINT=http://127.0.0.1:9000
```

### Running the API

```bash
cd backend/api

# Development run (with auto-reload via cargo-watch, optional)
cargo run

# Or with cargo-watch for auto-reload:
cargo install cargo-watch
cargo watch -x run
```

The API will start at `http://127.0.0.1:8080`.

### Building

```bash
cd backend/api
cargo build --release
```

---

## 3. Worker Service Development

### Environment Setup

**`backend/worker/.env.local`:**
```env
SERVER_HOST=127.0.0.1
SERVER_PORT=8081
REDIS_HOST=127.0.0.1
REDIS_PORT=6379
REDIS_USERNAME=default
REDIS_PASSWORD=redispassword123
AWS_ACCESS_KEY_ID=minioadmin
AWS_SECRET_ACCESS_KEY=minioadmin123
AWS_REGION=us-east-1
MINIO_ENDPOINT=http://127.0.0.1:9000
```

### Running the Worker

```bash
cd backend/worker
cargo run
```

The worker starts 4 job consumer threads and begins listening for jobs on the `job-queue` Redis list. It will also start an HTTP server on port 8081.

### Testing Workers Locally

1. Start API and Worker
2. Use `curl` to submit a test job:

```bash
curl -X POST "http://localhost:8080/v1/video-player?youtubeUrl=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DdQw4w9WgXcQ"
# Returns: {"jobId":"...", "status":"pending"}

# Poll status
curl "http://localhost:8080/v1/video-player/{jobId}/status"
```

> ⚠️ **Note:** yt-dlp may need a YouTube cookie to download age-restricted or quota-limited videos. Upload a cookie file:
> ```bash
> curl -X POST http://localhost:8080/v1/video/cookie -F "cookie=@/path/to/cookie.txt"
> ```

---

## 4. Web Frontend Development

### Environment Setup

**`web/.env.local`** (create if not exists):
```env
VITE_API_BASE_URL=http://localhost:8080
```

### Running the Frontend

```bash
cd web

# Install dependencies
yarn install

# Start development server (HMR enabled)
yarn dev
```

The app starts at `http://localhost:5173`.

### Available Scripts

| Command | Description |
|---------|-------------|
| `yarn dev` | Start Vite dev server with Hot Module Replacement |
| `yarn build` | Production build (TypeScript check + Vite bundle) |
| `yarn preview` | Preview production build locally |
| `yarn lint` | ESLint check |
| `yarn lint:fix` | ESLint auto-fix |
| `yarn format` | Prettier format all files |

### Adding shadcn/ui Components

```bash
cd web
npx shadcn-ui add <component>
# Example: npx shadcn-ui add dialog input toast
```

---

## 5. Running Everything Together

For full local development, run these in separate terminals:

```bash
# Terminal 1: Infrastructure
docker-compose -f deploy/compose/base-docker-compose.local.yml up

# Terminal 2: API
cd backend/api && cargo run

# Terminal 3: Worker
cd backend/worker && cargo run

# Terminal 4: Web
cd web && yarn dev
```

Then open `http://localhost:5173` in your browser.

---

## 6. Testing

### API / Worker (Rust)

No automated test suite is currently present. Run ad-hoc tests with `curl` or via the web UI.

To verify the stack end-to-end:
1. Upload a YouTube cookie: `POST /v1/video/cookie`
2. Submit a video: `POST /v1/video-player`
3. Poll status until `"completed"`
4. Fetch playlist: `GET /v1/video-player/{id}/playlist`

### Frontend (Web)

No automated test suite configured. Use the dev server for manual testing.

---

## 7. Common Development Tasks

### Check Redis Queue

```bash
docker exec -it yt-redis redis-cli -a redispassword123
> LLEN job-queue          # How many jobs waiting
> KEYS job:*             # All job status keys
> GET job:{jobId}:status
```

### Browse MinIO

Visit `http://localhost:9001` in browser with credentials `minioadmin` / `minioadmin123`.

### Clean All Jobs (Redis)

```bash
curl -X DELETE http://localhost:8080/v1/video-player/jobs
```

### Clean MinIO Storage

```bash
curl -X POST http://localhost:8080/v1/video/clean
```

### Inspect Logs (Docker)

```bash
docker logs yt-redis -f
docker logs minio -f
```

---

## 8. Makefile Targets

Check `Makefile` in the project root for any available convenience targets:

```bash
make
# or
cat Makefile
```
