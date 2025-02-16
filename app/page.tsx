"use client"

import { TagsMultiselect } from "@/components/ui/multi-select"

export default function Home() {
  const handleSelect = (selectedOptions: string[]) => {
    console.log("Selected options:", selectedOptions)
  }

  return (
    <div className="p-4 max-w-md mx-auto">
      <h1 className="text-2xl font-bold mb-4">Select Frameworks</h1>
      <TagsMultiselect
        placeholder="Select tags..."
        emptyMessage="No frameworks found."
      />
    </div>
  )
}

