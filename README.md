# Paperstack

AI・LLM・プログラミング関連の最新論文、IETF RFC、およびOSS脆弱性情報を統合的に閲覧できるデスクトップアプリケーション。

## 機能

### 論文リーダー

- **ニュースサイト風UI** - 論文をニュース形式で閲覧
- **カテゴリフィルタ** - AI全般、LLM、コード生成、アルゴリズム、アーキテクチャ
- **日本語要約** - Groq API (Llama 3.3 70B) で1〜2文の要約を自動生成
- **ローカルキャッシュ** - SQLiteで論文データを保存

### RFC Easy Reader

- **RFC閲覧** - IETF RFCを取得して閲覧
- **難易度別要約** - Easy / Normal / Technical の3段階で要約を生成
- **実装ガイド** - RFCの実装方法を日本語で解説
- **日本語翻訳** - セクションごとの翻訳機能
- **ブックマーク・履歴** - 閲覧履歴とブックマーク管理

### 脆弱性スキャナー

- **マルチエコシステム対応** - npm, Cargo, pip, Go, Maven, NuGet, RubyGems, Composer など12種類のパッケージマネージャをサポート
- **OSV データベース連携** - Google OSV APIを利用した最新の脆弱性情報取得
- **プロジェクトスキャン** - ローカルプロジェクトの依存関係をスキャンして脆弱性を検出
- **深刻度フィルタ** - Critical / High / Medium / Low で脆弱性をフィルタリング
- **詳細情報表示** - CVSSスコア、影響バージョン、修正バージョン、参考リンクを表示

## 技術スタック

| 要素 | 技術 |
|------|------|
| フレームワーク | Tauri v2 |
| フロントエンド | React 18 + TypeScript |
| バックエンド | Rust |
| データベース | SQLite |
| 外部API | Papers With Code API, IETF RFC API, OSV API, Groq API |

## セットアップ

### 必要条件

- Node.js 18+
- pnpm
- Rust (rustup)

### 環境変数

Groq APIキーを設定してください（論文要約・RFC解説機能に必要）:

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

## 対応エコシステム（脆弱性スキャン）

| パッケージマネージャ | 言語/プラットフォーム |
|---------------------|---------------------|
| npm | JavaScript/TypeScript |
| Cargo (crates.io) | Rust |
| pip (PyPI) | Python |
| Go Modules | Go |
| Maven/Gradle | Java |
| NuGet | .NET |
| RubyGems | Ruby |
| Composer (Packagist) | PHP |
| Pub | Dart/Flutter |
| Hex | Elixir |
| CocoaPods | iOS |
| SwiftPM | Swift |

## ライセンス

MIT
