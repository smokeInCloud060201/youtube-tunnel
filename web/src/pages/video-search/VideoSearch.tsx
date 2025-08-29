import { useLocation } from "react-router-dom";
import { useCallback, useEffect, useState } from "react";
import { useApi } from "@/ApiContext.tsx";

const VideoSearch = () => {
  const { search } = useLocation();
  const query = new URLSearchParams(search);
  const queryText = query.get("query");

  const api = useApi();

  const [searchResult, setSearchResult] = useState([]);

  const handleSearch = useCallback(async () => {
    if (queryText) {
      const { data } = await api.get("/api/private/search/v1", { params: { q: queryText } });
      console.log("Data ", data);
      setSearchResult(data);
    }
  }, [queryText]);

  useEffect(() => {
    handleSearch();
  }, []);

  return <div>{searchResult}</div>;
};

export { VideoSearch };
