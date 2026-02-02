import Link from "next/link";
import { MapPin, Clock, MessageSquare, DollarSign } from "lucide-react";
import { cn, formatCurrency, formatRelativeTime } from "@/lib/utils";

export interface ProblemCardProps {
  problem: {
    id: string;
    title: string;
    description: string;
    category: string;
    locationName?: string;
    bountyAmount: bigint;
    directionCount: number;
    deadline: string;
    createdAt: string;
    status: "open" | "in_progress" | "resolved" | "closed";
  };
}

const categoryColors: Record<string, string> = {
  Infrastructure: "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200",
  Environment: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  Safety: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
  Health: "bg-pink-100 text-pink-800 dark:bg-pink-900 dark:text-pink-200",
  Education: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
  Transportation: "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
  Housing: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
  Community: "bg-teal-100 text-teal-800 dark:bg-teal-900 dark:text-teal-200",
  Economic: "bg-indigo-100 text-indigo-800 dark:bg-indigo-900 dark:text-indigo-200",
  Other: "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200",
};

const statusColors: Record<string, string> = {
  open: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  in_progress: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
  resolved: "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
  closed: "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200",
};

export function ProblemCard({ problem }: ProblemCardProps) {
  const daysUntilDeadline = Math.ceil(
    (new Date(problem.deadline).getTime() - Date.now()) / (1000 * 60 * 60 * 24)
  );

  return (
    <Link href={`/problems/${problem.id}`}>
      <div className="group rounded-lg border bg-card text-card-foreground shadow-sm hover:shadow-md transition-all hover:border-primary/50 p-4 h-full flex flex-col">
        <div className="flex items-start justify-between gap-2 mb-2">
          <span
            className={cn(
              "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium",
              categoryColors[problem.category] || categoryColors.Other
            )}
          >
            {problem.category}
          </span>
          <span
            className={cn(
              "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium capitalize",
              statusColors[problem.status]
            )}
          >
            {problem.status.replace("_", " ")}
          </span>
        </div>

        <h3 className="font-semibold text-lg mb-2 line-clamp-2 group-hover:text-primary transition-colors">
          {problem.title}
        </h3>

        <p className="text-sm text-muted-foreground line-clamp-2 mb-4 flex-1">
          {problem.description}
        </p>

        <div className="flex flex-wrap items-center gap-3 text-sm text-muted-foreground">
          {problem.locationName && (
            <span className="flex items-center gap-1">
              <MapPin className="h-3.5 w-3.5" />
              {problem.locationName}
            </span>
          )}
          <span className="flex items-center gap-1">
            <Clock className="h-3.5 w-3.5" />
            {daysUntilDeadline > 0 ? `${daysUntilDeadline}d left` : "Expired"}
          </span>
          <span className="flex items-center gap-1">
            <MessageSquare className="h-3.5 w-3.5" />
            {problem.directionCount} directions
          </span>
        </div>

        <div className="flex items-center justify-between mt-4 pt-4 border-t">
          <span className="flex items-center gap-1 text-sm font-medium">
            <DollarSign className="h-4 w-4 text-success" />
            {formatCurrency(problem.bountyAmount)} bounty
          </span>
          <span className="text-xs text-muted-foreground">
            {formatRelativeTime(problem.createdAt)}
          </span>
        </div>
      </div>
    </Link>
  );
}
