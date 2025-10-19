import CustomIcon from "@/components/icon/CustomIcon.tsx";
import IconSVG from "@/assets/svg/NavMenu.svg?react";
import type { IconProps } from "@/types";

const NavMenu = ({ className, ...rest }: IconProps) => {
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

export default NavMenu;
