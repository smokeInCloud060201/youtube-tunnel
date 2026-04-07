# Architecture – Web Frontend

## Executive Summary

The `web` service is a **React 18 Single-Page Application (SPA)** built with Vite and TypeScript. It provides the user interface for the YouTube Tunnel platform: a YouTube-style search UI and an HLS video player backed by `hls.js`.

The frontend is stateless – it communicates exclusively with the `api` service and does not have a backend of its own.

---

## Technology Stack

| Category | Technology | Version |
|----------|-----------|---------|
| Language | TypeScript | ~5.8 |
| UI Framework | React | 18.3 |
| Build Tool | Vite | 7.1 |
| Styling | TailwindCSS | 3 |
| Component Library | shadcn/ui (Radix UI primitives) | Latest |
| Icons | Lucide React | 0.542 |
| HLS Player | hls.js | 1.6 |
| HTTP Client | Axios | 0.14 |
| Routing | React Router | 6 |
| Class Utilities | clsx + tailwind-merge | 2.x / 3.x |

---

## Architecture Pattern

**Feature-based SPA** with React Router for client-side routing:

```
main.tsx (BrowserRouter + ThemeProvider)
  └── App.tsx (Route definitions)
       ├── / and /video/:id → BaseLayout > Home
       ├── /search         → BaseLayout > VideoSearch
       └── /*              → BaseLayout > PageNotFound (lazy)
```

### Component Hierarchy

```
ThemeProvider (CSS variable theming)
  └── BrowserRouter
       └── App (Routes)
            └── BaseLayout (Navigation + Page wrapper)
                 ├── Home
                 │    ├── VideoPlayer (hls.js)
                 │    └── Search/Input area
                 └── VideoSearch
                      └── Search results grid
```

---

## Directory Structure

```
src/
├── main.tsx             # Entry point: ReactDOM.createRoot, providers
├── App.tsx              # Router: 4 routes
├── ApiContext.tsx       # React context for API base URL
├── vite-env.d.ts        # Vite environment type declarations
│
├── pages/
│   ├── home/            # Home page: video player + URL input
│   ├── video-search/    # Search results listing
│   ├── layouts/         # BaseLayout component (wraps all pages)
│   └── 404_not_found/   # Not-found page (lazy-loaded)
│
├── components/
│   ├── ui/              # shadcn/ui: Button, Select, Avatar, DropdownMenu
│   ├── video/
│   │   └── VideoPlayer.tsx   # hls.js HLS video player
│   ├── navigation/      # Top navigation bar
│   ├── mode_toggle/     # Dark/light theme toggle button
│   ├── icon/            # Custom SVG icon components
│   └── theme-provider.tsx # Theme context + CSS variable switching
│
├── services/
│   ├── api.base.ts      # Axios instance: baseURL, 30s timeout, interceptors
│   ├── video.ts         # loadVideo(), getVideoStatus(), getVideoPlaylist()
│   └── search_service.ts # searchVideos()
│
├── hooks/               # Custom React hooks
├── hook/                # Additional hooks (possible consolidation candidate)
├── lib/                 # Utility functions (cn() helper)
├── types/               # TypeScript interfaces (video.type.ts, etc.)
├── utils/
│   └── app.config.ts    # VITE_* env var exports (URL_BASE_HOST)
└── styles/              # Global CSS / Tailwind base layer
```

---

## Routing

| Route | Page | Description |
|-------|------|-------------|
| `/` | `Home` | Main page with video player (if video URL param present) |
| `/video/:id` | `Home` | Video player with specific YouTube video ID |
| `/search` | `VideoSearch` | Search results page |
| `/*` | `PageNotFound` | 404 catch-all (lazy-loaded for performance) |

---

## API Integration

The frontend communicates with the backend API via a centralized **Axios instance** (`services/api.base.ts`):

- **Base URL**: configured via `VITE_API_BASE_URL` env variable (accessed through `utils/app.config.ts`)
- **Timeout**: 30 seconds
- **Request interceptor**: Appends `_={timestamp}` to GET requests to prevent caching
- **Response interceptor**: Logs API errors to console

### Service Functions

| Function | HTTP Call | Description |
|----------|-----------|-------------|
| `loadVideo(videoId)` | `POST /v1/video-player?youtubeUrl=...` | Submit YouTube video for processing |
| `getVideoStatus(jobId)` | `GET /v1/video-player/{id}/status` | Poll job status and progress |
| `getVideoPlaylist(jobId)` | `GET /v1/video-player/{id}/playlist` | Fetch HLS M3U8 playlist |
| `searchVideos(query)` | `GET /v1/search?q=...` | Search YouTube videos |

---

## Video Playback Flow (hls.js)

1. User enters YouTube video URL or ID
2. `loadVideo(id)` → `POST /v1/video-player` → returns `{ jobId, status }`
3. Poll `getVideoStatus(jobId)` at intervals
4. When `status === "completed"`:
   - Call `getVideoPlaylist(jobId)` to get M3U8 content
   - Initialize `hls.js` with the playlist URL pointing to MinIO
   - hls.js fetches `.ts` segments directly from MinIO for playback
5. `hls.js` handles buffering, seeking, and segment management

---

## Theming

The app uses a **CSS variable-based theme system**:
- `ThemeProvider` toggles a `dark` class on the document root
- TailwindCSS `darkMode: "class"` reads this class
- shadcn/ui components inherit colors from CSS variables
- User preference is persisted (likely in `localStorage`)

---

## Build Configuration

| Config File | Purpose |
|-------------|---------|
| `vite.config.ts` | Vite bundler config, path aliases (`@/` → `src/`) |
| `tsconfig.json` | TypeScript project references |
| `tsconfig.app.json` | App-specific TS config (strict, bundler module resolution) |
| `tailwind.config.js` | TailwindCSS content paths + custom theme extensions |
| `components.json` | shadcn/ui CLI config (style, base color, import paths) |
| `eslint.config.js` | Flat config ESLint with TypeScript + React rules |
| `prettier.config.js` | Code formatting rules |

---

## Configuration (Environment Variables)

| Variable | Description | Example |
|----------|-------------|---------|
| `VITE_API_BASE_URL` | Backend API base URL | `http://localhost:8080` |

> Environment files: `.env`, `.env.local`, `.env.production` (Vite conventions)

---

## Production Deployment

The web app is served as a static SPA by **Nginx** inside Docker:
- `web.Dockerfile` builds with `vite build`
- Output is served by Nginx from `nginx/` config
- Traefik provides SSL termination and routing to `yt.sonbn.xyz`

---

## Key Dependencies Notes

| Dependency | Purpose | Note |
|-----------|---------|------|
| `hls.js` | HLS video playback in browser | Core streaming technology |
| `@radix-ui/*` | Accessible UI primitives | Via shadcn/ui |
| `@tailwindcss/vite` | Vite-integrated Tailwind | v4 plugin (CSS-first config) |
| `shadcn-ui` | Component generator CLI | Run `npx shadcn-ui add <component>` to add new ones |
| `vite-plugin-svgr` | SVG as React components | Used for icon imports |
