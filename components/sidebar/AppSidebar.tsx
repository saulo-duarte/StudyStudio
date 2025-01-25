import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
} from "@/components/ui/sidebar"

import { Home, ListTodo } from "lucide-react"

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
  ]

  return (
    <Sidebar className="hidden lg:block w-64 h-screen">
      <SidebarHeader className="h-16 flex items-center px-4 text-xl font-bold border-b">StudyStudio</SidebarHeader>
      <SidebarContent>
        <SidebarMenu>
          {sidebarItems.map((item) => (
            <SidebarMenuItem key={item.title}>
              <SidebarMenuButton asChild>
                <a href={item.url} className="flex items-center gap-2">
                  <item.icon className="w-5 h-5" />
                  <span>{item.title}</span>
                </a>
              </SidebarMenuButton>
            </SidebarMenuItem>
          ))}
        </SidebarMenu>
      </SidebarContent>
    </Sidebar>
  )
}

