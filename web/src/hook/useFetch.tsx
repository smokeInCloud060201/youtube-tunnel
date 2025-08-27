import type { AxiosResponse } from "axios";
import { useState } from "react";

const useFetch = <T = unknown,>() => {
  const [isLoading, setIsLoading] = useState(false);
  const [data, setData] = useState<T | null>(null);
  const [error, setError] = useState<unknown>(null);

  const fetchData = async (promise: Promise<AxiosResponse<T>>) => {
    setIsLoading(true);
    setError(null);

    try {
      const res = await promise;
      setData(res.data);
    } catch (err) {
      setError(err);
    } finally {
      setIsLoading(false);
    }
  };

  return { isLoading, data, error, fetchData };
};

export default useFetch;
