import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
  } from "@/components/ui/dialog";
  import { RiStickyNoteAddFill } from "react-icons/ri";
  import { CreateTaskForm } from "./TaskForm"
  
  export function TasksDialog() {
    
    return (
      <Dialog>
        <DialogTrigger className="flex mt-4 p-2 h-[36px] rounded-lg font-lato font-semibold text-black/80 text-sm bg-primary">
          <RiStickyNoteAddFill size={20}/> 
          Create Task
          </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle> Add new task</DialogTitle>
              <DialogDescription>
                Inform the data
              </DialogDescription>
          </DialogHeader>
          <CreateTaskForm />
        </DialogContent>
      </Dialog>
    );
  }
  