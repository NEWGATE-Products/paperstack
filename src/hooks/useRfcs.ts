import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Rfc, RfcFilter, RfcListResponse, SummaryLevel } from "../types/rfc";

interface UseRfcsReturn {
  rfcs: Rfc[];
  total: number;
  page: number;
  limit: number;
  loading: boolean;
  error: string | null;
  filter: RfcFilter;
  setFilter: (filter: RfcFilter) => void;
  setPage: (page: number) => void;
  fetchRfcs: () => Promise<void>;
  refreshFromServer: () => Promise<number>;
  translateAbstract: (rfcId: string) => Promise<string>;
  translateTitle: (rfcId: string) => Promise<string>;
  generateSummary: (rfcId: string, level: SummaryLevel) => Promise<string>;
  updateRfcInList: (rfcId: string, updates: Partial<Rfc>) => void;
}

export function useRfcs(initialLimit = 20): UseRfcsReturn {
  const [rfcs, setRfcs] = useState<Rfc[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [limit] = useState(initialLimit);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<RfcFilter>({});

  const fetchRfcs = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await invoke<RfcListResponse>("get_rfcs", {
        filter: Object.keys(filter).length > 0 ? filter : null,
        page,
        limit,
      });
      setRfcs(response.rfcs);
      setTotal(response.total);
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to fetch RFCs:", e);
    } finally {
      setLoading(false);
    }
  }, [filter, page, limit]);

  const refreshFromServer = useCallback(async (): Promise<number> => {
    setLoading(true);
    setError(null);
    try {
      const count = await invoke<number>("fetch_rfcs");
      // Refresh the list after fetching
      await fetchRfcs();
      return count;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to refresh RFCs from server:", e);
      throw e;
    } finally {
      setLoading(false);
    }
  }, [fetchRfcs]);

  const translateAbstract = useCallback(async (rfcId: string): Promise<string> => {
    try {
      const translation = await invoke<string>("translate_rfc_abstract", { rfcId });
      
      // Update local state with the translation
      setRfcs(prevRfcs => 
        prevRfcs.map(rfc => 
          rfc.id === rfcId ? { ...rfc, abstractJa: translation } : rfc
        )
      );
      
      return translation;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to translate abstract:", e);
      throw e;
    }
  }, []);

  const translateTitle = useCallback(async (rfcId: string): Promise<string> => {
    try {
      const translation = await invoke<string>("translate_rfc_title", { rfcId });
      
      // Update local state with the translation
      setRfcs(prevRfcs => 
        prevRfcs.map(rfc => 
          rfc.id === rfcId ? { ...rfc, titleJa: translation } : rfc
        )
      );
      
      return translation;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to translate title:", e);
      throw e;
    }
  }, []);

  const generateSummary = useCallback(async (rfcId: string, level: SummaryLevel): Promise<string> => {
    try {
      const summary = await invoke<string>("generate_rfc_summary", { rfcId, level });
      
      // Update local state with the summary
      setRfcs(prevRfcs => 
        prevRfcs.map(rfc => {
          if (rfc.id !== rfcId) return rfc;
          
          return {
            ...rfc,
            summaryEasy: level === "easy" ? summary : rfc.summaryEasy,
            summaryNormal: level === "normal" ? summary : rfc.summaryNormal,
            summaryTechnical: level === "technical" ? summary : rfc.summaryTechnical,
          };
        })
      );
      
      return summary;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to generate summary:", e);
      throw e;
    }
  }, []);

  const updateRfcInList = useCallback((rfcId: string, updates: Partial<Rfc>) => {
    setRfcs(prevRfcs =>
      prevRfcs.map(rfc =>
        rfc.id === rfcId ? { ...rfc, ...updates } : rfc
      )
    );
  }, []);

  // Fetch RFCs when filter or page changes
  useEffect(() => {
    fetchRfcs();
  }, [fetchRfcs]);

  return {
    rfcs,
    total,
    page,
    limit,
    loading,
    error,
    filter,
    setFilter,
    setPage,
    fetchRfcs,
    refreshFromServer,
    translateAbstract,
    translateTitle,
    generateSummary,
    updateRfcInList,
  };
}

