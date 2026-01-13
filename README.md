# Paperstack

AI・LLM・プログラミング関連の最新論文およびRFCを取得し、読みやすいUIで閲覧できるデスクトップアプリケーション。

## 機能

### 論文機能

- 📰 **ニュースサイト風UI** - 論文をニュース形式で閲覧
- 🔍 **カテゴリフィルタ** - AI全般、LLM、コード生成、アルゴリズム、アーキテクチャ
- 💡 **日本語要約** - Groq API (Llama 3.3 70B) で1〜2文の要約を自動生成
- 💾 **ローカルキャッシュ** - SQLiteで論文データを保存

### RFC Easy Reader機能

- 📖 **RFC閲覧** - IETF RFCを取得して閲覧
- 🎓 **難易度別要約** - Easy / Normal / Technical の3段階で要約を生成
- 🔧 **実装ガイド** - RFCの実装方法を日本語で解説
- 🌐 **日本語翻訳** - セクションごとの翻訳機能

## 技術スタック

| 要素 | 技術 |
|------|------|
| フレームワーク | Tauri v2 |
| フロントエンド | React 18 + TypeScript |
| バックエンド | Rust |
| データベース | SQLite |
| 外部API | Papers With Code API, IETF RFC API, Groq API |

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

または、アプリ内の設定画面からAPIキーを設定することもできます。

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

## 論文カテゴリ

| カテゴリ | タスク |
| --------- | -------- |
| AI全般 | machine-learning, deep-learning, reinforcement-learning |
| LLM | language-modelling, text-generation, question-answering |
| コード生成 | code-generation, program-synthesis |
| アルゴリズム | optimization, graph-neural-networks |
| アーキテクチャ | transformers, attention, neural-architecture-search |

## ライセンス

MIT
