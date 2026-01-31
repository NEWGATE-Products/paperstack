import { useState, useCallback } from "react";
import type { Vulnerability } from "../../types/vuln";
import { useVulnerabilities } from "../../hooks/useVulnerabilities";
import { useVulnScanner } from "../../hooks/useVulnScanner";
import { VulnFilter } from "./VulnFilter";
import { VulnCard } from "./VulnCard";
import { VulnDetail } from "./VulnDetail";
import { DirectoryScanner } from "./DirectoryScanner";
import { ScanResultList } from "./ScanResultList";
import { IconWarning, IconShield, IconSearch } from "../icons";

type VulnSubTab = "list" | "scan";

export function VulnList() {
  const [subTab, setSubTab] = useState<VulnSubTab>("list");
  const [selectedVuln, setSelectedVuln] = useState<Vulnerability | null>(null);

  const {
    vulnerabilities,
    total,
    page,
    limit,
    loading,
    error,
    filter,
    setFilter,
    setPage,
    fetchFromApi,
  } = useVulnerabilities(20);

  const {
    scanResult,
    scanning,
    error: scanError,
    scanDirectory,
    clearScanResult,
  } = useVulnScanner();

  const handleViewDetail = useCallback((vuln: Vulnerability) => {
    setSelectedVuln(vuln);
  }, []);

  const handleCloseDetail = useCallback(() => {
    setSelectedVuln(null);
  }, []);

  const handleScan = useCallback(async (path: string) => {
    await scanDirectory(path);
  }, [scanDirectory]);

  const totalPages = Math.ceil(total / limit);

  return (
    <div className="vuln-list-container">
      {/* Sub Tab Navigation */}
      <div className="vuln-sub-tabs">
        <button
          type="button"
          className={`vuln-sub-tab ${subTab === "list" ? "active" : ""}`}
          onClick={() => setSubTab("list")}
        >
          <IconShield size={16} />
          一覧
        </button>
        <button
          type="button"
          className={`vuln-sub-tab ${subTab === "scan" ? "active" : ""}`}
          onClick={() => setSubTab("scan")}
        >
          <IconSearch size={16} />
          スキャン
        </button>
      </div>

      {/* List Tab */}
      {subTab === "list" && (
        <>
          <VulnFilter
            filter={filter}
            onFilterChange={setFilter}
            onRefresh={fetchFromApi}
            vulnCount={total}
            loading={loading}
          />

          {error && (
            <div className="error-message">
              <p>
                <IconWarning size={16} className="inline-icon" /> エラー: {error}
              </p>
            </div>
          )}

          {loading && vulnerabilities.length === 0 && (
            <div className="loading-state">
              <div className="loading-spinner" />
              <p>脆弱性情報を読み込み中...</p>
            </div>
          )}

          {!loading && vulnerabilities.length === 0 && (
            <div className="empty-state">
              <IconShield size={48} className="empty-icon" />
              <p>脆弱性情報がありません</p>
              <p className="hint">
                「更新」ボタンを押して最新の脆弱性情報を取得してください
              </p>
            </div>
          )}

          <div className="vuln-cards">
            {vulnerabilities.map((vuln) => (
              <VulnCard
                key={vuln.id}
                vulnerability={vuln}
                onViewDetail={handleViewDetail}
              />
            ))}
          </div>

          {totalPages > 1 && (
            <div className="pagination">
              <button
                type="button"
                className="page-btn"
                onClick={() => setPage(page - 1)}
                disabled={page <= 1 || loading}
              >
                ← 前へ
              </button>
              <span className="page-info">
                {page} / {totalPages} ページ
              </span>
              <button
                type="button"
                className="page-btn"
                onClick={() => setPage(page + 1)}
                disabled={page >= totalPages || loading}
              >
                次へ →
              </button>
            </div>
          )}
        </>
      )}

      {/* Scan Tab */}
      {subTab === "scan" && (
        <>
          {!scanResult ? (
            <DirectoryScanner
              onScan={handleScan}
              scanning={scanning}
              error={scanError}
            />
          ) : (
            <ScanResultList result={scanResult} onClear={clearScanResult} />
          )}
        </>
      )}

      {/* Detail Modal */}
      {selectedVuln && (
        <VulnDetail vulnerability={selectedVuln} onClose={handleCloseDetail} />
      )}
    </div>
  );
}
