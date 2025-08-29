import { useApi } from "@/services/useApi.ts";

const searchVideo = (query: string) => {
  const api = useApi();
  return api.get("/api/private/search/v1", { params: { query } });
};

export { searchVideo };
