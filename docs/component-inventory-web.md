# Component Inventory – Web Frontend

## Overview

The web frontend uses a **shadcn/ui** component library built on top of **Radix UI** primitives, styled with **TailwindCSS**. Components are organized into primitive UI building blocks and feature-specific components.

---

## Layout Components

### `BaseLayout`
- **Location:** `src/pages/layouts/`
- **Type:** Page wrapper / Layout
- **Description:** Wraps all pages. Contains the `Navigation` top bar and renders child page content below it.
- **Used by:** All 4 routes in `App.tsx`

---

## Navigation Components

### `Navigation` (assumed)
- **Location:** `src/components/navigation/`
- **Type:** Top-level navigation bar
- **Description:** Application header with navigation links (Home, Search), logo, and theme toggle
- **Contains:** `ModeToggle`, navigation links, possibly `Avatar`

### `ModeToggle`
- **Location:** `src/components/mode_toggle/`
- **Type:** Theme toggle button
- **Description:** Toggles between light and dark mode using the `ThemeProvider` context
- **Dependencies:** `ThemeProvider`, Lucide React icons

### `ThemeProvider`
- **Location:** `src/components/theme-provider.tsx`
- **Type:** Context provider
- **Description:** Manages light/dark theme via CSS class on `documentElement`. Persists preference (localStorage).
- **Pattern:** React Context + `useTheme()` hook

---

## Feature Components

### `VideoPlayer`
- **Location:** `src/components/video/VideoPlayer.tsx`
- **Type:** Media player
- **Description:** HLS video player component powered by `hls.js`. Accepts an M3U8 playlist URL and renders an HTML5 `<video>` element with HLS streaming support.
- **Props:** Playlist URL / job status context (exact props in file)
- **Dependencies:** `hls.js` 1.6
- **Key behavior:**
  - Checks for native HLS support (Safari); falls back to hls.js
  - Handles HLS source attachment and error events
  - Likely shows loading state while job is processing

---

## shadcn/ui Primitive Components (`src/components/ui/`)

These are generated shadcn/ui components wrapping Radix UI primitives:

| Component | Radix Package | Purpose |
|-----------|--------------|---------|
| `Button` | `@radix-ui/react-slot` | Styled button with variants |
| `Select` | `@radix-ui/react-select` | Accessible dropdown select |
| `Avatar` | `@radix-ui/react-avatar` | User avatar with fallback |
| `DropdownMenu` | `@radix-ui/react-dropdown-menu` | Contextual action menu |

---

## Icon Components

### Custom Icon Wrappers
- **Location:** `src/components/icon/`
- **Type:** SVG icon components
- **Description:** Custom SVG icons wrapped as React components (supplementing `lucide-react`)

---

## Page Components

### `Home`
- **Location:** `src/pages/home/`
- **Routes:** `/`, `/video/:id`
- **Description:** Main landing page featuring the video player and URL input. Handles YouTube video ID extraction and the full video loading flow (submit → poll → play).

### `VideoSearch`
- **Location:** `src/pages/video-search/`
- **Route:** `/search`
- **Description:** Search results page. Displays search results from `/v1/search` as a video grid/list. Clicking a result navigates to the home page with the video ID.

### `PageNotFound`
- **Location:** `src/pages/404_not_found/`
- **Route:** `/*`
- **Description:** 404 error page. **Lazy-loaded** via `React.lazy()` for bundle optimization.

---

## Utility / Provider Components

### `ApiContext`
- **Location:** `src/ApiContext.tsx`
- **Type:** React Context
- **Description:** Provides the API base URL as context. Allows components deep in the tree to access the API base URL without prop drilling.

---

## Component Summary Table

| Component | Category | Location | Key Dependencies |
|-----------|----------|----------|-----------------|
| `BaseLayout` | Layout | `pages/layouts/` | `Navigation` |
| `Navigation` | Navigation | `components/navigation/` | `ModeToggle`, `lucide-react` |
| `ModeToggle` | UI Control | `components/mode_toggle/` | `ThemeProvider` |
| `ThemeProvider` | Provider | `components/theme-provider.tsx` | React Context |
| `VideoPlayer` | Feature | `components/video/` | `hls.js` |
| `Button` | Primitive | `components/ui/` | Radix Slot |
| `Select` | Primitive | `components/ui/` | Radix Select |
| `Avatar` | Primitive | `components/ui/` | Radix Avatar |
| `DropdownMenu` | Primitive | `components/ui/` | Radix DropdownMenu |
| `Home` | Page | `pages/home/` | `VideoPlayer`, `video.ts` |
| `VideoSearch` | Page | `pages/video-search/` | `search_service.ts` |
| `PageNotFound` | Page | `pages/404_not_found/` | — |
| `ApiContext` | Provider | `src/ApiContext.tsx` | React Context |

---

## Adding New shadcn/ui Components

```bash
cd web
npx shadcn-ui add <component-name>
# Examples:
npx shadcn-ui add dialog
npx shadcn-ui add toast
npx shadcn-ui add input
```

Components are added to `src/components/ui/` and follow the shadcn/ui styling conventions.

---

## Design System Notes

- **Base color palette**: Configured in `tailwind.config.js` and `globals.css`
- **Theme tokens**: CSS variables (e.g., `--background`, `--foreground`, `--primary`) defined in `styles/`
- **Icon library**: `lucide-react` (tree-shakeable) with custom SVG supplements in `components/icon/`
- **Class utility**: `cn()` helper in `lib/` combines `clsx` + `tailwind-merge` for conditional class composition
