import type { Vulnerability, Severity } from "../../types/vuln";
import { getEcosystemLabel } from "../../types/vuln";
import { SeverityBadge } from "./SeverityBadge";
import { IconExternalLink, IconShield } from "../icons";

interface VulnCardProps {
  vulnerability: Vulnerability;
  installedVersion?: string;
  onViewDetail?: (vuln: Vulnerability) => void;
}

export function VulnCard({ vulnerability, installedVersion, onViewDetail }: VulnCardProps) {
  const handleClick = () => {
    onViewDetail?.(vulnerability);
  };

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return "不明";
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

  return (
    <div className="vuln-card" onClick={handleClick}>
      <div className="vuln-card-header">
        <div className="vuln-card-id">
          <IconShield size={16} className="inline-icon" />
          <span className="vuln-id">{vulnerability.id}</span>
        </div>
        <SeverityBadge severity={vulnerability.severity as Severity} />
      </div>

      <h3 className="vuln-card-title">{vulnerability.title}</h3>

      <div className="vuln-card-meta">
        <div className="vuln-meta-item">
          <span className="meta-label">パッケージ:</span>
          <span className="meta-value">{vulnerability.affectedPackage}</span>
        </div>
        <div className="vuln-meta-item">
          <span className="meta-label">エコシステム:</span>
          <span className="meta-value ecosystem-badge">
            {getEcosystemLabel(vulnerability.affectedEcosystem)}
          </span>
        </div>
        {installedVersion && (
          <div className="vuln-meta-item">
            <span className="meta-label">インストール済み:</span>
            <span className="meta-value version-installed">{installedVersion}</span>
          </div>
        )}
        {vulnerability.cvssScore && (
          <div className="vuln-meta-item">
            <span className="meta-label">CVSS:</span>
            <span className="meta-value cvss-score">{vulnerability.cvssScore.toFixed(1)}</span>
          </div>
        )}
      </div>

      {vulnerability.affectedVersions && (
        <div className="vuln-versions">
          <span className="versions-label">影響バージョン:</span>
          <span className="versions-value">{vulnerability.affectedVersions}</span>
        </div>
      )}

      {vulnerability.fixedVersions && (
        <div className="vuln-fixed">
          <span className="fixed-label">修正バージョン:</span>
          <span className="fixed-value">{vulnerability.fixedVersions}</span>
        </div>
      )}

      <div className="vuln-card-footer">
        <span className="vuln-date">公開: {formatDate(vulnerability.publishedAt)}</span>
        {vulnerability.references.length > 0 && (
          <a
            href={vulnerability.references[0]}
            target="_blank"
            rel="noopener noreferrer"
            className="vuln-link"
            onClick={(e) => e.stopPropagation()}
          >
            <IconExternalLink size={14} />
            詳細
          </a>
        )}
      </div>
    </div>
  );
}
