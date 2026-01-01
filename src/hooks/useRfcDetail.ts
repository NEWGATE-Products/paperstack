import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Rfc, SummaryLevel } from "../types/rfc";

interface UseRfcDetailReturn {
  rfc: Rfc | null;
  content: string | null;
  loading: boolean;
  loadingContent: boolean;
  loadingSummary: boolean;
  loadingGuide: boolean;
  loadingTranslation: boolean;
  error: string | null;
  fetchRfc: (rfcId: string) => Promise<void>;
  fetchContent: (rfcNumber: number) => Promise<void>;
  generateSummary: (rfcId: string, level: SummaryLevel) => Promise<string>;
  generateImplementationGuide: (rfcId: string) => Promise<string>;
  translateSection: (text: string) => Promise<string>;
}

export function useRfcDetail(): UseRfcDetailReturn {
  const [rfc, setRfc] = useState<Rfc | null>(null);
  const [content, setContent] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [loadingContent, setLoadingContent] = useState(false);
  const [loadingSummary, setLoadingSummary] = useState(false);
  const [loadingGuide, setLoadingGuide] = useState(false);
  const [loadingTranslation, setLoadingTranslation] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchRfc = useCallback(async (rfcId: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<Rfc | null>("get_rfc_by_id", { rfcId });
      setRfc(result);

      // Add to history
      if (result) {
        await invoke("add_rfc_history", { rfcId });
      }
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to fetch RFC:", e);
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchContent = useCallback(async (rfcNumber: number) => {
    setLoadingContent(true);
    setError(null);
    try {
      const result = await invoke<string>("get_rfc_content", { rfcNumber });
      setContent(result);
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to fetch RFC content:", e);
    } finally {
      setLoadingContent(false);
    }
  }, []);

  const generateSummary = useCallback(
    async (rfcId: string, level: SummaryLevel): Promise<string> => {
      setLoadingSummary(true);
      setError(null);
      try {
        const summary = await invoke<string>("generate_rfc_summary", {
          rfcId,
          level,
        });

        // Update local state
        if (rfc && rfc.id === rfcId) {
          setRfc({
            ...rfc,
            summaryEasy: level === "easy" ? summary : rfc.summaryEasy,
            summaryNormal: level === "normal" ? summary : rfc.summaryNormal,
            summaryTechnical:
              level === "technical" ? summary : rfc.summaryTechnical,
          });
        }

        return summary;
      } catch (e) {
        const errorMessage = e instanceof Error ? e.message : String(e);
        setError(errorMessage);
        throw e;
      } finally {
        setLoadingSummary(false);
      }
    },
    [rfc]
  );

  const generateImplementationGuide = useCallback(
    async (rfcId: string): Promise<string> => {
      setLoadingGuide(true);
      setError(null);
      try {
        const guide = await invoke<string>("generate_rfc_implementation_guide", {
          rfcId,
        });

        // Update local state
        if (rfc && rfc.id === rfcId) {
          setRfc({
            ...rfc,
            implementationGuide: guide,
          });
        }

        return guide;
      } catch (e) {
        const errorMessage = e instanceof Error ? e.message : String(e);
        setError(errorMessage);
        throw e;
      } finally {
        setLoadingGuide(false);
      }
    },
    [rfc]
  );

  const translateSection = useCallback(async (text: string): Promise<string> => {
    setLoadingTranslation(true);
    setError(null);
    try {
      const translation = await invoke<string>("translate_rfc_section", {
        text,
      });
      return translation;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      throw e;
    } finally {
      setLoadingTranslation(false);
    }
  }, []);

  return {
    rfc,
    content,
    loading,
    loadingContent,
    loadingSummary,
    loadingGuide,
    loadingTranslation,
    error,
    fetchRfc,
    fetchContent,
    generateSummary,
    generateImplementationGuide,
    translateSection,
  };
}

