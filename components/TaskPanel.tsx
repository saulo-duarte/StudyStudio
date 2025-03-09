"use client";

import React from "react";
import { TaskCard } from "./TaskCard";
import { useTasks } from "@/hooks/data/use-task-data";
import { 
    Loader2, 
    List, 
    CheckCircle,
    Clock,
    AlertTriangle,
} from "lucide-react";

export const TaskPanel: React.FC = () => {
  const { metrics, loading, error } = useTasks();

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <Loader2 className="animate-spin text-gray-400" size={32} />
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4 text-center text-red-500 font-medium">
        Erro: {error}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 p-4">
      <TaskCard title="Total Tasks" value={metrics.total} icon={<List size={24} />} />
      <TaskCard title="Completed" value={metrics.completed} icon={<CheckCircle size={24} className="text-green-400" />} />
      <TaskCard title="Pending" value={metrics.pending} icon={<Clock size={24} className="text-yellow-400" />} />
      <TaskCard title="Overdue" value={metrics.overdue} icon={<AlertTriangle size={24} className="text-red-400" />} />
    </div>
  );
};