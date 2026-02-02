import { NextRequest, NextResponse } from "next/server";
import prisma from "@/lib/db";
import { ProblemCategory, ProblemStatus } from "@prisma/client";

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const category = searchParams.get("category") as ProblemCategory | null;
    const status = searchParams.get("status") as ProblemStatus | null;
    const limit = parseInt(searchParams.get("limit") || "20");
    const offset = parseInt(searchParams.get("offset") || "0");
    const sortBy = searchParams.get("sortBy") || "createdAt";
    const sortOrder = searchParams.get("sortOrder") || "desc";

    const where: Record<string, unknown> = {};
    if (category) where.category = category;
    if (status) where.status = status;

    const [problems, total] = await Promise.all([
      prisma.problem.findMany({
        where,
        include: {
          creator: {
            select: {
              id: true,
              displayName: true,
            },
          },
          _count: {
            select: {
              directions: true,
            },
          },
        },
        orderBy: {
          [sortBy]: sortOrder,
        },
        take: limit,
        skip: offset,
      }),
      prisma.problem.count({ where }),
    ]);

    return NextResponse.json({
      problems: problems.map((p) => ({
        ...p,
        directionCount: p._count.directions,
        _count: undefined,
      })),
      total,
      limit,
      offset,
    });
  } catch (error) {
    console.error("Error fetching problems:", error);
    return NextResponse.json(
      { error: "Failed to fetch problems" },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const {
      title,
      description,
      locationLat,
      locationLng,
      locationName,
      category,
      deadline,
      creatorId,
    } = body;

    // Validate required fields
    if (!title || !description || !locationLat || !locationLng || !category || !deadline || !creatorId) {
      return NextResponse.json(
        { error: "Missing required fields" },
        { status: 400 }
      );
    }

    // Validate title length
    if (title.length > 100) {
      return NextResponse.json(
        { error: "Title must be 100 characters or less" },
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

    // Validate deadline is in the future
    if (new Date(deadline) <= new Date()) {
      return NextResponse.json(
        { error: "Deadline must be in the future" },
        { status: 400 }
      );
    }

    // Create the problem
    const problem = await prisma.problem.create({
      data: {
        title,
        description,
        locationLat,
        locationLng,
        locationName,
        category: category as ProblemCategory,
        deadline: new Date(deadline),
        creatorId,
      },
      include: {
        creator: {
          select: {
            id: true,
            displayName: true,
          },
        },
      },
    });

    // Update user's reputation
    await prisma.reputationCache.upsert({
      where: { userId: creatorId },
      create: {
        userId: creatorId,
        problemsPosted: 1,
      },
      update: {
        problemsPosted: { increment: 1 },
      },
    });

    return NextResponse.json(problem, { status: 201 });
  } catch (error) {
    console.error("Error creating problem:", error);
    return NextResponse.json(
      { error: "Failed to create problem" },
      { status: 500 }
    );
  }
}
