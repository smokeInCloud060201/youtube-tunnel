import CustomIcon from "@/components/icon/CustomIcon.tsx";
import YoutubeLogoSVG from "@/assets/svg/Youtube-Logo.svg?react";
import type { IconProps } from "@/types";

const YoutubeLogo = ({ className, ...rest }: IconProps) => {
  const darkTheme = { color: "white" };
  const lightTheme = { color: "black" };

  return (
    <CustomIcon
      darkTheme={darkTheme}
      lightTheme={lightTheme}
      icon={YoutubeLogoSVG}
      className={className}
      {...rest}
    />
  );
};

export default YoutubeLogo;
