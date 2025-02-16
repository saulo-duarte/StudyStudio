'use client';

import {
  KanbanBoard,
  KanbanCard,
  KanbanCards,
  KanbanHeader,
  KanbanProvider,
} from '@/components/ui/kibo-ui/kanban';
import type { DragEndEvent } from '@/components/ui/kibo-ui/kanban';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Task } from '@/types/task';
import { Calendar } from 'lucide-react';

const dateFormatter = new Intl.DateTimeFormat('en-US', {
  month: 'short',
  day: 'numeric',
  year: 'numeric'
});

const shortDateFormatter = new Intl.DateTimeFormat('en-US', {
  month: 'short',
  day: 'numeric'
});

const statusColors: Record<string, string> = {
  todo: 'bg-[#f55b5b]',
  inprogress: 'bg-[#4f8cc9]',
  done: 'bg-[#4fc96d]',
  backlog: 'bg-[#b34fc9]',
};

const TasksKanban = () => {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    const fetchTasks = async () => {
      try {
        const data: Task[] = await invoke('get_all_tasks');
        setTasks(data);
        console.log(data);
      } catch (error) {
        console.error('Failed to fetch tasks:', error);
      }
    };

    fetchTasks();
  }, []);

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    if (!over) return;

    const newStatus = over.id as Task['status'];

    setTasks((prevTasks) =>
      prevTasks.map((task) =>
        String(task.id) === active.id ? { ...task, status: newStatus } : task
      )
    );
  };

  return (
    <KanbanProvider onDragEnd={handleDragEnd} className="p-4">
      {['todo', 'inprogress', 'done', 'backlog'].map((status) => {
        const formattedStatus = status
          .replace('_', ' ')
          .replace(/\b\w/g, c => c.toUpperCase());

        return (
          <KanbanBoard key={status} id={status} className="">
            <KanbanHeader 
              name={formattedStatus} 
              className={`${statusColors[status]} p-2 text-black rounded-md`}
              color={statusColors[status]}
            />
            <KanbanCards>
              {tasks
                .filter((task) => task.status.toLowerCase() === status)
                .map((task, index) => (
                  <KanbanCard
                    key={task.id}
                    id={String(task.id)}
                    name={task.name}
                    parent={status}
                    index={index}
                  >
                    <div className="flex flex-col gap-2">
                      <p className="m-0 font-bold text-lg">{task.name}</p>
                      {task.description && (
                        <p className="m-0 text-muted-foreground text-xs py-4">
                          {task.description}
                        </p>
                      )}
                    </div>
                    <p className="flex m-0 text-muted-foreground text-md gap-4">
                      <Calendar size={16}/>
                      {shortDateFormatter.format(new Date(task.created_at))} -{' '}
                      {dateFormatter.format(new Date(task.due_date))}
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
