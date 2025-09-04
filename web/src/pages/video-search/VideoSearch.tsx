import { useLocation } from "react-router-dom";
import { useCallback, useEffect, useState } from "react";
import { baseApi } from "@/services";
import VideoSearchList from "@/pages/video-search/VideoSearchList.tsx";

const VideoSearch = () => {
  const { search } = useLocation();
  const query = new URLSearchParams(search);
  const queryText = query.get("query");

  const [searchResult, setSearchResult] = useState([]);

  const handleSearch = useCallback(async (queryText: string | null) => {
    if (queryText) {
      const { data } = await baseApi.get("/api/private/search/v1", {
        headers: { Authorization: `Bearer ${queryText}` },
        params: { q: queryText },
      });
      setSearchResult(data);
    }
  }, []);

  useEffect(() => {
    handleSearch(queryText);
  }, [queryText]);

  return (
    <div>
      <VideoSearchList items={searchResult} />
    </div>
  );
};

export default VideoSearch;
