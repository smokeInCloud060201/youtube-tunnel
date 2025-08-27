import { api } from "@/services/api.base.ts";

const getVideoByLink = (url: string) => {
  return api.get("/api/stream", { params: { url } });
};

export { getVideoByLink };
