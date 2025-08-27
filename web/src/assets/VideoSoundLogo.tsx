import type { IconProps } from "@/types";
import CustomIcon from "@/components/icon/CustomIcon.tsx";
import IconSVG from "@/assets/svg/video-sound.svg?react";

const VideoSoundLogo = ({ className }: IconProps) => {
  const darkTheme = { color: "white" };
  const lightTheme = { color: "white" };

  return (
    <CustomIcon
      darkTheme={darkTheme}
      lightTheme={lightTheme}
      icon={IconSVG}
      className={className}
    />
  );
};

export default VideoSoundLogo;
