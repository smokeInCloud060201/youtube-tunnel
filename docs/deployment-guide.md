# Deployment Guide

## Overview

YouTube Tunnel is deployed as 3 Docker containers orchestrated with Docker Compose, behind a **Traefik** reverse proxy for SSL termination and routing. The CI/CD pipeline is managed by **Jenkins**.

**Production URL:** `https://yt.sonbn.xyz`

---

## Architecture

```
Internet
  │
  ▼
Traefik (HTTPS/SSL via Let's Encrypt)
  ├── yt.sonbn.xyz        → web container (port 80)
  └── yt.sonbn.xyz/api/*  → api container (port 8080, /api prefix stripped)
  
Internal Docker Networks:
  yt-network:       api ↔ worker (internal communication)
  sunflower_data:   api ↔ worker ↔ worker ↔ shared services (Redis, MinIO)
  proxy_network:    api ↔ web ↔ Traefik
```

---

## Services

| Service | Image | Internal Port | Network(s) |
|---------|-------|--------------|-----------|
| `api` | `youtube-tunnel-api:latest` | 8080 | yt-network, proxy_network, sunflower_data |
| `worker` | `youtube-tunnel-worker:latest` | 8081 | yt-network, sunflower_data |
| `web` | `youtube-tunnel-web:latest` | 80 | yt-network, proxy_network, sunflower_data |

**External dependencies** (managed separately, on `sunflower_data` network):
- **Redis 7.2** – job queue and job state storage
- **MinIO** – video segment and cookie file storage

---

## CI/CD Pipeline (Jenkins)

**Pipeline file:** `deploy/Jenkinsfile.groovy`

### Pipeline Stages

1. **Clean Images** – Remove old Docker images for all 3 services
2. **Build Images** (parallel) – Build `api`, `worker`, and `web` images simultaneously
3. **Deploy** – Generate `.env`, create Docker network if needed, deploy via docker-compose

### Required Jenkins Credentials

Configure these in Jenkins Credential Manager:

| Credential ID | Description |
|--------------|-------------|
| `YOUTUBE_API_KEY` | YouTube Data API v3 key |
| `YOUTUBE_BASE_HOST` | YouTube base URL |
| `MINIO_CONTAINER_NAME` | MinIO container name on sunflower_data network |
| `MINIO_API_DOMAIN` | MinIO API domain (for HTTPS endpoint) |
| `MINIO_ROOT_USER` | MinIO access key |
| `MINIO_ROOT_PASSWORD` | MinIO secret key |
| `DATA_NETWORK` | Docker network name for shared data services |
| `TRAEFIK_NETWORK` | Docker network name for Traefik proxy |
| `REDIS_CONTAINER_NAME` | Redis container hostname |
| `REDIS_PORT` | Redis port |
| `REDIS_USERNAME` | Redis username |
| `REDIS_PASSWORD` | Redis password |

---

## Dockerfiles

### API (`deploy/docker/api.Dockerfile`)
- **Stage 1:** `rust:alpine` – Compile Rust binary
- **Final:** Alpine + compiled binary
- **Exposes:** Port 8080

### Worker (`deploy/docker/worker.Dockerfile`)
- **Stage 1:** `rust:1.90.0-alpine` – Compile Rust binary
- **Stage 2:** `alpine:3.18` – Build FFmpeg 6.1 from source with GPL codecs
- **Final:** Alpine + yt-dlp (via pip venv) + FFmpeg + compiled Rust binary
- **Exposes:** Port 8081
- **User:** Non-root `appuser` (UID 10001)

> ⚠️ FFmpeg source tarball must exist at `backend/worker/tools/ffmpeg-6.1.tar.gz` before building.

### Web (`deploy/docker/web.Dockerfile`)
- **Stage 1:** Node.js – `yarn install && yarn build` (Vite output to `dist/`)
- **Final:** Nginx Alpine – serves static files
- **Exposes:** Port 80

---

## Docker Networks Setup

```bash
# Create internal service network (one-time)
docker network create yt-network

# External networks (must exist from other services):
# - sunflower_data: shared data infrastructure network
# - <TRAEFIK_NETWORK>: Traefik proxy network
```

---

## Environment Variables (Production)

These are injected at runtime by the Jenkins pipeline into a `.env` file:

### API Container

