import { Input } from "@/components/ui/input.tsx";
import SearchIcon from "@/assets/SearchIcon.tsx";
import { type ChangeEvent } from "react";
import type { KeyboardEvent } from "react";

interface Props {
  updateValue: (value: ChangeEvent<HTMLInputElement>) => void;
  value: string;
  handleSearch: () => void;
}

const Search = ({ updateValue, value, handleSearch }: Props) => {
  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      handleSearch();
    }
  };

  return (
    <div>
      <div className="flex items-center justify-center w-[462px] h-[36px] gap-1">
        <Input
          value={value}
          type="text"
          onChange={updateValue}
          onKeyDown={(e) => handleKeyDown(e)}
          className="rounded"
          width={362}
          height={36}
          placeholder="Search"
        />
        <SearchIcon
          className="w-[64px] bg-search-gray-800 dark:bg-search-gray-300 h-full p-2 cursor-pointer"
          onClick={handleSearch}
        />
      </div>
    </div>
  );
};

export default Search;
