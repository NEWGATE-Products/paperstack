# 実装タスク: RFCかんたんリーダー

## 概要
既存のAI論文ニュースリーダーにRFC閲覧機能を追加する。
論文タブとRFCタブを切り替え可能にし、かんたんモード・AI要約・実装ガイドを提供する。

---

## Phase 1: データベース拡張

- [x] **1.1** rfcsテーブルのマイグレーション追加
  - `db/mod.rs` にCREATE TABLE rfcs文を追加
  - インデックス作成（number, status, published_date）
  
- [x] **1.2** rfc_categoriesテーブルの作成
  - RFCとカテゴリの多対多関係を管理
  
- [x] **1.3** rfc_bookmarksテーブルの作成
  - ブックマーク + メモ機能用
  
- [x] **1.4** rfc_historyテーブルの作成
  - 閲覧履歴管理用

- [x] **1.5** Rfc構造体の定義（`db/models.rs`）
  - 設計書の型定義に従い実装
  - RfcFilter, RfcBookmark, RfcHistory 構造体も追加

---

## Phase 2: バックエンド - RFC Editor APIクライアント

- [x] **2.1** `api/rfc_editor.rs` 新規作成
  - RFC Editor Index XML取得機能
  - HTTPクライアント（reqwest）でXMLダウンロード
  
- [x] **2.2** XMLパーサー実装
  - quick-xml クレートを使用
  - `<rfc-entry>` を Rfc 構造体に変換
  - doc-id, title, abstract, status, date, authors, keywords をパース
  
- [x] **2.3** カテゴリ自動分類ロジック
  - キーワードベースでHTTP/DNS/セキュリティ等に分類
  
- [x] **2.4** RFC本文取得機能
  - `rfc-editor.org/rfc/rfcXXXX.txt` からテキスト取得
  - 翻訳・要約用

---

## Phase 3: バックエンド - Groq API拡張

- [x] **3.1** RFC要約メソッド追加（`api/groq.rs`）
  - `generate_rfc_summary_easy()` - 小学生向け
  - `generate_rfc_summary_normal()` - 一般向け
  - `generate_rfc_summary_technical()` - 技術者向け
  
- [x] **3.2** 実装ガイド生成メソッド追加
  - `generate_implementation_guide()` - 実装可否・推奨スタック等
  
- [x] **3.3** RFC翻訳メソッド追加
  - `translate_rfc_section()` - セクション単位翻訳

---

## Phase 4: バックエンド - DBクエリ

- [x] **4.1** `db/rfc_queries.rs` 新規作成
  - `upsert_rfc()` - RFC保存（UPSERT）
  - `get_rfcs()` - フィルタリング付き一覧取得
  - `get_rfc_by_id()` - 単一RFC取得
  
- [x] **4.2** 要約・翻訳キャッシュ更新
  - `update_rfc_summary()` - 各レベルの要約保存
  - `update_rfc_implementation_guide()` - 実装ガイド保存
  - `update_rfc_translation()` - 翻訳保存
  
- [x] **4.3** ブックマーク操作
  - `add_bookmark()`, `remove_bookmark()`, `get_bookmarks()`
  - `update_bookmark_memo()` - メモ更新
  
- [x] **4.4** 履歴操作
  - `add_history()`, `get_history()`
  
- [x] **4.5** カテゴリ操作
  - `get_all_categories()` - 使用中のカテゴリ一覧
  - `insert_rfc_category()` - カテゴリ紐付け

---

## Phase 5: バックエンド - Tauriコマンド

- [x] **5.1** `commands/rfc_commands.rs` 新規作成
  - エラー型定義（RfcCommandError）
  
- [x] **5.2** RFC一覧コマンド
  - `get_rfcs` - フィルタリング・ページネーション対応
  - `fetch_rfcs` - RFC Editor XMLから取得・保存
  
- [x] **5.3** RFC詳細コマンド
  - `get_rfc_by_id` - 単一RFC取得
  - `get_rfc_content` - RFC本文取得
  
- [x] **5.4** AI要約コマンド
  - `generate_rfc_summary` - レベル指定で要約生成
  - `generate_implementation_guide` - 実装ガイド生成
  
- [x] **5.5** 翻訳コマンド
  - `translate_rfc_section` - セクション翻訳
  
- [x] **5.6** ブックマーク・履歴コマンド
  - `add_rfc_bookmark`, `remove_rfc_bookmark`, `get_rfc_bookmarks`
  - `add_rfc_history`, `get_rfc_history`
  
- [x] **5.7** コマンド登録
  - `lib.rs` または `main.rs` に新コマンドを登録

---

## Phase 6: フロントエンド - 型定義・フック

- [x] **6.1** `types/rfc.ts` 新規作成
  - Rfc, RfcFilter, RfcStatus, RfcListResponse 型定義
  - RfcBookmark, RfcHistory, SummaryLevel 型定義
  
- [x] **6.2** `hooks/useRfcs.ts` 新規作成
  - RFC一覧取得・フィルタリング
  - ページネーション状態管理
  
- [x] **6.3** `hooks/useRfcDetail.ts` 新規作成
  - RFC詳細・本文取得
  - 要約・翻訳生成トリガー
  
