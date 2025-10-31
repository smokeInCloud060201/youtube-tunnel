import { useLocation } from "react-router-dom";
import { useCallback, useEffect, useState, useMemo } from "react";
import VideoSearchList from "@/pages/video-search/VideoSearchList.tsx";
import { searchVideo } from "@/services/search_service.ts";
import type { VideoItemProps } from "@/types/video.type.ts";

const VideoSearch: React.FC = () => {
  const { search } = useLocation();
  const queryText = useMemo(() => {
    const query = new URLSearchParams(search);
    return query.get("query");
  }, [search]);

  const [searchResult, setSearchResult] = useState<VideoItemProps[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const handleSearch = useCallback(async (query: string | null) => {
    if (!query) {
      setSearchResult([]);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const data = await searchVideo(query);
      setSearchResult(data);
    } catch (err) {
      const error = err instanceof Error ? err : new Error('Failed to search videos');
      setError(error);
      console.error('Search error:', error);
      setSearchResult([]);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    handleSearch(queryText);
  }, [handleSearch, queryText]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <div className="text-center">
          <div className="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-white mb-4" />
          <p className="text-lg">Searching videos...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <div className="text-center">
          <p className="text-xl text-red-500 mb-4">Error searching videos</p>
          <p className="text-gray-400 mb-4">{error.message}</p>
          <button
            onClick={() => handleSearch(queryText)}
            className="px-4 py-2 bg-blue-500 rounded hover:bg-blue-600"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div>
      <VideoSearchList items={searchResult} />
    </div>
  );
};

export default VideoSearch;

