"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { ProblemCard } from "@/components/problems/problem-card";
import { Filter, Plus, Search } from "lucide-react";

const categories = [
  "All",
  "Infrastructure",
  "Environment",
  "Safety",
  "Health",
  "Education",
  "Transportation",
  "Housing",
  "Community",
  "Economic",
  "Other",
];

const statuses = ["All", "Open", "In Progress", "Resolved", "Closed"];

interface Problem {
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
}

// Mock data for initial display
const mockProblems: Problem[] = [
  {
    id: "1",
    title: "Pothole on Main Street causing traffic hazards",
    description: "Large pothole at the intersection of Main St and Oak Ave has been causing accidents and damage to vehicles.",
    category: "Infrastructure",
    locationName: "Downtown",
    bountyAmount: BigInt(500_000_000),
    directionCount: 3,
    deadline: new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString(),
    createdAt: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
    status: "open",
  },
  {
    id: "2",
    title: "Illegal dumping in community park",
    description: "Construction waste and household garbage being dumped near the playground area.",
    category: "Environment",
    locationName: "Riverside Park",
    bountyAmount: BigInt(250_000_000),
    directionCount: 5,
    deadline: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
    createdAt: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
    status: "open",
  },
  {
    id: "3",
    title: "Broken streetlights creating safety concerns",
    description: "Multiple streetlights on Elm Street have been out for weeks.",
    category: "Safety",
    locationName: "Elm Street",
    bountyAmount: BigInt(150_000_000),
    directionCount: 2,
    deadline: new Date(Date.now() + 21 * 24 * 60 * 60 * 1000).toISOString(),
    createdAt: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(),
    status: "open",
  },
  {
    id: "4",
    title: "Bus stop needs shelter and seating",
    description: "The bus stop on 5th Ave has no protection from weather. Elderly residents struggle.",
    category: "Transportation",
    locationName: "5th Avenue",
    bountyAmount: BigInt(300_000_000),
    directionCount: 4,
    deadline: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
    createdAt: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000).toISOString(),
    status: "in_progress",
  },
];

export default function ProblemsPage() {
  const [problems, setProblems] = useState<Problem[]>(mockProblems);
  const [selectedCategory, setSelectedCategory] = useState("All");
  const [selectedStatus, setSelectedStatus] = useState("All");
  const [searchQuery, setSearchQuery] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    // In production, fetch from API
    const filteredProblems = mockProblems.filter((problem) => {
      const matchesCategory =
        selectedCategory === "All" || problem.category === selectedCategory;
      const matchesStatus =
        selectedStatus === "All" ||
        problem.status.toLowerCase().replace("_", " ") === selectedStatus.toLowerCase();
      const matchesSearch =
        searchQuery === "" ||
        problem.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        problem.description.toLowerCase().includes(searchQuery.toLowerCase());
      return matchesCategory && matchesStatus && matchesSearch;
    });
    setProblems(filteredProblems);
  }, [selectedCategory, selectedStatus, searchQuery]);

  return (
    <div className="container py-8 px-4 md:px-6">
      <div className="flex flex-col gap-6">
        {/* Header */}
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
          <div>
            <h1 className="text-3xl font-bold tracking-tight">Problems</h1>
            <p className="text-muted-foreground">
              Browse and contribute to solving civic problems in your community
            </p>
          </div>
          <Link
            href="/problems/new"
            className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground shadow hover:bg-primary/90 h-10 px-4"
          >
            <Plus className="h-4 w-4" />
            Post Problem
          </Link>
        </div>

        {/* Filters */}
        <div className="flex flex-col sm:flex-row gap-4">
          {/* Search */}
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <input
              type="text"
              placeholder="Search problems..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="flex h-10 w-full rounded-md border border-input bg-background px-10 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            />
          </div>

          {/* Category Filter */}
          <div className="flex items-center gap-2">
            <Filter className="h-4 w-4 text-muted-foreground" />
            <select
              value={selectedCategory}
              onChange={(e) => setSelectedCategory(e.target.value)}
              className="flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            >
              {categories.map((cat) => (
                <option key={cat} value={cat}>
                  {cat}
                </option>
              ))}
            </select>
          </div>

          {/* Status Filter */}
          <select
            value={selectedStatus}
            onChange={(e) => setSelectedStatus(e.target.value)}
            className="flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
          >
            {statuses.map((status) => (
              <option key={status} value={status}>
                {status}
              </option>
            ))}
          </select>
        </div>

        {/* Results */}
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary" />
          </div>
        ) : problems.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-muted-foreground">No problems found matching your criteria.</p>
          </div>
        ) : (
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {problems.map((problem) => (
              <ProblemCard key={problem.id} problem={problem} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
