import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { RfcBookmark } from "../types/rfc";

interface UseRfcBookmarksReturn {
  bookmarks: RfcBookmark[];
  loading: boolean;
  error: string | null;
  addBookmark: (rfcId: string, memo?: string) => Promise<void>;
  removeBookmark: (rfcId: string) => Promise<void>;
  isBookmarked: (rfcId: string) => boolean;
  refreshBookmarks: () => Promise<void>;
}

export function useRfcBookmarks(): UseRfcBookmarksReturn {
  const [bookmarks, setBookmarks] = useState<RfcBookmark[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refreshBookmarks = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<RfcBookmark[]>("get_rfc_bookmarks");
      setBookmarks(result);
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : String(e);
      setError(errorMessage);
      console.error("Failed to fetch bookmarks:", e);
    } finally {
      setLoading(false);
    }
  }, []);

  const addBookmark = useCallback(
    async (rfcId: string, memo?: string) => {
      try {
        await invoke("add_rfc_bookmark", { rfcId, memo: memo ?? null });
        await refreshBookmarks();
      } catch (e) {
        const errorMessage = e instanceof Error ? e.message : String(e);
        setError(errorMessage);
        throw e;
      }
    },
    [refreshBookmarks]
  );

  const removeBookmark = useCallback(
    async (rfcId: string) => {
      try {
        await invoke("remove_rfc_bookmark", { rfcId });
        await refreshBookmarks();
      } catch (e) {
        const errorMessage = e instanceof Error ? e.message : String(e);
        setError(errorMessage);
        throw e;
      }
    },
    [refreshBookmarks]
  );

  const isBookmarked = useCallback(
    (rfcId: string): boolean => {
      return bookmarks.some((b) => b.rfcId === rfcId);
    },
    [bookmarks]
  );

  // Load bookmarks on mount
  useEffect(() => {
    refreshBookmarks();
  }, [refreshBookmarks]);

  return {
    bookmarks,
    loading,
    error,
    addBookmark,
    removeBookmark,
    isBookmarked,
    refreshBookmarks,
  };
}

