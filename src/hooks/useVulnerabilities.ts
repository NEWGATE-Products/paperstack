import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  Vulnerability,
  VulnFilter,
  VulnListResponse,
} from "../types/vuln";

export interface UseVulnerabilitiesResult {
  vulnerabilities: Vulnerability[];
  total: number;
  page: number;
  limit: number;
  loading: boolean;
  error: string | null;
  filter: VulnFilter;
  setFilter: (filter: VulnFilter) => void;
  setPage: (page: number) => void;
  refresh: () => Promise<void>;
  fetchFromApi: (ecosystems: string[]) => Promise<number>;
}

export function useVulnerabilities(initialLimit = 20): UseVulnerabilitiesResult {
  const [vulnerabilities, setVulnerabilities] = useState<Vulnerability[]>([]);
  const [total, setTotal] = useState(0);
  const [page, setPage] = useState(1);
  const [limit] = useState(initialLimit);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<VulnFilter>({});

  const loadVulnerabilities = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const response = await invoke<VulnListResponse>("get_vulnerabilities", {
        filter,
        page,
        limit,
      });

      setVulnerabilities(response.vulnerabilities);
      setTotal(response.total);
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to load vulnerabilities:", e);
    } finally {
      setLoading(false);
    }
  }, [filter, page, limit]);

  const fetchFromApi = useCallback(async (ecosystems: string[]): Promise<number> => {
    setLoading(true);
    setError(null);

    try {
      const count = await invoke<number>("fetch_vulnerabilities", {
        ecosystems,
      });

      // Reload after fetching
      await loadVulnerabilities();

      return count;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to fetch vulnerabilities:", e);
      return 0;
    } finally {
      setLoading(false);
    }
  }, [loadVulnerabilities]);

  const handleSetFilter = useCallback((newFilter: VulnFilter) => {
    setFilter(newFilter);
    setPage(1); // Reset to first page when filter changes
  }, []);

  const handleSetPage = useCallback((newPage: number) => {
    setPage(newPage);
  }, []);

  // Load vulnerabilities on mount and when filter/page changes
  useEffect(() => {
    loadVulnerabilities();
  }, [loadVulnerabilities]);

  return {
    vulnerabilities,
    total,
    page,
    limit,
    loading,
    error,
    filter,
    setFilter: handleSetFilter,
    setPage: handleSetPage,
    refresh: loadVulnerabilities,
    fetchFromApi,
  };
}
