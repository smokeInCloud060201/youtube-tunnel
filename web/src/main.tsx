import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@/styles/index.scss";
import App from "./App.tsx";
import { BrowserRouter } from "react-router-dom";
import { ThemeProvider } from "@/components";
import { Auth0Provider } from "@auth0/auth0-react";
import { ApiProvider } from "@/ApiContext.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Auth0Provider
      domain="dev-fcog2dztfmr14aao.us.auth0.com"
      clientId="2GU26qrp4MaAsqyXDKxhbt6HUTzJlaSZ"
      authorizationParams={{
        redirect_uri: `${window.location.origin}/callback`,
        audience: "http://localhost:8080",
        prompt: "consent",
        scope: "openid profile email offline_access",
      }}
      cacheLocation="localstorage"
      useRefreshTokens={true}
    >
      <BrowserRouter>
        <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
          <ApiProvider>
            <App />
          </ApiProvider>
        </ThemeProvider>
      </BrowserRouter>
    </Auth0Provider>
  </StrictMode>
);
