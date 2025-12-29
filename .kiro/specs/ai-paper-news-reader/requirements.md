# 要件定義: AI論文ニュースリーダー

## 機能要件

### FR-1: 論文取得
- **FR-1.1**: Papers With Code APIから論文を取得できること
- **FR-1.2**: 公開日の降順で論文を取得できること
- **FR-1.3**: タスク（カテゴリ）でフィルタリングできること

### FR-2: カテゴリフィルタリング
- **FR-2.1**: 以下のカテゴリで論文をフィルタリングできること
  - AI全般: machine-learning, deep-learning, reinforcement-learning
  - LLM: language-modelling, text-generation, question-answering
  - コード生成: code-generation, program-synthesis
  - アルゴリズム: optimization, graph-neural-networks
  - アーキテクチャ: transformers, attention, neural-architecture-search

### FR-3: 要約生成
- **FR-3.1**: Groq API (Llama 3.3 70B) で日本語要約を生成できること
- **FR-3.2**: 要約は1〜2文で簡潔にすること
- **FR-3.3**: 要約をローカルDBにキャッシュすること

### FR-4: ユーザーインターフェース
- **FR-4.1**: ニュースサイト風のレイアウトで表示すること
- **FR-4.2**: カテゴリタブで切り替えできること
- **FR-4.3**: 各論文カードに以下を表示すること
  - 論文タイトル
  - 日本語要約
  - タグ（タスク・手法）
  - 元論文へのリンク

### FR-5: データ永続化
- **FR-5.1**: SQLiteで論文データを保存すること
- **FR-5.2**: 論文とタスクの関連を保存すること

## 非機能要件

### NFR-1: パフォーマンス
- **NFR-1.1**: アプリ起動時間は3秒以内
- **NFR-1.2**: 論文一覧の表示は1秒以内

### NFR-2: ユーザビリティ
- **NFR-2.1**: 直感的なUI/UX
- **NFR-2.2**: 日本語での表示

### NFR-3: 信頼性
- **NFR-3.1**: API障害時もキャッシュデータで動作すること

