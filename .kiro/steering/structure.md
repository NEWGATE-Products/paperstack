# Project Structure

```
paper-news/
├── .kiro/
│   ├── steering/          # プロジェクト全体のコンテキスト
│   └── specs/             # 機能仕様
├── src/                   # React フロントエンド
│   ├── components/        # UIコンポーネント
│   │   ├── PaperCard.tsx
│   │   ├── CategoryTabs.tsx
│   │   ├── Header.tsx
│   │   └── PaperList.tsx
│   ├── hooks/             # カスタムフック
│   ├── lib/               # ユーティリティ
│   ├── types/             # TypeScript型定義
│   ├── App.tsx
│   ├── main.tsx
│   └── styles/            # スタイル
├── src-tauri/             # Rust バックエンド
│   ├── src/
│   │   ├── main.rs        # エントリーポイント
│   │   ├── lib.rs         # ライブラリルート
│   │   ├── db/            # データベース操作
│   │   │   ├── mod.rs
│   │   │   └── models.rs
│   │   ├── api/           # 外部API連携
│   │   │   ├── mod.rs
│   │   │   ├── pwc.rs     # Papers With Code
│   │   │   └── groq.rs    # Groq API
│   │   └── commands/      # Tauri コマンド
│   │       └── mod.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

## データフロー
1. ユーザーがカテゴリを選択
2. フロントエンドがTauri IPCでRustバックエンドを呼び出し
3. バックエンドがSQLiteからキャッシュ済み論文を取得
4. キャッシュがない/古い場合、Papers With Code APIから取得
5. 要約がない論文はGroq APIで日本語要約を生成
6. 結果をフロントエンドに返却し表示

