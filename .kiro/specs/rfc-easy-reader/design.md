# 設計: RFCかんたんリーダー

## アーキテクチャ概要

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Tauri App                                    │
├─────────────────────────────────────────────────────────────────────┤
│  Frontend (React + TypeScript)                                       │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ ナビゲーション: [論文] [RFC]                                   │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ RFC検索・フィルター                                             ││
│  │ [RFC番号] [キーワード] [カテゴリ▼] [ステータス▼] [年▼]        ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ RFCカード 🎯 かんたんモード                                     ││
│  │ ┌────────────────────────────────────────────────────────────┐││
│  │ │ 📄 RFC 9114 - HTTP/3                                       │││
│  │ │ ────────────────────────────────────────────────────────── │││
│  │ │ 🎈 ひとことまとめ:                                          │││
│  │ │ 「インターネットをもっと早くするための新しいルールだよ！」   │││
│  │ │ ────────────────────────────────────────────────────────── │││
│  │ │ 📝 概要: HTTP/3 is the third version of the Hypertext...   │││
│  │ │ 🏷️ [Internet Standard] [HTTP] [2022]                       │││
│  │ │ [📖 詳細を見る] [⭐ ブックマーク] [🔄 要約生成]              │││
│  │ └────────────────────────────────────────────────────────────┘││
│  └────────────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────────────┤
│  Backend (Rust)                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ RFC取得       │  │ AI要約/翻訳  │  │ DB管理       │              │
│  │ (RFC Editor) │  │ (Groq API)   │  │ (SQLite)    │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## データベース設計

### rfcs テーブル
```sql
CREATE TABLE IF NOT EXISTS rfcs (
    id TEXT PRIMARY KEY,              -- RFC番号 (例: "RFC9114")
    number INTEGER NOT NULL UNIQUE,   -- RFC番号 (数値)
    title TEXT NOT NULL,              -- タイトル
    abstract TEXT,                    -- 概要 (Abstract)
    status TEXT,                      -- ステータス (PROPOSED STANDARD等)
    published_date TEXT,              -- 公開日 (YYYY-MM)
    authors TEXT,                     -- 著者 (JSON配列)
    keywords TEXT,                    -- キーワード (JSON配列)
    
    -- AI生成コンテンツ (キャッシュ)
    summary_easy TEXT,                -- 小学生向け要約
    summary_normal TEXT,              -- 一般向け要約
    summary_technical TEXT,           -- 技術者向け要約
    implementation_guide TEXT,        -- 実装ガイド (技術者向け)
    title_ja TEXT,                    -- タイトル日本語訳
    abstract_ja TEXT,                 -- 概要日本語訳
    
    -- メタデータ
    fetched_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_rfcs_number ON rfcs(number);
CREATE INDEX idx_rfcs_status ON rfcs(status);
CREATE INDEX idx_rfcs_published ON rfcs(published_date);
```

### rfc_categories テーブル
```sql
CREATE TABLE IF NOT EXISTS rfc_categories (
    rfc_id TEXT NOT NULL,
    category TEXT NOT NULL,           -- カテゴリ (HTTP, DNS, Security等)
    PRIMARY KEY (rfc_id, category),
    FOREIGN KEY (rfc_id) REFERENCES rfcs(id)
);

CREATE INDEX idx_rfc_categories_category ON rfc_categories(category);
```

### rfc_bookmarks テーブル
```sql
CREATE TABLE IF NOT EXISTS rfc_bookmarks (
    rfc_id TEXT PRIMARY KEY,
    memo TEXT,                        -- ユーザーメモ
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (rfc_id) REFERENCES rfcs(id)
);
```

### rfc_history テーブル
```sql
CREATE TABLE IF NOT EXISTS rfc_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rfc_id TEXT NOT NULL,
    viewed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (rfc_id) REFERENCES rfcs(id)
);

CREATE INDEX idx_rfc_history_viewed ON rfc_history(viewed_at DESC);
```

---

## APIインターフェース

### Tauri Commands (IPC)

