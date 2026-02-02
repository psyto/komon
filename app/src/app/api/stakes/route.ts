import { NextRequest, NextResponse } from "next/server";
import prisma from "@/lib/db";
import { Outcome } from "@prisma/client";

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const userId = searchParams.get("userId");
    const directionId = searchParams.get("directionId");
    const limit = parseInt(searchParams.get("limit") || "20");
    const offset = parseInt(searchParams.get("offset") || "0");

    const where: Record<string, unknown> = {};
    if (userId) where.userId = userId;
    if (directionId) where.directionId = directionId;

    const [stakes, total] = await Promise.all([
      prisma.stake.findMany({
        where,
        include: {
          user: {
            select: {
              id: true,
              displayName: true,
            },
          },
          direction: {
            select: {
              id: true,
              description: true,
              status: true,
              outcome: true,
              problem: {
                select: {
                  id: true,
                  title: true,
                },
              },
            },
          },
        },
        orderBy: {
          createdAt: "desc",
        },
        take: limit,
        skip: offset,
      }),
      prisma.stake.count({ where }),
    ]);

    return NextResponse.json({
      stakes,
      total,
      limit,
      offset,
    });
  } catch (error) {
    console.error("Error fetching stakes:", error);
    return NextResponse.json(
      { error: "Failed to fetch stakes" },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { userId, directionId, amount, outcome } = body;

    // Validate required fields
    if (!userId || !directionId || !amount || !outcome) {
      return NextResponse.json(
        { error: "Missing required fields" },
        { status: 400 }
      );
    }

    // Validate amount
    const amountBigInt = BigInt(amount);
    if (amountBigInt <= 0) {
      return NextResponse.json(
        { error: "Amount must be greater than zero" },
        { status: 400 }
      );
    }

    // Validate outcome
    if (!Object.values(Outcome).includes(outcome as Outcome)) {
      return NextResponse.json(
        { error: "Invalid outcome. Must be YES or NO" },
        { status: 400 }
      );
    }

    // Verify direction exists and is active
    const direction = await prisma.direction.findUnique({
      where: { id: directionId },
      select: {
        status: true,
        yesSupply: true,
        noSupply: true,
        usdcLiquidity: true,
        problem: {
          select: {
            status: true,
          },
        },
      },
    });

    if (!direction) {
      return NextResponse.json(
        { error: "Direction not found" },
        { status: 404 }
      );
    }

    if (direction.status === "SETTLED" || direction.status === "REJECTED") {
      return NextResponse.json(
        { error: "Direction is no longer accepting stakes" },
        { status: 400 }
      );
    }

    if (direction.problem.status === "RESOLVED" || direction.problem.status === "CLOSED") {
      return NextResponse.json(
        { error: "Problem is no longer active" },
        { status: 400 }
      );
    }

    // Calculate tokens to mint (1:1 for MVP)
    const tokenAmount = amountBigInt;

    // Create or update stake
    const stake = await prisma.stake.upsert({
      where: {
        userId_directionId_outcome: {
          userId,
          directionId,
          outcome: outcome as Outcome,
        },
      },
      create: {
        userId,
        directionId,
        amount: amountBigInt,
        outcome: outcome as Outcome,
        tokenAmount,
      },
      update: {
        amount: { increment: amountBigInt },
        tokenAmount: { increment: tokenAmount },
      },
      include: {
        direction: {
          select: {
            id: true,
            description: true,
          },
        },
      },
    });

    // Update direction supply and liquidity
    const updateData =
      outcome === Outcome.YES
        ? {
            yesSupply: { increment: tokenAmount },
            usdcLiquidity: { increment: amountBigInt },
          }
        : {
            noSupply: { increment: tokenAmount },
            usdcLiquidity: { increment: amountBigInt },
          };

    await prisma.direction.update({
      where: { id: directionId },
      data: updateData,
    });

    // Update user's reputation
    await prisma.reputationCache.upsert({
      where: { userId },
      create: {
        userId,
        totalStaked: amountBigInt,
      },
      update: {
        totalStaked: { increment: amountBigInt },
      },
    });

    return NextResponse.json(stake, { status: 201 });
  } catch (error) {
    console.error("Error creating stake:", error);
    return NextResponse.json(
      { error: "Failed to create stake" },
      { status: 500 }
    );
  }
}
