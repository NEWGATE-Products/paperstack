import { useState, useCallback } from "react";
import type { VulnFilter as VulnFilterType } from "../../types/vuln";
import { ECOSYSTEMS, SEVERITIES } from "../../types/vuln";
import { IconRefresh, IconSearch } from "../icons";

interface VulnFilterProps {
  filter: VulnFilterType;
  onFilterChange: (filter: VulnFilterType) => void;
  onRefresh: (ecosystems: string[]) => Promise<number>;
  vulnCount: number;
  loading: boolean;
}

export function VulnFilter({
  filter,
  onFilterChange,
  onRefresh,
  vulnCount,
  loading,
}: VulnFilterProps) {
  const [searchInput, setSearchInput] = useState(filter.search || "");
  const [refreshing, setRefreshing] = useState(false);

  const handleEcosystemChange = useCallback(
    (e: React.ChangeEvent<HTMLSelectElement>) => {
      const value = e.target.value;
      onFilterChange({
        ...filter,
        ecosystem: value === "all" ? undefined : value,
      });
    },
    [filter, onFilterChange]
  );

  const handleSeverityChange = useCallback(
    (e: React.ChangeEvent<HTMLSelectElement>) => {
      const value = e.target.value;
      onFilterChange({
        ...filter,
        severity: value === "all" ? undefined : value,
      });
    },
    [filter, onFilterChange]
  );

  const handleSearchSubmit = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      onFilterChange({
        ...filter,
        search: searchInput.trim() || undefined,
      });
    },
    [filter, searchInput, onFilterChange]
  );

  const handleRefresh = useCallback(async () => {
    setRefreshing(true);
    try {
      // 選択されているエコシステムのみ、または全て
      const ecosystems = filter.ecosystem ? [filter.ecosystem] : [];
      await onRefresh(ecosystems);
    } finally {
      setRefreshing(false);
    }
  }, [filter.ecosystem, onRefresh]);

  const isLoading = loading || refreshing;

  return (
    <div className="vuln-filter">
      <div className="vuln-filter-row">
        <div className="vuln-filter-group">
          <label htmlFor="ecosystem-filter">エコシステム</label>
          <select
            id="ecosystem-filter"
            value={filter.ecosystem || "all"}
            onChange={handleEcosystemChange}
            disabled={isLoading}
          >
            <option value="all">すべて</option>
            {ECOSYSTEMS.map((eco) => (
              <option key={eco.id} value={eco.id}>
                {eco.label}
              </option>
            ))}
          </select>
        </div>

        <div className="vuln-filter-group">
          <label htmlFor="severity-filter">深刻度</label>
          <select
            id="severity-filter"
            value={filter.severity || "all"}
            onChange={handleSeverityChange}
            disabled={isLoading}
          >
            <option value="all">すべて</option>
            {SEVERITIES.map((sev) => (
              <option key={sev.id} value={sev.id}>
                {sev.label}
              </option>
            ))}
          </select>
        </div>

        <form onSubmit={handleSearchSubmit} className="vuln-filter-search">
          <input
            type="text"
            placeholder="CVE ID、パッケージ名で検索..."
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            disabled={isLoading}
          />
          <button type="submit" disabled={isLoading} className="search-btn">
            <IconSearch size={16} />
          </button>
        </form>

        <button
          type="button"
          onClick={handleRefresh}
          disabled={isLoading}
          className="refresh-btn"
          title="最新の脆弱性情報を取得"
        >
          <IconRefresh size={16} className={isLoading ? "spinning" : ""} />
          更新
        </button>
      </div>

      <div className="vuln-filter-info">
        <span className="vuln-count">
          {vulnCount.toLocaleString()} 件の脆弱性
        </span>
      </div>
    </div>
  );
}
