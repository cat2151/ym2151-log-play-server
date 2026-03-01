Last updated: 2026-03-02

# Development Status

## 現在のIssues
- [Issue #183](../issue-notes/183.md) は、41個のファイルが500行を超えており、リファクタリングの検討が推奨されています。
- [Issue #178](../issue-notes/178.md) では、Windows実機でデモが安定して動作するか（ヨレないか）の動作確認が求められています。
- [Issue #118](../issue-notes/118.md) および [Issue #138](../issue-notes/138.md) は、Agentが生成したWindows用コードのTDD不足によるビルドエラーやハルシネーションの可能性について、継続的な監視と対策が検討されています。

## 次の一手候補
1. [Issue #183](../issue-notes/183.md) 大規模ファイルのリファクタリング候補の特定
   - 最初の小さな一歩: 500行を超えているファイルの中から、最も影響度が高く、かつリファクタリングしやすい候補を3つ特定し、選定理由を記述する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `issue-notes/183.md`と、`src/`ディレクトリ内のRustソースファイル全般

     実行内容: `issue-notes/183.md`に記載されている「500行を超過しているファイルリスト」を参照し、`src/`ディレクトリ内のRustソースファイルの中から、リファクタリングの優先順位が高いと思われるファイル（例: 依存関係が少ない、特定機能に特化している）を3つ選定し、それぞれの選定理由を記述してください。

     確認事項: 選定にあたり、ファイルの依存関係や役割、過去の変更履歴を考慮し、Agentのハルシネーションに繋がるような曖昧な選定基準は避けてください。

     期待する出力: Markdown形式で、選定された3つのファイルパス、現在の行数、選定理由をリストアップしてください。
     ```

2. [Issue #178](../issue-notes/178.md) Windows実機でのデモ動作確認手順の明確化
   - 最初の小さな一歩: `src/demo_server_interactive.rs`のデモが「ヨレない」ことを確認するためのWindows実機での具体的な手動テスト手順を記述する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/demo_server_interactive.rs`, `issue-notes/178.md`

     実行内容: `src/demo_server_interactive.rs` をWindows実機で実行し、`demoがヨレないか` を確認するための具体的な手動テスト手順を記述してください。手順には、環境構築（もし必要なら）、ビルドコマンド、実行コマンド、確認すべき現象（「ヨレない」とは具体的にどういう状態か）を含めてください。

     確認事項: `issue-notes/178.md`の内容を考慮し、既存のテストコード (例: `src/tests/demo_server_interactive_tests.rs`) を参考に、手動テストでしか確認できない側面（リアルタイム性、UIとの連携など）に焦点を当ててください。

     期待する出力: Markdown形式で、Windows実機での`demo_server_interactive.rs`手動テスト手順を詳細に記述してください。
     ```

3. [Issue #118](../issue-notes/118.md) AgentによるWindowsコード品質向上策の検討（CI強化）
   - 最初の小さな一歩: Linux CI上でWindowsターゲットのコンパイルチェックを行う既存の`build_windows.yml`ワークフローを調査し、現在の機能と課題を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `.github/workflows/build_windows.yml`, `issue-notes/118.md`

     実行内容: `.github/workflows/build_windows.yml`の現在の実装内容と、それが[Issue #118](../issue-notes/118.md)で言及されている「agentがPRしたWindows用codeが、TDDされていないためハルシネーション検知と修正がされずビルドが通らない」問題に対してどの程度効果があるか、または不足している点を分析してください。特に、Windowsターゲットのコンパイルエラーを検出する能力について焦点を当ててください。

     確認事項: `issue-notes/118.md`の「対策案」「方法の案」セクションを参考に、現在のワークフローがTDDの補助として機能しているか、または改善の余地があるかを検討してください。

     期待する出力: Markdown形式で、`.github/workflows/build_windows.yml`の現状の評価と、[Issue #118](../issue-notes/118.md)の課題解決に向けた改善点（例: より厳密なコンパイルオプションの適用、テストカバレッジの拡充、Agentへのフィードバック強化など）を提案してください。

---
Generated at: 2026-03-02 07:01:43 JST
