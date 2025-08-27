import type { RefObject } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

interface Props {
  value: string;
  videoRef: RefObject<HTMLVideoElement | null>;
  onChangeQuality: (quality: string) => void;
}

const VideoQuality = ({ value, videoRef, onChangeQuality }: Props) => {
  console.log(value);
  const handleQualityChange = (newQuality: string) => {
    if (!videoRef?.current) return;

    const video = videoRef.current;
    const currentTime = video.currentTime;
    const isPlaying = !video.paused;

    onChangeQuality(newQuality);

    video.src = `http://localhost:8080/api/stream?url=${encodeURIComponent(
      "https://www.youtube.com/watch?v=n7DBCe4QpIE"
    )}&enableVideo=true&quality=${newQuality}`;

    video.onloadedmetadata = () => {
      video.currentTime = currentTime;
      if (isPlaying) {
        video.play();
      }
      video.onloadedmetadata = null;
    };

    video.load();
  };

  return (
    <Select onValueChange={handleQualityChange}>
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder="Quality" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="144p">144p</SelectItem>
        <SelectItem value="240p">240p</SelectItem>
        <SelectItem value="360p">360p</SelectItem>
        <SelectItem value="480p">480p</SelectItem>
        <SelectItem value="720p">720p</SelectItem>
        <SelectItem value="1080p">1080p</SelectItem>
        <SelectItem value="best">Best</SelectItem>
      </SelectContent>
    </Select>
  );
};

export default VideoQuality;
