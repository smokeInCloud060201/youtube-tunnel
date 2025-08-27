import type { IconProps } from "@/types";
import CustomIcon from "@/components/icon/CustomIcon.tsx";
import IconSVG from "@/assets/svg/video-pause.svg?react";

const VideoPauseLogo = ({ className }: IconProps) => {
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

export default VideoPauseLogo;
