"use client"

import { useTheme } from "next-themes"
import { Toaster as Sonner } from "sonner"
import { CheckCircle, XCircle, AlertTriangle, Info, Loader2 } from "lucide-react";

type ToasterProps = React.ComponentProps<typeof Sonner>

const Toaster = ({ ...props }: ToasterProps) => {
  const { theme = "system" } = useTheme()

  return (
    <Sonner
      theme={theme as ToasterProps["theme"]}
      className="toaster group"
      icons={{
        success: <CheckCircle className="text-blue-500 w-10 h-10" />,
        error: <XCircle className="text-red-500 w-10 h-10" />,
        warning: <AlertTriangle className="text-yellow-500 w-10 h-10" />,
        info: <Info className="text-blue-500 w-10 h-10" />,
        loading: <Loader2 className="animate-spin text-gray-500 w-10 h-10" />,
      }}
      toastOptions={{
        classNames: {
          toast:
            "group toast group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toast-success]:border-blue-500 border-2 shadow-lg rounded-lg px-6 py-4 max-w-lg transition-all duration-300",
          description: "group-[.toast]:text-muted-foreground text-sm",
          actionButton:
            "group-[.toast]:bg-primary group-[.toast]:text-primary-foreground px-3 py-1 rounded-md hover:opacity-90 transition",
          cancelButton:
            "group-[.toast]:bg-muted group-[.toast]:text-muted-foreground px-3 py-1 rounded-md hover:bg-opacity-80 transition",
        },
      }}
      {...props}
    />
  )
}

export { Toaster }
