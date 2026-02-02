import { NextRequest, NextResponse } from "next/server";
import prisma from "@/lib/db";

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const problemId = searchParams.get("problemId");
    const proposerId = searchParams.get("proposerId");
    const limit = parseInt(searchParams.get("limit") || "20");
    const offset = parseInt(searchParams.get("offset") || "0");

    const where: Record<string, unknown> = {};
    if (problemId) where.problemId = problemId;
    if (proposerId) where.proposerId = proposerId;

    const [directions, total] = await Promise.all([
      prisma.direction.findMany({
        where,
        include: {
          proposer: {
            select: {
              id: true,
              displayName: true,
            },
          },
          problem: {
            select: {
              id: true,
              title: true,
              status: true,
            },
          },
          aiAnalysis: true,
          _count: {
            select: {
              stakes: true,
            },
          },
        },
        orderBy: {
          createdAt: "desc",
        },
        take: limit,
        skip: offset,
      }),
      prisma.direction.count({ where }),
    ]);

    return NextResponse.json({
      directions: directions.map((d) => ({
        ...d,
        stakeCount: d._count.stakes,
        _count: undefined,
      })),
      total,
      limit,
      offset,
    });
  } catch (error) {
    console.error("Error fetching directions:", error);
    return NextResponse.json(
      { error: "Failed to fetch directions" },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { problemId, proposerId, description } = body;

    // Validate required fields
    if (!problemId || !proposerId || !description) {
      return NextResponse.json(
        { error: "Missing required fields" },
        { status: 400 }
      );
    }

    // Validate description length
    if (description.length > 500) {
      return NextResponse.json(
        { error: "Description must be 500 characters or less" },
        { status: 400 }
      );
    }

    // Verify problem exists and is open
    const problem = await prisma.problem.findUnique({
      where: { id: problemId },
      select: { status: true, title: true, description: true, category: true },
    });

    if (!problem) {
      return NextResponse.json(
        { error: "Problem not found" },
        { status: 404 }
      );
    }

    if (problem.status !== "OPEN" && problem.status !== "IN_PROGRESS") {
      return NextResponse.json(
        { error: "Problem is not accepting new directions" },
        { status: 400 }
      );
    }

    // Create the direction
    const direction = await prisma.direction.create({
      data: {
        problemId,
        proposerId,
        description,
      },
      include: {
        proposer: {
          select: {
            id: true,
            displayName: true,
          },
        },
      },
    });

    // Update user's reputation
    await prisma.reputationCache.upsert({
      where: { userId: proposerId },
      create: {
        userId: proposerId,
        directionsProposed: 1,
      },
      update: {
        directionsProposed: { increment: 1 },
      },
    });

    return NextResponse.json(direction, { status: 201 });
  } catch (error) {
    console.error("Error creating direction:", error);
    return NextResponse.json(
      { error: "Failed to create direction" },
      { status: 500 }
    );
  }
}
