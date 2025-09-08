import { useEffect } from "react";
import { baseApi } from "./api.base";

export const useApi = () => {
  useEffect(() => {
    const interceptor = baseApi.interceptors.request.use(async (config) => {
      return config;
    });

    return () => {
      baseApi.interceptors.request.eject(interceptor);
    };
  }, []);

  return baseApi;
};
