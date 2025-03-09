"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function AuthWrapper({ children }: { children: React.ReactNode }) {
  const router = useRouter();

  useEffect(() => {
    invoke<number>("get_active_users_count")
      .then((count) => {
        if (count !== 1) {
          router.push("/auth");
        }
      })
      .catch((error) => {
        console.error("Erro ao obter contagem de usuários ativos:", error);
      });
  }, [router]);

  return <>{children}</>;
}
