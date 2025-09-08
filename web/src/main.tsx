import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@/styles/index.scss";
import App from "./App.tsx";
import { BrowserRouter } from "react-router-dom";
import { ThemeProvider } from "@/components";
import { ApiProvider } from "@/ApiContext.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <ApiProvider>
          <App />
        </ApiProvider>
      </ThemeProvider>
    </BrowserRouter>
  </StrictMode>
);
