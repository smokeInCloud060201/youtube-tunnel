import { Navigation } from "@/components";
import * as React from "react";

interface Props {
  children?: React.ReactNode;
}

const BaseLayout = ({ children }: Props) => {
  return (
    <div>
      <Navigation />
      <div className="m-8 px-8">{children}</div>
    </div>
  );
};

export default BaseLayout;
