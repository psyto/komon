"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import {
  User,
  Trophy,
  Target,
  TrendingUp,
  DollarSign,
  Zap,
  MapPin,
  MessageSquare,
  ArrowRight,
  Shield,
  Wallet,
} from "lucide-react";
import { cn, formatCurrency, formatPercent, formatDate, truncateAddress } from "@/lib/utils";
import { SovereignTierBadge } from "@/components/sovereign";
import { SovereignIdentity, getStakeLimit, fetchSovereignIdentity } from "@/lib/solana/sovereign";

// Mock user data - in production, fetch from Komon program
const mockUser = {
  id: "demo-user",
  displayName: "CivicChampion",
  createdAt: new Date(Date.now() - 90 * 24 * 60 * 60 * 1000).toISOString(),
  reputation: {
    problemsPosted: 12,
    directionsProposed: 45,
    directionsWon: 28,
    directionsLost: 8,
    winRate: 77.8,
    totalEarned: BigInt(15000_000_000),
    totalStaked: BigInt(25000_000_000),
    currentStreak: 5,
    bestStreak: 12,
    level: 15,
    experience: BigInt(45000),
    rank: 1,
  },
};

const mockProblems = [
  {
    id: "p1",
    title: "Pothole on Main Street",
    status: "open",
    directionCount: 3,
    bountyAmount: BigInt(500_000_000),
    createdAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
  },
  {
    id: "p2",
    title: "Broken playground equipment",
    status: "resolved",
    directionCount: 5,
    bountyAmount: BigInt(250_000_000),
    createdAt: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString(),
  },
];

const mockStakes = [
  {
    id: "s1",
    direction: {
      id: "d1",
      description: "Organize community petition",
      problem: { id: "p3", title: "Traffic light needed at intersection" },
    },
    amount: BigInt(100_000_000),
    outcome: "YES",
    status: "active",
    createdAt: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
  },
  {
    id: "s2",
    direction: {
      id: "d2",
      description: "Contact city council directly",
      problem: { id: "p4", title: "Park needs better lighting" },
    },
    amount: BigInt(50_000_000),
    outcome: "NO",
    status: "won",
    earnings: BigInt(85_000_000),
    createdAt: new Date(Date.now() - 15 * 24 * 60 * 60 * 1000).toISOString(),
  },
];

