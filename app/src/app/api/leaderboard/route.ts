import { NextRequest, NextResponse } from "next/server";
import prisma from "@/lib/db";

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const sortBy = searchParams.get("sortBy") || "totalEarned";
    const limit = parseInt(searchParams.get("limit") || "50");
    const offset = parseInt(searchParams.get("offset") || "0");

    const validSortFields = [
      "totalEarned",
      "winRate",
      "directionsWon",
      "currentStreak",
      "level",
      "experience",
    ];

    if (!validSortFields.includes(sortBy)) {
      return NextResponse.json(
        { error: `Invalid sortBy field. Must be one of: ${validSortFields.join(", ")}` },
        { status: 400 }
      );
    }

    const [leaders, total] = await Promise.all([
      prisma.reputationCache.findMany({
        include: {
          user: {
            select: {
              id: true,
              displayName: true,
              walletAddress: true,
            },
          },
        },
        orderBy: {
          [sortBy]: "desc",
        },
        take: limit,
        skip: offset,
      }),
      prisma.reputationCache.count(),
    ]);

    // Add rank to each entry
    const leaderboard = leaders.map((entry, index) => ({
      rank: offset + index + 1,
      user: {
        id: entry.user.id,
        displayName: entry.user.displayName || `User ${entry.user.id.slice(0, 8)}`,
        walletAddress: entry.user.walletAddress,
      },
      stats: {
        problemsPosted: entry.problemsPosted,
        directionsProposed: entry.directionsProposed,
        directionsWon: entry.directionsWon,
        directionsLost: entry.directionsLost,
        winRate: entry.winRate,
        totalEarned: entry.totalEarned.toString(),
        totalStaked: entry.totalStaked.toString(),
        currentStreak: entry.currentStreak,
        bestStreak: entry.bestStreak,
        level: entry.level,
        experience: entry.experience.toString(),
      },
    }));

    return NextResponse.json({
      leaderboard,
      total,
      limit,
      offset,
      sortBy,
    });
  } catch (error) {
    console.error("Error fetching leaderboard:", error);
    return NextResponse.json(
      { error: "Failed to fetch leaderboard" },
      { status: 500 }
    );
  }
}
