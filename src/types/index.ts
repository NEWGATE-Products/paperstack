export interface Paper {
  id: string;
  title: string;
  titleJa: string | null;
  abstract: string | null;
  summaryJa: string | null;
  urlPdf: string | null;
  urlPaper: string | null;
  published: string | null;
  fetchedAt: string | null;
  tasks: string[];
}

export interface Category {
  id: string;
  name: string;
  tasks: string[];
}

export type LoadingState = "idle" | "loading" | "success" | "error";

export interface Settings {
  groqApiKey: string | null;
  hasGroqApiKey: boolean;
}
