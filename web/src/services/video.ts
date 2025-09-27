import { baseApi } from "@/services/api.base.ts";

const loadVideo = (videoId: string) => {
  const videoSource = `https://www.youtube.com/watch?v=${videoId}`
  return baseApi.post(`/api/v1/video?youtubeUrl=${videoSource}`).then(res => res.data);
};

export { loadVideo };
