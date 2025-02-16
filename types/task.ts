export interface Task {
  id: number | null;
  user_id: number;
  tag: string,
  name: string;
  description: string | null;
  status: "todo" | "in_progress" | "done";
  created_at: string;
  updated_at: string;
  due_date: string;
}