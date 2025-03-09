import { RangeCalendar } from "@/components/ui/calendar-rac";
import { 
  getLocalTimeZone, 
  today, 
  ZonedDateTime, 
  parseDate,
  CalendarDate
} from "@internationalized/date";
import { Popover, PopoverTrigger, PopoverContent } from "@/components/ui/popover";
import { Button } from "@/components/ui/button";
import { Calendar } from "lucide-react";
import { useTasksFilter } from "@/hooks/context/use-task-context";
import type { DateRange } from "react-aria-components";

export const RangeCalendarFilter = () => {
  const now = today(getLocalTimeZone());
  const { dateRange, setDateRange } = useTasksFilter();

  const createCalendarDate = (date: Date): CalendarDate => {
    return new CalendarDate(
      date.getFullYear(),
      date.getMonth() + 1, 
      date.getDate()
    );
  };

  const handleDateChange = (range: DateRange | null) => {
    if (range) {
      const start = new Date(range.start.year, range.start.month - 1, range.start.day, 0, 0, 0, 0);
      const end = new Date(range.end.year, range.end.month - 1, range.end.day, 23, 59, 59, 999);
  
      setDateRange({ start, end });
    }
  };

  return (
    <Popover>
      <PopoverTrigger asChild title="Range date filter">
        <Button variant="outline">
          <Calendar /> Date
        </Button>
      </PopoverTrigger>
      <PopoverContent className="p-0">
        <RangeCalendar
          className="rounded-lg border border-border p-2"
          value={dateRange ? {
            start: dateRange.start instanceof CalendarDate 
              ? dateRange.start 
              : createCalendarDate(new Date(dateRange.start)),
            end: dateRange.end instanceof CalendarDate 
              ? dateRange.end 
              : createCalendarDate(new Date(dateRange.end)),
          } : undefined}
          onChange={handleDateChange}
        />
      </PopoverContent>
    </Popover>
  );
};