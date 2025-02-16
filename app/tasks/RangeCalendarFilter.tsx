"use client";

import { RangeCalendar } from "@/components/ui/calendar-rac";
import { getLocalTimeZone, today } from "@internationalized/date";
import { useState } from "react";
import type { DateRange } from "react-aria-components";
import { Popover, PopoverTrigger, PopoverContent } from "@/components/ui/popover";
import { Button } from "@/components/ui/button";
import { Calendar } from "lucide-react";

export default function RangeCalendarFilter() {
  const now = today(getLocalTimeZone());
  const [date, setDate] = useState<DateRange | null>({
    start: now,
    end: now.add({ days: 3 }),
  });

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
          value={date}
          onChange={setDate}
        />
      </PopoverContent>
    </Popover>
  );
}
