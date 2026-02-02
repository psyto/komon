"use client";

import { useState } from "react";
import { Trophy, TrendingUp, Zap, Target, Medal } from "lucide-react";
import { cn, formatCurrency, formatPercent } from "@/lib/utils";

// Mock leaderboard data
const mockLeaderboard = [
  {
    rank: 1,
    user: { id: "1", displayName: "CivicChampion", walletAddress: "Abc...xyz" },
    stats: {
      problemsPosted: 12,
      directionsProposed: 45,
      directionsWon: 28,
      directionsLost: 8,
      winRate: 77.8,
      totalEarned: "15000000000",
      totalStaked: "25000000000",
      currentStreak: 5,
      bestStreak: 12,
      level: 15,
      experience: "45000",
    },
  },
  {
    rank: 2,
    user: { id: "2", displayName: "ProblemSolver99", walletAddress: "Def...uvw" },
    stats: {
      problemsPosted: 8,
      directionsProposed: 38,
      directionsWon: 22,
      directionsLost: 10,
      winRate: 68.8,
      totalEarned: "12500000000",
      totalStaked: "20000000000",
      currentStreak: 3,
      bestStreak: 8,
      level: 12,
      experience: "32000",
    },
  },
  {
    rank: 3,
    user: { id: "3", displayName: "LocalHero", walletAddress: "Ghi...rst" },
    stats: {
      problemsPosted: 15,
      directionsProposed: 32,
      directionsWon: 18,
      directionsLost: 6,
      winRate: 75.0,
      totalEarned: "10000000000",
      totalStaked: "15000000000",
      currentStreak: 2,
      bestStreak: 10,
      level: 11,
      experience: "28000",
    },
  },
  {
    rank: 4,
    user: { id: "4", displayName: "CommunityFirst", walletAddress: "Jkl...opq" },
    stats: {
      problemsPosted: 20,
      directionsProposed: 25,
      directionsWon: 15,
      directionsLost: 5,
      winRate: 75.0,
      totalEarned: "8500000000",
      totalStaked: "12000000000",
      currentStreak: 4,
      bestStreak: 7,
      level: 10,
      experience: "24000",
    },
  },
  {
    rank: 5,
    user: { id: "5", displayName: "ActionTaker", walletAddress: "Mno...lmn" },
    stats: {
      problemsPosted: 5,
      directionsProposed: 42,
      directionsWon: 20,
      directionsLost: 12,
      winRate: 62.5,
      totalEarned: "7200000000",
      totalStaked: "18000000000",
      currentStreak: -2,
      bestStreak: 6,
      level: 9,
      experience: "20000",
    },
  },
];

const sortOptions = [
  { value: "totalEarned", label: "Total Earned" },
  { value: "winRate", label: "Win Rate" },
  { value: "directionsWon", label: "Directions Won" },
  { value: "currentStreak", label: "Current Streak" },
  { value: "level", label: "Level" },
];