```typescript
// RFC一覧取得 (フィルタリング対応)
invoke<RfcListResponse>('get_rfcs', {
  filter?: RfcFilter,
  page?: number,
  limit?: number
})

// RFCデータ取得 (RFC Editor XMLから)
invoke<void>('fetch_rfcs')

// RFC詳細取得
invoke<Rfc>('get_rfc_by_id', { rfcId: string })

// RFC本文取得 (翻訳用)
invoke<string>('get_rfc_content', { rfcNumber: number })

// AI要約生成
invoke<string>('generate_rfc_summary', {
  rfcId: string,
  level: 'easy' | 'normal' | 'technical'
})

// 実装ガイド生成 (技術者向け)
invoke<string>('generate_implementation_guide', {
  rfcId: string
})

// 翻訳生成
invoke<string>('translate_rfc_section', {
  rfcId: string,
  section: string,
  text: string
})

// ブックマーク操作
invoke<void>('add_rfc_bookmark', { rfcId: string, memo?: string })
invoke<void>('remove_rfc_bookmark', { rfcId: string })
invoke<RfcBookmark[]>('get_rfc_bookmarks')

// 履歴操作
invoke<void>('add_rfc_history', { rfcId: string })
invoke<RfcHistory[]>('get_rfc_history', { limit?: number })

// カテゴリ一覧取得
invoke<string[]>('get_rfc_categories')
```

---

## 型定義

### Frontend (TypeScript)

```typescript
// RFC基本情報
interface Rfc {
  id: string;                    // "RFC9114"
  number: number;                // 9114
  title: string;
  abstract: string | null;
  status: RfcStatus;
  publishedDate: string | null;  // "2022-06"
  authors: string[];
  keywords: string[];
  categories: string[];
  
  // AI生成コンテンツ
  summaryEasy: string | null;
  summaryNormal: string | null;
  summaryTechnical: string | null;
  implementationGuide: string | null;  // 実装ガイド (技術者向け)
  titleJa: string | null;
  abstractJa: string | null;
  
  // UI状態
  isBookmarked: boolean;
}

// RFCステータス
type RfcStatus = 
  | 'INTERNET STANDARD'
  | 'PROPOSED STANDARD'
  | 'DRAFT STANDARD'
  | 'BEST CURRENT PRACTICE'
  | 'INFORMATIONAL'
  | 'EXPERIMENTAL'
  | 'HISTORIC'
  | 'UNKNOWN';

// フィルター条件
interface RfcFilter {
  search?: string;               // キーワード検索
  rfcNumber?: number;            // RFC番号
  status?: RfcStatus[];          // ステータス
  categories?: string[];         // カテゴリ
  yearFrom?: number;             // 公開年（開始）
  yearTo?: number;               // 公開年（終了）
}

// 一覧レスポンス
interface RfcListResponse {
  rfcs: Rfc[];
  total: number;
  page: number;
  limit: number;
}

// ブックマーク
interface RfcBookmark {
  rfcId: string;
  memo: string | null;
  createdAt: string;
}

// 閲覧履歴
interface RfcHistory {
  rfcId: string;
  viewedAt: string;
}

// 要約レベル
type SummaryLevel = 'easy' | 'normal' | 'technical';
```

### Backend (Rust)

```rust
// src/db/models.rs に追加

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rfc {
    pub id: String,
    pub number: i32,
    pub title: String,
    #[serde(rename = "abstract")]
    pub r#abstract: Option<String>,
    pub status: String,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,
    pub authors: Vec<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    
    #[serde(rename = "summaryEasy")]
    pub summary_easy: Option<String>,
    #[serde(rename = "summaryNormal")]
    pub summary_normal: Option<String>,
    #[serde(rename = "summaryTechnical")]
    pub summary_technical: Option<String>,
    #[serde(rename = "implementationGuide")]
    pub implementation_guide: Option<String>,  // 実装ガイド (技術者向け)
    #[serde(rename = "titleJa")]
    pub title_ja: Option<String>,
    #[serde(rename = "abstractJa")]
    pub abstract_ja: Option<String>,
    
    #[serde(rename = "isBookmarked")]
    pub is_bookmarked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfcFilter {
    pub search: Option<String>,
    pub rfc_number: Option<i32>,
    pub status: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub year_from: Option<i32>,
    pub year_to: Option<i32>,
}
```

---

## コンポーネント設計

### フロントエンド (React)

```
src/
├── components/
│   ├── rfc/
│   │   ├── RfcList.tsx              # RFC一覧コンテナ
│   │   ├── RfcCard.tsx              # RFCカード (かんたんモード対応)
│   │   ├── RfcFilter.tsx            # フィルター・検索UI
│   │   ├── RfcDetail.tsx            # RFC詳細表示 (翻訳・対訳)
│   │   ├── RfcSummaryBadge.tsx      # 要約レベル選択UI
│   │   ├── RfcBookmarkButton.tsx    # ブックマークボタン
│   │   ├── RfcStatusBadge.tsx       # ステータスバッジ
│   │   └── RfcImplementationGuide.tsx # 実装ガイド表示 (技術者向け)
│   ├── common/
│   │   └── Tooltip.tsx           # 専門用語ツールチップ
│   └── navigation/
│       └── TabNav.tsx            # 論文/RFC切り替えナビ
├── hooks/
│   ├── useRfcs.ts                # RFC一覧フック
│   ├── useRfcDetail.ts           # RFC詳細フック
│   ├── useRfcBookmarks.ts        # ブックマークフック
│   └── useRfcHistory.ts          # 履歴フック
└── types/
    └── rfc.ts                    # RFC関連型定義
```

