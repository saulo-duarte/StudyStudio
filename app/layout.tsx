"use client";

import "./globals.css";
import { TopMenuRoot } from "./components/TopMenuRoot";
import { ThemeProvider } from "next-themes";
import { usePathname, useRouter } from "next/navigation";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from 'react'
import { Anta, Lato } from 'next/font/google'
import { Toaster } from "@/components/ui/toaster";
import { AppSidebar } from "@/components/sidebar/AppSidebar";

const anta = Anta({
  subsets: ['latin'],
  weight: '400',
  variable: '--font-anta'
})

const lato = Lato({
  subsets: ['latin'],
  weight: ['100', '300', '400', '700', '900'],
  style: ['normal', 'italic'],
  variable: '--font-lato'
})

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const [isLoading, setIsLoading] = useState(true);
  const [shouldRender, setShouldRender] = useState(false);
  const pathname = usePathname();

  useEffect(() => {
    const checkAuth = async () => {
      try {
        const activeUsers = await invoke<number>("get_active_users_count");
        if (activeUsers < 1) {
          router.push("/auth");
        }
        else if (activeUsers < 1 && pathname !== "/auth"){
          router.push("/");
        } else {
          setShouldRender(true);
        }
      } catch (error) {
        console.error('Auth error:', error);
        router.push("/auth");
      } finally {
        setIsLoading(false);
      }
    };

    checkAuth();
  }, [router]);

  if (isLoading) {
    return (
      <div className="flex h-screen w-full items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary" />
      </div>
    );
  }

  return shouldRender ? <>{children}</> : null;
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();
  const isAuthPage = pathname.startsWith("/auth");

  return (
    <html lang="en" className={`${anta.variable} ${lato.variable}`} suppressHydrationWarning>
      <body>
        <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
          {isAuthPage ? (
            <main>{children}</main>
          ) : (
            <AuthProvider>
              <div className="hidden-on-auth">
                  <div className="w-full flex h-screen overflow-hidden">
                <AppSidebar />
                    <div className="w-full flex flex-col flex-1 overflow-hidden">
                      <TopMenuRoot />
                      <main className="flex-1 overflow-y-auto pr-6 pl-6">
                        {children}
                        <Toaster />
                      </main>
                    </div>
                  </div>
              </div>
            </AuthProvider>
          )}
        </ThemeProvider>
      </body>
    </html>
  );
}