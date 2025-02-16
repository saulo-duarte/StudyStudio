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
import { toast } from "@/hooks/use-toast";
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

const formSchema = z.object({
  name: z.string().min(2, {
    message: "Task name must be at least 2 characters long.",
  }),
  description: z.string().optional(),
  due_date: z.date({
    required_error: "Please select a due date.",
  }),
});

export function CreateTaskForm() {
  const [activeUserId, setActiveUserId] = useState<number | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);
  // Estado para armazenar as tags selecionadas (array de strings)
  const [selectedTags, setSelectedTags] = useState<TagOption[]>([]);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      description: "",
      due_date: new Date(),
    },
  });

  // Busca o usuário ativo (necessário para criar a task)
  useEffect(() => {
    const fetchActiveUser = async () => {
      try {
        const userId = await invoke<number | null>("get_active_user_id");
        setActiveUserId(userId || null);
      } catch (error) {
        toast({
          title: "Error fetching user",
          description:
            error instanceof Error ? error.message : "Unknown error",
          variant: "destructive",
        });
      }
    };

    fetchActiveUser();
  }, []);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    if (!activeUserId) {
      toast({
        title: "No active user found",
        description: "Please create or activate a user first",
        variant: "destructive",
      });
      return;
    }

    setIsSubmitting(true);
    try {
      const isoDate = values.due_date.toISOString().split(".")[0] + "Z";
      console.log("Due date ISO string:", isoDate);

      await invoke("create_task", {
        name: values.name,
        userId: activeUserId,
        description: values.description || null,
        dueDate: isoDate,
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
      toast({
        title: "Task created successfully!",
        description: "Your task has been created.",
      });
    } catch (error) {
      console.error("Error creating task:", error);
      let errorMessage = "Unknown error";
      if (error instanceof Error) {
        errorMessage = error.message;
      } else {
        try {
          errorMessage = JSON.stringify(error);
        } catch {
          errorMessage = String(error);
        }
      }
      toast({
        title: "Error creating task",
        description: errorMessage,
        variant: "destructive",
      });
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
            name="name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Task Name</FormLabel>
                <FormControl>
                  <Input
                    placeholder="Write report"
                    className="border border-gray-300 rounded-md p-2"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* Campo para descrição */}
          <FormField
            control={form.control}
            name="description"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Description</FormLabel>
                <FormControl>
                  <Textarea
                    placeholder="Provide a detailed description of the task..."
                    className="border border-gray-300 rounded-md p-2 h-32 resize-none"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* Campo para data de vencimento */}
          <FormField
            control={form.control}
            name="due_date"
            render={({ field }) => (
              <FormItem className="flex flex-col">
                <FormLabel>Due Date</FormLabel>
                <Popover modal={false}>
                  <PopoverTrigger asChild>
                    <FormControl>
                      <Button
                        variant="outline"
                        className={cn(
                          "w-full justify-between pl-3 text-left font-normal",
                          !field.value && "text-muted-foreground"
                        )}
                      >
                        {field.value
                          ? format(field.value, "PPP")
                          : "Select the date"}
                        <CalendarIcon className="ml-2 h-5 w-5 opacity-60" />
                      </Button>
                    </FormControl>
                  </PopoverTrigger>
                  <PopoverContent
                    className="w-auto p-0 pointer-events-auto"
                    align="start"
                  >
                    <Calendar
                      value={
                        field.value
                          ? fromDate(field.value, getLocalTimeZone())
                          : undefined
                      }
                      onChange={(date) =>
                        field.onChange(date.toDate(getLocalTimeZone()))
                      }
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
              emptyMessage="No tags found."
              value={selectedTags}
              onChange={(newTags: TagOption[]) => setSelectedTags(newTags)}
            />
          </FormItem>

          <div className="relative">
            <Button
              type="submit"
              disabled={isSubmitting}
              className="w-full py-3 font-semibold"
            >
              {isSubmitting ? "Creating Task..." : "Create Task"}
            </Button>
            <AnimatePresence>
              {showSuccess && (
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.4 }}
                  className="absolute inset-0 flex flex-col items-center justify-center bg-primary text-black/90 rounded-md"
                >
                  <CheckCircle className="mb-2 h-8 w-8" />
                  <span className="text-lg font-semibold">
                    Task Created Successfully!
                  </span>
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
