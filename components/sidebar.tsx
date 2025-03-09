"use client";

import Link from "next/link";
import { Home, Users, FileText, Settings } from "lucide-react";


const menuItems = [
  { icon: Home, label: "Home", href: "/" },
  { icon: Users, label: "Auth", href: "/auth" },
  { icon: FileText, label: "Tasks", href: "/tasks" },
];

export default function Sidebar() {
  return (
    <aside className="h-screen w-20 bg-gray-900 text-white flex flex-col items-center py-4">
      <div className="mb-4">
        <img src="/App Logo.svg" alt="App Logo" height={60} width={60} />
      </div>
      {menuItems.map(({ icon: Icon, label, href }) => (
        <Link key={href} href={href} className="flex flex-col items-center gap-1 p-4 hover:bg-gray-800 w-full">
          <Icon className="w-8 h-8" />
          <span className="text-xs">{label}</span>
        </Link>
      ))}
    </aside>
  );
}
