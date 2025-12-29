import { useState } from "react";
import { open } from "@tauri-apps/plugin-shell";
import type { Paper } from "../types";

interface PaperCardProps {
  paper: Paper;
  onGenerateSummary: (paperId: string) => Promise<string>;
}

export function PaperCard({ paper, onGenerateSummary }: PaperCardProps) {
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleGenerateSummary = async () => {
    setIsGenerating(true);
    setError(null);
    try {
      await onGenerateSummary(paper.id);
    } catch (e) {
      const message = e instanceof Error ? e.message : "要約生成に失敗しました";
      // Make API key error more user-friendly
      if (message.includes("GROQ_API_KEY") || message.includes("Missing API key") || message.includes("APIキー")) {
        setError("APIキーが未設定です。⚙️設定からAPIキーを入力してください。");
      } else {
        setError(message);
      }
    } finally {
      setIsGenerating(false);
    }
  };

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return null;
    try {
      const date = new Date(dateStr);
      return date.toLocaleDateString("ja-JP", {
        year: "numeric",
        month: "short",
        day: "numeric",
      });
    } catch {
      return dateStr;
    }
  };

  const openExternal = async (url: string) => {
    try {
      await open(url);
    } catch (e) {
      console.error("Failed to open URL:", e);
    }
  };

  return (
    <article className="paper-card">
      <div className="paper-card-header">
        <h2 className="paper-title">
          {paper.urlPaper ? (
            <a
              href={paper.urlPaper}
              className="paper-title-link"
              onClick={(e) => {
                e.preventDefault();
                openExternal(paper.urlPaper!);
              }}
            >
              {paper.title}
            </a>
          ) : (
            paper.title
          )}
        </h2>
        {paper.published && (
          <span className="paper-date">{formatDate(paper.published)}</span>
        )}
      </div>

      <div className="paper-summary">
        {paper.summaryJa ? (
          <p>💡 {paper.summaryJa}</p>
        ) : (
          <div className="paper-summary-placeholder">
            {isGenerating ? (
              <span>✨ 要約を生成中...</span>
            ) : error ? (
              <span style={{ color: "#991b1b" }}>❌ {error}</span>
            ) : (
              <>
                <span>要約がありません。</span>
                <button
                  className="paper-summary-btn"
                  onClick={handleGenerateSummary}
                  disabled={isGenerating}
                >
                  🔮 AIで要約を生成
                </button>
              </>
            )}
          </div>
        )}
      </div>

      <div className="paper-footer">
        <div className="paper-tags">
          {paper.tasks.map((task) => (
            <span key={task} className="paper-tag">
              🏷️ {formatTaskName(task)}
            </span>
          ))}
        </div>
        <div className="paper-links">
          {paper.urlPdf && (
            <a
              href={paper.urlPdf}
              className="paper-link"
              onClick={(e) => {
                e.preventDefault();
                openExternal(paper.urlPdf!);
              }}
            >
              📄 PDF
            </a>
          )}
          {paper.urlPaper && (
            <a
              href={paper.urlPaper}
              className="paper-link"
              onClick={(e) => {
                e.preventDefault();
                openExternal(paper.urlPaper!);
              }}
            >
              🔗 論文ページ
            </a>
          )}
        </div>
      </div>
    </article>
  );
}

function formatTaskName(task: string): string {
  const taskNames: Record<string, string> = {
    "machine-learning": "機械学習",
    "deep-learning": "深層学習",
    "reinforcement-learning": "強化学習",
    "language-modelling": "言語モデル",
    "text-generation": "テキスト生成",
    "question-answering": "質問応答",
    "code-generation": "コード生成",
    "program-synthesis": "プログラム合成",
    optimization: "最適化",
    "graph-neural-networks": "グラフNN",
    transformers: "Transformer",
    attention: "Attention",
    "neural-architecture-search": "NAS",
    architecture: "アーキテクチャ",
  };
  return taskNames[task] || task;
}
