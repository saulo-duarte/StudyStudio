import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Tag } from "@/types/tag";

function useTagData() {
  const [data, setData] = useState<Tag | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchTagData() {
      try {
        setLoading(true);
        const result = await invoke<Tag>("get_all_tags");
        setData(result);
      } catch (err) {
        setError(err instanceof Error ? err : new Error(String(err)));
      } finally {
        setLoading(false);
      }
    }
    fetchTagData();
  }, []);

  return { data, loading, error };
}

export default useTagData;
