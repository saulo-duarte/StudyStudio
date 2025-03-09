"use client"

import { CreateTaskForm } from "./TaskForm";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Task } from "@/types/Task";
import TasksKanban from "./TasksKanban";
import { TasksDialog } from "./TasksDialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Kanban, Search, TrashIcon } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";  
import { RangeCalendarFilter } from "./RangeCalendarFilter";
import FiltersToolTip from "./FilterToolTip";
import TasksTable from "./TasksTable";
import Image from "next/image";
import { TagManager } from "@/components/TagUpdate";

export default function TasksPage() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const loadTasks = async () => {
      try {
        const result = await invoke<Task[]>("get_all_tasks");
        setTasks(result);
      } catch (error) {
        console.error("Falha ao carregar tasks:", error);
      } finally {
        setIsLoading(false);
      }
    };

    loadTasks();
  }, []);

  if (isLoading) {
    return <div className="text-center py-10">Loading...</div>;
  }

  if (tasks.length === 0) {
    return (
      <div className="flex flex-col h-full">
        <header className="flex justify-between text-3xl font-semibold font-lato py-2">Tasks
          <TasksDialog />
        </header>
        <div className="flex flex-col items-center">
          <h1 className="text-5xl font-lato font-bold">
            Welcome  <span className="text-primary"> User </span>
            User to your Tasks Page</h1>
          <Image src="/svg/Start Tasks.svg" width={800} height={400} alt="Create Tasks" />
          <p className="text-xl text-foreground font-lato">Create your first 
            <span className="ml-2 mr-2 text-primary">Task</span>
            To Visualize them
          </p>
        </div>
      </div>
    );
  }

  return (
    <div>
      <header className="flex justify-between">
        <h1 className="text-3xl font-semibold font-lato py-4">Tasks</h1>
        <div className="flex">
        <TagManager />
        <TasksDialog />
        </div>
      </header>
      <div>
        <Tabs defaultValue="kanban">
          <div className="flex items-center justify-between py-2 w-full gap-4 border-b-[2.5px] border-border">
            <TabsList className="flex w-[300px] bg-background -mb-4 font-lato font-medium">
              <TabsTrigger value="table"> Table </TabsTrigger>
              <TabsTrigger value="kanban"> <Kanban /> Kanban </TabsTrigger>
              <TabsTrigger value="list"> List </TabsTrigger>
            </TabsList>
            <div className="flex items-center justify-end gap-4">
              <RangeCalendarFilter />
              <div className="relative text-white">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-foreground w-5 h-5" />
                <Input type="text" placeholder="Search tasks..." className="pl-10 w-64" />
              </div>
              <FiltersToolTip />
              <Button variant="outline" className="hover:text-red-500" title="Clean filters">
                <TrashIcon /> Filters
              </Button>
            </div>
          </div>

          <TabsContent value="table">
            <TasksTable />
          </TabsContent>

          <TabsContent value="kanban">
            <TasksKanban />
          </TabsContent>

          <TabsContent value="list">
            list
          </TabsContent>
        </Tabs>
      </div>
    </div>
  );
}