### バックエンド (Rust)

```
src-tauri/src/
├── api/
│   ├── mod.rs
│   ├── rfc_editor.rs             # RFC Editor XMLクライアント [NEW]
│   └── groq.rs                   # 要約・翻訳メソッド追加
├── commands/
│   ├── mod.rs
│   └── rfc_commands.rs           # RFCコマンド [NEW]
└── db/
    ├── mod.rs                    # マイグレーション追加
    ├── models.rs                 # Rfc構造体追加
    └── rfc_queries.rs            # RFCクエリ [NEW]
```

---

## RFC Editor XMLパーサー設計

### XMLフォーマット例
```xml
<rfc-index>
  <rfc-entry>
    <doc-id>RFC9114</doc-id>
    <title>HTTP/3</title>
    <author>
      <name>M. Bishop</name>
    </author>
    <date>
      <month>June</month>
      <year>2022</year>
    </date>
    <abstract>
      <p>HTTP/3 is the third version of the Hypertext Transfer Protocol...</p>
    </abstract>
    <current-status>PROPOSED STANDARD</current-status>
    <keywords>
      <kw>HTTP</kw>
      <kw>QUIC</kw>
    </keywords>
  </rfc-entry>
</rfc-index>
```

### パース処理フロー
```
1. RFC Editor Index XMLをダウンロード
   GET https://www.rfc-editor.org/rfc-index.xml
   
2. XMLをパース (quick-xml クレート使用)
   
3. 各 <rfc-entry> を Rfc 構造体に変換
   - doc-id → id, number
   - title → title
   - abstract/p → abstract
   - current-status → status
   - date → published_date ("YYYY-MM" 形式)
   - author/name → authors (配列)
   - keywords/kw → keywords (配列), categories
   
4. SQLiteに保存 (UPSERT)
```

---

## AI要約プロンプト設計

### 小学生向け (easy)
```
あなたは小学生にインターネットの仕組みを教える先生です。
以下のRFCの内容を、小学5年生でもわかるように1〜2文で説明してください。

ルール:
- 難しい言葉は使わない
- 身近な例えを使う
- 絵文字を1つ使ってもOK
- 「〜だよ」「〜なんだ」のような口調で

RFC番号: {number}
タイトル: {title}
概要: {abstract}
```

### 一般向け (normal)
```
以下のRFCの内容を、IT知識のない一般の方にもわかるように
日本語で2〜3文で要約してください。

RFC番号: {number}
タイトル: {title}
概要: {abstract}
```

### 技術者向け (technical)
```
以下のRFCの内容を、ソフトウェアエンジニア向けに
技術的なポイントを含めて日本語で3〜4文で要約してください。

RFC番号: {number}
タイトル: {title}
概要: {abstract}
```

### 実装ガイド (implementation_guide)
```
あなたはシニアソフトウェアエンジニアです。
以下のRFCが定義するプロトコル/仕様を実装する場合のガイドを
日本語で作成してください。

## 出力フォーマット（Markdown）:

### 実装可否
- このRFCは実装可能か（プロトコル/アルゴリズム/データ形式など）
- 実装不可の場合（情報提供のみ、運用ガイドライン等）はその旨を記載

### 実装概要（実装可能な場合）
- 主要コンポーネント/モジュール構成
- データ構造の概要

### 推奨技術スタック
- 言語: (例: Rust, Go, Python)
- ライブラリ: (例: tokio, hyper)
- 既存実装の例: (例: curl, nginx)

### 実装時の注意点
- セキュリティ考慮事項
- パフォーマンス最適化ポイント
- エッジケース・例外処理

### 参考リソース
- 関連RFC
- 公式テストスイート（あれば）
- 参考となるOSS実装

---
RFC番号: {number}
タイトル: {title}
概要: {abstract}
```

---

## カテゴリ自動分類ロジック

キーワードベースでRFCをカテゴリに分類：

