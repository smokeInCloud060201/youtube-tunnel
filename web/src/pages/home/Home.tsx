import { Video } from "@/components";
import { useParams } from "react-router-dom";

const Home = () => {
  const { id } = useParams<{ id: string }>();

  return (
    <div>
      <Video id={id || ""} />
    </div>
  );
};

export default Home;
