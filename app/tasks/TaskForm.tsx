"use client";

import * as React from "react";
import { useEffect, useState } from "react";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { invoke } from "@tauri-apps/api/core";
import { format } from "date-fns";
import { getLocalTimeZone, fromDate } from "@internationalized/date";
import { motion, AnimatePresence } from "framer-motion";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Calendar } from "@/components/ui/calendar-rac";
import { CalendarIcon, CheckCircle } from "lucide-react";
import { cn } from "@/lib/utils";
import { Separator } from "@/components/ui/separator";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { TagOption, TagsMultiselect } from "@/components/ui/multi-select";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"; 

const formSchema = z.object({
  title: z.string().min(2, {
    message: "Task name must be at least 2 characters long.",
  }),
  description: z.string().optional(),
  due_date: z.date({
    required_error: "Please select a due date.",
  }),
  priority: z.enum(["low", "medium", "high"], {
    message: "Priority must be Low, Medium, or High.",
  }),
});

export function CreateTaskForm() {
  const [activeUserId, setActiveUserId] = useState<number | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);
  const [selectedTags, setSelectedTags] = useState<TagOption[]>([]);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: "",
      description: "",
      due_date: new Date(),
      priority: "medium",
    },
  });

  useEffect(() => {
    const fetchActiveUser = async () => {
      try {
        const userId = await invoke<number | null>("get_active_user_id");
        setActiveUserId(userId || null);
      } catch (error) {
        toast.error("Error", {
          description: error instanceof Error ? error.message : "Unknown error",
        });
      }
    };

    fetchActiveUser();
  }, []);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    if (!activeUserId) {
      toast.error("No active user found", {
        description: "Please create or activate a user first",
      });
      return;
    }

    setIsSubmitting(true);
    try {
      const isoDate = values.due_date.toISOString().split(".")[0] + "Z";

      await invoke("create_task", {
        title: values.title,
        userId: activeUserId,
        description: values.description || null,
        dueDate: isoDate,
        priority: values.priority,
        tags: selectedTags.map(tag => ({
          name: tag.label,
          color: tag.color,
        })),
      });

      setShowSuccess(true);
      setTimeout(() => {
        setShowSuccess(false);
        form.reset();
        setSelectedTags([]);
      }, 2500);

      toast.success("Task created successfully!", {
        description: "Your task has been created.",
      });
    } catch (error) {
      console.error("Erro completo:", error); // Log detalhado no console
  let errorMessage = error instanceof Error ? error.message : JSON.stringify(error);
  toast.error("Error creating task", { description: errorMessage });
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.3 }}
      className="shadow-2xl rounded-2xl p-4"
    >
      <Separator className="mb-4 h-[2px]" />
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <FormField
            control={form.control}
            name="title"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Task Name</FormLabel>
                <FormControl>
                  <Input placeholder="Write report" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="description"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Description</FormLabel>
                <FormControl>
                  <Textarea placeholder="Provide a detailed description..." {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* Campo de Prioridade */}
          <FormField
            control={form.control}
            name="priority"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Priority</FormLabel>
                <Select onValueChange={field.onChange} defaultValue={field.value}>
                  <FormControl>
                    <SelectTrigger>
                      <SelectValue placeholder="Select priority" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    <SelectItem value="low">Low</SelectItem>
                    <SelectItem value="medium">Medium</SelectItem>
                    <SelectItem value="high">High</SelectItem>
                  </SelectContent>
                </Select>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="due_date"
            render={({ field }) => (
              <FormItem className="flex flex-col">
                <FormLabel>Due Date</FormLabel>
                <Popover modal={false}>
                  <PopoverTrigger asChild>
                    <FormControl>
                      <Button variant="outline" className="w-full justify-between">
                        {field.value ? format(field.value, "PPP") : "Select the date"}
                        <CalendarIcon className="ml-2 h-5 w-5 opacity-60" />
                      </Button>
                    </FormControl>
                  </PopoverTrigger>
                  <PopoverContent className="w-auto p-0 pointer-events-auto">
                    <Calendar
                      value={field.value ? fromDate(field.value, getLocalTimeZone()) : undefined}
                      onChange={(date) => field.onChange(date.toDate(getLocalTimeZone()))}
                    />
                  </PopoverContent>
                </Popover>
                <FormMessage />
              </FormItem>
            )}
          />

          <FormItem>
            <FormLabel>Tags</FormLabel>
            <TagsMultiselect
              placeholder="Select tags..."
              value={selectedTags}
              onChange={(newTags) => setSelectedTags(newTags)}
            />
          </FormItem>

          <div className="relative">
            <Button type="submit" disabled={isSubmitting} className="w-full py-3 font-semibold">
              {isSubmitting ? "Creating Task..." : "Create Task"}
            </Button>
            <AnimatePresence>
              {showSuccess && (
                <motion.div className="absolute inset-0 flex flex-col items-center justify-center bg-primary text-black/90 rounded-md">
                  <CheckCircle className="mb-2 h-8 w-8" />
                  <span className="text-lg font-semibold">Task Created Successfully!</span>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </form>
      </Form>
    </motion.div>
  );
}

export default CreateTaskForm;
