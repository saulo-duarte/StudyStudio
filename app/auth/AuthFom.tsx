"use client"

import * as z from "zod"
import { invoke } from '@tauri-apps/api/core';
import {
  Form,
  FormControl,    
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { 
  Card, 
  CardHeader, 
  CardTitle, 
  CardDescription,
  CardContent, 
} from "@/components/ui/card";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod"


const formSchema = z.object({
  name: z.string()
        .min(2, "Name must be at least 2 characters")
        .max(30, "Name must be at most 30 characters")
        .nonempty("Name is required"),
});

type FormName = z.infer<typeof formSchema>;

export function AuthForm() {
  const router = useRouter();
  const [isLoading, setLoading] = useState(false);
  
  const form = useForm<FormName>({
    resolver: zodResolver(formSchema),
    defaultValues: {
        name: "",
    }
  });

  async function onSubmit(data: FormName){
    try {
      setLoading(true);
      
      if (!data.name?.trim()) {
        throw new Error("Name is required");
      }

      await invoke("register_user", {
        name: data.name.trim(),
      });

      router.push("/");

    } catch (error) {
      const backendMessage = error instanceof Error 
        ? error.message 
        : "An error occurred while creating your account. Please try again.";
    } finally {
      setLoading(false);
    }
  } 

  return(
    <Card className="w-full max-w-md mx-auto">
      <CardHeader>
        <CardTitle>Welcome to Study Studio!</CardTitle>
        <CardDescription>Tell a little bit about you</CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) =>(
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="Your Name"
                      {...field}
                      disabled={isLoading}
                    />
                  </FormControl>
                  <FormDescription>How should we call you?</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
      </CardContent>
    </Card>

  )

}