import { useState } from "react";
import type { Rfc, SummaryLevel } from "../../types/rfc";
import { getCategoryName } from "../../types/rfc";
import { RfcStatusBadge } from "./RfcStatusBadge";

interface RfcCardProps {
  rfc: Rfc;
  onViewDetail: (rfc: Rfc) => void;
  onToggleBookmark: (rfcId: string, isBookmarked: boolean) => void;
  onGenerateSummary: (rfcId: string, level: SummaryLevel) => Promise<string>;
  onTranslateAbstract: (rfcId: string) => Promise<string>;
  onTranslateTitle: (rfcId: string) => Promise<string>;
}

export function RfcCard({
  rfc,
  onViewDetail,
  onToggleBookmark,
  onGenerateSummary,
  onTranslateAbstract,
  onTranslateTitle,
}: RfcCardProps) {
  const [summaryLevel, setSummaryLevel] = useState<SummaryLevel>("easy");
  const [isGenerating, setIsGenerating] = useState(false);
  const [isTranslatingAbstract, setIsTranslatingAbstract] = useState(false);
  const [isTranslatingTitle, setIsTranslatingTitle] = useState(false);
  const [expanded, setExpanded] = useState(false);

  // Get current summary based on level
  const getCurrentSummary = () => {
    switch (summaryLevel) {
      case "easy":
        return rfc.summaryEasy;
      case "normal":
        return rfc.summaryNormal;
      case "technical":
        return rfc.summaryTechnical;
    }
  };

  const currentSummary = getCurrentSummary();

  const handleGenerateSummary = async () => {
    setIsGenerating(true);
    try {
      await onGenerateSummary(rfc.id, summaryLevel);
    } catch (e) {
      console.error("Failed to generate summary:", e);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleTranslateAbstract = async () => {
    setIsTranslatingAbstract(true);
    try {
      await onTranslateAbstract(rfc.id);
    } catch (e) {
      console.error("Failed to translate abstract:", e);
    } finally {
      setIsTranslatingAbstract(false);
    }
  };

  const handleTranslateTitle = async () => {
    setIsTranslatingTitle(true);
    try {
      await onTranslateTitle(rfc.id);
    } catch (e) {
      console.error("Failed to translate title:", e);
    } finally {
      setIsTranslatingTitle(false);
    }
  };

  return (
    <div className="rfc-card">
      {/* Header */}
      <div className="rfc-card-header">
        <div className="rfc-card-title-row">
          <span className="rfc-number">📄 {rfc.id}</span>
          <div className="rfc-card-actions">
            {!rfc.titleJa && (
              <button
                type="button"
                className="translate-btn-small"
                onClick={handleTranslateTitle}
                disabled={isTranslatingTitle}
                title="タイトルを日本語に翻訳"
              >
                {isTranslatingTitle ? "..." : "🌐"}
              </button>
            )}
            <button
              type="button"
              className={`bookmark-btn ${rfc.isBookmarked ? "active" : ""}`}
              onClick={() => onToggleBookmark(rfc.id, rfc.isBookmarked)}
              title={rfc.isBookmarked ? "ブックマーク解除" : "ブックマーク"}
            >
              {rfc.isBookmarked ? "⭐" : "☆"}
            </button>
          </div>
        </div>
        <h3 className="rfc-title">{rfc.title}</h3>
        {rfc.titleJa && <p className="rfc-title-ja">📌 {rfc.titleJa}</p>}
      </div>

      {/* Summary Section (かんたんモード) */}
      <div className="rfc-summary-section">
        <div className="summary-level-tabs">
          <button
            type="button"
            className={`tab ${summaryLevel === "easy" ? "active" : ""}`}
            onClick={() => setSummaryLevel("easy")}
          >
            🎈 かんたん
          </button>
          <button
            type="button"
            className={`tab ${summaryLevel === "normal" ? "active" : ""}`}
            onClick={() => setSummaryLevel("normal")}
          >
            📝 一般
          </button>
          <button
            type="button"
            className={`tab ${summaryLevel === "technical" ? "active" : ""}`}
            onClick={() => setSummaryLevel("technical")}
          >
            🔧 技術者
          </button>
        </div>

        <div className="summary-content">
          {currentSummary ? (
            <p className="summary-text">{currentSummary}</p>
          ) : (
            <div className="summary-empty">
              <p>まだ要約がありません</p>
              <button
                type="button"
                className="generate-btn"
                onClick={handleGenerateSummary}
                disabled={isGenerating}
              >
                {isGenerating ? "生成中..." : "🔄 要約を生成"}
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Abstract (expandable) */}
      {rfc.abstract && (
        <div className="rfc-abstract">
          <div className="abstract-header">
            <button
              type="button"
              className="expand-btn"
              onClick={() => setExpanded(!expanded)}
            >
              📝 概要 {expanded ? "▲" : "▼"}
            </button>
            {expanded && !rfc.abstractJa && (
              <button
                type="button"
                className="translate-btn"
                onClick={handleTranslateAbstract}
                disabled={isTranslatingAbstract}
                title="日本語に翻訳"
              >
                {isTranslatingAbstract ? "翻訳中..." : "🌐 翻訳"}
              </button>
            )}
          </div>
          {expanded && (
            <div className="abstract-content">
              {rfc.abstractJa ? (
                <>
                  <p className="abstract-text-ja">{rfc.abstractJa}</p>
                  <details className="abstract-original">
                    <summary>原文を表示</summary>
                    <p className="abstract-text">{rfc.abstract}</p>
                  </details>
                </>
              ) : (
                <p className="abstract-text">{rfc.abstract}</p>
              )}
            </div>
          )}
        </div>
      )}

      {/* Metadata */}
      <div className="rfc-metadata">
        <RfcStatusBadge status={rfc.status} />
        {rfc.categories.map((cat) => (
          <span key={cat} className="category-badge">
            {getCategoryName(cat)}
          </span>
        ))}
        {rfc.publishedDate && (
          <span className="date-badge">📅 {rfc.publishedDate}</span>
        )}
      </div>

      {/* Actions */}
      <div className="rfc-actions">
        <button type="button" className="action-btn primary" onClick={() => onViewDetail(rfc)}>
          📖 詳細を見る
        </button>
      </div>
    </div>
  );
}

