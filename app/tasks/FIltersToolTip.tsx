import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Filter } from "lucide-react";

export default function FiltersToolTip() {
  return (
    <TooltipProvider delayDuration={5}>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="outline">
            <Filter />
          </Button>
        </TooltipTrigger>
        <TooltipContent className="bg-muted">
          <div className="space-y-1">
            <p className="text-foreground font-semibold font-lato">Tooltip with title and icon</p>
            <p className="text-xs text-muted-foreground">
              Tooltips are made to be highly customizable, with features like dynamic placement,
              rich content, and a robust API.
            </p>
          </div>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}