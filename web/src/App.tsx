import { Routes, Route } from "react-router-dom";
import { Home } from "@/pages/home";
import BaseLayout from "@/pages/layouts/BaseLayout.tsx";
import NotFoundPage from "@/pages/404_not_found/NotFoundPage.tsx";

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
        path="/*"
        element={
          <BaseLayout>
            <NotFoundPage />
          </BaseLayout>
        }
      />
    </Routes>
  );
};

export default App;
