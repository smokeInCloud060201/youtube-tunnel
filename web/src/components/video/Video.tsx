import { useRef, useState } from "react";
import VideoControl from "@/components/video/VideoControl.tsx";

const Video = () => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [quality, setQuality] = useState("720p");

  return (
    <div className="app-video relative w-[640px] h-[360px]">
      <video ref={videoRef} className="w-full h-full bg-black" muted>
        <source
          src={`http://localhost:8080/api/stream?url=${encodeURIComponent(
            "https://www.youtube.com/watch?v=n7DBCe4QpIE"
          )}&enableVideo=true&quality=${quality}`}
          type="video/mp4"
        />
      </video>

      {/* Custom Video Controls */}
      <VideoControl videoRef={videoRef} quality={quality} setQuality={setQuality} />
    </div>
  );
};

export default Video;
