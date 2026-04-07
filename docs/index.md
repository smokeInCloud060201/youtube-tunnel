# YouTube Tunnel – Documentation Index

> **Primary entry point for AI-assisted development.** All documentation about this project is indexed here.

---

## Project Overview

- **Type:** Multi-part monorepo (3 services)
- **Primary Language:** Rust (backend), TypeScript (frontend)
- **Architecture:** Microservice-style with Redis queue and MinIO object storage
- **Production URL:** `https://yt.sonbn.xyz`

---

## Quick Reference

### Parts

| Part | Root | Type | Tech Stack |
|------|------|------|-----------|
| `api` | `backend/api/` | REST API Service | Rust · Actix-web 4 · Redis · MinIO |
| `worker` | `backend/worker/` | Async Job Consumer | Rust · Tokio · yt-dlp · FFmpeg · MinIO |
| `web` | `web/` | React SPA | TypeScript · React 18 · Vite 7 · TailwindCSS |

### Getting Started (Local Dev)

```bash
# 1. Start infrastructure
docker network create yt-network
docker-compose -f deploy/compose/base-docker-compose.local.yml up -d

# 2. Start API
cd backend/api && cargo run

# 3. Start Worker  
cd backend/worker && cargo run

# 4. Start Web
cd web && yarn dev
```

→ Open `http://localhost:5173`

---

## Generated Documentation

### Architecture

- [Project Overview](./project-overview.md)
- [Architecture – API Service](./architecture-api.md)
- [Architecture – Worker Service](./architecture-worker.md)
- [Architecture – Web Frontend](./architecture-web.md)
- [Integration Architecture](./integration-architecture.md)

### Reference

- [API Contracts](./api-contracts-api.md)
- [Source Tree Analysis](./source-tree-analysis.md)
- [Component Inventory – Web](./component-inventory-web.md)

### Guides

- [Development Guide](./development-guide.md)
- [Deployment Guide](./deployment-guide.md)

---

## Using This Documentation for AI-Assisted Development

### For UI-only features
Reference: [Architecture – Web](./architecture-web.md) + [Component Inventory](./component-inventory-web.md)

### For API-only features
Reference: [Architecture – API](./architecture-api.md) + [API Contracts](./api-contracts-api.md)

### For worker / processing features
Reference: [Architecture – Worker](./architecture-worker.md)

### For full-stack features
Reference: All three architecture docs + [Integration Architecture](./integration-architecture.md)

### For brownfield PRD creation
Start from this index and the relevant architecture docs. Then run the PRD workflow.

---

## Scan Metadata

- **Generated:** 2026-04-07
- **Scan Level:** Deep
- **Workflow Mode:** initial_scan
- **State File:** [project-scan-report.json](./project-scan-report.json)
- **Parts Metadata:** [project-parts.json](./project-parts.json)
