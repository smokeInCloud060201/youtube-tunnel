import type { IconProps } from "@/types";
import CustomIcon from "@/components/icon/CustomIcon.tsx";
import IconSVG from "@/assets/svg/video-full-screen.svg?react";

const VideoFullLogo = ({ className }: IconProps) => {
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

export default VideoFullLogo;
