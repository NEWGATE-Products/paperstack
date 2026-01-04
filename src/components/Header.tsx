import { IconBooks, IconRefresh, IconSettings } from "./icons";

interface HeaderProps {
  onRefresh?: () => void;
  onOpenSettings: () => void;
  isLoading: boolean;
  hasApiKey: boolean;
}

export function Header({ onRefresh, onOpenSettings, isLoading, hasApiKey }: HeaderProps) {
  return (
    <header className="header">
      <div className="header-content">
        <div>
          <h1 className="header-title">
            <span className="header-title-icon"><IconBooks size={28} /></span>
            Paperstack
          </h1>
          <p className="header-subtitle">
            AI・LLM・プログラミング関連の論文・RFCを整理して読む
          </p>
        </div>
        <div className="header-actions">
          {onRefresh && (
            <button
              className="btn btn-primary"
              onClick={onRefresh}
              disabled={isLoading}
            >
              {isLoading ? (
                <>
                  <span className="loading-spinner" style={{ width: 16, height: 16, borderWidth: 2 }} />
                  取得中...
                </>
              ) : (
                <>
                  <IconRefresh size={16} /> 最新を取得
                </>
              )}
            </button>
          )}
          <button
            className="btn btn-primary"
            onClick={onOpenSettings}
            title={hasApiKey ? "APIキー設定済み" : "APIキー未設定"}
          >
            <IconSettings size={18} /> {!hasApiKey && <span style={{ color: "#fca5a5" }}>!</span>}
          </button>
        </div>
      </div>
    </header>
  );
}
