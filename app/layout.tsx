"use client";

import "./globals.css";
import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/sidebar/AppSidebar";
import { TopMenuRoot } from "./components/TopMenuRoot";
import { ThemeProvider } from "next-themes";
import { usePathname } from "next/navigation";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();

  const isAuthPage = pathname.startsWith("/auth");

  return (
    <html lang="en" suppressHydrationWarning>
      <body>
        <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
          {isAuthPage ? (
            <div>
              <main>
                {children}
              </main>
            </div>
          ) : (
            <SidebarProvider>
              <div className="w-full flex h-screen overflow-hidden">
                <AppSidebar />
                <div className="w-full flex flex-col flex-1 overflow-hidden">
                  <TopMenuRoot />
                  <main className="flex-1 overflow-y-auto p-4 md:p-6 lg:p-8">
                    {children}
                  </main>
                </div>
              </div>
            </SidebarProvider>
          )}
        </ThemeProvider>
      </body>
    </html>
  );
}
