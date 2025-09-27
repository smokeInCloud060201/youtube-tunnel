import { useParams } from "react-router-dom";
import { loadVideo } from "@/services/video.ts";
import VideoPlayer from "@/components/video/VideoPlayer.tsx";
import { useEffect, useState } from "react";

const Home = () => {
  const { id } = useParams<{ id: string }>();
  const [jobId, setJobId] = useState();


  useEffect(() => {
    if (id) {
      (async () => {
        const data = await loadVideo(id);
        setJobId(data?.jobId)
      })()
    }
  }, [id]);

  return <div>{jobId && <VideoPlayer jobId={jobId} />}</div>;
};

export default Home;
