import { TopMenuBreadcrumb } from "./TopMenuBreadcrumble"
import WindowControls from "./WindowControls"
import ThemeSwitcher from "./SwitchThemeButton"

export function TopMenuRoot() {
  return (
    <header className="flex items-center justify-between bg-background border-b h-16 px-4">
        <TopMenuBreadcrumb />
        <ThemeSwitcher />
        <WindowControls />
    </header>
  )
}
