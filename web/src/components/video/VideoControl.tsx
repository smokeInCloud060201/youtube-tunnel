import { type RefObject, useState, useEffect } from "react";
import ProgressBar from "./ProgressBar";
import ControlButtons from "./ControlButtons";
import QualitySettings from "./QualitySettings";

interface Props {
  videoRef: RefObject<HTMLVideoElement | null>;
  quality: string;
  setQuality: (q: string) => void;
}

const VideoControl = ({ videoRef, quality, setQuality }: Props) => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [isMuted, setIsMuted] = useState(true);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      if (videoRef.current && videoRef.current.duration) {
        setProgress((videoRef.current.currentTime / videoRef.current.duration) * 100);
      }
    }, 500);
    return () => clearInterval(interval);
  }, [videoRef]);

  return (
    <div className="absolute bottom-0 left-0 w-full p-2 bg-black bg-opacity-50 text-white flex flex-col">
      <ProgressBar videoRef={videoRef} progress={progress} setProgress={setProgress} />
      <div className="flex justify-between items-center">
        <ControlButtons
          videoRef={videoRef}
          isPlaying={isPlaying}
          setIsPlaying={setIsPlaying}
          isMuted={isMuted}
          setIsMuted={setIsMuted}
        />
        <QualitySettings videoRef={videoRef} quality={quality} setQuality={setQuality} />
      </div>
    </div>
  );
};

export default VideoControl;