| カテゴリ | キーワード |
|---------|-----------|
| HTTP | http, web, uri, url, html |
| DNS | dns, domain, resolver |
| メール | smtp, imap, pop, email, mail |
| セキュリティ | tls, ssl, security, crypto, certificate |
| ルーティング | bgp, ospf, routing, router |
| IPv6 | ipv6, icmpv6 |
| TCP/UDP | tcp, udp, transport |
| その他 | (上記に該当しない) |

---

## UI/UXガイドライン（かんたんモード）

### 文字サイズ
- タイトル: 20px (1.25rem)
- 本文: 18px (1.125rem)
- バッジ: 14px (0.875rem)

### 配色
- 背景: #FFF9E6 (温かみのあるクリーム色)
- メイン: #FF6B35 (明るいオレンジ)
- アクセント: #4ECDC4 (ターコイズ)
- テキスト: #2D3436 (ダークグレー)

### アイコン
- 📄 RFC文書
- 🎈 かんたんモード
- 📝 概要
- 🏷️ タグ
- ⭐ ブックマーク
- 🕐 履歴
- 🔍 検索
- 🔧 実装ガイド
- 💻 コード例

---

## 実装ガイドUI設計

### 技術者向け詳細表示
```
┌────────────────────────────────────────────────────────────────┐
│ 📄 RFC 9114 - HTTP/3                                           │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                                                │
│ [🎈 かんたん] [📝 一般] [🔧 技術者] ← タブ切り替え              │
│                                                                │
│ ┌──────────────────────────────────────────────────────────┐  │
│ │ 🔧 技術者向け要約                                         │  │
│ │                                                          │  │
│ │ HTTP/3はQUICプロトコル上で動作するHTTPの第3バージョン。  │  │
│ │ UDPベースでHead-of-Line Blockingを解消し、              │  │
│ │ 0-RTT接続確立による低レイテンシを実現...                 │  │
│ └──────────────────────────────────────────────────────────┘  │
│                                                                │
│ ┌──────────────────────────────────────────────────────────┐  │
│ │ 💻 実装ガイド                              [生成する]     │  │
│ ├──────────────────────────────────────────────────────────┤  │
│ │                                                          │  │
│ │ ## 実装可否                                               │  │
│ │ ✅ 実装可能 (トランスポートプロトコル)                    │  │
│ │                                                          │  │
│ │ ## 実装概要                                               │  │
│ │ - QUICトランスポート層                                    │  │
│ │ - HTTP/3フレーミング層                                    │  │
│ │ - QPACK (ヘッダー圧縮)                                    │  │
│ │                                                          │  │
│ │ ## 推奨技術スタック                                       │  │
│ │ 言語: Rust, Go, C++                                       │  │
│ │ ライブラリ:                                               │  │
│ │   - Rust: quinn, quiche                                   │  │
│ │   - Go: quic-go                                           │  │
│ │ 既存実装:                                                 │  │
│ │   - curl (--http3)                                        │  │
│ │   - nginx (quic module)                                   │  │
│ │                                                          │  │
│ │ ## 実装時の注意点                                         │  │
│ │ ⚠️ UDPファイアウォール対応                                │  │
│ │ ⚠️ 証明書検証の実装                                       │  │
│ │ ⚠️ コネクションマイグレーション対応                       │  │
│ │                                                          │  │
│ │ ## 参考リソース                                           │  │
│ │ - RFC 9000 (QUIC)                                         │  │
│ │ - RFC 9001 (QUIC-TLS)                                     │  │
│ │ - https://quicwg.org/                                     │  │
│ └──────────────────────────────────────────────────────────┘  │
│                                                                │
│ [📖 RFC本文を見る] [⭐ ブックマーク] [📋 コピー]               │
└────────────────────────────────────────────────────────────────┘
```

### 実装不可の場合の表示例
```
┌──────────────────────────────────────────────────────────────┐
│ 💻 実装ガイド                                                │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│ ## 実装可否                                                   │
│ ℹ️ 実装対象外 (運用ガイドライン)                              │
│                                                              │
│ このRFCは実装を目的としたものではなく、                       │
│ ベストプラクティスや運用ガイドラインを定義しています。        │
│                                                              │
│ ## 関連する実装可能なRFC                                      │
│ - RFC XXXX: 関連プロトコル仕様                                │
│ - RFC YYYY: データ形式仕様                                    │
└──────────────────────────────────────────────────────────────┘
```

---

## ステータス
- [x] 設計ドラフト作成
- [x] レビュー
- [x] 設計承認

