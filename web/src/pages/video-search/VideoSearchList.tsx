import React from "react";
import type { VideoItemProps } from "@/types";
import VideoItem from "@/pages/video-search/VideoItem.tsx";

interface Props {
  items: VideoItemProps[];
}

const VideoSearchList: React.FC<Props> = React.memo(({ items }) => {
  if (!items || items.length === 0) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <p className="text-gray-400">No results found</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col">
      {items.map((item: VideoItemProps) => (
        <VideoItem key={item.id} data={item} />
      ))}
    </div>
  );
});

VideoSearchList.displayName = 'VideoSearchList';

export default VideoSearchList;