| Variable | Source | Description |
|----------|--------|-------------|
| `SERVER_HOST` | Hardcoded `0.0.0.0` | Bind all interfaces |
| `SERVER_PORT` | Hardcoded `8080` | HTTP listen port |
| `YOUTUBE_API_KEY` | Jenkins credential | YouTube Data API key |
| `YOUTUBE_HOST` | Jenkins credential | YouTube base URL |
| `REDIS_HOST` | Jenkins: `REDIS_CONTAINER_NAME` | Redis hostname |
| `REDIS_PORT` | Jenkins credential | Redis port |
| `REDIS_USERNAME` | Jenkins credential | Redis username |
| `REDIS_PASSWORD` | Jenkins credential | Redis password |
| `AWS_ACCESS_KEY_ID` | Jenkins: `MINIO_ROOT_USER` | MinIO access key |
| `AWS_SECRET_ACCESS_KEY` | Jenkins: `MINIO_ROOT_PASSWORD` | MinIO secret |
| `AWS_REGION` | Hardcoded `us-east-1` | Dummy region for S3 compat |
| `MINIO_ENDPOINT` | Computed `https://${MINIO_API_DOMAIN}` | MinIO HTTPS endpoint |

### Worker Container

| Variable | Source | Description |
|----------|--------|-------------|
| `SERVER_HOST` | Hardcoded `0.0.0.0` | Bind all interfaces |
| `SERVER_PORT` | Hardcoded `8081` | HTTP listen port |
| Redis vars | Same as API | — |
| `AWS_*` vars | Same as API | — |
| `MINIO_ENDPOINT` | Computed `http://${MINIO_CONTAINER_NAME}:9000` | Internal MinIO HTTP endpoint |

> Note: Worker uses **HTTP** (internal), API uses **HTTPS** (external) for MinIO.

---

## Manual Deployment Steps

If you need to deploy manually (outside Jenkins):

```bash
# 1. Set environment variables
export IMAGE_TAG=latest
export YOUTUBE_API_KEY=...
export YOUTUBE_BASE_HOST=https://www.youtube.com
export MINIO_CONTAINER_NAME=minio
export MINIO_API_DOMAIN=minio.example.com
export MINIO_ROOT_USER=minioadmin
export MINIO_ROOT_PASSWORD=...
export DATA_NETWORK=sunflower_data
export TRAEFIK_NETWORK=traefik_proxy
export REDIS_HOSTNAME=redis
export REDIS_PORT=6379
export REDIS_USERNAME=default
export REDIS_PASSWORD=...

# 2. Build images
docker build -f deploy/docker/api.Dockerfile -t youtube-tunnel-api:latest ./backend/api
docker build -f deploy/docker/worker.Dockerfile -t youtube-tunnel-worker:latest ./backend/worker
docker build -f deploy/docker/web.Dockerfile -t youtube-tunnel-web:latest ./web

# 3. Create network
docker network create yt-network || true

# 4. Deploy
docker-compose -f deploy/compose/app-docker-compose.yml up -d
```

---

## Traefik Configuration

API and Web are configured with Traefik labels in `app-docker-compose.yml`:

### Web
- Rule: `Host(\`yt.sonbn.xyz\`)`
- Priority: 10 (lower = catches all unmatched routes)
- TLS: Let's Encrypt (`le` cert resolver)

### API
- Rule: `Host(\`yt.sonbn.xyz\`) && PathPrefix(\`/api\`)`
- Priority: 20 (higher = matched before web)
- Middleware: `api-strip` removes `/api` prefix before forwarding
- TLS: Let's Encrypt

---

## MinIO One-Time Setup

After first deployment, manually create the `yt-videos` bucket:

1. Access MinIO Console at `https://<MINIO_API_DOMAIN>/minio`
2. Create bucket: `yt-videos`
3. Set bucket policy to allow public read (for HLS segment access by browsers):
   ```json
   {
     "Version": "2012-10-17",
     "Statement": [{
       "Effect": "Allow",
       "Principal": { "AWS": ["*"] },
       "Action": ["s3:GetObject"],
       "Resource": ["arn:aws:s3:::yt-videos/*"]
     }]
   }
   ```

> ⚠️ Public read is required so browsers can fetch `.ts` segments directly via hls.js without authentication.

---

## Health Checks

| Service | Check |
|---------|-------|
| API | HTTP GET `http://api:8080/` (Actix default response) |
| Worker | HTTP GET `http://worker:8081/` (Actix health server) |
| Web | Nginx serving index.html |

---

## Rollback

```bash
# To rollback, rebuild with previous code and redeploy
# Jenkins: re-trigger pipeline with previous commit

# Manual rollback:
docker-compose -f deploy/compose/app-docker-compose.yml down
# Rebuild images from previous tag / commit
docker-compose -f deploy/compose/app-docker-compose.yml up -d
```
