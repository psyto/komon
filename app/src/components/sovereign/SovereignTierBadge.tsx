"use client";

import { useState, useEffect } from "react";
import { Shield, Crown, Gem, Medal, Award, Star } from "lucide-react";
import { cn } from "@/lib/utils";
import {
  SovereignIdentity,
  getTierName,
  getTierColor,
  getStakeLimit,
} from "@/lib/solana/sovereign";

export interface SovereignTierBadgeProps {
  identity: SovereignIdentity | null;
  loading?: boolean;
  showDetails?: boolean;
  size?: "sm" | "md" | "lg";
  className?: string;
}

const tierIcons = {
  0: Star,     // None
  1: Medal,    // Bronze
  2: Award,    // Silver
  3: Medal,    // Gold
  4: Gem,      // Platinum
  5: Crown,    // Diamond
};

const tierGradients: Record<number, string> = {
  0: "from-gray-400 to-gray-600",
  1: "from-amber-600 to-amber-800",
  2: "from-slate-300 to-slate-500",
  3: "from-yellow-400 to-yellow-600",
  4: "from-slate-200 to-slate-400",
  5: "from-cyan-300 to-blue-500",
};

const sizeClasses = {
  sm: {
    badge: "px-2 py-1 text-xs",
    icon: "h-3 w-3",
  },
  md: {
    badge: "px-3 py-1.5 text-sm",
    icon: "h-4 w-4",
  },
  lg: {
    badge: "px-4 py-2 text-base",
    icon: "h-5 w-5",
  },
};

export function SovereignTierBadge({
  identity,
  loading = false,
  showDetails = false,
  size = "md",
  className,
}: SovereignTierBadgeProps) {
  const tier = identity?.tier ?? 0;
  const tierName = getTierName(tier);
  const stakeLimit = getStakeLimit(tier);
  const Icon = tierIcons[tier as keyof typeof tierIcons] || Star;
  const gradient = tierGradients[tier] || tierGradients[0];
  const sizeClass = sizeClasses[size];

  if (loading) {
    return (
      <div
        className={cn(
          "inline-flex items-center gap-1.5 rounded-full bg-muted animate-pulse",
          sizeClass.badge,
          className
        )}
      >
        <div className={cn("rounded-full bg-muted-foreground/20", sizeClass.icon)} />
        <span className="text-transparent">Loading</span>
      </div>
    );
  }

  return (
    <div className={cn("inline-flex flex-col", className)}>
      <div
        className={cn(
          "inline-flex items-center gap-1.5 rounded-full bg-gradient-to-r text-white font-medium shadow-sm",
          gradient,
          sizeClass.badge
        )}
      >
        <Shield className={sizeClass.icon} />
        <span>{tierName}</span>
        {tier > 0 && <Icon className={sizeClass.icon} />}
      </div>

      {showDetails && identity && (
        <div className="mt-2 text-xs text-muted-foreground space-y-1">
          <div className="flex justify-between">
            <span>Composite Score:</span>
            <span className="font-medium">{identity.compositeScore.toLocaleString()}</span>
          </div>
          <div className="flex justify-between">
            <span>Trading:</span>
            <span>{identity.tradingScore.toLocaleString()}</span>
          </div>
          <div className="flex justify-between">
            <span>Civic:</span>
            <span>{identity.civicScore.toLocaleString()}</span>
          </div>
          <div className="flex justify-between">
            <span>Developer:</span>
            <span>{identity.developerScore.toLocaleString()}</span>
          </div>
          <div className="flex justify-between">
            <span>Infra:</span>
            <span>{identity.infraScore.toLocaleString()}</span>
          </div>
          <div className="flex justify-between pt-1 border-t">
            <span>Stake Limit:</span>
            <span className="font-medium">
              {stakeLimit === Infinity ? "Unlimited" : `$${stakeLimit.toLocaleString()}`}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

// Standalone component for navigation bar
export function SovereignNavBadge({
  identity,
  loading = false,
}: {
  identity: SovereignIdentity | null;
  loading?: boolean;
}) {
  const tier = identity?.tier ?? 0;
  const tierName = getTierName(tier);
  const Icon = tierIcons[tier as keyof typeof tierIcons] || Star;
  const gradient = tierGradients[tier] || tierGradients[0];

  if (loading) {
    return (
      <div className="inline-flex items-center gap-1 px-2 py-1 rounded-full bg-muted animate-pulse">
        <div className="h-3 w-3 rounded-full bg-muted-foreground/20" />
        <span className="text-xs text-transparent">---</span>
      </div>
    );
  }

  if (!identity) {
    return (
      <div className="inline-flex items-center gap-1 px-2 py-1 rounded-full bg-muted text-muted-foreground text-xs">
        <Shield className="h-3 w-3" />
        <span>No SOVEREIGN ID</span>
      </div>
    );
  }

  return (
    <div
      className={cn(
        "inline-flex items-center gap-1 px-2 py-1 rounded-full bg-gradient-to-r text-white text-xs font-medium shadow-sm",
        gradient
      )}
      title={`SOVEREIGN ${tierName} - Score: ${identity.compositeScore}`}
    >
      <Shield className="h-3 w-3" />
      <span>{tierName}</span>
      {tier > 0 && <Icon className="h-3 w-3" />}
    </div>
  );
}
