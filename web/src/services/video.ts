import { baseApi } from "@/services/api.base.ts";
import type { VideoPlayerResponse, JobStatusResponse } from "@/types/video.type.ts";

const loadVideo = async (videoId: string, isAudio: boolean = false): Promise<VideoPlayerResponse> => {
  const videoSource = `https://www.youtube.com/watch?v=${videoId}`;
  const isAudioParam = isAudio ? '&isAudio=true' : '';
  const response = await baseApi.post<VideoPlayerResponse>(
    `/v1/video-player?youtubeUrl=${encodeURIComponent(videoSource)}${isAudioParam}`
  );
  return response.data;
};

const getVideoStatus = async (jobId: string): Promise<JobStatusResponse> => {
  const response = await baseApi.get<JobStatusResponse>(
    `/v1/video-player/${jobId}/status`
  );
  return response.data;
};

const getVideoPlaylist = async (jobId: string): Promise<string> => {
  const response = await baseApi.get<string>(
    `/v1/video-player/${jobId}/playlist`,
    {
      responseType: 'text',
    }
  );
  return response.data;
};

export { loadVideo, getVideoStatus, getVideoPlaylist };
