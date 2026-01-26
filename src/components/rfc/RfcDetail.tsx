import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-shell";
import type { Rfc, SummaryLevel } from "../../types/rfc";
import { getCategoryName } from "../../types/rfc";
import { RfcStatusBadge } from "./RfcStatusBadge";
import { useRfcDetail } from "../../hooks/useRfcDetail";
import {
  IconDocument,
  IconCalendar,
  IconUser,
  IconWarning,
  IconLightbulb,
  IconBalloon,
  IconEdit,
  IconWrench,
  IconRefresh,
  IconComputer,
  IconTag,
  IconLink,
} from "../icons";

interface RfcDetailProps {
  rfc: Rfc;
  onClose: () => void;
}

export function RfcDetail({ rfc: initialRfc, onClose }: RfcDetailProps) {
  const {
    rfc,
    loadingSummary,
    loadingGuide,
    error,
    fetchRfc,
    generateSummary,
    generateImplementationGuide,
  } = useRfcDetail();

  const [activeTab, setActiveTab] = useState<SummaryLevel>("easy");
  const [showImplementationGuide, setShowImplementationGuide] = useState(false);

  // Fetch latest RFC data on mount
  useEffect(() => {
    fetchRfc(initialRfc.id);
  }, [initialRfc.id, fetchRfc]);

  const currentRfc = rfc || initialRfc;

  const getCurrentSummary = () => {
    switch (activeTab) {
      case "easy":
        return currentRfc.summaryEasy;
      case "normal":
        return currentRfc.summaryNormal;
      case "technical":
        return currentRfc.summaryTechnical;
    }
  };

  const handleGenerateSummary = async () => {
    try {
      await generateSummary(currentRfc.id, activeTab);
    } catch (e) {
      console.error("Failed to generate summary:", e);
    }
  };

  const handleGenerateGuide = async () => {
    try {
      await generateImplementationGuide(currentRfc.id);
      setShowImplementationGuide(true);
    } catch (e) {
      console.error("Failed to generate implementation guide:", e);
    }
  };

  const currentSummary = getCurrentSummary();

  return (
    <div className="rfc-detail-overlay" onClick={onClose}>
      <div className="rfc-detail-modal" onClick={(e) => e.stopPropagation()}>
        {/* Header */}
        <div className="detail-header">
          <div className="detail-title-section">
            <span className="detail-rfc-number"><IconDocument size={16} className="inline-icon" /> {currentRfc.id}</span>
            <h2 className="detail-title">{currentRfc.title}</h2>
            {currentRfc.titleJa && (
              <p className="detail-title-ja">{currentRfc.titleJa}</p>
            )}
          </div>
          <button type="button" className="close-btn" onClick={onClose}>
            ✕
          </button>
        </div>

        {/* Metadata */}
        <div className="detail-metadata">
          <RfcStatusBadge status={currentRfc.status} />
          {currentRfc.categories.map((cat) => (
            <span key={cat} className="category-badge">
              {getCategoryName(cat)}
            </span>
          ))}
          {currentRfc.publishedDate && (
            <span className="date-badge"><IconCalendar size={12} className="inline-icon" /> {currentRfc.publishedDate}</span>
          )}
          {currentRfc.authors.length > 0 && (
            <span className="authors-badge">
              <IconUser size={12} className="inline-icon" /> {currentRfc.authors.slice(0, 3).join(", ")}
              {currentRfc.authors.length > 3 && ` 他${currentRfc.authors.length - 3}名`}
            </span>
          )}
        </div>

        {/* Error */}
        {error && (
          <div className="detail-error">
            <p><IconWarning size={16} className="inline-icon" /> {error}</p>
          </div>
        )}

        {/* Summary Section */}
        <div className="detail-section">
          <h3 className="section-title"><IconLightbulb size={18} className="inline-icon" /> 要約</h3>

          <div className="summary-tabs">
            <button
              type="button"
              className={`summary-tab ${activeTab === "easy" ? "active" : ""}`}
              onClick={() => setActiveTab("easy")}
            >
              <IconBalloon size={14} className="inline-icon" /> かんたん
            </button>
            <button
              type="button"
              className={`summary-tab ${activeTab === "normal" ? "active" : ""}`}
              onClick={() => setActiveTab("normal")}
            >
              <IconEdit size={14} className="inline-icon" /> 一般
            </button>
            <button
              type="button"
              className={`summary-tab ${activeTab === "technical" ? "active" : ""}`}
              onClick={() => setActiveTab("technical")}
            >
              <IconWrench size={14} className="inline-icon" /> 技術者
            </button>
          </div>

          <div className="summary-panel">
            {loadingSummary ? (
              <div className="loading-indicator">
                <div className="loading-spinner small" />
                <span>要約を生成中...</span>
              </div>
            ) : currentSummary ? (
              <p className="summary-text large">{currentSummary}</p>
            ) : (
              <div className="summary-empty">
                <p>この難易度の要約はまだありません</p>
                <button
                  type="button"
                  className="generate-btn"
                  onClick={handleGenerateSummary}
                  disabled={loadingSummary}
                >
                  <IconRefresh size={14} className="inline-icon" /> 要約を生成
                </button>
              </div>
            )}
          </div>
        </div>

        {/* Implementation Guide Section (Technical only) */}
        {activeTab === "technical" && (
          <div className="detail-section implementation-section">
            <div className="section-header">
              <h3 className="section-title"><IconComputer size={18} className="inline-icon" /> 実装ガイド</h3>
              {!currentRfc.implementationGuide && !showImplementationGuide && (
                <button
                  type="button"
                  className="generate-btn small"
                  onClick={handleGenerateGuide}
                  disabled={loadingGuide}
                >
                  {loadingGuide ? "生成中..." : "生成する"}
                </button>
              )}
            </div>

            {loadingGuide ? (
              <div className="loading-indicator">
                <div className="loading-spinner small" />
                <span>実装ガイドを生成中...</span>
              </div>
            ) : currentRfc.implementationGuide ? (
              <div className="implementation-guide">
                <pre className="guide-content">
                  {currentRfc.implementationGuide}
                </pre>
              </div>
            ) : showImplementationGuide ? (
              <p className="guide-empty">実装ガイドの生成に失敗しました</p>
            ) : null}
          </div>
        )}

        {/* Abstract Section */}
        <div className="detail-section">
          <h3 className="section-title"><IconDocument size={18} className="inline-icon" /> 概要 (Abstract)</h3>
          <div className="abstract-panel">
            {currentRfc.abstract ? (
              <p className="abstract-text">{currentRfc.abstract}</p>
            ) : (
              <p className="no-content">概要がありません</p>
            )}
          </div>
        </div>

        {/* Keywords */}
        {currentRfc.keywords.length > 0 && (
          <div className="detail-section">
            <h3 className="section-title"><IconTag size={18} className="inline-icon" /> キーワード</h3>
            <div className="keywords">
              {currentRfc.keywords.map((kw, i) => (
                <span key={i} className="keyword-tag">
                  {kw}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Actions */}
        <div className="detail-actions">
          <button
            type="button"
            className="action-btn external"
            onClick={() => {
              const url = `https://www.rfc-editor.org/rfc/rfc${currentRfc.number}.html`;
              open(url).catch((err) => console.error('Failed to open URL:', err));
            }}
          >
            <IconLink size={14} className="inline-icon" /> RFC本文を見る
          </button>
          <button type="button" className="action-btn" onClick={onClose}>
            閉じる
          </button>
        </div>
      </div>
    </div>
  );
}

