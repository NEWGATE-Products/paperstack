import { useState } from "react";
import type { Rfc, RfcFilter as RfcFilterType } from "../../types/rfc";
import { RfcCard } from "./RfcCard";
import { RfcFilter } from "./RfcFilter";
import { RfcDetail } from "./RfcDetail";
import { useRfcs } from "../../hooks/useRfcs";
import { useRfcBookmarks } from "../../hooks/useRfcBookmarks";
import { IconWarning, IconDocument } from "../icons";

export function RfcList() {
  const {
    rfcs,
    total,
    page,
    limit,
    loading,
    error,
    filter,
    setFilter,
    setPage,
    refreshFromServer,
    translateAbstract,
    translateTitle,
    generateSummary,
    updateRfcInList,
  } = useRfcs(20);

  const { addBookmark, removeBookmark } = useRfcBookmarks();
  const [selectedRfc, setSelectedRfc] = useState<Rfc | null>(null);

  const handleFilterChange = (newFilter: RfcFilterType) => {
    setFilter(newFilter);
    setPage(1); // Reset to first page when filter changes
  };

  const handleToggleBookmark = async (rfcId: string, isBookmarked: boolean) => {
    try {
      if (isBookmarked) {
        await removeBookmark(rfcId);
        updateRfcInList(rfcId, { isBookmarked: false });
      } else {
        await addBookmark(rfcId);
        updateRfcInList(rfcId, { isBookmarked: true });
      }
    } catch (e) {
      console.error("Failed to toggle bookmark:", e);
    }
  };

  const handleViewDetail = (rfc: Rfc) => {
    setSelectedRfc(rfc);
  };

  const handleCloseDetail = () => {
    setSelectedRfc(null);
  };

  const totalPages = Math.ceil(total / limit);

  return (
    <div className="rfc-list-container">
      {/* Filter */}
      <RfcFilter
        filter={filter}
        onFilterChange={handleFilterChange}
        onRefresh={refreshFromServer}
        rfcCount={total}
        loading={loading}
      />

      {/* Error Message */}
      {error && (
        <div className="error-message">
          <p><IconWarning size={16} className="inline-icon" /> エラー: {error}</p>
        </div>
      )}

      {/* Loading State */}
      {loading && rfcs.length === 0 && (
        <div className="loading-state">
          <div className="loading-spinner" />
          <p>RFCを読み込み中...</p>
        </div>
      )}

      {/* Empty State */}
      {!loading && rfcs.length === 0 && (
        <div className="empty-state">
          <p><IconDocument size={20} className="inline-icon" /> RFCが見つかりません</p>
          <p className="hint">
            「更新」ボタンを押してRFCデータを取得してください
          </p>
        </div>
      )}

      {/* RFC Cards */}
      <div className="rfc-cards">
        {rfcs.map((rfc) => (
          <RfcCard
            key={rfc.id}
            rfc={rfc}
            onViewDetail={handleViewDetail}
            onToggleBookmark={handleToggleBookmark}
            onGenerateSummary={generateSummary}
            onTranslateAbstract={translateAbstract}
            onTranslateTitle={translateTitle}
          />
        ))}
      </div>

      {/* Pagination */}
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

      {/* Detail Modal */}
      {selectedRfc && (
        <RfcDetail rfc={selectedRfc} onClose={handleCloseDetail} />
      )}
    </div>
  );
}

