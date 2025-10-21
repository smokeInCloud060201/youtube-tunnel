import { baseApi } from "@/services/api.base.ts";

const loadVideo = (videoId: string) => {
  const videoSource = `https://www.youtube.com/watch?v=${videoId}`
  return baseApi.post(`/v1/video-player?youtubeUrl=${videoSource}`).then(res => {
      return res.data
  });
};

export { loadVideo };
