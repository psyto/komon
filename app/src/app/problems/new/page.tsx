"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { ArrowLeft, MapPin, Sparkles, Loader2 } from "lucide-react";

const categories = [
  { value: "INFRASTRUCTURE", label: "Infrastructure", description: "Roads, bridges, utilities" },
  { value: "ENVIRONMENT", label: "Environment", description: "Pollution, parks, wildlife" },
  { value: "SAFETY", label: "Safety", description: "Crime, traffic, emergency" },
  { value: "HEALTH", label: "Health", description: "Healthcare access, sanitation" },
  { value: "EDUCATION", label: "Education", description: "Schools, libraries, programs" },
  { value: "TRANSPORTATION", label: "Transportation", description: "Public transit, bike lanes" },
  { value: "HOUSING", label: "Housing", description: "Affordability, homelessness" },
  { value: "COMMUNITY", label: "Community", description: "Social services, recreation" },
  { value: "ECONOMIC", label: "Economic", description: "Jobs, business development" },
  { value: "OTHER", label: "Other", description: "Doesn't fit other categories" },
];

export default function NewProblemPage() {
  const router = useRouter();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [aiAnalysis, setAiAnalysis] = useState<{
    feasibility: string;
    estimatedImpact: string;
    suggestedApproaches: string[];
  } | null>(null);

  const [formData, setFormData] = useState({
    title: "",
    description: "",
    category: "",
    locationName: "",
    locationLat: 37.7749, // Default to SF
    locationLng: -122.4194,
    deadline: "",
  });

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>
  ) => {
    setFormData((prev) => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  const handleAnalyze = async () => {
    if (!formData.title || !formData.description || !formData.category) {
      return;
    }

    setIsAnalyzing(true);
    try {
      const response = await fetch("/api/ai/analyze", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          type: "problem",
          title: formData.title,
          description: formData.description,
          category: formData.category,
        }),
      });

      if (response.ok) {
        const data = await response.json();
        setAiAnalysis(data);
      }
    } catch (error) {
      console.error("Failed to analyze:", error);
    }
    setIsAnalyzing(false);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      // In production, get real user ID from auth
      const response = await fetch("/api/problems", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          ...formData,
          creatorId: "demo-user", // Replace with real auth
        }),
      });

      if (response.ok) {
        const problem = await response.json();
        router.push(`/problems/${problem.id}`);
      }
    } catch (error) {
      console.error("Failed to create problem:", error);
    }
    setIsSubmitting(false);
  };

  // Set minimum deadline to tomorrow
  const tomorrow = new Date();
  tomorrow.setDate(tomorrow.getDate() + 1);
  const minDeadline = tomorrow.toISOString().split("T")[0];

  return (
    <div className="container py-8 px-4 md:px-6">
      <div className="max-w-2xl mx-auto">
        {/* Back link */}
        <Link
          href="/problems"
          className="inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground mb-6"
        >
          <ArrowLeft className="h-4 w-4" />
          Back to Problems
        </Link>

        <h1 className="text-3xl font-bold tracking-tight mb-2">Post a Problem</h1>
        <p className="text-muted-foreground mb-8">
          Identify a civic issue in your community that needs solving.
        </p>

        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Title */}
          <div>
            <label htmlFor="title" className="block text-sm font-medium mb-2">
              Problem Title <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="title"
              name="title"
              value={formData.title}
              onChange={handleChange}
              required
              maxLength={100}
              placeholder="Brief description of the problem"
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            />
            <p className="text-xs text-muted-foreground mt-1">
              {formData.title.length}/100 characters
            </p>
          </div>

          {/* Description */}
          <div>
            <label htmlFor="description" className="block text-sm font-medium mb-2">
              Description <span className="text-red-500">*</span>
            </label>
            <textarea
              id="description"
              name="description"
              value={formData.description}
              onChange={handleChange}
              required
              maxLength={500}
              rows={4}
              placeholder="Provide details about the problem, its impact, and any relevant context"
              className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            />
            <p className="text-xs text-muted-foreground mt-1">
              {formData.description.length}/500 characters
            </p>
          </div>

          {/* Category */}
          <div>
            <label htmlFor="category" className="block text-sm font-medium mb-2">
              Category <span className="text-red-500">*</span>
            </label>
            <select
              id="category"
              name="category"
              value={formData.category}
              onChange={handleChange}
              required
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            >
              <option value="">Select a category</option>
              {categories.map((cat) => (
                <option key={cat.value} value={cat.value}>
                  {cat.label} - {cat.description}
                </option>
              ))}
            </select>
          </div>

          {/* Location */}
          <div>
            <label htmlFor="locationName" className="block text-sm font-medium mb-2">
              Location <span className="text-red-500">*</span>
            </label>
            <div className="relative">
              <MapPin className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <input
                type="text"
                id="locationName"
                name="locationName"
                value={formData.locationName}
                onChange={handleChange}
                required
                placeholder="e.g., Downtown, Main Street, Central Park"
                className="flex h-10 w-full rounded-md border border-input bg-background pl-10 pr-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              />
            </div>
          </div>

          {/* Deadline */}
          <div>
            <label htmlFor="deadline" className="block text-sm font-medium mb-2">
              Deadline <span className="text-red-500">*</span>
            </label>
            <input
              type="date"
              id="deadline"
              name="deadline"
              value={formData.deadline}
              onChange={handleChange}
              required
              min={minDeadline}
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            />
            <p className="text-xs text-muted-foreground mt-1">
              When should this problem be resolved by?
            </p>
          </div>

          {/* AI Analysis */}
          <div className="border rounded-lg p-4">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <Sparkles className="h-5 w-5 text-primary" />
                <span className="font-medium">AI Analysis</span>
              </div>
              <button
                type="button"
                onClick={handleAnalyze}
                disabled={!formData.title || !formData.description || !formData.category || isAnalyzing}
                className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-8 px-3 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isAnalyzing ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin" />
                    Analyzing...
                  </>
                ) : (
                  "Analyze"
                )}
              </button>
            </div>

            {aiAnalysis ? (
              <div className="space-y-3 text-sm">
                <div>
                  <span className="font-medium">Feasibility: </span>
                  <span className="text-muted-foreground">{aiAnalysis.feasibility}</span>
                </div>
                <div>
                  <span className="font-medium">Impact: </span>
                  <span className="text-muted-foreground">{aiAnalysis.estimatedImpact}</span>
                </div>
                <div>
                  <span className="font-medium">Suggested Approaches:</span>
                  <ul className="list-disc list-inside text-muted-foreground mt-1">
                    {aiAnalysis.suggestedApproaches.map((approach, i) => (
                      <li key={i}>{approach}</li>
                    ))}
                  </ul>
                </div>
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">
                Fill in the title, description, and category, then click Analyze to get AI-powered
                insights about your problem.
              </p>
            )}
          </div>

          {/* Submit */}
          <div className="flex gap-4">
            <Link
              href="/problems"
              className="flex-1 inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-10 px-4"
            >
              Cancel
            </Link>
            <button
              type="submit"
              disabled={isSubmitting}
              className="flex-1 inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground shadow hover:bg-primary/90 h-10 px-4 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="h-4 w-4 animate-spin" />
                  Posting...
                </>
              ) : (
                "Post Problem"
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
