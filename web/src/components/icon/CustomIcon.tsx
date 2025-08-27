import * as React from "react";
import { useTheme } from "@/components";

interface Props extends React.SVGProps<SVGSVGElement> {
  icon: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
  darkTheme?: React.CSSProperties;
  lightTheme?: React.CSSProperties;
  className?: string;
}

const CustomIcon = ({ icon: IconSVG, darkTheme, lightTheme, className, ...rest }: Props) => {
  const { theme } = useTheme();

  const style = theme === "light" ? lightTheme : darkTheme;

  return <IconSVG style={style} className={className} {...rest} />;
};

export default CustomIcon;
