import { useAuth0 } from "@auth0/auth0-react";
import { useEffect } from "react";
import { baseApi } from "./api.base";

export const useApi = () => {
  const { getAccessTokenSilently } = useAuth0();

  useEffect(() => {
    const interceptor = baseApi.interceptors.request.use(async (config) => {
      const token = await getAccessTokenSilently();
      config.headers.Authorization = `Bearer ${token}`;
      return config;
    });

    return () => {
      baseApi.interceptors.request.eject(interceptor);
    };
  }, [getAccessTokenSilently]);

  return baseApi;
};
