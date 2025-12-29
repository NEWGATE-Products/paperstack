# 実装タスク: AI論文ニュースリーダー

## Phase 1: プロジェクト初期化

- [x] **1.1** Tauri v2 + React + TypeScriptプロジェクトの作成
- [x] **1.2** 依存関係の追加（Rust: rusqlite, reqwest, serde / React: 必要なライブラリ）

## Phase 2: バックエンド - データベース

- [x] **2.1** SQLiteデータベース初期化とマイグレーション
- [x] **2.2** papers テーブルのCRUD操作
- [x] **2.3** paper_tasks テーブルのCRUD操作

## Phase 3: バックエンド - 外部API

- [x] **3.1** Papers With Code API クライアント実装
- [x] **3.2** Groq API クライアント実装（要約生成）

## Phase 4: バックエンド - Tauriコマンド

- [x] **4.1** get_papers コマンド（DB から論文取得）
- [x] **4.2** fetch_papers コマンド（APIから論文取得・保存）
- [x] **4.3** generate_summary コマンド（要約生成・保存）
- [x] **4.4** get_categories コマンド（カテゴリ一覧）

## Phase 5: フロントエンド - 基盤

- [x] **5.1** TypeScript型定義
- [x] **5.2** Tauri IPC フック（usePapers, useCategories）
- [x] **5.3** 基本レイアウト・スタイル設定

## Phase 6: フロントエンド - UIコンポーネント

- [x] **6.1** Header コンポーネント
- [x] **6.2** CategoryTabs コンポーネント
- [x] **6.3** PaperCard コンポーネント
- [x] **6.4** PaperList コンポーネント
- [x] **6.5** App.tsx 統合

## Phase 7: 統合・仕上げ

- [x] **7.1** エラーハンドリング
- [x] **7.2** ローディング状態
- [x] **7.3** 最終テスト・調整
