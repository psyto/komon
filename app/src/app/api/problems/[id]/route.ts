import { NextRequest, NextResponse } from "next/server";
import prisma from "@/lib/db";
import { ProblemStatus } from "@prisma/client";

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const { id } = await params;

    const problem = await prisma.problem.findUnique({
      where: { id },
      include: {
        creator: {
          select: {
            id: true,
            displayName: true,
            reputation: {
              select: {
                problemsPosted: true,
                directionsWon: true,
                winRate: true,
                level: true,
              },
            },
          },
        },
        directions: {
          include: {
            proposer: {
              select: {
                id: true,
                displayName: true,
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
        },
        aiAnalysis: true,
      },
    });

    if (!problem) {
      return NextResponse.json(
        { error: "Problem not found" },
        { status: 404 }
      );
    }

    return NextResponse.json(problem);
  } catch (error) {
    console.error("Error fetching problem:", error);
    return NextResponse.json(
      { error: "Failed to fetch problem" },
      { status: 500 }
    );
  }
}

export async function PATCH(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const { id } = await params;
    const body = await request.json();
    const { status, bountyAmount, resolvedAt, winningDirectionId } = body;

    const updateData: Record<string, unknown> = {};

    if (status) {
      // Validate status transition
      const problem = await prisma.problem.findUnique({
        where: { id },
        select: { status: true },
      });

      if (!problem) {
        return NextResponse.json(
          { error: "Problem not found" },
          { status: 404 }
        );
      }

      const validTransitions: Record<ProblemStatus, ProblemStatus[]> = {
        OPEN: [ProblemStatus.IN_PROGRESS, ProblemStatus.CLOSED],
        IN_PROGRESS: [ProblemStatus.OPEN, ProblemStatus.RESOLVED, ProblemStatus.CLOSED],
        RESOLVED: [],
        CLOSED: [],
      };

      if (!validTransitions[problem.status].includes(status)) {
        return NextResponse.json(
          { error: `Invalid status transition from ${problem.status} to ${status}` },
          { status: 400 }
        );
      }

      updateData.status = status;

      if (status === ProblemStatus.RESOLVED) {
        updateData.resolvedAt = resolvedAt || new Date();

        // Mark winning direction
        if (winningDirectionId) {
          await prisma.direction.update({
            where: { id: winningDirectionId },
            data: { isWinner: true, status: "SETTLED" },
          });
        }
      }
    }

    if (bountyAmount !== undefined) {
      updateData.bountyAmount = BigInt(bountyAmount);
    }

    const updatedProblem = await prisma.problem.update({
      where: { id },
      data: updateData,
    });

    return NextResponse.json(updatedProblem);
  } catch (error) {
    console.error("Error updating problem:", error);
    return NextResponse.json(
      { error: "Failed to update problem" },
      { status: 500 }
    );
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  try {
    const { id } = await params;

    // Check if problem exists and can be deleted
    const problem = await prisma.problem.findUnique({
      where: { id },
      select: { status: true, bountyAmount: true },
    });

    if (!problem) {
      return NextResponse.json(
        { error: "Problem not found" },
        { status: 404 }
      );
    }

    // Only allow deletion of open problems with no bounty
    if (problem.status !== ProblemStatus.OPEN) {
      return NextResponse.json(
        { error: "Only open problems can be deleted" },
        { status: 400 }
      );
    }

    if (problem.bountyAmount > 0) {
      return NextResponse.json(
        { error: "Cannot delete problems with bounty" },
        { status: 400 }
      );
    }

    await prisma.problem.delete({ where: { id } });

    return NextResponse.json({ success: true });
  } catch (error) {
    console.error("Error deleting problem:", error);
    return NextResponse.json(
      { error: "Failed to delete problem" },
      { status: 500 }
    );
  }
}
