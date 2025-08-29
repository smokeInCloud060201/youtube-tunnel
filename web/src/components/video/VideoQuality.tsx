import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

interface Props {
  value: string;
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

const VideoQuality = ({ value, onChangeQuality }: Props) => {
  return (
    <Select onValueChange={onChangeQuality}>
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder="Quality" />
      </SelectTrigger>
      <SelectContentCustom options={QUALITY_OPTIONS} value={value} />
    </Select>
  );
};

export default VideoQuality;
