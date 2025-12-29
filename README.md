# AI Paper News 📚

AI・LLM・プログラミング関連の最新論文を毎日取得し、Newsサイト風のUIで閲覧できるデスクトップアプリケーション。

## 機能

- 📰 **ニュースサイト風UI** - 論文をニュース形式で閲覧
- 🔍 **カテゴリフィルタ** - AI全般、LLM、コード生成、アルゴリズム、アーキテクチャ
- 💡 **日本語要約** - Groq API (Llama 3.3 70B) で1〜2文の要約を自動生成
- 💾 **ローカルキャッシュ** - SQLiteで論文データを保存

## 技術スタック

| 要素 | 技術 |
|------|------|
| フレームワーク | Tauri v2 |
| フロントエンド | React 18 + TypeScript |
| バックエンド | Rust |
| データベース | SQLite |
| 外部API | Papers With Code API, Groq API |

## セットアップ

### 必要条件

- Node.js 18+
- pnpm
- Rust (rustup)

### 環境変数

Groq APIキーを設定してください:

```bash
export GROQ_API_KEY="your-api-key-here"
```

[Groq Console](https://console.groq.com/) でAPIキーを取得できます。

### インストール

```bash
# 依存関係のインストール
pnpm install

# 開発サーバー起動
pnpm tauri dev
```

### ビルド

```bash
pnpm tauri build
```

## カテゴリ

| カテゴリ | タスク |
|---------|--------|
| AI全般 | machine-learning, deep-learning, reinforcement-learning |
| LLM | language-modelling, text-generation, question-answering |
| コード生成 | code-generation, program-synthesis |
| アルゴリズム | optimization, graph-neural-networks |
| アーキテクチャ | transformers, attention, neural-architecture-search |

## ライセンス

MIT

