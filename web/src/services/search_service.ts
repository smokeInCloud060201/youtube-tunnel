import { baseApi } from "@/services/api.base.ts";

const searchVideo = (query: string) => {
  return baseApi.get("/api/v1/search", { params: { q: query } });
};

export { searchVideo };
