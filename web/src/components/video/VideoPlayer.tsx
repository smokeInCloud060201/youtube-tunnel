import React, { useEffect, useRef } from "react";
import Hls from "hls.js";
import { URL_BASE_HOST } from "@/utils/app.config.ts";

export interface VideoPlayerProps {
  jobId: string;
  isAudio?: boolean;
}

const VideoPlayer: React.FC<VideoPlayerProps> = React.memo(({ jobId, isAudio = false }) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const hlsRef = useRef<Hls | null>(null);

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    video.muted = !isAudio; // Unmute for audio-only mode
    video.playsInline = true;

    if (Hls.isSupported()) {
      const hls = new Hls({
        liveDurationInfinity: true,
        liveSyncDurationCount: 3,
      });
      hls.attachMedia(video);
      hls.loadSource(`${URL_BASE_HOST}/v1/video-player/${jobId}/playlist`);

      hls.on(Hls.Events.MANIFEST_PARSED, () => {
        video.play().catch((err) => {
          console.warn("Autoplay failed:", err);
        });
      });

      hls.on(Hls.Events.MANIFEST_LOADED, (_event, data) => {
        const { levels } = data;
        // Detect if the manifest has no segments
        const hasNoSegments =
          !levels || levels.every((lvl: any) => !lvl.details || lvl.details.fragments.length === 0);
        if (hasNoSegments) {
          console.warn("Manifest has no segments — reloading...");
          setTimeout(() => {
            hls.loadSource(`${URL_BASE_HOST}/v1/video-player/${jobId}/playlist?_=${Date.now()}`);
          }, 3000);
        }
      });

      hls.on(Hls.Events.ERROR, (_event, data) => {
        if (data.details === Hls.ErrorDetails.MANIFEST_LOAD_ERROR) {
          console.warn("Retrying playlist load...");
          setTimeout(() => {
            hls.loadSource(`${URL_BASE_HOST}/v1/video-player/${jobId}/playlist?_=${Date.now()}`);
          }, 3000);
        }
      });

      hlsRef.current = hls;
    } else if (video.canPlayType("application/vnd.apple.mpegurl")) {
      video.src = `${URL_BASE_HOST}/v1/video-player/${jobId}/playlist?_=${Date.now()}`;
    }

    return () => {
      if (hlsRef.current) {
        hlsRef.current.destroy();
        hlsRef.current = null;
      }
    };
  }, [jobId, isAudio]);

  return (
    <video
      ref={videoRef}
      controls
      autoPlay
      playsInline
      muted={!isAudio}
      style={{
        height: isAudio ? "auto" : "calc(100vh - 120px)",
        width: "100%",
        margin: "0 24px",
        maxHeight: isAudio ? "200px" : undefined,
      }}
      className="rounded-lg shadow-lg"
    />
  );
});

VideoPlayer.displayName = 'VideoPlayer';

export default VideoPlayer;
