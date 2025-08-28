import { type RefObject, useState } from "react";
import { formatTime } from "@/utils/utils.ts";
import { Button } from "@/components/ui/button.tsx";
import * as React from "react";
import VideoSoundLogo from "@/assets/VideoSoundLogo.tsx";
import VideoMutedLogo from "@/assets/VideoMutedLogo.tsx";
import VideoPauseLogo from "@/assets/VideoPauseLogo.tsx";
import VideoPlayLogo from "@/assets/svg/VideoPlayLogo.tsx";
import VideoFullLogo from "@/assets/VideoFullLogo.tsx";
import VideoExistFullScreenLogo from "@/assets/VideoExitFullScreenLogo.tsx";
import QualitySettings from "@/components/video/QualitySettings.tsx";

interface Props {
  videoRef: RefObject<HTMLVideoElement | null>;
  isPlaying: boolean;
  setIsPlaying: (playing: boolean) => void;
  isMuted: boolean;
  setIsMuted: (muted: boolean) => void;
  quality: string;
  setQuality: (quality: string) => void;
}

interface ToggleButtonProps {
  active: boolean;
  activeIcon: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
  inActiveIcon: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
  onClick: () => void;
}

const ToggleButton = ({
  active,
  activeIcon: ActiveIcon,
  inActiveIcon: InactiveIcon,
  onClick,
}: ToggleButtonProps) => (
  <Button className="bg-transparent hover:bg-transparent" onClick={onClick}>
    {active ? <ActiveIcon /> : <InactiveIcon />}
  </Button>
);

const ControlButtons = ({
  videoRef,
  isPlaying,
  setIsPlaying,
  isMuted,
  setIsMuted,
  quality,
  setQuality,
}: Props) => {
  const [isFullscreen, setIsFullscreen] = useState(false);

  const togglePlay = async () => {
    if (!videoRef.current) return;
    if (videoRef.current.paused) {
      await videoRef.current.play();
      setIsPlaying(true);
    } else {
      videoRef.current.pause();
      setIsPlaying(false);
    }
  };

  const toggleMute = () => {
    if (!videoRef.current) return;
    videoRef.current.muted = !videoRef.current.muted;
    setIsMuted(!videoRef.current.muted);

    console.log("videoRefMuted ", videoRef.current.muted);
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
    <div className="flex gap-2 items-center justify-between">
      <div className="flex items-center justify-center">
        <ToggleButton
          active={isPlaying}
          onClick={togglePlay}
          activeIcon={VideoPauseLogo}
          inActiveIcon={VideoPlayLogo}
        />

        <ToggleButton
          active={isMuted}
          onClick={toggleMute}
          activeIcon={VideoSoundLogo}
          inActiveIcon={VideoMutedLogo}
        />

        <span className="text-sm">
          {formatTime(currentTime)} / {formatTime(duration)}
        </span>
      </div>
      <div className="flex items-center justify-center">
        <QualitySettings videoRef={videoRef} quality={quality} setQuality={setQuality} />
        <ToggleButton
          active={isFullscreen}
          onClick={toggleFullscreen}
          activeIcon={VideoExistFullScreenLogo}
          inActiveIcon={VideoFullLogo}
        />
      </div>
    </div>
  );
};

// @ts-ignore
export default ControlButtons;
