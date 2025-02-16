
import { Home, ListTodo, Lock } from "lucide-react"
import Link from "next/link";

export function AppSidebar() {
  const sidebarItems = [
    {
      title: "Home",
      url: "/",
      icon: Home,
    },
    {
      title: "Tasks",
      url: "/tasks",
      icon: ListTodo,
    },
    {
      title: "Auth",
      url: "/auth",
      icon: Lock
    }
  ]

  return (
    <div className="sticky left-0 top-0 h-screen w-20 bg-sidebar text-foreground flex flex-col items-center py-4 gap-6">
      {sidebarItems.map((item, index) => (
        <Link
          key={index}
          href={item.url}
          className="flex flex-col items-center gap-1 p-4 rounded-lg text-foreground hover:text-foreground transition hover:bg-muted"
        >
          <item.icon size={32} />
          <span className="text-xs">{item.title}</span>
        </Link>
      ))}
    </div>
  );
}