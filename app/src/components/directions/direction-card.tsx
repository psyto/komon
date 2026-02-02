"use client";

import { User, ThumbsUp, ThumbsDown, Sparkles, AlertTriangle } from "lucide-react";
import { cn, formatCurrency, formatRelativeTime } from "@/lib/utils";

interface DirectionCardProps {
  direction: {
    id: string;
    description: string;
    proposer: { id: string; displayName: string };
    yesSupply: bigint;
    noSupply: bigint;
    usdcLiquidity: bigint;
    createdAt: string;
    status: string;
    aiAnalysis?: {
      feasibilityScore: number;
      reasoning: string;
      strengths: string[];
      weaknesses: string[];
      estimatedCost: string;
      estimatedTime: string;
      riskLevel: string;
    };
  };
  onStake: (directionId: string, outcome: "YES" | "NO") => void;
}

const riskColors: Record<string, string> = {
  LOW: "text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/30",
  MEDIUM: "text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900/30",
  HIGH: "text-orange-600 bg-orange-100 dark:text-orange-400 dark:bg-orange-900/30",
  CRITICAL: "text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/30",
};

export function DirectionCard({ direction, onStake }: DirectionCardProps) {
  const totalSupply = Number(direction.yesSupply) + Number(direction.noSupply);
  const yesPercent = totalSupply > 0 ? (Number(direction.yesSupply) / totalSupply) * 100 : 50;
  const noPercent = totalSupply > 0 ? (Number(direction.noSupply) / totalSupply) * 100 : 50;

  return (
    <div className="rounded-lg border bg-card p-6">
      {/* Header */}
      <div className="flex items-start justify-between gap-4 mb-4">
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <User className="h-4 w-4" />
          <span>{direction.proposer.displayName}</span>
          <span>•</span>
          <span>{formatRelativeTime(direction.createdAt)}</span>
        </div>
        {direction.aiAnalysis && (
          <div className="flex items-center gap-2">
            <span
              className={cn(
                "inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium",
                riskColors[direction.aiAnalysis.riskLevel]
              )}
            >
              <AlertTriangle className="h-3 w-3" />
              {direction.aiAnalysis.riskLevel} Risk
            </span>
          </div>
        )}
      </div>

      {/* Description */}
      <p className="text-sm mb-4">{direction.description}</p>

      {/* AI Analysis */}
      {direction.aiAnalysis && (
        <div className="bg-muted/50 rounded-lg p-4 mb-4">
          <div className="flex items-center gap-2 mb-3">
            <Sparkles className="h-4 w-4 text-primary" />
            <span className="text-sm font-medium">AI Analysis</span>
            <span
              className={cn(
                "ml-auto text-sm font-bold",
                direction.aiAnalysis.feasibilityScore >= 70
                  ? "text-green-600"
                  : direction.aiAnalysis.feasibilityScore >= 50
                  ? "text-yellow-600"
                  : "text-red-600"
              )}
            >
              {direction.aiAnalysis.feasibilityScore}% Feasible
            </span>
          </div>
          <p className="text-sm text-muted-foreground mb-3">{direction.aiAnalysis.reasoning}</p>
          <div className="grid grid-cols-2 gap-4 text-xs">
            <div>
              <span className="text-muted-foreground">Est. Cost:</span>{" "}
              <span className="font-medium">{direction.aiAnalysis.estimatedCost}</span>
            </div>
            <div>
              <span className="text-muted-foreground">Est. Time:</span>{" "}
              <span className="font-medium">{direction.aiAnalysis.estimatedTime}</span>
            </div>
          </div>
        </div>
      )}

      {/* Stake Progress */}
      <div className="mb-4">
        <div className="flex justify-between text-sm mb-2">
          <span className="flex items-center gap-1 text-green-600">
            <ThumbsUp className="h-4 w-4" />
            YES ({yesPercent.toFixed(1)}%)
          </span>
          <span className="flex items-center gap-1 text-red-600">
            NO ({noPercent.toFixed(1)}%)
            <ThumbsDown className="h-4 w-4" />
          </span>
        </div>
        <div className="h-2 rounded-full bg-muted overflow-hidden flex">
          <div
            className="h-full bg-green-500 transition-all"
            style={{ width: `${yesPercent}%` }}
          />
          <div
            className="h-full bg-red-500 transition-all"
            style={{ width: `${noPercent}%` }}
          />
        </div>
        <div className="flex justify-between text-xs text-muted-foreground mt-1">
          <span>{formatCurrency(direction.yesSupply)}</span>
          <span>Total: {formatCurrency(direction.usdcLiquidity)}</span>
          <span>{formatCurrency(direction.noSupply)}</span>
        </div>
      </div>

      {/* Stake Buttons */}
      <div className="flex gap-2">
        <button
          onClick={() => onStake(direction.id, "YES")}
          className="flex-1 inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring h-10 px-4 border border-green-500 text-green-600 hover:bg-green-50 dark:hover:bg-green-900/20"
        >
          <ThumbsUp className="h-4 w-4" />
          Stake YES
        </button>
        <button
          onClick={() => onStake(direction.id, "NO")}
          className="flex-1 inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring h-10 px-4 border border-red-500 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20"
        >
          <ThumbsDown className="h-4 w-4" />
          Stake NO
        </button>
      </div>
    </div>
  );
}
