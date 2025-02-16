import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Tag } from "@/types/tag";
import { useToast } from "@/hooks/use-toast";

function useCreateTag(){
  const [loading, setLoading] = useState(true);
  const { toast } = useToast();
  const [error, setError] = useState<Error | null>();

  const createTag = async (tagData: Partial<Tag>): Promise<Tag | undefined> => {
    try {
      setLoading(true);
      const newTag = await invoke<Tag>("create_tag", { tag: tagData });
      
      toast({
        title: "Tag criada",
        description: "A tag foi criada com sucesso.",
      });
      
      return newTag;
    } catch (err) {
      const errorMsg = err instanceof Error ? err : new Error(String(err));
      setError(errorMsg);
      
      toast({
        title: "Erro ao criar tag",
        description: "Houve um problema ao criar a tag.",
        variant: "destructive",
      });
      
      return undefined;
    } finally {
      setLoading(false);
    }
  };

  return { createTag, loading, error };
}

export default useCreateTag;