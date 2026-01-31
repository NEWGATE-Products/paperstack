import { useEffect, useCallback } from "react";
import type { Vulnerability, Severity } from "../../types/vuln";
import { getEcosystemLabel } from "../../types/vuln";
import { SeverityBadge } from "./SeverityBadge";
import { IconClose, IconExternalLink, IconShield } from "../icons";

interface VulnDetailProps {
  vulnerability: Vulnerability;
  onClose: () => void;
}

export function VulnDetail({ vulnerability, onClose }: VulnDetailProps) {
  // Escape キーでモーダルを閉じる
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
      }
    },
    [onClose]
  );

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);
    document.body.style.overflow = "hidden";

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.body.style.overflow = "";
    };
  }, [handleKeyDown]);

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return "不明";
    try {
      const date = new Date(dateStr);
      return date.toLocaleDateString("ja-JP", {
        year: "numeric",
        month: "long",
        day: "numeric",
      });
    } catch {
      return dateStr;
    }
  };

  return (
    <div className="vuln-detail-backdrop" onClick={handleBackdropClick}>
      <div className="vuln-detail-modal">
        <div className="vuln-detail-header">
          <div className="vuln-detail-title-row">
            <IconShield size={24} className="vuln-detail-icon" />
            <h2 className="vuln-detail-id">{vulnerability.id}</h2>
            <SeverityBadge severity={vulnerability.severity as Severity} />
          </div>
          <button
            type="button"
            className="vuln-detail-close"
            onClick={onClose}
            aria-label="閉じる"
          >
            <IconClose size={20} />
          </button>
        </div>

        <div className="vuln-detail-content">
          <h3 className="vuln-detail-title">{vulnerability.title}</h3>

          <div className="vuln-detail-meta">
            <div className="vuln-detail-meta-item">
              <span className="meta-label">パッケージ</span>
              <span className="meta-value">{vulnerability.affectedPackage}</span>
            </div>
            <div className="vuln-detail-meta-item">
              <span className="meta-label">エコシステム</span>
              <span className="meta-value">
                {getEcosystemLabel(vulnerability.affectedEcosystem)}
              </span>
            </div>
            {vulnerability.cvssScore && (
              <div className="vuln-detail-meta-item">
                <span className="meta-label">CVSS スコア</span>
                <span className="meta-value cvss-score">
                  {vulnerability.cvssScore.toFixed(1)}
                </span>
              </div>
            )}
            <div className="vuln-detail-meta-item">
              <span className="meta-label">公開日</span>
              <span className="meta-value">
                {formatDate(vulnerability.publishedAt)}
              </span>
            </div>
            <div className="vuln-detail-meta-item">
              <span className="meta-label">データソース</span>
              <span className="meta-value source-badge">
                {vulnerability.source.toUpperCase()}
              </span>
            </div>
          </div>

          {vulnerability.description && (
            <div className="vuln-detail-section">
              <h4>説明</h4>
              <p className="vuln-description">{vulnerability.description}</p>
            </div>
          )}

          <div className="vuln-detail-section">
            <h4>影響を受けるバージョン</h4>
            <p className="vuln-versions">
              {vulnerability.affectedVersions || "情報なし"}
            </p>
          </div>

          <div className="vuln-detail-section">
            <h4>修正バージョン</h4>
            <p className={`vuln-fixed ${vulnerability.fixedVersions ? "has-fix" : "no-fix"}`}>
              {vulnerability.fixedVersions || "修正情報なし"}
            </p>
          </div>

          {vulnerability.references.length > 0 && (
            <div className="vuln-detail-section">
              <h4>参照リンク</h4>
              <ul className="vuln-references">
                {vulnerability.references.map((url, index) => (
                  <li key={index}>
                    <a
                      href={url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="vuln-reference-link"
                    >
                      <IconExternalLink size={14} />
                      {url}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>

        <div className="vuln-detail-footer">
          <button type="button" className="btn-secondary" onClick={onClose}>
            閉じる
          </button>
        </div>
      </div>
    </div>
  );
}
