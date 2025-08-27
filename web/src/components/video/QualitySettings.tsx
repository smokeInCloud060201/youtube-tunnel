import { type RefObject, useState } from "react";
import VideoQuality from "./VideoQuality";
import SettingLogo from "@/assets/SettingLogo.tsx";

interface Props {
  videoRef: RefObject<HTMLVideoElement | null>;
  quality: string;
  setQuality: (q: string) => void;
}

const QualitySettings = ({ videoRef, quality, setQuality }: Props) => {
  const [showQualityMenu, setShowQualityMenu] = useState(false);

  return (
    <div className="relative">
      <div onClick={() => setShowQualityMenu(!showQualityMenu)} className="cursor-pointer">
        <SettingLogo />
      </div>

      {showQualityMenu && (
        <div className="absolute bottom-8 right-0 bg-black bg-opacity-80 p-2 rounded">
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

export default QualitySettings;
