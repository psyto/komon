"use client";

import { useState, use } from "react";
import Link from "next/link";
import {
  MapPin,
  Clock,
  DollarSign,
  User,
  ThumbsUp,
  ThumbsDown,
  Sparkles,
  ArrowLeft,
  AlertTriangle,
} from "lucide-react";
import { cn, formatCurrency, formatDate, formatRelativeTime } from "@/lib/utils";
import { DirectionCard } from "@/components/directions/direction-card";
import { StakeDialog } from "@/components/staking/stake-dialog";

// Mock data
const mockProblem = {
  id: "1",
  title: "Pothole on Main Street causing traffic hazards",
  description:
    "Large pothole at the intersection of Main St and Oak Ave has been causing accidents and damage to vehicles for the past month. Multiple residents have reported flat tires and suspension damage. The city has been notified but no action has been taken.",
  category: "Infrastructure",
  locationName: "Downtown",
  locationLat: 37.7749,
  locationLng: -122.4194,
  bountyAmount: BigInt(500_000_000),
  deadline: new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString(),
  createdAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
  status: "open" as const,
  creator: {
    id: "user1",
    displayName: "CivicMinded",
  },
  aiAnalysis: {
    feasibility: "This problem is highly feasible to solve with standard road maintenance procedures. The city has dedicated budgets for pothole repairs.",
    estimatedImpact: "Fixing this pothole would improve safety for hundreds of daily commuters and prevent ongoing vehicle damage costs for residents.",
    suggestedApproaches: [
      "Contact city public works department with photo evidence",
      "Organize community petition to prioritize repair",
      "Report through official 311 service",
    ],
    risks: [
      "City may have budget constraints",
      "Winter weather could delay repairs",
    ],
  },
  directions: [
    {
      id: "d1",
      description:
        "Organize a community petition with 500+ signatures and submit to city council during public comment period. Include photos and damage reports.",
      proposer: { id: "user2", displayName: "ActionTaker" },
      yesSupply: BigInt(300_000_000),
      noSupply: BigInt(100_000_000),
      usdcLiquidity: BigInt(400_000_000),
      createdAt: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(),
      status: "ACTIVE" as const,
      aiAnalysis: {
        feasibilityScore: 78,
        reasoning: "Community petitions have successfully prompted city action on similar issues in this area.",
        strengths: ["High community engagement potential", "Creates public record"],
        weaknesses: ["Takes time to gather signatures", "No guarantee of response"],
        estimatedCost: "$50-100 for printing",
        estimatedTime: "2-4 weeks",
        riskLevel: "LOW" as const,
      },
    },
    {
      id: "d2",
      description:
        "Crowdfund repairs and hire a licensed contractor directly. Present completed work to city for reimbursement under citizen improvement programs.",
      proposer: { id: "user3", displayName: "PragmaticSolver" },
      yesSupply: BigInt(150_000_000),
      noSupply: BigInt(200_000_000),
      usdcLiquidity: BigInt(350_000_000),
      createdAt: new Date(Date.now() - 12 * 60 * 60 * 1000).toISOString(),
      status: "ACTIVE" as const,
      aiAnalysis: {
        feasibilityScore: 45,
        reasoning: "Direct repairs on public roads may face legal and liability issues without city approval.",
        strengths: ["Fastest path to repair", "Shows community initiative"],
        weaknesses: ["Legal complications", "May not qualify for reimbursement"],
        estimatedCost: "$500-1500",
        estimatedTime: "1-2 weeks",
        riskLevel: "HIGH" as const,
      },
    },
  ],
};

