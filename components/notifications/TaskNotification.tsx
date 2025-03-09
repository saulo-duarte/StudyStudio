import React, { useEffect, useState } from 'react';
import { Bell } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Task } from '@/types/Task';

export function TaskNotification() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    loadTasksForToday();
  }, []);

  const loadTasksForToday = async () => {
    try {
      const todayTasks = await invoke<Task[]>('get_tasks_for_today');
      setTasks(todayTasks);
      
      if (todayTasks.length > 0) {
        showTaskToast(todayTasks);
      }
    } catch (error) {
      console.error('Erro ao carregar tarefas:', error);
      toast.error("Error", {
        description: 'Não foi possível carregar as tarefas de hoje.',
      });
    }
  };

  const showTaskToast = (todayTasks: Task[]) => {
    toast.info(`${todayTasks.length} tarefas para hoje`, {
      description: (
        <div className="mt-2 max-h-[200px] overflow-y-auto">
          {todayTasks.slice(0, 3).map((task) => (
            <div key={task.id} className="mb-2 pb-2 border-b border-blue-500">
              <p className="font-medium">{task.title}</p>
              {task.description && (
                <p className="text-sm text-gray-500">{task.description}</p>
              )}
            </div>
          ))}
          {todayTasks.length > 3 && (
            <p className="text-sm text-gray-500">
              ...e mais {todayTasks.length - 3} tarefas
            </p>
          )}
        </div>
      ),
      duration: 5000,
      classNames: {
        toast:
          'group toast bg-background text-black border-2 border-blue-500 shadow-lg rounded-lg p-4 max-w-lg',
        description: 'text-sm text-gray-700', 
        actionButton:
          'bg-primary text-primary-foreground px-3 py-1 rounded-md hover:opacity-90 transition',
        cancelButton:
          'bg-muted text-muted-foreground px-3 py-1 rounded-md hover:bg-opacity-80 transition',
      },
    });
  };
  
  const markAsComplete = async (taskId: number) => {
    try {
      await invoke('mark_task_complete', { id: taskId });
      loadTasksForToday();
      toast('Tarefa concluída',{
        description: 'A tarefa foi marcada como concluída.',
      });
    } catch (error) {
      console.error('Erro ao marcar tarefa como concluída:', error);
    }
  };

  return (
    <Popover open={isOpen} onOpenChange={setIsOpen}>
      <PopoverTrigger asChild>
        <Button variant="ghost" size="sm" className="relative">
          <Bell className="h-5 w-5" />
          {tasks.length > 0 && (
            <Badge 
              className="absolute -top-1 -right-1 px-1 min-w-[18px] h-[18px] text-xs"
              variant="destructive"
            >
              {tasks.length}
            </Badge>
          )}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80 p-0" align="end">
        <div className="p-4 border-b border-gray-200">
          <h3 className="font-medium">Tarefas de Hoje</h3>
          <p className="text-sm text-gray-500">
            {tasks.length} {tasks.length === 1 ? 'tarefa pendente' : 'tarefas pendentes'}
          </p>
        </div>
        
        <div className="max-h-[300px] overflow-y-auto">
          {tasks.length > 0 ? (
            tasks.map((task) => (
              <div 
                key={task.id} 
                className="p-4 border-b border-gray-100 hover:bg-gray-900"
              >
                <div className="flex justify-between items-start">
                  <div>
                    <h4 className="font-medium">{task.title}</h4>
                    {task.description && (
                      <p className="text-sm text-gray-500 mt-1">{task.description}</p>
                    )}
                    {task.due_date && (
                      <p className="text-xs text-gray-400 mt-1">
                        Horário: {new Date(task.due_date).toLocaleTimeString()}
                      </p>
                    )}
                  </div>
                  <Button 
                    variant="ghost" 
                    size="sm" 
                    onClick={() => markAsComplete(task.id || 0)}
                    className="h-8 px-2"
                  >
                    Concluir
                  </Button>
                </div>
              </div>
            ))
          ) : (
            <div className="p-4 text-center text-gray-500">
              Nenhuma tarefa para hoje
            </div>
          )}
        </div>
        
        <div className="p-2 border-t border-gray-200">
          <Button 
            variant="outline" 
            size="sm" 
            className="w-full" 
            onClick={() => {
              setIsOpen(false);
            }}
          >
            Ver todas as tarefas
          </Button>
        </div>
      </PopoverContent>
    </Popover>
  );
}