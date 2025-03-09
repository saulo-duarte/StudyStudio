"use client";

import React from "react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import { CheckCircle, Clock, AlertTriangle, List } from "lucide-react";

interface TaskCardProps {
  title: string;
  value: number;
  description?: string;
  icon: React.ReactNode;
  className?: string;
}

export const TaskCard: React.FC<TaskCardProps> = ({
  title,
  value,
  description,
  icon,
  className,
}) => {
  return (
    <Card className={`rounded-xl shadow-md bg-gray-900 text-white border border-gray-900 transition-transform hover:scale-105 ${className}`}>
      <CardHeader className="flex items-center justify-between">
        <CardTitle className="text-xl font-semibold flex items-center gap-2">
          {icon}
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="text-4xl font-bold text-gray-100">{value}</div>
        {description && <CardDescription className="text-gray-400">{description}</CardDescription>}
      </CardContent>
    </Card>
  );
};