export default function ProblemDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const [problem] = useState(mockProblem);
  const [stakeDialogOpen, setStakeDialogOpen] = useState(false);
  const [selectedDirection, setSelectedDirection] = useState<string | null>(null);
  const [selectedOutcome, setSelectedOutcome] = useState<"YES" | "NO" | null>(null);

  const daysUntilDeadline = Math.ceil(
    (new Date(problem.deadline).getTime() - Date.now()) / (1000 * 60 * 60 * 24)
  );

  const handleStake = (directionId: string, outcome: "YES" | "NO") => {
    setSelectedDirection(directionId);
    setSelectedOutcome(outcome);
    setStakeDialogOpen(true);
  };

  return (
    <div className="container py-8 px-4 md:px-6">
      <div className="max-w-4xl mx-auto">
        {/* Back link */}
        <Link
          href="/problems"
          className="inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground mb-6"
        >
          <ArrowLeft className="h-4 w-4" />
          Back to Problems
        </Link>

        {/* Problem Header */}
        <div className="mb-8">
          <div className="flex items-start justify-between gap-4 mb-4">
            <span
              className={cn(
                "inline-flex items-center rounded-full px-3 py-1 text-sm font-medium",
                "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200"
              )}
            >
              {problem.category}
            </span>
            <span
              className={cn(
                "inline-flex items-center rounded-full px-3 py-1 text-sm font-medium capitalize",
                "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
              )}
            >
              {problem.status}
            </span>
          </div>

          <h1 className="text-3xl font-bold tracking-tight mb-4">{problem.title}</h1>

          <p className="text-muted-foreground mb-6">{problem.description}</p>

          <div className="flex flex-wrap items-center gap-4 text-sm">
            <span className="flex items-center gap-1">
              <MapPin className="h-4 w-4" />
              {problem.locationName}
            </span>
            <span className="flex items-center gap-1">
              <Clock className="h-4 w-4" />
              {daysUntilDeadline > 0 ? `${daysUntilDeadline} days left` : "Expired"}
            </span>
            <span className="flex items-center gap-1">
              <DollarSign className="h-4 w-4 text-success" />
              {formatCurrency(problem.bountyAmount)} bounty
            </span>
            <span className="flex items-center gap-1">
              <User className="h-4 w-4" />
              Posted by {problem.creator.displayName}
            </span>
          </div>
        </div>

        {/* AI Analysis */}
        {problem.aiAnalysis && (
          <div className="rounded-lg border bg-card p-6 mb-8">
            <div className="flex items-center gap-2 mb-4">
              <Sparkles className="h-5 w-5 text-primary" />
              <h2 className="text-lg font-semibold">AI Analysis</h2>
            </div>

            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium mb-1">Feasibility</h3>
                <p className="text-sm text-muted-foreground">{problem.aiAnalysis.feasibility}</p>
              </div>

              <div>
                <h3 className="text-sm font-medium mb-1">Estimated Impact</h3>
                <p className="text-sm text-muted-foreground">{problem.aiAnalysis.estimatedImpact}</p>
              </div>

              <div>
                <h3 className="text-sm font-medium mb-2">Suggested Approaches</h3>
                <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
                  {problem.aiAnalysis.suggestedApproaches.map((approach, i) => (
                    <li key={i}>{approach}</li>
                  ))}
                </ul>
              </div>

              {problem.aiAnalysis.risks.length > 0 && (
                <div>
                  <h3 className="text-sm font-medium mb-2 flex items-center gap-1">
                    <AlertTriangle className="h-4 w-4 text-yellow-500" />
                    Risks
                  </h3>
                  <ul className="list-disc list-inside text-sm text-muted-foreground space-y-1">
                    {problem.aiAnalysis.risks.map((risk, i) => (
                      <li key={i}>{risk}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Directions */}
        <div>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold">
              Proposed Directions ({problem.directions.length})
            </h2>
            <Link
              href={`/problems/${id}/propose`}
              className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4"
            >
              Propose Direction
            </Link>
          </div>

          <div className="space-y-4">
            {problem.directions.map((direction) => (
              <DirectionCard
                key={direction.id}
                direction={direction}
                onStake={handleStake}
              />
            ))}
          </div>

          {problem.directions.length === 0 && (
            <div className="text-center py-12 border rounded-lg">
              <p className="text-muted-foreground mb-4">
                No directions proposed yet. Be the first to suggest a solution!
              </p>
              <Link
                href={`/problems/${id}/propose`}
                className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4"
              >
                Propose Direction
              </Link>
            </div>
          )}
        </div>
      </div>

      {/* Stake Dialog */}
      <StakeDialog
        open={stakeDialogOpen}
        onOpenChange={setStakeDialogOpen}
        directionId={selectedDirection}
        outcome={selectedOutcome}
      />
    </div>
  );
}
