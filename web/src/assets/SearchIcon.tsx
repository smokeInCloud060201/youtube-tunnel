import CustomIcon from "@/components/icon/CustomIcon.tsx";
import IconSVG from "@/assets/svg/search.svg?react";
import type { IconProps } from "@/types";

const SearchIcon = ({ className, ...rest }: IconProps) => {
  const darkTheme = { color: "white" };
  const lightTheme = { color: "black" };

  return (
    <CustomIcon
      darkTheme={darkTheme}
      lightTheme={lightTheme}
      icon={IconSVG}
      className={className}
      {...rest}
    />
  );
};

export default SearchIcon;
