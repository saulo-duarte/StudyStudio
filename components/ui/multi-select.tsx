"use client";

import * as React from "react";
import { Check, ChevronsUpDown, Plus } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "./input";

export interface TagOption {
  value: string;
  label: string;
  color: string;
}

interface TagsMultiselectProps {
  placeholder?: string;
  emptyMessage?: string;
  value?: (TagOption | string)[];
  onChange?: (selectedOptions: TagOption[]) => void;
  className?: string;
}

const transformTag = (tag: any): TagOption => ({
  value: tag.id ? tag.id.toString() : (tag.value || ""),
  label: tag.name || tag.tag_name || tag.label || "Sem nome",
  color: tag.color || tag.tag_color || "#0000ff",
});

export function TagsMultiselect({
  placeholder = "Select tags...",
  emptyMessage = "No tags found.",
  value = [],
  onChange,
  className,
}: TagsMultiselectProps) {
  const [open, setOpen] = React.useState(false);
  const [selected, setSelected] = React.useState<TagOption[]>([]);
  const [search, setSearch] = React.useState("");
  const [options, setOptions] = React.useState<TagOption[]>([]);
  const [newTagName, setNewTagName] = React.useState("");
  const [newTagColor, setNewTagColor] = React.useState("#0000ff");

  React.useEffect(() => {
    const formattedSelected = value.map((val) =>
      typeof val === "string"
        ? options.find((opt) => opt.value === val) || { value: val, label: val, color: "#0000ff" }
        : val
    );
    setSelected(formattedSelected);
  }, [value, options]);

  const fetchTags = async () => {
    try {
        const tags = (await invoke("list_tags")) as any[];
        const formattedTags = tags
            .filter((tag) => tag != null)
            .map(transformTag)
            .filter((tag) => tag.label !== "");
        setOptions(formattedTags);
    } catch (error) {
        console.error("Erro ao buscar tags:",  JSON.stringify(error));
    }
};
  React.useEffect(() => {
    fetchTags();
  }, []);

  const handleSelect = (option: TagOption) => {
    const isSelected = selected.some((item) => item.value === option.value);
    const newSelected = isSelected
      ? selected.filter((item) => item.value !== option.value)
      : [...selected, option];

    setSelected(newSelected);
    onChange && onChange(newSelected);
  };

  const handleCreateNew = async () => {
    if (!newTagName.trim()) return;

    try {
      await invoke("create_tag", { name: newTagName.trim(), color: newTagColor });
      await fetchTags();
      setNewTagName("");
    } catch (error) {
      console.error("Erro ao criar tag:", error);
    }
  };

  const filteredOptions = options.filter((option) =>
    option.label.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className={cn("w-full justify-between", className)}
        >
          {selected.length > 0 ? (
            <div className="flex flex-wrap gap-1">
              {selected.map((option) => (
                <Badge key={option.value} style={{ backgroundColor: option.color }}>
                  {option.label}
                </Badge>
              ))}
            </div>
          ) : (
            placeholder
          )}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] p-0">
        <Command>
          <CommandInput placeholder="Search tags..." value={search} onValueChange={setSearch} />
          <CommandList>
            <CommandEmpty>{emptyMessage}</CommandEmpty>
            <CommandGroup>
              <ScrollArea className="h-[200px]">
                {filteredOptions.map((option) => (
                  <CommandItem key={option.value} onSelect={() => handleSelect(option)}>
                    <Check
                      className={cn(
                        "mr-2 h-4 w-4",
                        selected.some((item) => item.value === option.value) ? "opacity-100" : "opacity-0"
                      )}
                    />
                    <Badge style={{ backgroundColor: option.color }} className="mr-2">
                      {option.label}
                    </Badge>
                  </CommandItem>
                ))}
              </ScrollArea>
            </CommandGroup>
          </CommandList>
        </Command>
        <div className="p-2 border-t flex flex-col gap-2">
          <div className="flex items-center gap-2">
            <Input
              type="text"
              value={newTagName}
              onChange={(e) => setNewTagName(e.target.value)}
              placeholder="New tag name"
              className="border p-1 flex-1 rounded"
            />
            <Input
              type="color"
              value={newTagColor}
              onChange={(e) => setNewTagColor(e.target.value)}
              className="w-10 h-10 p-0 border-0 rounded"
            />
          </div>
          <Button variant="outline" className="w-full" onClick={handleCreateNew}>
            <Plus className="mr-2 h-4 w-4" />
            Create Tag
          </Button>
        </div>
      </PopoverContent>
    </Popover>
  );
}
