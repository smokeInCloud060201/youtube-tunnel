import { Routes, Route } from "react-router-dom";
import { Home } from "@/pages/home";
import BaseLayout from "@/pages/layouts/BaseLayout.tsx";

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
    </Routes>
  );
};

export default App;
