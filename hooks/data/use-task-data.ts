"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTasksFilter } from "@/hooks/context/use-task-context";
import { Task } from "@/types/Task";

interface TasksMetrics {
  total: number;
  completed: number;
  pending: number;
  overdue: number;
}

export const useTasks = () => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [metrics, setMetrics] = useState<TasksMetrics>({
    total: 0,
    completed: 0,
    pending: 0,
    overdue: 0,
  });
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // Obtendo o dateRange e a função de filtragem do contexto
  const { dateRange, filterTasksByDueDate } = useTasksFilter();

  const fetchTasks = async () => {
    setLoading(true);
    try {
      const fetched: Task[] = await invoke("get_all_tasks");

      const filteredTasks = filterTasksByDueDate(fetched);
      setTasks(filteredTasks);

      const total = filteredTasks.length;
      const completed = filteredTasks.filter(
        (task) => task.status.toLowerCase() === "done"
      ).length;
      const pending = filteredTasks.filter(
        (task) =>
          task.status.toLowerCase() === "todo" ||
          task.status.toLowerCase() === "in_progress"
      ).length;
      const now = new Date();
      const overdue = filteredTasks.filter(
        (task) =>
          new Date(task.due_date) < now &&
          task.status.toLowerCase() !== "done"
      ).length;

      setMetrics({ total, completed, pending, overdue });
    } catch (err: any) {
      setError(err.message || "Erro ao buscar tasks");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTasks();
    console.log(dateRange)
  }, [dateRange]); 

  return { tasks, metrics, loading, error, refetch: fetchTasks };
};
