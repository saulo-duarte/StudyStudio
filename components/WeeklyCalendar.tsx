import React, { useState } from "react";
import { Card } from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { addDays, startOfWeek, format, parseISO, isToday } from "date-fns";
import { useTasks } from "@/hooks/data/use-task-data";
import { ChevronLeft, ChevronRight, Calendar, Clock, CheckCircle, AlertCircle, Flag } from "lucide-react";

const daysOfWeek = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

const priorityColors: Record<string, string> = {
    low: "text-green-500 bg-green-900/20",
    medium: "text-yellow-500 bg-yellow-900/20",
    high: "text-red-500 bg-red-900/20",
  };
  
const statusColors: Record<string, string> = {
    Todo: "bg-gray-600 text-white",
    InProgress: "bg-blue-500 text-white",
    Done: "bg-green-500 text-white",
    Backlog: "bg-red-500 text-white",
  };

export function WeeklyCalendar() {
  const [selectedDate, setSelectedDate] = useState(new Date());
  const weekStart = startOfWeek(selectedDate, { weekStartsOn: 1 });
  const weekDates = daysOfWeek.map((_, idx) => addDays(weekStart, idx));

  const { tasks, metrics, loading, error } = useTasks();

  return (
    <Card className="p-6 shadow-lg rounded-xl bg-background text-foreground">
      <div className="flex justify-between items-center mb-6">
        <Button variant="outline" onClick={() => setSelectedDate(addDays(selectedDate, -7))}>
          <ChevronLeft className="w-5 h-5" /> Previous Week
        </Button>
        <div className="text-xl font-semibold flex items-center gap-2">
          <Calendar className="w-5 h-5 text-primary" />
          {format(weekStart, "MM/dd/yyyy")} - {format(addDays(weekStart, 6), "MM/dd/yyyy")}
        </div>
        <Button variant="outline" onClick={() => setSelectedDate(addDays(selectedDate, 7))}>
          Next Week <ChevronRight className="w-5 h-5" />
        </Button>
      </div>
      {loading ? (
        <p className="text-center text-muted-foreground">Loading tasks...</p>
      ) : error ? (
        <p className="text-center text-red-500">Error loading tasks</p>
      ) : (
        <Table className="border-collapse w-full">
          <TableHeader>
            <TableRow className="bg-gray-900">
              {weekDates.map((date, index) => (
                <TableCell
                  key={index}
                  className={`text-center font-bold py-2 border-b ${isToday(date) ? "bg-primary text-black" : ""}`}
                >
                  {daysOfWeek[index]} <br />
                  <span className={`text-xs font-medium ${isToday(date) ? "text-black" : "text-foreground"}`}>{format(date, "MM/dd")}</span>
                </TableCell>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow>
              {weekDates.map((date, colIdx) => (
                <TableCell key={colIdx} className="h-40 border border-muted p-3 align-top">
                  {tasks
                    .filter((task) => format(parseISO(task.due_date), "yyyy-MM-dd") === format(date, "yyyy-MM-dd"))
                    .map((task, idx) => (
                        <div
                          key={idx}
                          className="p-3 rounded-lg mb-2 text-sm font-medium flex flex-col gap-2 shadow-md border border-muted bg-gray-800"
                        >
                          <span className="truncate font-semibold">{task.title}</span>
                          <div className="flex justify-between items-center">
                            <span
                            className={`text-xs px-2 py-1 rounded-lg font-semibold w-fit ${statusColors[task.status] || "bg-gray-700 text-white"}`}
                            >
                            {task.status.replace("_", " ")}
                            </span>
                            <Flag className={`w-4 h-4 ${priorityColors[task.priority.toLowerCase()]}`} />
                        </div>
                      </div>
                    ))}
                </TableCell>
              ))}
            </TableRow>
          </TableBody>
        </Table>
      )}
    </Card>
  );
}
