"use client";

import { useState } from "react";
import { X, DollarSign, AlertCircle, Shield } from "lucide-react";
import { cn, formatCurrency } from "@/lib/utils";
import { SovereignIdentity, getStakeLimit, getTierName } from "@/lib/solana/sovereign";

interface StakeDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  directionId: string | null;
  outcome: "YES" | "NO" | null;
  sovereignIdentity?: SovereignIdentity | null;
}

// Mock SOVEREIGN identity for demo
const mockSovereignIdentity: SovereignIdentity | null = {
  owner: null as any,
  tradingScore: 2500,
  civicScore: 6800,
  developerScore: 1200,
  infraScore: 800,
  compositeScore: 3264,
  tier: 2,
};

export function StakeDialog({
  open,
  onOpenChange,
  directionId,
  outcome,
  sovereignIdentity = mockSovereignIdentity,
}: StakeDialogProps) {
  const [amount, setAmount] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const tier = sovereignIdentity?.tier ?? 0;
  const stakeLimit = getStakeLimit(tier);
  const tierName = getTierName(tier);
  const amountNum = parseFloat(amount) || 0;
  const exceedsLimit = stakeLimit !== Infinity && amountNum > stakeLimit;

  if (!open || !directionId || !outcome) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    // Simulate API call
    await new Promise((resolve) => setTimeout(resolve, 1000));

    // In production, call the stakes API
    console.log("Staking", { directionId, outcome, amount });

    setIsSubmitting(false);
    setAmount("");
    onOpenChange(false);
  };

  const presetAmounts = [10, 25, 50, 100];

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50"
        onClick={() => onOpenChange(false)}
      />

      {/* Dialog */}
      <div className="relative bg-background rounded-lg shadow-lg w-full max-w-md mx-4 p-6">
        {/* Close button */}
        <button
          onClick={() => onOpenChange(false)}
          className="absolute top-4 right-4 text-muted-foreground hover:text-foreground"
        >
          <X className="h-5 w-5" />
        </button>

        {/* Header */}
        <div className="mb-6">
          <h2 className="text-xl font-semibold">
            Stake on{" "}
            <span
              className={cn(
                "font-bold",
                outcome === "YES" ? "text-green-600" : "text-red-600"
              )}
            >
              {outcome}
            </span>
          </h2>
          <p className="text-sm text-muted-foreground mt-1">
            If this direction {outcome === "YES" ? "succeeds" : "fails"}, you'll share the pool
            with other {outcome} stakers.
          </p>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit}>
          {/* Amount Input */}
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Amount (USD)</label>
            <div className="relative">
              <DollarSign className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="0.00"
                min="1"
                step="0.01"
                className="flex h-10 w-full rounded-md border border-input bg-background pl-10 pr-4 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              />
            </div>
          </div>

          {/* Preset Amounts */}
          <div className="flex gap-2 mb-6">
            {presetAmounts.map((preset) => (
              <button
                key={preset}
                type="button"
                onClick={() => setAmount(preset.toString())}
                className={cn(
                  "flex-1 h-9 rounded-md text-sm font-medium transition-colors",
                  amount === preset.toString()
                    ? "bg-primary text-primary-foreground"
                    : "border border-input bg-background hover:bg-accent"
                )}
              >
                ${preset}
              </button>
            ))}
          </div>

          {/* SOVEREIGN Tier Info */}
          <div className="flex items-center justify-between p-3 rounded-md bg-muted/50 mb-4">
            <div className="flex items-center gap-2">
              <Shield className="h-4 w-4 text-primary" />
              <span className="text-sm font-medium">{tierName} Tier</span>
            </div>
            <span className="text-sm text-muted-foreground">
              Limit: {stakeLimit === Infinity ? "Unlimited" : `$${stakeLimit.toLocaleString()}`}
            </span>
          </div>

          {/* Exceeds limit warning */}
          {exceedsLimit && (
            <div className="flex items-start gap-2 p-3 rounded-md bg-red-50 dark:bg-red-950/50 mb-4 border border-red-200 dark:border-red-900">
              <AlertCircle className="h-4 w-4 text-red-600 mt-0.5" />
              <p className="text-xs text-red-600">
                Amount exceeds your {tierName} tier limit of ${stakeLimit.toLocaleString()}.
                Increase your SOVEREIGN score to unlock higher limits.
              </p>
            </div>
          )}

          {/* Info */}
          <div className="flex items-start gap-2 p-3 rounded-md bg-muted/50 mb-6">
            <AlertCircle className="h-4 w-4 text-muted-foreground mt-0.5" />
            <p className="text-xs text-muted-foreground">
              Stakes are locked until the problem is resolved. If your outcome is correct,
              you'll receive your proportional share of the total pool. If incorrect, you'll
              lose your stake.
            </p>
          </div>

          {/* Submit Button */}
          <button
            type="submit"
            disabled={!amount || parseFloat(amount) <= 0 || isSubmitting || exceedsLimit}
            className={cn(
              "w-full inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring h-10 px-4",
              outcome === "YES"
                ? "bg-green-600 text-white hover:bg-green-700"
                : "bg-red-600 text-white hover:bg-red-700",
              "disabled:opacity-50 disabled:cursor-not-allowed"
            )}
          >
            {isSubmitting ? (
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
            ) : (
              <>
                Stake {amount ? formatCurrency(parseFloat(amount) * 1_000_000) : "$0.00"} on{" "}
                {outcome}
              </>
            )}
          </button>
        </form>
      </div>
    </div>
  );
}
