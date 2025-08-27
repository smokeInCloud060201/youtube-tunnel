import { useRef, useState } from "react";
import VideoQuality from "@/components/video/VideoQuality.tsx";
import SettingLogo from "@/assets/SettingLogo.tsx";

const Video = () => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [quality, setQuality] = useState("720p");
  const [showQualityMenu, setShowQualityMenu] = useState(false);

  return (
    <div className="app-video relative w-[400px]">
      <video className="block w-full" ref={videoRef} height={360} controls muted>
        <source
          src={`http://localhost:8080/api/stream?url=${encodeURIComponent(
            "https://www.youtube.com/watch?v=n7DBCe4QpIE"
          )}&enableVideo=true&quality=${quality}`}
          type="video/mp4"
        />
      </video>

      <div
        className="absolute bottom-[5%] right-[11%] cursor-pointer z-10"
        onClick={() => setShowQualityMenu(!showQualityMenu)}
      >
        <SettingLogo />
      </div>

      {showQualityMenu && (
        <div className="absolute top-10 right-2.5 bg-white p-2.5 rounded z-20">
          <VideoQuality
            value={quality}
            videoRef={videoRef}
            onChangeQuality={(q) => {
              setQuality(q);
              setShowQualityMenu(false);
            }}
          />
        </div>
      )}
    </div>
  );
};

export default Video;
