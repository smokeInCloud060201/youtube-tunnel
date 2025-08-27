import { type RefObject } from "react";
import { formatTime } from "@/utils/utils.ts";
import { Input } from "@/components/ui/input.tsx";
import * as React from "react";

interface Props {
  videoRef: RefObject<HTMLVideoElement | null>;
  progress: number;
  setProgress: (value: number) => void;
}

const ProgressBar = ({ videoRef, progress, setProgress }: Props) => {
  const [hoverTime, setHoverTime] = React.useState<number | null>(null);

  const handleSeek = (value: number) => {
    if (!videoRef.current || !videoRef.current.duration) return;
    videoRef.current.currentTime = (value / 100) * videoRef.current.duration;
    setProgress(value);
  };

  const handleMouseMove = (e: React.MouseEvent<HTMLInputElement>) => {
    if (!videoRef.current || !videoRef.current.duration) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    setHoverTime(percent * videoRef.current.duration);
  };

  const handleMouseLeave = () => setHoverTime(null);

  const duration = videoRef.current?.duration || 0;

  return (
    <div className="relative">
      <Input
        type="range"
        min={0}
        max={100}
        value={progress}
        onChange={(e) => handleSeek(Number(e.target.value))}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
        className="w-full mb-1"
      />
      {hoverTime !== null && (
        <div
          className="absolute text-xs bg-black px-1 rounded -top-6"
          style={{ left: `${(hoverTime / duration) * 100}%`, transform: "translateX(-50%)" }}
        >
          {formatTime(hoverTime)}
        </div>
      )}
    </div>
  );
};

export default ProgressBar;
