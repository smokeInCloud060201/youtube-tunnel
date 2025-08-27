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

interface SelectContentCustomProps {
  options?: string[];
  value: string;
}

const QUALITY_OPTIONS = ["144p", "240p", "360p", "480p", "720p", "1080p", "Best"];

const SelectContentCustom = ({ options, value }: SelectContentCustomProps) => {
  if (!options) return <div></div>;
  return (
    <SelectContent side="top">
      {options.map((option) => {
        const classNames = option === value ? "bg-search-gray-300" : "";
        return (
          <SelectItem key={option} value={option} className={classNames}>
            {option}
          </SelectItem>
        );
      })}
    </SelectContent>
  );
};

const VideoQuality = ({ value, videoRef, onChangeQuality }: Props) => {
  const handleQualityChange = async (newQuality: string) => {
    if (!videoRef?.current) return;

    const video = videoRef.current;
    const currentTime = video.currentTime;
    const isPlaying = !video.paused;

    onChangeQuality(newQuality);

    video.src = `http://localhost:8080/api/stream?url=${encodeURIComponent(
      "https://www.youtube.com/watch?v=n7DBCe4QpIE"
    )}&enableVideo=true&quality=${newQuality}`;

    video.onloadedmetadata = async () => {
      video.currentTime = currentTime;
      if (isPlaying) {
        await video.play();
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
      <SelectContentCustom options={QUALITY_OPTIONS} value={value} />
    </Select>
  );
};

export default VideoQuality;
