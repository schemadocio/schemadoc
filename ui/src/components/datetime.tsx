import dayjs from "dayjs";

import DurationPlugin from "dayjs/plugin/duration";
import RelativeTime from "dayjs/plugin/relativeTime";

dayjs.extend(DurationPlugin);
dayjs.extend(RelativeTime);

export const humanizeDateTimeOffset = (value: string): string => {
  return dayjs.duration(dayjs(value).diff(dayjs())).humanize(true);
};
