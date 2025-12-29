import type { Paper, LoadingState } from "../types";
import { PaperCard } from "./PaperCard";

interface PaperListProps {
  papers: Paper[];
  loading: LoadingState;
  error: string | null;
  onGenerateSummary: (paperId: string) => Promise<string>;
  onRefresh: () => void;
}

export function PaperList({
  papers,
  loading,
  error,
  onGenerateSummary,
  onRefresh,
}: PaperListProps) {
  if (loading === "loading") {
    return (
      <div className="loading">
        <div className="loading-spinner" />
        <p>論文を読み込み中...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="error-state">
        <p>❌ エラーが発生しました: {error}</p>
        <button className="btn btn-primary" onClick={onRefresh} style={{ marginTop: 16 }}>
          🔄 再試行
        </button>
      </div>
    );
  }

  if (papers.length === 0) {
    return (
      <div className="empty-state">
        <div className="empty-state-icon">📭</div>
        <h3 className="empty-state-title">論文がありません</h3>
        <p className="empty-state-description">
          「最新を取得」ボタンを押して、最新の論文を取得してください。
        </p>
        <button className="btn btn-primary" onClick={onRefresh}>
          🔄 最新を取得
        </button>
      </div>
    );
  }

  return (
    <div className="paper-list">
      {papers.map((paper) => (
        <PaperCard
          key={paper.id}
          paper={paper}
          onGenerateSummary={onGenerateSummary}
        />
      ))}
    </div>
  );
}

