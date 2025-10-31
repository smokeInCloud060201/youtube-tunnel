import { baseApi } from "@/services/api.base.ts";
import type { VideoItemProps } from "@/types/video.type.ts";

const searchVideo = async (query: string): Promise<VideoItemProps[]> => {
  const response = await baseApi.get<VideoItemProps[]>("/v1/search", {
    params: { q: query },
  });
  return response.data;
};

export { searchVideo };
