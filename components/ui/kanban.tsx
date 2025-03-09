'use client';

import { Card } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import {
  DndContext,
  rectIntersection,
  useDraggable,
  useDroppable,
} from '@dnd-kit/core';
import type { DragEndEvent } from '@dnd-kit/core';
import type { ReactNode } from 'react';

export type { DragEndEvent } from '@dnd-kit/core';

export type Status = {
  id: string;
  name: string;
  color: string;
};

export type Feature = {
  id: string;
  name: string;
  startAt: Date;
  endAt: Date;
  status: Status;
};

export type KanbanBoardProps = {
  id: Status['id'];
  children: ReactNode;
  className?: string;
};

export const KanbanBoard = ({ id, children, className }: KanbanBoardProps) => {
  const { isOver, setNodeRef } = useDroppable({ id });

  return (
    <div
      className={cn(
        'flex h-full min-h-40 flex-col gap-2 rounded-md border p-2 text-xs shadow-sm outline outline-2 transition-all',
        isOver ? 'outline-primary' : 'outline-transparent',
        className
      )}
      ref={setNodeRef}
    >
      {children}
    </div>
  );
};

export type KanbanCardProps = Pick<Feature, 'id' | 'name'> & {
  index: number;
  parent: string;
  children?: ReactNode;
  className?: string;
};

export const KanbanCard = ({
  id,
  name,
  index,
  parent,
  children,
  className,
}: KanbanCardProps) => {
  const { attributes, listeners, setNodeRef, transform, isDragging } =
    useDraggable({
      id,
      data: { index, parent },
    });

  return (
    <Card
      className={cn(
        'rounded-md p-3 shadow-sm',
        isDragging && 'cursor-grabbing',
        className
      )}
      style={{
        transform: transform
          ? `translateX(${transform.x}px) translateY(${transform.y}px)`
          : 'none',
      }}
      {...listeners}
      {...attributes}
      ref={setNodeRef}
    >
      {children ?? <p className="m-0 font-medium text-sm">{name}</p>}
    </Card>
  );
};

export type KanbanCardsProps = {
  children: ReactNode;
  className?: string;
};

export const KanbanCards = ({ children, className }: KanbanCardsProps) => (
  <div className={cn('flex flex-1 flex-col gap-4', className)}>{children}</div>
);

export type KanbanHeaderProps = {
  status: string;
  name: string;
  count?: number;
  className?: string;
};

const headerStyles: Record<string, { borderColor: string; textColor: string; backgroundColor: string }> = {
  todo: {
    borderColor: "#dbd163",
    textColor: "#dbd163",
    backgroundColor: "#544f0e",
  },
  in_progress: {
    borderColor: "#63a1db",
    textColor: "#63a1db",
    backgroundColor: "#668ade",
  },
  done: {
    borderColor: "#6bdb63",
    textColor: "#6bdb63",
    backgroundColor: "#65b55c",
  },
  backlog: {
    borderColor: "#8550ab",
    textColor: "#8550ab",
    backgroundColor: "#9c66de",
  },
};

export const KanbanHeader = ({ status, name, count, className }: KanbanHeaderProps) => {
  const styles = headerStyles[status] || { borderColor: "#000", textColor: "#000" };

  return (
    <div 
      className={cn(
        'flex items-center gap-2 border-l-4 pl-2 rounded-lg',
        className
      )}
      style={{ borderColor: styles.borderColor }}
    >
      <div
        className="h-2 w-2 rounded-full"
        style={{ backgroundColor: styles.textColor }}
      />
      <p className="m-0 font-lato font-semibold text-[16px]" style={{ color: styles.textColor }}>
         {name}
      </p>
      {typeof count === "number" && (
         <span className="ml-2 text-xs font-medium" style={{ color: styles.textColor }}>
           {count}
         </span>
      )}
    </div>
  );
};

export type KanbanProviderProps = {
  children: ReactNode;
  onDragEnd: (event: DragEndEvent) => void;
  className?: string;
};

export const KanbanProvider = ({
  children,
  onDragEnd,
  className,
}: KanbanProviderProps) => (
  <DndContext collisionDetection={rectIntersection} onDragEnd={onDragEnd}>
    <div
      className={cn('grid w-full auto-cols-fr grid-flow-col gap-4', className)}
    >
      {children}
    </div>
  </DndContext>
);
