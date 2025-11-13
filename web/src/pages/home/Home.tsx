import { useParams } from "react-router-dom";
import { loadVideo } from "@/services/video.ts";
import VideoPlayer from "@/components/video/VideoPlayer.tsx";
import { useEffect, useState, useCallback } from "react";
import type { VideoPlayerResponse } from "@/types/video.type.ts";

const Home: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [jobId, setJobId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const handleLoadVideo = useCallback(async (videoId: string) => {
    setIsLoading(true);
    setError(null);
    setJobId(null);

    try {
      const data: VideoPlayerResponse = await loadVideo(videoId);
      setJobId(data.jobId);
    } catch (err) {
      const error = err instanceof Error ? err : new Error('Failed to load video');
      setError(error);
      console.error('Failed to load video:', error);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    if (id) {
      handleLoadVideo(id);
    } else {
      setJobId(null);
      setError(null);
    }
  }, [id, handleLoadVideo]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-center">
          <div className="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-white mb-4" />
          <p className="text-lg">Loading video...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-center">
          <p className="text-xl text-red-500 mb-4">Error loading video</p>
          <p className="text-gray-400 mb-4">{error.message}</p>
          <button
            onClick={() => id && handleLoadVideo(id)}
            className="px-4 py-2 bg-blue-500 rounded hover:bg-blue-600"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  if (!id) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-center">
          <p className="text-xl mb-4">Welcome to YouTube Tunnel</p>
          <p className="text-gray-400">Enter a video ID in the URL or search for videos</p>
        </div>
      </div>
    );
  }

  if (!jobId) {
    return null;
  }

  return (
    <div>
      <VideoPlayer jobId={jobId} />
    </div>
  );
};

export default Home;
