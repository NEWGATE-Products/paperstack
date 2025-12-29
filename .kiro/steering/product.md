# Product Context

## プロダクト名
AI論文ニュースリーダー (AI Paper News Reader)

## 概要
AI・LLM・プログラミング関連の最新論文を毎日取得し、Newsサイト風のUIで閲覧できるデスクトップアプリケーション。

## 主要機能
1. **論文取得**: Papers With Code APIから毎日12:00〜14:00（JST）に自動取得
2. **カテゴリフィルタリング**: AI全般、LLM、コード生成、アルゴリズム、アーキテクチャ
3. **要約生成**: Groq API（Llama 3.3 70B）で日本語1〜2文の要約を生成
4. **ニュース風UI**: カテゴリタブ、論文カード（タイトル・要約・タグ・リンク）

## ターゲットユーザー
- AI/ML研究者・エンジニア
- 最新の論文動向を効率的に追いたい人
- 日本語で論文概要を把握したい人

## カテゴリマッピング
| カテゴリ | タスクスラッグ |
|---------|---------------|
| AI全般 | machine-learning, deep-learning, reinforcement-learning |
| LLM | language-modelling, text-generation, question-answering |
| コード生成 | code-generation, program-synthesis |
| アルゴリズム | optimization, graph-neural-networks |
| アーキテクチャ | transformers, attention, neural-architecture-search |