export default function ProfilePage() {
  const { publicKey, connected } = useWallet();
  const { connection } = useConnection();
  const [user] = useState(mockUser);
  const [activeTab, setActiveTab] = useState<"problems" | "stakes">("problems");
  const [sovereignIdentity, setSovereignIdentity] = useState<SovereignIdentity | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function loadIdentity() {
      if (!publicKey) {
        setSovereignIdentity(null);
        return;
      }

      setLoading(true);
      try {
        const identity = await fetchSovereignIdentity(connection, publicKey);
        setSovereignIdentity(identity);
      } catch (error) {
        console.error("Error fetching SOVEREIGN identity:", error);
        setSovereignIdentity(null);
      } finally {
        setLoading(false);
      }
    }

    loadIdentity();
  }, [publicKey, connection]);

  const stakeLimit = getStakeLimit(sovereignIdentity?.tier ?? 0);

  // Calculate level progress
  const currentLevelXp = user.reputation.level * (user.reputation.level + 1) * 50;
  const nextLevelXp = (user.reputation.level + 1) * (user.reputation.level + 2) * 50;
  const xpProgress =
    ((Number(user.reputation.experience) - currentLevelXp) / (nextLevelXp - currentLevelXp)) * 100;

  // Not connected state
  if (!connected) {
    return (
      <div className="container py-16 px-4 md:px-6">
        <div className="max-w-md mx-auto text-center">
          <div className="h-20 w-20 rounded-full bg-primary/10 flex items-center justify-center mx-auto mb-6">
            <Wallet className="h-10 w-10 text-primary" />
          </div>
          <h1 className="text-2xl font-bold mb-4">Connect Your Wallet</h1>
          <p className="text-muted-foreground mb-8">
            Connect your Solana wallet to view your profile, reputation, and SOVEREIGN identity.
          </p>
          <WalletMultiButton className="!bg-primary !text-primary-foreground !rounded-md !h-10 !text-sm !font-medium hover:!bg-primary/90 mx-auto" />
        </div>
      </div>
    );
  }

  return (
    <div className="container py-8 px-4 md:px-6">
      <div className="max-w-4xl mx-auto">
        {/* Profile Header */}
        <div className="rounded-lg border bg-card p-6 mb-8">
          <div className="flex items-start gap-6">
            <div className="h-20 w-20 rounded-full bg-primary/10 flex items-center justify-center">
              <User className="h-10 w-10 text-primary" />
            </div>
            <div className="flex-1">
              <div className="flex items-center gap-3 mb-2">
                <h1 className="text-2xl font-bold">
                  {publicKey ? truncateAddress(publicKey.toBase58(), 6) : user.displayName}
                </h1>
                {user.reputation.rank && user.reputation.rank <= 10 && (
                  <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                    <Trophy className="h-3 w-3" />
                    #{user.reputation.rank}
                  </span>
                )}
                <SovereignTierBadge identity={sovereignIdentity} loading={loading} size="sm" />
              </div>
              <p className="text-sm text-muted-foreground mb-4">
                {publicKey && (
                  <span className="font-mono text-xs">{publicKey.toBase58()}</span>
                )}
              </p>

              {/* Level Progress */}
              <div className="mb-4">
                <div className="flex items-center justify-between text-sm mb-1">
                  <span className="flex items-center gap-1 font-medium">
                    <Target className="h-4 w-4 text-primary" />
                    Level {user.reputation.level}
                  </span>
                  <span className="text-muted-foreground">
                    {Number(user.reputation.experience).toLocaleString()} XP
                  </span>
                </div>
                <div className="h-2 rounded-full bg-muted overflow-hidden">
                  <div
                    className="h-full bg-primary transition-all"
                    style={{ width: `${xpProgress}%` }}
                  />
                </div>
              </div>

              {/* Quick Stats */}
              <div className="flex flex-wrap gap-4 text-sm">
                <span className="flex items-center gap-1">
                  <TrendingUp
                    className={cn(
                      "h-4 w-4",
                      user.reputation.currentStreak > 0 ? "text-green-500" : "text-red-500"
                    )}
                  />
                  {user.reputation.currentStreak > 0 ? "+" : ""}
                  {user.reputation.currentStreak} streak
                </span>
                <span className="flex items-center gap-1">
                  <Zap className="h-4 w-4 text-yellow-500" />
                  {formatPercent(user.reputation.winRate)} win rate
                </span>
                <span className="flex items-center gap-1">
                  <DollarSign className="h-4 w-4 text-success" />
                  {formatCurrency(user.reputation.totalEarned)} earned
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
          <div className="rounded-lg border bg-card p-4 text-center">
            <MapPin className="h-5 w-5 text-primary mx-auto mb-2" />
            <div className="text-2xl font-bold">{user.reputation.problemsPosted}</div>
            <div className="text-sm text-muted-foreground">Problems Posted</div>
          </div>
          <div className="rounded-lg border bg-card p-4 text-center">
            <MessageSquare className="h-5 w-5 text-primary mx-auto mb-2" />
            <div className="text-2xl font-bold">{user.reputation.directionsProposed}</div>
            <div className="text-sm text-muted-foreground">Directions Proposed</div>
          </div>
          <div className="rounded-lg border bg-card p-4 text-center">
            <Trophy className="h-5 w-5 text-green-500 mx-auto mb-2" />
            <div className="text-2xl font-bold">{user.reputation.directionsWon}</div>
            <div className="text-sm text-muted-foreground">Directions Won</div>
          </div>
          <div className="rounded-lg border bg-card p-4 text-center">
            <Target className="h-5 w-5 text-yellow-500 mx-auto mb-2" />
            <div className="text-2xl font-bold">{user.reputation.bestStreak}</div>
            <div className="text-sm text-muted-foreground">Best Streak</div>
          </div>
        </div>

        {/* SOVEREIGN Identity */}
        <div className="rounded-lg border bg-card p-6 mb-8">
          <div className="flex items-center gap-2 mb-4">
            <Shield className="h-5 w-5 text-primary" />
            <h2 className="text-lg font-semibold">SOVEREIGN Identity</h2>
          </div>

          {loading ? (
            <div className="animate-pulse">
              <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
                {[...Array(5)].map((_, i) => (
                  <div key={i} className="text-center p-3 rounded-lg bg-muted/50">
                    <div className="h-6 bg-muted rounded w-16 mx-auto mb-1" />
                    <div className="h-3 bg-muted rounded w-12 mx-auto" />
                  </div>
                ))}
              </div>
            </div>
          ) : sovereignIdentity ? (
            <>
              <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
                <div className="text-center p-3 rounded-lg bg-muted/50">
                  <div className="text-xl font-bold text-primary">
                    {sovereignIdentity.compositeScore.toLocaleString()}
                  </div>
                  <div className="text-xs text-muted-foreground">Composite Score</div>
                </div>
                <div className="text-center p-3 rounded-lg bg-muted/50">
                  <div className="text-xl font-bold">
                    {sovereignIdentity.tradingScore.toLocaleString()}
                  </div>
                  <div className="text-xs text-muted-foreground">Trading</div>
                </div>
                <div className="text-center p-3 rounded-lg bg-muted/50">
                  <div className="text-xl font-bold text-green-500">
                    {sovereignIdentity.civicScore.toLocaleString()}
                  </div>
                  <div className="text-xs text-muted-foreground">Civic</div>
                </div>
                <div className="text-center p-3 rounded-lg bg-muted/50">
                  <div className="text-xl font-bold">
                    {sovereignIdentity.developerScore.toLocaleString()}
                  </div>
                  <div className="text-xs text-muted-foreground">Developer</div>
                </div>
                <div className="text-center p-3 rounded-lg bg-muted/50">
                  <div className="text-xl font-bold">
                    {sovereignIdentity.infraScore.toLocaleString()}
                  </div>
                  <div className="text-xs text-muted-foreground">Infra</div>
                </div>
              </div>
              <div className="mt-4 pt-4 border-t flex items-center justify-between text-sm">
                <span className="text-muted-foreground">
                  Your civic participation on Komon contributes to your SOVEREIGN score
                </span>
                <span className="font-medium">
                  Stake Limit: {stakeLimit === Infinity ? "Unlimited" : `$${stakeLimit.toLocaleString()}`}
                </span>
              </div>
            </>
          ) : (
            <div className="text-center py-6">
              <p className="text-muted-foreground mb-4">
                No SOVEREIGN identity found for this wallet.
              </p>
              <p className="text-sm text-muted-foreground">
                Create a SOVEREIGN identity to unlock tier-based benefits across the ecosystem.
              </p>
            </div>
          )}
        </div>

        {/* Tabs */}
        <div className="border-b mb-6">
          <div className="flex gap-4">
            <button
              onClick={() => setActiveTab("problems")}
              className={cn(
                "px-4 py-2 text-sm font-medium border-b-2 -mb-px transition-colors",
                activeTab === "problems"
                  ? "border-primary text-primary"
                  : "border-transparent text-muted-foreground hover:text-foreground"
              )}
            >
              My Problems
            </button>
            <button
              onClick={() => setActiveTab("stakes")}
              className={cn(
                "px-4 py-2 text-sm font-medium border-b-2 -mb-px transition-colors",
                activeTab === "stakes"
                  ? "border-primary text-primary"
                  : "border-transparent text-muted-foreground hover:text-foreground"
              )}
            >
              My Stakes
            </button>
          </div>
        </div>

        {/* Tab Content */}
        {activeTab === "problems" ? (
          <div className="space-y-4">
            {mockProblems.map((problem) => (
              <Link
                key={problem.id}
                href={`/problems/${problem.id}`}
                className="block rounded-lg border bg-card p-4 hover:border-primary/50 transition-colors"
              >
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-medium mb-1">{problem.title}</h3>
                    <div className="flex items-center gap-3 text-sm text-muted-foreground">
                      <span
                        className={cn(
                          "capitalize",
                          problem.status === "open"
                            ? "text-green-600"
                            : problem.status === "resolved"
                            ? "text-purple-600"
                            : "text-muted-foreground"
                        )}
                      >
                        {problem.status}
                      </span>
                      <span>{problem.directionCount} directions</span>
                      <span>{formatCurrency(problem.bountyAmount)} bounty</span>
                    </div>
                  </div>
                  <ArrowRight className="h-5 w-5 text-muted-foreground" />
                </div>
              </Link>
            ))}
            {mockProblems.length === 0 && (
              <div className="text-center py-8 text-muted-foreground">
                You haven't posted any problems yet.
              </div>
            )}
          </div>
        ) : (
          <div className="space-y-4">
            {mockStakes.map((stake) => (
              <div key={stake.id} className="rounded-lg border bg-card p-4">
                <div className="flex items-start justify-between">
                  <div>
                    <Link
                      href={`/problems/${stake.direction.problem.id}`}
                      className="text-sm text-muted-foreground hover:text-foreground"
                    >
                      {stake.direction.problem.title}
                    </Link>
                    <p className="font-medium mt-1">{stake.direction.description}</p>
                    <div className="flex items-center gap-3 text-sm mt-2">
                      <span
                        className={cn(
                          "font-medium",
                          stake.outcome === "YES" ? "text-green-600" : "text-red-600"
                        )}
                      >
                        {stake.outcome}
                      </span>
                      <span>{formatCurrency(stake.amount)} staked</span>
                      {stake.status === "won" && stake.earnings && (
                        <span className="text-success">
                          +{formatCurrency(stake.earnings)} earned
                        </span>
                      )}
                    </div>
                  </div>
                  <span
                    className={cn(
                      "text-xs font-medium px-2 py-1 rounded-full",
                      stake.status === "active"
                        ? "bg-blue-100 text-blue-800"
                        : stake.status === "won"
                        ? "bg-green-100 text-green-800"
                        : "bg-red-100 text-red-800"
                    )}
                  >
                    {stake.status}
                  </span>
                </div>
              </div>
            ))}
            {mockStakes.length === 0 && (
              <div className="text-center py-8 text-muted-foreground">
                You haven't placed any stakes yet.
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
