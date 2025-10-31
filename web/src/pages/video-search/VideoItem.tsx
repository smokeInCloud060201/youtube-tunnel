import React from "react";
import type { VideoItemProps } from "@/types";
import { Card, CardContent } from "@/components/ui/card.tsx";
import { Avatar, AvatarFallback } from "@/components/ui/avatar.tsx";
import { timeAgo } from "@/utils/utils.ts";
import { useNavigate } from "react-router-dom";

interface Props {
  data: VideoItemProps;
}

const VideoItem: React.FC<Props> = React.memo(({ data }) => {
  const { channelTitle, title, id, thumbnails, description, publishTime } = data;
  const navigate = useNavigate();

  const goToVideo = React.useCallback(() => {
    navigate(`/video/${id}`);
  }, [navigate, id]);

  const thumbnailSrc = React.useMemo(
    () => `data:image/jpeg;base64, ${thumbnails[0] ?? ''}`,
    [thumbnails]
  );

  return (
    <Card
      className="flex rounded-none border-none items-center justify-start cursor-pointer hover:opacity-40 transition-opacity"
      onClick={goToVideo}
    >
      <CardContent className="flex flex-row items-start justify-start">
        <div>
          <img
            className="rounded-2xl"
            src={thumbnailSrc}
            alt={title}
            loading="lazy"
          />
        </div>
        <div className="px-5 gap-2 flex flex-col items-start justify-start">
          <span className="text-xl">{title}</span>
          <span className="text-sm">1 Tr Lượt xem - {timeAgo(publishTime)}</span>
          <div className="flex items-center gap-4 text-sm">
            <Avatar>
              <AvatarFallback>{channelTitle.charAt(0).toUpperCase()}</AvatarFallback>
            </Avatar>
            <span>{channelTitle}</span>
          </div>
          <span className="text-sm line-clamp-2">{description}</span>
        </div>
      </CardContent>
    </Card>
  );
});

VideoItem.displayName = 'VideoItem';

export default VideoItem;
