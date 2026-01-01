import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { RfcHistory } from "../types/rfc";

interface UseRfcHistoryReturn {
  history: RfcHistory[];
  loading: boolean;
  error: string | null;
  refreshHistory: (limit?: number) => Promise<void>;
}

export function useRfcHistory(limit = 50): UseRfcHistoryReturn {
  const [history, setHistory] = useState<RfcHistory[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refreshHistory = useCallback(async (historyLimit?: number) => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<RfcHistory[]>("get_rfc_history", {
        limit: historyLimit ?? limit,
      });
      setHistory(result);
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to fetch history:", e);
    } finally {
      setLoading(false);
    }
  }, [limit]);

  // Load history on mount
  useEffect(() => {
    refreshHistory();
  }, [refreshHistory]);

  return {
    history,
    loading,
    error,
    refreshHistory,
  };
}

