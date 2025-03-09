"use client";

import { createContext, useContext, useState, ReactNode } from "react";
import { Task } from "@/types/Task";

const getCurrentMonthRange = () => {
  const now = new Date();
  const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1, 0, 0, 0);
  const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0, 23, 59, 59, 999);
  return { start: startOfMonth, end: endOfMonth };
};

interface TasksFilterContextType {
  dateRange: { start: Date; end: Date };
  setDateRange: (range: { start: Date; end: Date }) => void;
  filterTasksByDueDate: (tasks: Task[]) => Task[];
}

const TasksFilterContext = createContext<TasksFilterContextType | undefined>(undefined);

export const TasksFilterProvider = ({ children }: { children: ReactNode }) => {
  const [dateRange, setDateRange] = useState(getCurrentMonthRange());

  const filterTasksByDueDate = (tasks: Task[]) => {
    if (!dateRange) return tasks;
  
    const startDate = new Date(dateRange.start);
    const endDate = new Date(dateRange.end);
  
    return tasks.filter((task) => {
      if (!task.due_date) return false;
  
      const dueDate = new Date(task.due_date);
  
      console.log("DueDate:", dueDate, "Start:", startDate, "End:", endDate);
  
      return dueDate.getTime() >= startDate.getTime() && dueDate.getTime() <= endDate.getTime();
    });
  };
  

  return (
    <TasksFilterContext.Provider value={{ dateRange, setDateRange, filterTasksByDueDate }}>
      {children}
    </TasksFilterContext.Provider>
  );
};

export const useTasksFilter = () => {
  const context = useContext(TasksFilterContext);
  if (!context) {
    throw new Error("useTasksFilter deve ser usado dentro de um TasksFilterProvider");
  }
  return context;
};


