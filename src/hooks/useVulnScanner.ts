import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { ScanResult, ScanHistory, Vulnerability } from "../types/vuln";

export interface UseVulnScannerResult {
  scanResult: ScanResult | null;
  scanHistory: ScanHistory[];
  scanning: boolean;
  loadingHistory: boolean;
  error: string | null;
  scanDirectory: (path: string) => Promise<ScanResult | null>;
  loadScanHistory: (limit?: number) => Promise<void>;
  getVulnerabilityDetail: (vulnId: string) => Promise<Vulnerability | null>;
  clearScanResult: () => void;
}

export function useVulnScanner(): UseVulnScannerResult {
  const [scanResult, setScanResult] = useState<ScanResult | null>(null);
  const [scanHistory, setScanHistory] = useState<ScanHistory[]>([]);
  const [scanning, setScanning] = useState(false);
  const [loadingHistory, setLoadingHistory] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const scanDirectory = useCallback(async (path: string): Promise<ScanResult | null> => {
    setScanning(true);
    setError(null);

    try {
      const result = await invoke<ScanResult>("scan_directory", { path });
      setScanResult(result);
      return result;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to scan directory:", e);
      return null;
    } finally {
      setScanning(false);
    }
  }, []);

  const loadScanHistory = useCallback(async (limit = 20): Promise<void> => {
    setLoadingHistory(true);

    try {
      const history = await invoke<ScanHistory[]>("get_scan_history", { limit });
      setScanHistory(history);
    } catch (e) {
      console.error("Failed to load scan history:", e);
    } finally {
      setLoadingHistory(false);
    }
  }, []);

  const getVulnerabilityDetail = useCallback(async (vulnId: string): Promise<Vulnerability | null> => {
    try {
      const vuln = await invoke<Vulnerability | null>("get_vulnerability_detail", {
        vulnId,
      });
      return vuln;
    } catch (e) {
      console.error("Failed to get vulnerability detail:", e);
      return null;
    }
  }, []);

  const clearScanResult = useCallback(() => {
    setScanResult(null);
    setError(null);
  }, []);

  return {
    scanResult,
    scanHistory,
    scanning,
    loadingHistory,
    error,
    scanDirectory,
    loadScanHistory,
    getVulnerabilityDetail,
    clearScanResult,
  };
}
