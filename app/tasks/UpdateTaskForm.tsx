"use client";

import * as React from "react";
import { useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { invoke } from "@tauri-apps/api/core";
import { format } from "date-fns";
import { motion, AnimatePresence } from "framer-motion";
import { Loader2, CalendarIcon } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Calendar } from "@/components/ui/calendar-rac";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetFooter,
} from "@/components/ui/sheet";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { getLocalTimeZone, fromDate } from "@internationalized/date";
import { cn } from "@/lib/utils";
import { TagsMultiselect, TagOption } from "@/components/ui/multi-select";

export interface Task {
  id: number;
  title: string;
  description: string;
  status: string;
  priority: string;
  due_date: Date;
  created_at?: string;
  tags?: TagOption[];
}

interface UpdateTaskFormProps {
  task: Task;
  onUpdate: (updatedTask: Task) => void;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function UpdateTaskForm({
  task,
  onUpdate,
  open,
  onOpenChange,
}: UpdateTaskFormProps) {
  const [isLoading, setIsLoading] = useState(false);

  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
  } = useForm<Task>({
    defaultValues: {
      ...task,
      tags: Array.isArray(task.tags)
        ? task.tags.map((tag) => ({
            value: (tag as any).id?.toString() || tag.value || "", 
            label: (tag as any).name || tag.label || "Sem nome",
            color: (tag as any).color || "#0000ff",
          }))
        : [],
    }
  });

  const onSubmit = async (data: Task) => {
    setIsLoading(true);
    try {
      const formattedDueDate =
        data.due_date instanceof Date
          ? format(data.due_date, "yyyy-MM-dd'T'HH:mm")
          : data.due_date;
  
          const formattedTags = data.tags?.map((tag: TagOption) => ({
            id: tag.value ? Number(tag.value) : undefined,  // Converte `value` para número, se existir
            name: tag.label,
            color: tag.color,
          }));;
  
      console.log("🛠 Dados enviados para o backend:", {
        taskId: task.id,
        title: data.title,
        description: data.description,
        status: data.status,
        priority: data.priority,
        dueDate: formattedDueDate,
        tags: formattedTags,
      });
  
      const updatedTask = await invoke<Task>("update_task", {
        taskId: task.id,
        title: data.title,
        description: data.description,
        status: data.status,
        priority: data.priority,
        dueDate: formattedDueDate,
        tags: formattedTags,
      });
  
      onUpdate(updatedTask);
      onOpenChange(false);
      toast.info("Task updated", {
        description: "Your task has been successfully updated.",
      });
    } catch (error) {
      console.error("❌ Erro ao atualizar task:", error);
      toast.error("Update failed", {
        description: "There was an error updating your task. Please try again.",
      });
    } finally {
      setIsLoading(false);
    }
  };
  
  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent className="sm:max-w-[425px]">
        <SheetHeader>
          <SheetTitle>Edit Task</SheetTitle>
          <SheetDescription>
            Make changes to your task here. Click save when you're done.
          </SheetDescription>
        </SheetHeader>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4 mt-4">
          <div className="space-y-1">
            <Label htmlFor="name">Task Name</Label>
            <Input
              id="name"
              {...register("title", { required: "Task name is required" })}
            />
            {errors.title && (
              <p className="text-sm text-red-500">{errors.title.message}</p>
            )}
          </div>
          <div className="space-y-1">
            <Label htmlFor="description">Description</Label>
            <Textarea id="description" {...register("description")} />
          </div>
          <div className="space-y-1">
            <Label htmlFor="status">Status</Label>
            <Controller
              name="status"
              control={control}
              rules={{ required: "Status is required" }}
              render={({ field }) => (
                <Select onValueChange={field.onChange} defaultValue={field.value}>
                  <SelectTrigger>
                    <SelectValue placeholder="Select a status" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="todo">To Do</SelectItem>
                    <SelectItem value="in_progress">In Progress</SelectItem>
                    <SelectItem value="done">Done</SelectItem>
                  </SelectContent>
                </Select>
              )}
            />
            {errors.status && (
              <p className="text-sm text-red-500">{errors.status.message}</p>
            )}
          </div>
          <div className="space-y-1">
            <Label>Due Date</Label>
            <Controller
              name="due_date"
              control={control}
              rules={{ required: "Due date is required" }}
              render={({ field }) => (
                <Popover modal={true}>
                  <PopoverTrigger asChild>
                    <Button
                      variant="outline"
                      className={cn(
                        "w-full justify-between pl-3 text-left font-normal",
                        !field.value && "text-muted-foreground"
                      )}
                    >
                      {field.value ? format(field.value, "PPP") : "Select the date"}
                      <CalendarIcon className="ml-2 h-5 w-5 opacity-60" />
                    </Button>
                  </PopoverTrigger>
                  <PopoverContent
                    className="w-auto p-0 pointer-events-auto"
                    align="start"
                  >
                    <Calendar
                      value={
                        field.value
                          ? fromDate(new Date(field.value), getLocalTimeZone())
                          : undefined
                      }
                      onChange={(date) => {
                        if (date) {
                          field.onChange(date.toDate(getLocalTimeZone()));
                        }
                      }}
                    />
                  </PopoverContent>
                </Popover>
              )}
            />
          </div>

          <div className="space-y-1">
            <Label htmlFor="priority">Priority</Label>
            <Controller
              name="priority"
              control={control}
              rules={{ required: "Priority is required" }}
              render={({ field }) => (
                <Select onValueChange={field.onChange} defaultValue={field.value}>
                  <SelectTrigger>
                    <SelectValue placeholder="Select priority" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="low">🟢 Low</SelectItem>
                    <SelectItem value="medium">🟡 Medium</SelectItem>
                    <SelectItem value="high">🔴 High</SelectItem>
                  </SelectContent>
                </Select>
              )}
            />
            {errors.priority && (
              <p className="text-sm text-red-500">{errors.priority.message}</p>
            )}
          </div>

          <div className="space-y-1">
            <Label>Tags</Label>
            <Controller
              name="tags"
              control={control}
              render={({ field }) => (
                <TagsMultiselect
                  placeholder="Select tags..."
                  emptyMessage="No tags found."
                  value={field.value}
                  onChange={field.onChange}
                />
              )}
            />
          </div>

          <SheetFooter>
            <Button type="submit" className="w-full" disabled={isLoading}>
              <AnimatePresence mode="wait">
                {isLoading ? (
                  <motion.div
                    key="loading"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    className="flex items-center"
                  >
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Updating...
                  </motion.div>
                ) : (
                  <motion.span
                    key="save"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                  >
                    Save Changes
                  </motion.span>
                )}
              </AnimatePresence>
            </Button>
          </SheetFooter>
        </form>
      </SheetContent>
    </Sheet>
  );
}
