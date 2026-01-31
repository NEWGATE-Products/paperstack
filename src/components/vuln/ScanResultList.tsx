import { useMemo, useState } from "react";
import type { ScanResult, Vulnerability } from "../../types/vuln";
import { VulnCard } from "./VulnCard";
import { VulnDetail } from "./VulnDetail";
import { IconCheck, IconWarning, IconFolder, IconClose } from "../icons";

interface ScanResultListProps {
  result: ScanResult;
  onClear: () => void;
}

export function ScanResultList({ result, onClear }: ScanResultListProps) {
  const [selectedVuln, setSelectedVuln] = useState<Vulnerability | null>(null);

  // 深刻度でグループ化
  const groupedBySevetity = useMemo(() => {
    const groups: Record<string, typeof result.vulnerabilities> = {
      critical: [],
      high: [],
      medium: [],
      low: [],
    };

    for (const match of result.vulnerabilities) {
      const severity = match.vulnerability.severity.toLowerCase();
      if (groups[severity]) {
        groups[severity].push(match);
      } else {
        groups.medium.push(match);
      }
    }

    return groups;
  }, [result.vulnerabilities]);

  // 統計情報
  const stats = useMemo(() => {
    return {
      total: result.vulnerabilities.length,
      critical: groupedBySevetity.critical.length,
      high: groupedBySevetity.high.length,
      medium: groupedBySevetity.medium.length,
      low: groupedBySevetity.low.length,
    };
  }, [result.vulnerabilities.length, groupedBySevetity]);

  const formatDate = (dateStr: string) => {
    try {
      const date = new Date(dateStr);
      return date.toLocaleString("ja-JP", {
        year: "numeric",
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return dateStr;
    }
  };

  const handleViewDetail = (vuln: Vulnerability) => {
    setSelectedVuln(vuln);
  };

  const handleCloseDetail = () => {
    setSelectedVuln(null);
  };

  const hasVulnerabilities = result.vulnerabilities.length > 0;

  return (
    <div className="scan-result">
      <div className="scan-result-header">
        <div className="scan-result-info">
          <h3 className="scan-result-title">
            <IconFolder size={20} className="inline-icon" />
            スキャン結果
          </h3>
          <div className="scan-result-meta">
            <span className="meta-item">
              <strong>ディレクトリ:</strong> {result.directory}
            </span>
            <span className="meta-item">
              <strong>スキャン日時:</strong> {formatDate(result.scannedAt)}
            </span>
            <span className="meta-item">
              <strong>パッケージ数:</strong> {result.totalPackages}
            </span>
            <span className="meta-item">
              <strong>エコシステム:</strong> {result.ecosystems.join(", ")}
            </span>
          </div>
        </div>
        <button
          type="button"
          className="btn-secondary scan-result-close"
          onClick={onClear}
          title="結果をクリア"
        >
          <IconClose size={16} />
        </button>
      </div>

      <div className="scan-result-summary">
        {hasVulnerabilities ? (
          <>
            <div className="summary-alert warning">
              <IconWarning size={20} className="inline-icon" />
              <span>
                {stats.total} 件の脆弱性が検出されました
              </span>
            </div>
            <div className="summary-stats">
              {stats.critical > 0 && (
                <span className="stat-badge critical">
                  緊急: {stats.critical}
                </span>
              )}
              {stats.high > 0 && (
                <span className="stat-badge high">
                  重要: {stats.high}
                </span>
              )}
              {stats.medium > 0 && (
                <span className="stat-badge medium">
                  警告: {stats.medium}
                </span>
              )}
              {stats.low > 0 && (
                <span className="stat-badge low">
                  注意: {stats.low}
                </span>
              )}
            </div>
          </>
        ) : (
          <div className="summary-alert success">
            <IconCheck size={20} className="inline-icon" />
            <span>脆弱性は検出されませんでした</span>
          </div>
        )}
      </div>

      {hasVulnerabilities && (
        <div className="scan-result-groups">
          {groupedBySevetity.critical.length > 0 && (
            <div className="severity-group critical">
              <h4 className="group-title">
                <span className="severity-indicator critical" />
                緊急 ({groupedBySevetity.critical.length})
              </h4>
              <div className="group-cards">
                {groupedBySevetity.critical.map((match) => (
                  <VulnCard
                    key={`${match.vulnerability.id}-${match.packageName}`}
                    vulnerability={match.vulnerability}
                    installedVersion={match.installedVersion}
                    onViewDetail={handleViewDetail}
                  />
                ))}
              </div>
            </div>
          )}

          {groupedBySevetity.high.length > 0 && (
            <div className="severity-group high">
              <h4 className="group-title">
                <span className="severity-indicator high" />
                重要 ({groupedBySevetity.high.length})
              </h4>
              <div className="group-cards">
                {groupedBySevetity.high.map((match) => (
                  <VulnCard
                    key={`${match.vulnerability.id}-${match.packageName}`}
                    vulnerability={match.vulnerability}
                    installedVersion={match.installedVersion}
                    onViewDetail={handleViewDetail}
                  />
                ))}
              </div>
            </div>
          )}

          {groupedBySevetity.medium.length > 0 && (
            <div className="severity-group medium">
              <h4 className="group-title">
                <span className="severity-indicator medium" />
                警告 ({groupedBySevetity.medium.length})
              </h4>
              <div className="group-cards">
                {groupedBySevetity.medium.map((match) => (
                  <VulnCard
                    key={`${match.vulnerability.id}-${match.packageName}`}
                    vulnerability={match.vulnerability}
                    installedVersion={match.installedVersion}
                    onViewDetail={handleViewDetail}
                  />
                ))}
              </div>
            </div>
          )}

          {groupedBySevetity.low.length > 0 && (
            <div className="severity-group low">
              <h4 className="group-title">
                <span className="severity-indicator low" />
                注意 ({groupedBySevetity.low.length})
              </h4>
              <div className="group-cards">
                {groupedBySevetity.low.map((match) => (
                  <VulnCard
                    key={`${match.vulnerability.id}-${match.packageName}`}
                    vulnerability={match.vulnerability}
                    installedVersion={match.installedVersion}
                    onViewDetail={handleViewDetail}
                  />
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      {selectedVuln && (
        <VulnDetail vulnerability={selectedVuln} onClose={handleCloseDetail} />
      )}
    </div>
  );
}
