import Link from "next/link";
import { MapPin, Trophy, Zap, Users, ArrowRight } from "lucide-react";
import { ProblemCard } from "@/components/problems/problem-card";

// Mock data for demo
const recentProblems = [
  {
    id: "1",
    title: "Pothole on Main Street causing traffic hazards",
    description: "Large pothole at the intersection of Main St and Oak Ave has been causing accidents and damage to vehicles for the past month.",
    category: "Infrastructure",
    locationName: "Downtown",
    bountyAmount: BigInt(500_000_000), // 500 USDC
    directionCount: 3,
    deadline: new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString(),
    createdAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
    status: "open" as const,
  },
  {
    id: "2",
    title: "Illegal dumping in community park",
    description: "Construction waste and household garbage being dumped near the playground area. Health hazard for children.",
    category: "Environment",
    locationName: "Riverside Park",
    bountyAmount: BigInt(250_000_000), // 250 USDC
    directionCount: 5,
    deadline: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
    createdAt: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
    status: "open" as const,
  },
  {
    id: "3",
    title: "Broken streetlights creating safety concerns",
    description: "Multiple streetlights on Elm Street have been out for weeks, making the area unsafe for pedestrians at night.",
    category: "Safety",
    locationName: "Elm Street",
    bountyAmount: BigInt(150_000_000), // 150 USDC
    directionCount: 2,
    deadline: new Date(Date.now() + 21 * 24 * 60 * 60 * 1000).toISOString(),
    createdAt: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(),
    status: "open" as const,
  },
];

const stats = [
  { label: "Problems Posted", value: "247", icon: MapPin },
  { label: "Solutions Proposed", value: "892", icon: Zap },
  { label: "Problems Resolved", value: "156", icon: Trophy },
  { label: "Active Solvers", value: "1,234", icon: Users },
];

export default function Home() {
  return (
    <div className="flex flex-col gap-12 py-8">
      {/* Hero Section */}
      <section className="container px-4 md:px-6">
        <div className="flex flex-col items-center gap-4 text-center">
          <h1 className="text-4xl font-bold tracking-tighter sm:text-5xl md:text-6xl lg:text-7xl">
            Direct the Future.
            <br />
            <span className="text-primary">Get Rewarded.</span>
          </h1>
          <p className="mx-auto max-w-[700px] text-muted-foreground md:text-xl">
            Identify local problems, propose solutions, and earn rewards when your ideas work.
            Human direction + AI execution. Shared problems deserve shared solutions.
          </p>
          <div className="flex gap-4 mt-4">
            <Link
              href="/problems"
              className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground shadow hover:bg-primary/90 h-10 px-6"
            >
              Explore Problems
              <ArrowRight className="h-4 w-4" />
            </Link>
            <Link
              href="/problems/new"
              className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-10 px-6"
            >
              Post a Problem
            </Link>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="container px-4 md:px-6">
        <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
          {stats.map((stat) => {
            const Icon = stat.icon;
            return (
              <div
                key={stat.label}
                className="flex flex-col items-center gap-2 rounded-lg border bg-card p-4 text-card-foreground shadow-sm"
              >
                <Icon className="h-6 w-6 text-primary" />
                <span className="text-2xl font-bold">{stat.value}</span>
                <span className="text-sm text-muted-foreground">{stat.label}</span>
              </div>
            );
          })}
        </div>
      </section>

      {/* How It Works */}
      <section className="container px-4 md:px-6">
        <h2 className="text-2xl font-bold tracking-tight mb-6 text-center">How It Works</h2>
        <div className="grid gap-6 md:grid-cols-3">
          <div className="flex flex-col items-center gap-3 text-center p-6 rounded-lg border bg-card">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10 text-primary font-bold text-xl">
              1
            </div>
            <h3 className="font-semibold">Post a Problem</h3>
            <p className="text-sm text-muted-foreground">
              Identify a civic issue in your community. Add location, description, and deadline.
            </p>
          </div>
          <div className="flex flex-col items-center gap-3 text-center p-6 rounded-lg border bg-card">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10 text-primary font-bold text-xl">
              2
            </div>
            <h3 className="font-semibold">Propose Directions</h3>
            <p className="text-sm text-muted-foreground">
              Submit solutions with AI-powered feasibility analysis. Stake on directions you believe in.
            </p>
          </div>
          <div className="flex flex-col items-center gap-3 text-center p-6 rounded-lg border bg-card">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10 text-primary font-bold text-xl">
              3
            </div>
            <h3 className="font-semibold">Get Rewarded</h3>
            <p className="text-sm text-muted-foreground">
              When a solution is verified, supporters of the winning direction share the bounty.
            </p>
          </div>
        </div>
      </section>

      {/* Recent Problems */}
      <section className="container px-4 md:px-6">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold tracking-tight">Recent Problems</h2>
          <Link
            href="/problems"
            className="text-sm text-primary hover:underline flex items-center gap-1"
          >
            View all
            <ArrowRight className="h-4 w-4" />
          </Link>
        </div>
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {recentProblems.map((problem) => (
            <ProblemCard key={problem.id} problem={problem} />
          ))}
        </div>
      </section>
    </div>
  );
}
