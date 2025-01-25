import { TopMenuBreadcrumb } from "./TopMenuBreadcrumble"
import { SidebarTrigger } from "@/components/ui/sidebar"
import ThemeSwitcher from "./SwitchThemeButton"

export function TopMenuRoot() {
  return (
    <header className="flex items-center justify-between bg-background border-b h-16 px-4">
      <div className="flex items-center gap-4">
        <SidebarTrigger />
        <TopMenuBreadcrumb />
      </div>
      <ThemeSwitcher />
    </header>
  )
}
