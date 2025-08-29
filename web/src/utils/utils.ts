const formatTime = (seconds: number) => {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
};

const timeAgo = (dateString: string): string => {
  const date = new Date(dateString);
  const now = new Date();
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  const intervals: [number, string][] = [
    [60, "second"],
    [60, "minute"],
    [24, "hour"],
    [7, "day"],
    [4.34524, "week"], // ~30.44 days / 7
    [12, "month"],
    [Number.POSITIVE_INFINITY, "year"],
  ];

  let count = seconds;
  let unit = "second";

  for (let i = 0; i < intervals.length; i++) {
    if (count < intervals[i][0]) {
      unit = intervals[i][1];
      break;
    }
    count = Math.floor(count / intervals[i][0]);
  }

  return `${count} ${unit}${count !== 1 ? "s" : ""} ago`;
};

export { formatTime, timeAgo };
