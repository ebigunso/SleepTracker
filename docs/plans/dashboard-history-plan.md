# Plan: Dashboard History Navigation

- status: done
- generated: 2026-02-01
- last_updated: 2026-02-01

## Overview
- 目的: ダッシュボードで過去任意の日付の睡眠エントリを閲覧/編集/追加できるようにし、表示量を抑えた操作しやすいUIを提供する。
- アプローチ概要: 既定のウィンドウ（日数）で範囲表示し、日付ジャンプと前後ナビゲーションで過去任意日のアクセスを可能にする。APIは既存の`/api/sleep/range`と`/api/exercise/intensity`を利用する。

## Scope / Non-goals
- Scope:
  - ダッシュボードの表示範囲を可変にし、日付ジャンプ/前後移動/UI導線を追加
  - Range取得に合わせて運動強度の取得期間を同期
  - 日付行から日別ビューへ遷移できる導線追加
- Non-goals:
  - バックエンドAPIの新規追加/変更
  - トレンド画面や日別画面の大規模改修

## Current Context (repo)
- 既存構造・関連モジュール:
  - sleep-ui/src/routes/+page.svelte: 直近7日表示のダッシュボード
  - sleep-ui/src/routes/+page.server.ts: /api/sleep/recent と /api/exercise/intensity 取得
  - sleep-ui/src/lib/components/WeekRow.svelte: 日付行のUI
- 参考実装/類似箇所:
  - sleep-ui/src/routes/trends/+page.svelte: 期間選択UI

## Open Questions (max 3)
- Q1: 既定のウィンドウ日数は14日でよいか？（仮: 14日）

## Tasks

### T1: 設計整理（範囲UI/遷移）
- type: design
- owns:
  - docs/plans/dashboard-history-plan.md
- depends_on: []
- description: |
  ウィンドウ日数・ナビゲーション・ジャンプ導線の設計を確定する。
- acceptance:
  - 既定日数、前後移動、ジャンプ導線が明記されている
- validation:
  - レビュー観点としてUI過密にならないことを確認

### T2: ダッシュボードの範囲取得/ナビゲーション実装
- type: impl
- owns:
  - sleep-ui/src/routes/+page.server.ts
  - sleep-ui/src/routes/+page.svelte
  - sleep-ui/src/lib/components/WeekRow.svelte
- depends_on: [T1]
- description: |
  クエリパラメータに基づく期間取得、日付ジャンプ、前後移動、日付行リンクのUIを実装する。
- acceptance:
  - 過去任意の日付へのジャンプが可能
  - 前後移動で期間を切り替えられる
  - 表示量は固定ウィンドウ（14日）に収まる
  - 日付行から日別ビューへ遷移できる
- validation:
  - command: npm test
  - notes: UI変更のため実行不可なら未実行で可

## Rollback / Safety
- 変更を戻す手順:
  - ダッシュボード関連のSvelteファイルを元に戻す
- フラグ/設定で無効化できるか:
  - なし

## Notes
- 重要な仮定:
  - 既定の表示ウィンドウは14日
- 想定リスク:
  - クエリパラメータの扱いで日付が未来になる場合はクランプする
