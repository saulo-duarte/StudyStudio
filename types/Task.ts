export interface Task {
    id: number | null;
    user_id: number;
    tag: string,
    title: string;
    description: string | null;
    status: string;
    priority: string;
    created_at: string;
    updated_at: string;
    due_date: string;
  }