export default function LeaderboardPage() {
  const [leaderboard] = useState(mockLeaderboard);
  const [sortBy, setSortBy] = useState("totalEarned");

  return (
    <div className="container py-8 px-4 md:px-6">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="flex items-center gap-3 mb-2">
          <Trophy className="h-8 w-8 text-yellow-500" />
          <h1 className="text-3xl font-bold tracking-tight">Leaderboard</h1>
        </div>
        <p className="text-muted-foreground mb-8">
          Top problem solvers ranked by their contributions and success rate
        </p>

        {/* Sort Controls */}
        <div className="flex items-center gap-4 mb-6">
          <span className="text-sm text-muted-foreground">Sort by:</span>
          <div className="flex gap-2">
            {sortOptions.map((option) => (
              <button
                key={option.value}
                onClick={() => setSortBy(option.value)}
                className={cn(
                  "px-3 py-1.5 rounded-md text-sm font-medium transition-colors",
                  sortBy === option.value
                    ? "bg-primary text-primary-foreground"
                    : "bg-muted hover:bg-muted/80"
                )}
              >
                {option.label}
              </button>
            ))}
          </div>
        </div>

        {/* Leaderboard Table */}
        <div className="rounded-lg border overflow-hidden">
          <table className="w-full">
            <thead className="bg-muted/50">
              <tr>
                <th className="px-4 py-3 text-left text-sm font-medium">Rank</th>
                <th className="px-4 py-3 text-left text-sm font-medium">User</th>
                <th className="px-4 py-3 text-right text-sm font-medium">Win Rate</th>
                <th className="px-4 py-3 text-right text-sm font-medium">Earned</th>
                <th className="px-4 py-3 text-right text-sm font-medium">Streak</th>
                <th className="px-4 py-3 text-right text-sm font-medium">Level</th>
              </tr>
            </thead>
            <tbody>
              {leaderboard.map((entry, index) => (
                <tr
                  key={entry.user.id}
                  className={cn(
                    "border-t",
                    index < 3 && "bg-yellow-50/50 dark:bg-yellow-900/10"
                  )}
                >
                  <td className="px-4 py-4">
                    <div className="flex items-center gap-2">
                      {entry.rank <= 3 ? (
                        <Medal
                          className={cn(
                            "h-5 w-5",
                            entry.rank === 1
                              ? "text-yellow-500"
                              : entry.rank === 2
                              ? "text-gray-400"
                              : "text-amber-600"
                          )}
                        />
                      ) : (
                        <span className="w-5 text-center text-muted-foreground">
                          {entry.rank}
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="px-4 py-4">
                    <div>
                      <div className="font-medium">{entry.user.displayName}</div>
                      <div className="text-xs text-muted-foreground">
                        {entry.stats.directionsWon}W / {entry.stats.directionsLost}L
                      </div>
                    </div>
                  </td>
                  <td className="px-4 py-4 text-right">
                    <span
                      className={cn(
                        "font-medium",
                        entry.stats.winRate >= 70
                          ? "text-green-600"
                          : entry.stats.winRate >= 50
                          ? "text-yellow-600"
                          : "text-red-600"
                      )}
                    >
                      {formatPercent(entry.stats.winRate)}
                    </span>
                  </td>
                  <td className="px-4 py-4 text-right">
                    <span className="font-medium text-success">
                      {formatCurrency(BigInt(entry.stats.totalEarned))}
                    </span>
                  </td>
                  <td className="px-4 py-4 text-right">
                    <span
                      className={cn(
                        "inline-flex items-center gap-1 font-medium",
                        entry.stats.currentStreak > 0
                          ? "text-green-600"
                          : entry.stats.currentStreak < 0
                          ? "text-red-600"
                          : "text-muted-foreground"
                      )}
                    >
                      {entry.stats.currentStreak > 0 ? (
                        <TrendingUp className="h-4 w-4" />
                      ) : entry.stats.currentStreak < 0 ? (
                        <Zap className="h-4 w-4" />
                      ) : null}
                      {Math.abs(entry.stats.currentStreak)}
                    </span>
                  </td>
                  <td className="px-4 py-4 text-right">
                    <span className="inline-flex items-center gap-1 font-medium">
                      <Target className="h-4 w-4 text-primary" />
                      {entry.stats.level}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {/* Stats Summary */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mt-8">
          <div className="rounded-lg border p-4 text-center">
            <div className="text-2xl font-bold text-primary">1,234</div>
            <div className="text-sm text-muted-foreground">Active Solvers</div>
          </div>
          <div className="rounded-lg border p-4 text-center">
            <div className="text-2xl font-bold text-success">$125,000</div>
            <div className="text-sm text-muted-foreground">Total Earned</div>
          </div>
          <div className="rounded-lg border p-4 text-center">
            <div className="text-2xl font-bold">892</div>
            <div className="text-sm text-muted-foreground">Directions Proposed</div>
          </div>
          <div className="rounded-lg border p-4 text-center">
            <div className="text-2xl font-bold">156</div>
            <div className="text-sm text-muted-foreground">Problems Resolved</div>
          </div>
        </div>
      </div>
    </div>
  );
}
