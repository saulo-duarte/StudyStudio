"use client";

import {
  KanbanBoard,
  KanbanCard,
  KanbanCards,
  KanbanHeader,
  KanbanProvider,
} from "@/components/ui/kanban";
import type { DragEndEvent } from "@/components/ui/kanban";
import { useTasks } from "@/hooks/data/use-task-data";
import { Calendar } from "lucide-react";
import { LuFlag } from "react-icons/lu";
import { Task } from "@/types/Task";
import { invoke } from "@tauri-apps/api/core";

const dateFormatter = new Intl.DateTimeFormat("en-US", {
  month: "short",
  day: "numeric",
  year: "numeric",
});

const shortDateFormatter = new Intl.DateTimeFormat("en-US", {
  month: "short",
  day: "numeric",
});

const priorityStyles: Record<"Low" | "Medium" | "High", string> = {
  Low: "bg-green-400 text-green-900",
  Medium: "bg-yellow-200 text-yellow-900",
  High: "bg-red-400 text-red-900",
};

const normalizeStatus = (status: string) => {
  const mapping: Record<string, string> = {
    InProgress: "in_progress",
    Todo: "todo",
    Done: "done",
    Backlog: "backlog",
  };
  return mapping[status] || status.toLowerCase();
};

const TasksKanban = () => {
  const { tasks, loading, error, refetch } = useTasks(); // Agora usando o hook

  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event;
    if (!over) return;

    const newStatus = over.id as Task["status"];

    try {
      const taskId = parseInt(active.id);

      await invoke("update_task", {
        taskId: taskId,
        status: newStatus,
      });

      console.log(`Tarefa ${taskId} atualizada para status: ${newStatus}`);
      refetch(); // Atualiza os dados após mudança de status
    } catch (error) {
      console.error("Erro ao atualizar a tarefa no banco de dados:", error);
    }
  };

  if (loading) return <p>Carregando...</p>;
  if (error) return <p>Erro: {error}</p>;

  return (
    <KanbanProvider onDragEnd={handleDragEnd} className="p-4">
      {["todo", "in_progress", "done", "backlog"].map((status) => {
        const formattedStatus = status
          .replace("_", " ")
          .replace(/\b\w/g, (c) => c.toUpperCase());

        return (
          <KanbanBoard key={status} id={status} className="gap-8">
            <KanbanHeader
              name={formattedStatus}
              className="border-l-4 p-2 text-black rounded-md"
              status={status}
            />
            <KanbanCards>
              {tasks
                .filter((task) => normalizeStatus(task.status) === status)
                .map((task, index) => (
                  <KanbanCard
                    key={task.id}
                    id={String(task.id)}
                    name={task.title}
                    parent={status}
                    index={index}
                  >
                    <div className="flex flex-col gap-2">
                      {task.tags && task.tags.length > 0 && (
                        <div className="flex flex-wrap gap-2">
                          {task.tags.map((tag) => (
                            <span
                              key={tag.id}
                              className="text-xs font-semibold px-2 py-1 rounded-[10px]"
                              style={{ backgroundColor: tag.color, color: "#000" }}
                            >
                              {tag.name}
                            </span>
                          ))}
                        </div>
                      )}

                      <p className="m-0 font-bold text-lg">{task.title}</p>

                      {task.description && (
                        <p className="m-0 text-muted-foreground text-xs">
                          {task.description}
                        </p>
                      )}
                    </div>

                    <div className="h-[1px] w-full bg-muted mt-2 mb-2"></div>

                    <p className="flex items-center justify-between m-0 text-muted-foreground text-sm gap-2">
                      <Calendar size={14} />
                      <span className="font-semibold ml-[-2px]">Date</span>
                      {shortDateFormatter.format(new Date(task.created_at))} -{" "}
                      {dateFormatter.format(new Date(task.due_date))}

                      {["Low", "Medium", "High"].includes(task.priority) && (
                        <span
                          className={`flex items-center gap-1 text-xs font-medium px-2 py-1 rounded-md ${
                            priorityStyles[task.priority as "Low" | "Medium" | "High"]
                          }`}
                        >
                          <LuFlag size={12} />
                          {task.priority.charAt(0).toUpperCase() + task.priority.slice(1)}
                        </span>
                      )}
                    </p>
                  </KanbanCard>
                ))}
            </KanbanCards>
          </KanbanBoard>
        );
      })}
    </KanbanProvider>
  );
};

export default TasksKanban;
