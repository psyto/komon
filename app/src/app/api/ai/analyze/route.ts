import { NextRequest, NextResponse } from "next/server";
import Anthropic from "@anthropic-ai/sdk";
import prisma from "@/lib/db";

const anthropic = new Anthropic({
  apiKey: process.env.ANTHROPIC_API_KEY || "",
});

interface ProblemAnalysisResult {
  feasibility: string;
  estimatedImpact: string;
  suggestedApproaches: string[];
  risks: string[];
  similarProblems: string[];
}

interface DirectionAnalysisResult {
  feasibilityScore: number;
  reasoning: string;
  strengths: string[];
  weaknesses: string[];
  estimatedCost: string;
  estimatedTime: string;
  riskLevel: "LOW" | "MEDIUM" | "HIGH" | "CRITICAL";
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { type, problemId, directionId, title, description, category, directionDescription } = body;

    if (!process.env.ANTHROPIC_API_KEY) {
      return NextResponse.json(
        { error: "AI service not configured" },
        { status: 503 }
      );
    }

    if (type === "problem") {
      if (!title || !description || !category) {
        return NextResponse.json(
          { error: "Missing required fields for problem analysis" },
          { status: 400 }
        );
      }

      const analysis = await analyzeProblem(title, description, category);

      // Save to database if problemId provided
      if (problemId) {
        await prisma.problemAnalysis.upsert({
          where: { problemId },
          create: {
            problemId,
            ...analysis,
          },
          update: analysis,
        });
      }

      return NextResponse.json(analysis);
    } else if (type === "direction") {
      if (!directionDescription) {
        return NextResponse.json(
          { error: "Missing direction description" },
          { status: 400 }
        );
      }

      // Get problem context
      let problemContext = { title: title || "", description: description || "", category: category || "" };
      if (problemId && !title) {
        const problem = await prisma.problem.findUnique({
          where: { id: problemId },
          select: { title: true, description: true, category: true },
        });
        if (problem) {
          problemContext = problem;
        }
      }

      const analysis = await analyzeDirection(
        problemContext.title,
        problemContext.description,
        problemContext.category,
        directionDescription
      );

      // Save to database if directionId provided
      if (directionId) {
        await prisma.directionAnalysis.upsert({
          where: { directionId },
          create: {
            directionId,
            ...analysis,
          },
          update: analysis,
        });
      }

      return NextResponse.json(analysis);
    } else {
      return NextResponse.json(
        { error: "Invalid analysis type. Must be 'problem' or 'direction'" },
        { status: 400 }
      );
    }
  } catch (error) {
    console.error("Error in AI analysis:", error);
    return NextResponse.json(
      { error: "Failed to analyze" },
      { status: 500 }
    );
  }
}

async function analyzeProblem(
  title: string,
  description: string,
  category: string
): Promise<ProblemAnalysisResult> {
  const prompt = `You are an expert civic problem analyst. Analyze the following civic problem and provide a structured assessment.

Problem Title: ${title}
Description: ${description}
Category: ${category}

Provide your analysis in the following JSON format:
{
  "feasibility": "A brief assessment of how feasible it is to solve this problem (2-3 sentences)",
  "estimatedImpact": "The potential impact if this problem is solved (2-3 sentences)",
  "suggestedApproaches": ["approach 1", "approach 2", "approach 3"],
  "risks": ["risk 1", "risk 2"],
  "similarProblems": ["similar problem that has been solved before", "another example"]
}

Be practical and realistic in your assessment. Focus on actionable insights.`;

  const response = await anthropic.messages.create({
    model: "claude-sonnet-4-20250514",
    max_tokens: 1024,
    messages: [{ role: "user", content: prompt }],
  });

  const content = response.content[0];
  if (content.type !== "text") {
    throw new Error("Unexpected response type");
  }

  // Extract JSON from response
  const jsonMatch = content.text.match(/\{[\s\S]*\}/);
  if (!jsonMatch) {
    throw new Error("Failed to parse AI response");
  }

  return JSON.parse(jsonMatch[0]) as ProblemAnalysisResult;
}

async function analyzeDirection(
  problemTitle: string,
  problemDescription: string,
  problemCategory: string,
  directionDescription: string
): Promise<DirectionAnalysisResult> {
  const prompt = `You are an expert civic solution analyst. Analyze the following proposed solution (direction) for a civic problem.

Problem Title: ${problemTitle}
Problem Description: ${problemDescription}
Category: ${problemCategory}

Proposed Solution: ${directionDescription}

Provide your analysis in the following JSON format:
{
  "feasibilityScore": <number 0-100>,
  "reasoning": "A brief explanation of your feasibility score (2-3 sentences)",
  "strengths": ["strength 1", "strength 2"],
  "weaknesses": ["weakness 1", "weakness 2"],
  "estimatedCost": "rough cost estimate (e.g., '$500-$1000', 'Minimal', 'Significant')",
  "estimatedTime": "rough time estimate (e.g., '1-2 weeks', '1-3 months')",
  "riskLevel": "LOW" | "MEDIUM" | "HIGH" | "CRITICAL"
}

Be practical and realistic. A score of 70+ means likely to work, 50-69 is uncertain, below 50 is unlikely.`;

  const response = await anthropic.messages.create({
    model: "claude-sonnet-4-20250514",
    max_tokens: 1024,
    messages: [{ role: "user", content: prompt }],
  });

  const content = response.content[0];
  if (content.type !== "text") {
    throw new Error("Unexpected response type");
  }

  // Extract JSON from response
  const jsonMatch = content.text.match(/\{[\s\S]*\}/);
  if (!jsonMatch) {
    throw new Error("Failed to parse AI response");
  }

  return JSON.parse(jsonMatch[0]) as DirectionAnalysisResult;
}
