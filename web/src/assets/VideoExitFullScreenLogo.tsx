import type { IconProps } from "@/types";
import CustomIcon from "@/components/icon/CustomIcon.tsx";
import IconSVG from "@/assets/svg/video-exists-full-screen.svg?react";

const VideoExistFullScreenLogo = ({ className }: IconProps) => {
  const darkTheme = { color: "white", width: 24, height: 24 };
  const lightTheme = { color: "white", width: 24, height: 24 };

  return (
    <CustomIcon
      darkTheme={darkTheme}
      lightTheme={lightTheme}
      icon={IconSVG}
      className={className}
    />
  );
};

export default VideoExistFullScreenLogo;
