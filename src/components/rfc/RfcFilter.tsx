import { useState, useCallback } from "react";
import type { RfcFilter as RfcFilterType } from "../../types/rfc";
import { RFC_CATEGORIES, RFC_STATUSES } from "../../types/rfc";

interface RfcFilterProps {
  filter: RfcFilterType;
  onFilterChange: (filter: RfcFilterType) => void;
  onRefresh: () => Promise<number>;
  rfcCount: number;
  loading: boolean;
}

export function RfcFilter({
  filter,
  onFilterChange,
  onRefresh,
  rfcCount,
  loading,
}: RfcFilterProps) {
  const [searchInput, setSearchInput] = useState(filter.search || "");
  const [rfcNumberInput, setRfcNumberInput] = useState(
    filter.rfcNumber?.toString() || ""
  );
  const [isRefreshing, setIsRefreshing] = useState(false);

  const handleSearchSubmit = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      onFilterChange({
        ...filter,
        search: searchInput || undefined,
      });
    },
    [filter, searchInput, onFilterChange]
  );

  const handleRfcNumberSubmit = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      const num = parseInt(rfcNumberInput, 10);
      onFilterChange({
        ...filter,
        rfcNumber: isNaN(num) ? undefined : num,
      });
    },
    [filter, rfcNumberInput, onFilterChange]
  );

  const handleCategoryChange = useCallback(
    (categoryId: string) => {
      const currentCategories = filter.categories || [];
      const newCategories = currentCategories.includes(categoryId)
        ? currentCategories.filter((c) => c !== categoryId)
        : [...currentCategories, categoryId];

      onFilterChange({
        ...filter,
        categories: newCategories.length > 0 ? newCategories : undefined,
      });
    },
    [filter, onFilterChange]
  );

  const handleStatusChange = useCallback(
    (status: string) => {
      const currentStatuses = filter.status || [];
      const newStatuses = currentStatuses.includes(status)
        ? currentStatuses.filter((s) => s !== status)
        : [...currentStatuses, status];

      onFilterChange({
        ...filter,
        status: newStatuses.length > 0 ? newStatuses : undefined,
      });
    },
    [filter, onFilterChange]
  );

  const handleYearFromChange = useCallback(
    (e: React.ChangeEvent<HTMLSelectElement>) => {
      const year = parseInt(e.target.value, 10);
      onFilterChange({
        ...filter,
        yearFrom: isNaN(year) ? undefined : year,
      });
    },
    [filter, onFilterChange]
  );

  const handleYearToChange = useCallback(
    (e: React.ChangeEvent<HTMLSelectElement>) => {
      const year = parseInt(e.target.value, 10);
      onFilterChange({
        ...filter,
        yearTo: isNaN(year) ? undefined : year,
      });
    },
    [filter, onFilterChange]
  );

  const handleClearFilters = useCallback(() => {
    setSearchInput("");
    setRfcNumberInput("");
    onFilterChange({});
  }, [onFilterChange]);

  const handleRefresh = useCallback(async () => {
    setIsRefreshing(true);
    try {
      const count = await onRefresh();
      alert(`${count.toLocaleString()} 件のRFCを取得しました`);
    } catch (e) {
      console.error("Failed to refresh:", e);
      alert("RFCの取得に失敗しました");
    } finally {
      setIsRefreshing(false);
    }
  }, [onRefresh]);

  // Generate year options (1969 to current year)
  const currentYear = new Date().getFullYear();
  const years = Array.from({ length: currentYear - 1968 }, (_, i) => currentYear - i);

  const hasActiveFilters =
    filter.search ||
    filter.rfcNumber ||
    (filter.categories && filter.categories.length > 0) ||
    (filter.status && filter.status.length > 0) ||
    filter.yearFrom ||
    filter.yearTo;

  return (
    <div className="rfc-filter">
      {/* Search Row */}
      <div className="filter-row search-row">
        <form onSubmit={handleRfcNumberSubmit} className="rfc-number-search">
          <input
            type="text"
            placeholder="RFC番号"
            value={rfcNumberInput}
            onChange={(e) => setRfcNumberInput(e.target.value)}
            className="filter-input small"
          />
          <button type="submit" className="search-btn">
            🔍
          </button>
        </form>

        <form onSubmit={handleSearchSubmit} className="keyword-search">
          <input
            type="text"
            placeholder="キーワード検索（タイトル・概要）"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            className="filter-input"
          />
          <button type="submit" className="search-btn">
            🔍
          </button>
        </form>

        <button
          type="button"
          className="refresh-btn"
          onClick={handleRefresh}
          disabled={isRefreshing || loading}
          title="RFC一覧を更新"
        >
          {isRefreshing ? "更新中..." : "🔄 更新"}
        </button>
      </div>

      {/* Category Filters */}
      <div className="filter-row category-row">
        <span className="filter-label">カテゴリ:</span>
        <div className="filter-chips">
          {RFC_CATEGORIES.filter((c) => c.id !== "other").map((cat) => (
            <button
              type="button"
              key={cat.id}
              className={`filter-chip ${
                filter.categories?.includes(cat.id) ? "active" : ""
              }`}
              onClick={() => handleCategoryChange(cat.id)}
            >
              {cat.name}
            </button>
          ))}
        </div>
      </div>

      {/* Status Filters */}
      <div className="filter-row status-row">
        <span className="filter-label">ステータス:</span>
        <div className="filter-chips">
          {RFC_STATUSES.map((status) => (
            <button
              type="button"
              key={status.value}
              className={`filter-chip ${
                filter.status?.includes(status.value) ? "active" : ""
              }`}
              onClick={() => handleStatusChange(status.value)}
            >
              {status.label}
            </button>
          ))}
        </div>
      </div>

      {/* Year Range */}
      <div className="filter-row year-row">
        <span className="filter-label">公開年:</span>
        <select
          value={filter.yearFrom || ""}
          onChange={handleYearFromChange}
          className="year-select"
        >
          <option value="">開始年</option>
          {years.map((year) => (
            <option key={year} value={year}>
              {year}
            </option>
          ))}
        </select>
        <span className="year-separator">〜</span>
        <select
          value={filter.yearTo || ""}
          onChange={handleYearToChange}
          className="year-select"
        >
          <option value="">終了年</option>
          {years.map((year) => (
            <option key={year} value={year}>
              {year}
            </option>
          ))}
        </select>

        {hasActiveFilters && (
          <button type="button" className="clear-btn" onClick={handleClearFilters}>
            ✕ フィルタをクリア
          </button>
        )}
      </div>

      {/* Results Count */}
      <div className="filter-results">
        <span>
          {loading ? "読み込み中..." : `${rfcCount.toLocaleString()} 件のRFC`}
        </span>
      </div>
    </div>
  );
}

