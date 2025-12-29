import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Category } from "../types";

export function useCategories() {
  const [categories, setCategories] = useState<Category[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string>("all");

  useEffect(() => {
    const fetchCategories = async () => {
      try {
        const result = await invoke<Category[]>("get_categories");
        setCategories(result);
      } catch (e) {
        console.error("Failed to fetch categories:", e);
      }
    };
    fetchCategories();
  }, []);

  return {
    categories,
    selectedCategory,
    setSelectedCategory,
  };
}

