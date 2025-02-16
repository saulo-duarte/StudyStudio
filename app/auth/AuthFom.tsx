"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { useRouter } from "next/navigation"
import { useState } from "react"
import * as z from "zod"
import { invoke } from '@tauri-apps/api/core'
import { toast } from "sonner"

// Componentes UI
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { 
  Card, 
  CardHeader, 
  CardTitle, 
  CardDescription,
  CardContent, 
} from "@/components/ui/card"
import { BgAnimateButton } from "@/components/ui/animated-button"

const formSchema = z.object({
  name: z.string()
    .min(2, "Name must be at least 2 characters")
    .max(30, "Name must be at most 30 characters")
    .nonempty("Name is required"),
})

type FormName = z.infer<typeof formSchema>

export function AuthForm() {
  const router = useRouter()
  const [isLoading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  
  const form = useForm<FormName>({
    resolver: zodResolver(formSchema),
    defaultValues: { name: "" }
  })

  async function onSubmit(data: FormName) {
    try {
      setLoading(true)
      setError(null)

      const result = await invoke<string>("create_user", {
        name: data.name.trim()
      })

      toast.success("Registration successful!", {
        description: result
      })
      
      router.push("/")
    } catch (error) {
      const message = error instanceof Error 
        ? error.message 
        : "An error occurred while creating your account. Please try again."
      
      setError(message)
      toast.error("Registration failed", {
        description: message
      })
    } finally {
      setLoading(false)
    }
  } 

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader>
        <CardTitle className="text-3xl">Welcome to Study Studio!</CardTitle>
        <CardDescription className="text-lg">
          Tell a little bit about you
        </CardDescription>
      </CardHeader>
      
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel className="font-lato font-semibold">
                    Name
                  </FormLabel>
                  <FormControl>
                    <Input
                      placeholder="Your Name"
                      {...field}
                      disabled={isLoading}
                      autoComplete="name"
                    />
                  </FormControl>
                  <FormDescription>
                    How should we call you?
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            {error && (
              <p className="text-red-500 text-sm">{error}</p>
            )}

            <BgAnimateButton
              gradient={"nebula"}
              key={"pulse"}
              rounded={"sm"}
              className="w-full"
              type="submit"
              disabled={isLoading}
            >
              {isLoading ? "Processing..." : "Sign In"}
            </BgAnimateButton>
          </form>
        </Form>
      </CardContent>
    </Card>
  )
}