import { useEffect, useRef, useState, useCallback } from 'react';
import { getVideoStatus } from '@/services/video.ts';
import type { JobStatusResponse } from '@/types/video.type.ts';

interface UseVideoStatusOptions {
  enabled?: boolean;
  pollInterval?: number;
  onStatusChange?: (status: JobStatusResponse) => void;
}

export const useVideoStatus = (
  jobId: string | null | undefined,
  options: UseVideoStatusOptions = {}
) => {
  const { enabled = true, pollInterval = 2000, onStatusChange } = options;
  const [status, setStatus] = useState<JobStatusResponse | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const intervalRef = useRef<number | null>(null);
  const onStatusChangeRef = useRef(onStatusChange);

  // Update ref when callback changes
  useEffect(() => {
    onStatusChangeRef.current = onStatusChange;
  }, [onStatusChange]);

  const fetchStatus = useCallback(async () => {
    if (!jobId || !enabled) return;

    setIsLoading(true);
    setError(null);

    try {
      const newStatus = await getVideoStatus(jobId);
      setStatus(newStatus);
      onStatusChangeRef.current?.(newStatus);

      // Stop polling if completed or failed
      if (newStatus.status === 'completed' || newStatus.status === 'failed') {
        if (intervalRef.current) {
          clearInterval(intervalRef.current);
          intervalRef.current = null;
        }
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error('Unknown error');
      setError(error);
      console.error('Failed to fetch video status:', error);
    } finally {
      setIsLoading(false);
    }
  }, [jobId, enabled]);

  useEffect(() => {
    if (!jobId || !enabled) {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      return;
    }

    // Initial fetch
    fetchStatus();

    // Set up polling interval
    intervalRef.current = window.setInterval(() => {
      fetchStatus();
    }, pollInterval);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, [jobId, enabled, pollInterval, fetchStatus]);

  return { status, isLoading, error, refetch: fetchStatus };
};

