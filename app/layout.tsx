"use client";

import { usePathname } from "next/navigation";
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import AuthWrapper from "@/utils/auth-wrapper";
import { Toaster } from "@/components/ui/sonner";
import Sidebar from "@/components/sidebar";
import { TasksFilterProvider } from "@/hooks/context/use-task-context";
import { TaskNotification } from "@/components/notifications/TaskNotification";

const inter = Inter({
  subsets: ["latin"],
  weight: ["100", "200", "300", "400", "500", "600", "700", "800", "900"],
  variable: "--font-inter",
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();
  const hideSidebar = pathname.startsWith("/auth");

  return (
    <html lang="en" className="dark">
      <body className={`${inter.variable} antialiased`}>
        <div className="flex h-screen">
          {!hideSidebar && <Sidebar />}

          <div className="flex-1 overflow-auto p-6">
            <AuthWrapper>
              <TasksFilterProvider>
                <TaskNotification />
              {children}
              </TasksFilterProvider>
            </AuthWrapper>
          </div>
        </div>

        <Toaster />
      </body>
    </html>
  );
}
