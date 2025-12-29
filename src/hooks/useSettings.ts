import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "../types";

interface SettingsResponse {
  groq_api_key: string | null;
  has_groq_api_key: boolean;
}

export function useSettings() {
  const [settings, setSettings] = useState<Settings>({
    groqApiKey: null,
    hasGroqApiKey: false,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadSettings = useCallback(async () => {
    try {
      const result = await invoke<SettingsResponse>("get_settings");
      setSettings({
        groqApiKey: result.groq_api_key,
        hasGroqApiKey: result.has_groq_api_key,
      });
      setError(null);
    } catch (e: unknown) {
      const message = e instanceof Error ? e.message : 
        (typeof e === "object" && e !== null && "message" in e) 
          ? String((e as { message: unknown }).message) 
          : String(e);
      setError(message);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  const saveGroqApiKey = useCallback(async (apiKey: string) => {
    setLoading(true);
    try {
      const result = await invoke<SettingsResponse>("save_settings", {
        settingsInput: { groq_api_key: apiKey || null },
      });
      setSettings({
        groqApiKey: result.groq_api_key,
        hasGroqApiKey: result.has_groq_api_key,
      });
      setError(null);
      return true;
    } catch (e: unknown) {
      const message = e instanceof Error ? e.message : 
        (typeof e === "object" && e !== null && "message" in e) 
          ? String((e as { message: unknown }).message) 
          : String(e);
      setError(message);
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  return {
    settings,
    loading,
    error,
    saveGroqApiKey,
    refreshSettings: loadSettings,
  };
}

