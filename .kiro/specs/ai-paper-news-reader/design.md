# 設計: AI論文ニュースリーダー

## アーキテクチャ概要

```
┌─────────────────────────────────────────────────────────┐
│                      Tauri App                          │
├─────────────────────────────────────────────────────────┤
│  Frontend (React + TypeScript)                          │
│  ┌───────────┐ ┌───────────┐ ┌───────────┐            │
│  │    AI     │ │    LLM    │ │   Code    │ ← カテゴリ   │
│  └───────────┘ └───────────┘ └───────────┘            │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 論文カード                                       │   │
│  │ - タイトル / 日本語要約 / タグ / リンク           │   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│  Backend (Rust)                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ 論文取得      │  │ 要約生成      │  │ DB管理       │ │
│  │ (PWC API)    │  │ (Groq API)   │  │ (SQLite)    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## データベース設計

### papers テーブル
```sql
CREATE TABLE papers (
    id TEXT PRIMARY KEY,           -- Papers With Code ID
    title TEXT NOT NULL,           -- 論文タイトル
    abstract TEXT,                 -- アブストラクト
    summary_ja TEXT,               -- 日本語要約
    url_pdf TEXT,                  -- PDFリンク
    url_paper TEXT,                -- 論文ページURL
    published DATE,                -- 公開日
    fetched_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### paper_tasks テーブル
```sql
CREATE TABLE paper_tasks (
    paper_id TEXT NOT NULL,        -- 論文ID (FK)
    task_slug TEXT NOT NULL,       -- タスクスラッグ
    category TEXT,                 -- カテゴリ名
    PRIMARY KEY (paper_id, task_slug),
    FOREIGN KEY (paper_id) REFERENCES papers(id)
);
```

## APIインターフェース

### Tauri Commands (IPC)

```typescript
// 論文一覧取得
invoke<Paper[]>('get_papers', { category?: string, limit?: number })

// 論文取得（API経由で最新を取得）
invoke<Paper[]>('fetch_papers', { tasks: string[] })

// 要約生成
invoke<string>('generate_summary', { paperId: string })

// カテゴリ一覧取得
invoke<Category[]>('get_categories')
```

### 型定義

```typescript
interface Paper {
  id: string;
  title: string;
  abstract: string | null;
  summaryJa: string | null;
  urlPdf: string | null;
  urlPaper: string | null;
  published: string | null;
  tasks: string[];
}

interface Category {
  id: string;
  name: string;
  tasks: string[];
}
```

## コンポーネント設計

### フロントエンド

1. **App.tsx** - メインレイアウト
2. **Header.tsx** - アプリヘッダー
3. **CategoryTabs.tsx** - カテゴリ切り替えタブ
4. **PaperList.tsx** - 論文一覧コンテナ
5. **PaperCard.tsx** - 論文カード

### バックエンド (Rust)

1. **db/mod.rs** - DB接続・マイグレーション
2. **db/models.rs** - データモデル
3. **api/pwc.rs** - Papers With Code API クライアント
4. **api/groq.rs** - Groq API クライアント
5. **commands/mod.rs** - Tauri コマンド

