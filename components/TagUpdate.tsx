"use client";

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Pencil, Trash2 } from "lucide-react";
import { toast } from "sonner";

interface Tag {
  id: number;
  name: string;
  color: string;
}

export function TagManager() {
  const [tags, setTags] = useState<Tag[]>([]);
  const [selectedTag, setSelectedTag] = useState<Tag | null>(null);
  const [newTagName, setNewTagName] = useState("");
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  useEffect(() => {
    fetchTags();
  }, []);

  const fetchTags = async () => {
    try {
      const tagsList: Tag[] = await invoke("list_tags");
      setTags(tagsList);
    } catch (err) {
      toast.error("Error fetching tags", {
        description: String(err),
      });
    }
  };

  const handleUpdateTag = async () => {
    if (!selectedTag || !newTagName) return;

    try {
      const result: string = await invoke("update_tag", {
        id: selectedTag.id,
        tagName: newTagName,
      });
      
      setTags(tags.map(tag =>
        tag.id === selectedTag.id ? { ...tag, name: newTagName } : tag
      ));
      setIsDialogOpen(false);
      setNewTagName("");
      setSelectedTag(null);
      
      toast.success("Tag updated", {
        description: result,
      });
    } catch (err) {
      toast.error("Error updating tag", {
        description: String(err),
      });
    }
  };

  const handleDeleteTag = async (id: number) => {
    try {
      const result: string = await invoke("delete_tag", { id });
      
      setTags(tags.filter(tag => tag.id !== id));
      
      toast.success("Tag deleted", {
        description: result,
      });
    } catch (err) {
      toast.error("Error deleting tag", {
        description: String(err),
      });
    }
  };

  const openEditDialog = (tag: Tag) => {
    setSelectedTag(tag);
    setNewTagName(tag.name);
    setIsDialogOpen(true);
  };

  return (
    <div className="p-4">
      <Dialog>
        <DialogTrigger asChild>
          <Button variant="outline" className="mb-4">
            <Pencil className="mr-2 h-4 w-4" />
            Edit Tags
          </Button>
        </DialogTrigger>
        <DialogContent className="max-w-xl">
          <DialogHeader>
            <DialogTitle className="text-xl">Edit Tags</DialogTitle>
          </DialogHeader>
          <ScrollArea className="h-[400px] w-full rounded-md">
            <Table className="border-none">
              <TableHeader>
                <TableRow className="border-none hover:bg-transparent">
                  <TableHead className="font-medium">Name</TableHead>
                  <TableHead className="font-medium">Color</TableHead>
                  <TableHead className="font-medium">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {tags.map((tag) => (
                  <TableRow key={tag.id} className="border-none">
                    <TableCell>{tag.name}</TableCell>
                    <TableCell>
                      <div
                        className="w-6 h-6 rounded-full"
                        style={{ backgroundColor: tag.color }}
                      />
                    </TableCell>
                    <TableCell>
                      <div className="flex space-x-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => openEditDialog(tag)}
                        >
                          <Pencil className="h-4 w-4" />
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleDeleteTag(tag.id)}
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </ScrollArea>
        </DialogContent>
      </Dialog>

      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Update Tag</DialogTitle>
          </DialogHeader>
          {selectedTag && (
            <div className="space-y-4">
              <div>
                <label htmlFor="tagName" className="text-sm font-medium">
                  Tag Name
                </label>
                <Input
                  id="tagName"
                  value={newTagName}
                  onChange={(e) => setNewTagName(e.target.value)}
                  placeholder="Enter new tag name"
                />
              </div>
              <div className="flex justify-end space-x-2">
                <Button
                  variant="outline"
                  onClick={() => setIsDialogOpen(false)}
                >
                  Cancel
                </Button>
                <Button onClick={handleUpdateTag}>Update</Button>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}