import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Paper, LoadingState } from "../types";

// Extract error message from various error types (including Tauri error objects)
function extractErrorMessage(e: unknown): string {
  if (e instanceof Error) {
    return e.message;
  }
  if (typeof e === "object" && e !== null && "message" in e) {
    return String((e as { message: unknown }).message);
  }
  return String(e);
}

export function usePapers() {
  const [papers, setPapers] = useState<Paper[]>([]);
  const [loading, setLoading] = useState<LoadingState>("idle");
  const [error, setError] = useState<string | null>(null);

  const getPapers = useCallback(async (category?: string, limit?: number) => {
    setLoading("loading");
    setError(null);
    try {
      const result = await invoke<Paper[]>("get_papers", {
        category: category === "all" ? null : category,
        limit,
      });
      setPapers(result);
      setLoading("success");
    } catch (e: unknown) {
      setError(extractErrorMessage(e));
      setLoading("error");
    }
  }, []);

  const fetchPapers = useCallback(async (tasks: string[]) => {
    setLoading("loading");
    setError(null);
    try {
      const result = await invoke<Paper[]>("fetch_papers", { tasks });
      setPapers(result);
      setLoading("success");
    } catch (e: unknown) {
      setError(extractErrorMessage(e));
      setLoading("error");
    }
  }, []);

  const generateSummary = useCallback(async (paperId: string) => {
    try {
      const summary = await invoke<string>("generate_summary", { paperId });
      setPapers((prev) =>
        prev.map((p) => (p.id === paperId ? { ...p, summaryJa: summary } : p))
      );
      return summary;
    } catch (e: unknown) {
      // Handle Tauri error object which has { message: string } structure
      const errorMessage = extractErrorMessage(e);
      throw new Error(errorMessage);
    }
  }, []);

  return {
    papers,
    loading,
    error,
    getPapers,
    fetchPapers,
    generateSummary,
  };
}

