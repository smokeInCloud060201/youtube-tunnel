import React, { useEffect, useRef } from "react";
import Hls from "hls.js";

export interface VideoPlayerProps {
  jobId: string;
}

const VideoPlayer: React.FC<VideoPlayerProps> = ({ jobId }) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const hlsRef = useRef<Hls | null>(null);

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    if (Hls.isSupported()) {
      const hls = new Hls({
        liveDurationInfinity: true,
        liveSyncDurationCount: 3,
      });
      hls.attachMedia(video);
      hls.loadSource(`http://localhost:8080/api/v1/video/${jobId}/playlist`);
      hls.on(Hls.Events.ERROR, (_event, data) => {
        if (data.details === Hls.ErrorDetails.MANIFEST_LOAD_ERROR) {
          console.warn("Retrying playlist load...");
          setTimeout(() => {
            hls.loadSource(`http://localhost:8080/api/v1/video/${jobId}/playlist`);
          }, 3000);
        }
      });
      hlsRef.current = hls;
    } else if (video.canPlayType("application/vnd.apple.mpegurl")) {
      video.src = `http://localhost:8080/api/v1/video/${jobId}/playlist`;
    }

    return () => {
      if (hlsRef.current) {
        hlsRef.current.destroy();
        hlsRef.current = null;
      }
    };
  }, [jobId]);

  return <video ref={videoRef} controls autoPlay style={{ height: "calc(100vh - 120px)" }}/>;
};

export default VideoPlayer;
