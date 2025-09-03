import { useRef, useState } from "react";
import VideoControl from "@/components/video/VideoControl.tsx";

interface Props {
  id: string;
}

const Video = ({ id }: Props) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [quality, setQuality] = useState("720p");

  const handleQualityChange = async (newQuality: string) => {
    if (!videoRef?.current) return;

    const video = videoRef.current;
    const currentTime = video.currentTime;
    const isPlaying = !video.paused;

    setQuality(newQuality);

    video.src = `http://yt.sonbn.xyz/api/public/stream/v1?url=${id}&enableVideo=true&quality=${newQuality}`;

    video.onloadedmetadata = async () => {
      video.currentTime = currentTime;
      if (isPlaying) {
        await video.play();
      }
      video.onloadedmetadata = null;
    };

    video.load();
  };

  return (
    <div className="app-video relative w-[640px] h-[360px]">
      <video ref={videoRef} className="w-full h-full bg-black" muted>
        <source
          src={`http://yt.sonbn.xyz/api/public/stream/v1?url=${id}&enableVideo=true&quality=${quality}`}
          type="video/mp4"
        />
      </video>

      {/* Custom Video Controls */}
      <VideoControl
        videoRef={videoRef}
        quality={quality}
        handleQualityChange={handleQualityChange}
      />
    </div>
  );
};

export default Video;
