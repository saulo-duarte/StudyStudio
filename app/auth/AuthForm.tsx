"use client"

import type React from "react"

import { useState } from "react"
import { z } from "zod"
import { invoke } from "@tauri-apps/api/core"
import { toast } from "sonner"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"

import { Loader2 } from "lucide-react"

const formSchema = z.object({
  name: z.string().min(2, {
    message: "Name must be at least 2 characters.",
  }),
})

export default function AuthForm() {
  const [name, setName] = useState("")
  const [error, setError] = useState<string | null>(null)
  const [isLoading, setIsLoading] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    setIsLoading(true)

    try {
      const result = formSchema.safeParse({ name })

      if (!result.success) {
        setError(result.error.errors[0].message)
        setIsLoading(false)
        return
      }

      await invoke("create_user", { name })

      toast("Event has been created", {
        description: "Sunday, December 03, 2023 at 9:00 AM",
        action: {
          label: "Undo",
          onClick: () => console.log("Undo"),
        },
      })

      setName("")
    } catch (err) {
      console.error("Error creating user:", err)
      setError(typeof err === "string" ? err : "Failed to create user. Please try again.")

      toast.error("Failed to create user", {
        description: typeof err === "string" ? err : "An unexpected error occurred.",
        duration: 3000, 
      });
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <Card className="w-full max-w-lg mx-auto">
      <CardHeader className="flex flex-col gap-4">
        <div className="flex gap-4 items-center">
          <img src="/App Logo.svg" alt="App Logo" height={60} width={60} />
          <CardTitle className="text-6xl">Study Studio</CardTitle>
        </div>
        <CardDescription>To continue, Tell about your name</CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="name">Name</Label>
            <Input
              id="name"
              placeholder="Enter your name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              disabled={isLoading}
            />
            {error && <p className="text-sm text-destructive">{error}</p>}
          </div>
          <Button type="submit" className="w-full" disabled={isLoading}>
            {isLoading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Creating...
              </>
            ) : (
              "Create Account"
            )}
          </Button>
        </form>
      </CardContent>
      <CardFooter className="flex justify-center text-sm text-muted-foreground">
      </CardFooter>
    </Card>
  )
}

