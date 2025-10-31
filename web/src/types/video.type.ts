export interface VideoItemProps {
  id: string;
  title: string;
  description: string;
  thumbnails: string[];
  channelTitle: string;
  publishTime: string;
  publishedAt: string;
}

export interface VideoPlayerResponse {
  jobId: string;
  status: string;
}

export interface JobStatusResponse {
  status: 'pending' | 'processing' | 'completed' | 'failed' | 'unknown';
  progress: number | null;
}
