import { useLocation } from "react-router-dom";
import { useCallback, useEffect, useState } from "react";
import VideoSearchList from "@/pages/video-search/VideoSearchList.tsx";
import { searchVideo } from "@/services/search_service.ts";

const VideoSearch = () => {
  const { search } = useLocation();
  const query = new URLSearchParams(search);
  const queryText = query.get("query");

  const [searchResult, setSearchResult] = useState([]);

  const handleSearch = useCallback(async (queryText: string | null) => {
    if (queryText) {
      const { data } = await searchVideo(queryText);
      setSearchResult(data);
    }
  }, []);

  useEffect(() => {
    handleSearch(queryText);
  }, [handleSearch, queryText]);

  return (
    <div>
      <VideoSearchList items={searchResult} />
    </div>
  );
};

export default VideoSearch;
