import { useState } from "react";
import VideoQuality from "./VideoQuality";
import SettingLogo from "@/assets/SettingLogo.tsx";

interface Props {
  quality: string;
  handleQualityChange: (quality: string) => void;
}

const QualitySettings = ({ quality, handleQualityChange }: Props) => {
  const [showQualityMenu, setShowQualityMenu] = useState(false);

  return (
    <div className="relative">
      <button onClick={() => setShowQualityMenu(!showQualityMenu)} className="cursor-pointer">
        <SettingLogo />
      </button>

      {showQualityMenu && (
        <div className="absolute bottom-8 right-0 bg-black bg-opacity-80 p-2 rounded">
          <VideoQuality
            value={quality}
            onChangeQuality={(p) => {
              setShowQualityMenu(false);
              handleQualityChange(p);
            }}
          />
        </div>
      )}
    </div>
  );
};

export default QualitySettings;
