import { type RefObject, useState } from "react";
import { formatTime } from "@/utils/utils.ts";

interface Props {
  videoRef: RefObject<HTMLVideoElement | null>;
  isPlaying: boolean;
  setIsPlaying: (playing: boolean) => void;
  isMuted: boolean;
  setIsMuted: (muted: boolean) => void;
}

const ControlButtons = ({ videoRef, isPlaying, setIsPlaying, isMuted, setIsMuted }: Props) => {
  const [isFullscreen, setIsFullscreen] = useState(false);

  const togglePlay = () => {
    if (!videoRef.current) return;
    if (videoRef.current.paused) {
      videoRef.current.play();
      setIsPlaying(true);
    } else {
      videoRef.current.pause();
      setIsPlaying(false);
    }
  };

  const toggleMute = () => {
    if (!videoRef.current) return;
    videoRef.current.muted = !videoRef.current.muted;
    setIsMuted(videoRef.current.muted);
  };

  const toggleFullscreen = () => {
    if (!videoRef.current) return;

    const container = videoRef.current.parentElement;
    if (!container) return;

    if (!document.fullscreenElement) {
      container.requestFullscreen?.();
      setIsFullscreen(true);
    } else {
      document.exitFullscreen?.();
      setIsFullscreen(false);
    }
  };

  const currentTime = videoRef.current?.currentTime || 0;
  const duration = videoRef.current?.duration || 0;

  return (
    <div className="flex gap-2 items-center">
      <button onClick={togglePlay}>{isPlaying ? "⏸" : "▶️"}</button>
      <button onClick={toggleMute}>{isMuted ? "🔇" : "🔊"}</button>
      <span className="text-sm">
        {formatTime(currentTime)} / {formatTime(duration)}
      </span>
      <button onClick={toggleFullscreen}>{isFullscreen ? "🡼" : "⛶"}</button>
    </div>
  );
};

export default ControlButtons;
