import { lazy, Suspense } from "react";
import { Routes, Route } from "react-router-dom";
import { BaseLayout } from "@/pages/layouts";
import { Home } from "@/pages/home";
import { VideoSearch } from "@/pages/video-search";

// Lazy load 404 page as it's less frequently used
const PageNotFound = lazy(() => 
  import("@/pages/404_not_found").then(module => ({ default: module.PageNotFound }))
);

const LoadingFallback = () => (
  <div className="flex items-center justify-center h-screen">
    <div className="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-white" />
  </div>
);

const App: React.FC = () => {
  return (
    <Routes>
      <Route
        path="/"
        element={
          <BaseLayout>
            <Home />
          </BaseLayout>
        }
      />
      <Route
        path="/video/:id"
        element={
          <BaseLayout>
            <Home />
          </BaseLayout>
        }
      />
      <Route
        path="/search"
        element={
          <BaseLayout>
            <VideoSearch />
          </BaseLayout>
        }
      />
      <Route
        path="/*"
        element={
          <BaseLayout>
            <Suspense fallback={<LoadingFallback />}>
              <PageNotFound />
            </Suspense>
          </BaseLayout>
        }
      />
    </Routes>
  );
};

export default App;