- [x] **6.4** `hooks/useRfcBookmarks.ts` 新規作成
  - ブックマーク追加・削除・一覧
  
- [x] **6.5** `hooks/useRfcHistory.ts` 新規作成
  - 閲覧履歴取得

---

## Phase 7: フロントエンド - 共通コンポーネント

- [x] **7.1** `components/navigation/TabNav.tsx` 新規作成
  - 論文/RFC タブ切り替えUI（App.tsxに統合）
  - アクティブ状態のスタイリング
  
- [x] **7.2** `components/common/Tooltip.tsx` 新規作成
  - 専門用語ツールチップ表示（RfcCard内に統合）

---

## Phase 8: フロントエンド - RFCコンポーネント

- [x] **8.1** `components/rfc/RfcFilter.tsx` 新規作成
  - RFC番号検索
  - キーワード検索
  - カテゴリ・ステータス・年のドロップダウン
  
- [x] **8.2** `components/rfc/RfcStatusBadge.tsx` 新規作成
  - ステータス別の色分けバッジ
  
- [x] **8.3** `components/rfc/RfcCard.tsx` 新規作成
  - RFC番号・タイトル・概要表示
  - かんたんモード「ひとことまとめ」表示
  - ブックマーク・要約生成ボタン
  
- [x] **8.4** `components/rfc/RfcList.tsx` 新規作成
  - RfcCard一覧表示
  - ページネーション/無限スクロール
  - ローディング・空状態のUI
  
- [x] **8.5** `components/rfc/RfcBookmarkButton.tsx` 新規作成
  - ブックマーク追加/削除トグル（RfcCard内に統合）
  - メモ入力ダイアログ
  
- [x] **8.6** `components/rfc/RfcSummaryBadge.tsx` 新規作成
  - かんたん/一般/技術者 タブ切り替え（RfcCard内に統合）
  - 要約生成トリガーボタン

---

## Phase 9: フロントエンド - RFC詳細・実装ガイド

- [x] **9.1** `components/rfc/RfcDetail.tsx` 新規作成
  - RFC詳細表示（タイトル、概要、メタ情報）
  - 要約レベル切り替えタブ
  - 翻訳表示（原文・訳文並列）
  
- [x] **9.2** `components/rfc/RfcImplementationGuide.tsx` 新規作成
  - 実装ガイド表示（RfcDetail内に統合）
  - 生成ボタン・ローディング状態
  - 実装可否の判定表示
  
- [x] **9.3** RFC詳細モーダルまたはページ
  - 詳細表示の統合
  - 閲覧履歴への自動追加

---

## Phase 10: フロントエンド - スタイリング

- [x] **10.1** かんたんモードスタイル実装
  - 大きめ文字サイズ（18-20px）
  - 温かみのある配色（クリーム背景、オレンジアクセント）
  - アイコン・絵文字の活用
  
- [x] **10.2** RFCカード・リストのスタイル
  - 概要の省略表示（2-3行）
  - ステータスバッジのカラーリング
  
- [x] **10.3** 実装ガイドのMarkdownスタイル
  - コードブロック、リスト、見出しのスタイリング
  - 注意点アイコン（⚠️）の表示

---

## Phase 11: 統合

- [x] **11.1** App.tsx 更新
  - TabNav統合
  - RFC/論文の切り替えロジック
  
- [x] **11.2** ルーティング（必要に応じて）
  - RFC詳細ページへの遷移（モーダルで実装）
  
- [x] **11.3** グローバル状態の調整
  - アクティブタブ状態の管理

---

## Phase 12: テスト・品質

- [x] **12.1** バックエンドユニットテスト
  - XMLパーサーテスト
  - DBクエリテスト
  
- [x] **12.2** E2Eテスト
  - RFC一覧表示
  - フィルタリング動作
  - 要約生成フロー
  
- [x] **12.3** パフォーマンス確認
  - 初期表示2秒以内
  - キャッシュ表示500ms以内

---

## Phase 13: 仕上げ

- [x] **13.1** エラーハンドリング
  - ネットワークエラー
  - API制限エラー
  - パースエラー
  
- [x] **13.2** ローディング状態
  - スケルトンUI
  - プログレス表示
  
- [x] **13.3** 空状態・エラー状態UI
  - 検索結果なし
  - データ取得失敗
  
- [x] **13.4** 最終調整・リファクタリング

---

## 依存関係

```
Phase 1 (DB) 
    ↓
Phase 2 (RFC Editor API) → Phase 3 (Groq拡張)
    ↓                           ↓
Phase 4 (DBクエリ) ←────────────┘
    ↓
Phase 5 (Tauriコマンド)
    ↓
Phase 6 (型定義・フック)
    ↓
Phase 7 (共通コンポーネント) → Phase 8 (RFCコンポーネント)
                                    ↓
                              Phase 9 (詳細・実装ガイド)
                                    ↓
                              Phase 10 (スタイリング)
                                    ↓
                              Phase 11 (統合)
                                    ↓
                              Phase 12-13 (テスト・仕上げ)
```

---

## Cargo.toml 追加依存関係

```toml
[dependencies]
quick-xml = "0.31"  # XMLパース用
```

---

## ステータス
- [x] タスク定義ドラフト作成
- [x] レビュー
- [x] タスク承認

