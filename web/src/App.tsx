import { Routes, Route } from "react-router-dom";
import { BaseLayout } from "@/pages/layouts";
import { Home } from "@/pages/home";
import { VideoSearch } from "@/pages/video-search";
import { PageNotFound } from "@/pages/404_not_found";

const App = () => {
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
            <PageNotFound />
          </BaseLayout>
        }
      />
    </Routes>
  );
};

export default App;
