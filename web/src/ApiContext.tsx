import { createContext, useContext, useEffect, useMemo } from "react";
import { baseApi } from "@/services";
import type { AxiosInstance } from "axios";

const ApiContext = createContext<AxiosInstance | null>(null);

export const ApiProvider = ({ children }: { children: React.ReactNode }) => {
  useEffect(() => {
    const interceptor = baseApi.interceptors.request.use(async (config) => {
      return config;
    });

    return () => {
      baseApi.interceptors.request.eject(interceptor);
    };
  }, []);

  const api = useMemo(() => baseApi, []);

  return <ApiContext.Provider value={api}>{children}</ApiContext.Provider>;
};

export const useApi = () => {
  const ctx = useContext(ApiContext);
  if (!ctx) {
    throw new Error("useApi must be used within an ApiProvider");
  }
  return ctx;
};
