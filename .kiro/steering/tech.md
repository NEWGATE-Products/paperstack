# Technical Context

## 技術スタック

| 要素 | 技術 |
|------|------|
| フレームワーク | Tauri v2 |
| フロントエンド | React 18 + TypeScript |
| バックエンド | Rust |
| データベース | SQLite (rusqlite) |
| 状態管理 | React Context + hooks |
| スタイリング | CSS Modules or Tailwind CSS |

## 外部API

### Papers With Code API
- ベースURL: `https://paperswithcode.com/api/v1/`
- 認証: 不要
- エンドポイント:
  - `GET /papers/` - 論文一覧取得
  - `GET /papers/{id}/` - 論文詳細
  - `GET /tasks/` - タスク一覧

### Groq API
- ベースURL: `https://api.groq.com/openai/v1/`
- 認証: API Key (Bearer token)
- モデル: `llama-3.3-70b-versatile`
- 制限: 6,000リクエスト/日

## 開発ツール
- パッケージマネージャ: pnpm (フロントエンド), cargo (Rust)
- テスト: Vitest (フロントエンド), cargo test (Rust)
- フォーマッタ: Prettier, rustfmt

## 環境変数
```
GROQ_API_KEY=<your-groq-api-key>
```

