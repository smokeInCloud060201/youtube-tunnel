import type { VideoItemProps } from "@/types";
import VideoItem from "@/pages/video-search/VideoItem.tsx";

interface Props {
  items: VideoItemProps[];
}

const VideoSearchList = ({ items }: Props) => {
  return items ? (
    <div className="flex flex-col">
      {items.map((item: VideoItemProps) => (
        <VideoItem key={item?.id} data={item} />
      ))}
    </div>
  ) : (
    <div className="flex items-center justify-center">No result</div>
  );
};

export default VideoSearchList;
