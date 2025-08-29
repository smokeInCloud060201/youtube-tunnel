import type { VideoItemProps } from "@/types";
import { Card, CardContent } from "@/components/ui/card.tsx";
import { Avatar, AvatarFallback } from "@/components/ui/avatar.tsx";
import { timeAgo } from "@/utils/utils.ts";
import { useNavigate } from "react-router-dom";

interface Props {
  data: VideoItemProps;
}

const VideoItem = ({ data }: Props) => {
  const { channelTitle, title, id, thumbnails, description, publishTime } = data;

  const navigate = useNavigate();

  const goToVideo = () => {
    navigate(`/video/${id}`);
  };

  return (
    <Card
      className="flex rounded-none border-none items-center justify-start cursor-pointer hover:opacity-40"
      onClick={goToVideo}
    >
      <CardContent className="flex flex-row items-start justify-start">
        <div>
          <img
            className="rounded-2xl"
            src={`data:image/jpeg;base64, ${thumbnails[1]}`}
            alt={"Thumbnail"}
          />
        </div>
        <div className="px-5 gap-2 flex flex-col items-start justify-start">
          <span className="text-xl">{title}</span>
          <span className="text-sm">1 Tr Lượt xem - {timeAgo(publishTime)}</span>
          <div className="flex items-center gap-4 text-sm">
            <Avatar>
              <AvatarFallback>C</AvatarFallback>
            </Avatar>
            <span>{channelTitle}</span>
          </div>
          <span className="text-sm">{description}</span>
        </div>
      </CardContent>
    </Card>
  );
};

// @ts-ignore
export default VideoItem;
