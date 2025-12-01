Last updated: 2025-12-02

# Development Status

## 現在のIssues
- [Issue #118](../issue-notes/118.md) では、Agentが生成するWindows用コードのTDD不足によるビルド失敗問題の解決策が検討されています。
- [Issue #121](../issue-notes/121.md), [Issue #120](../issue-notes/120.md), [Issue #119](../issue-notes/119.md) は、コマンドラインヘルプの改善、サーバーコマンドのシンプル化、およびスケジュールクリア機能の統合による操作性の向上を目指しています。
- [Issue #117](../issue-notes/117.md) では、クライアント側のデモインタラクティブモードにおけるフレーズ開始タイミングのブレの原因分析と、YM2151トーンエディタでの動作確認が求められています。

## 次の一手候補
1. [Issue #118](../issue-notes/118.md): AgentがPRしたWindows用codeが、TDDされていないためハルシネーション検知と修正がされずビルドが通らない
   - 最初の小さな一歩: GitHub ActionsのLinux Runner上でRustのWindowsターゲット向けコンパイルチェック（`cargo check --target x86_64-pc-windows-gnu`など）が実行可能か、その具体的な方法とagentでのTDD適用可能性について調査する。
   - Agent実行プロンプ:
     ```
     対象ファイル: _research/windows_build_check_on_linux_ci.md (新規作成)

     実行内容: GitHub ActionsのLinux Runner環境でRustプロジェクトのWindowsターゲット向け（例: `x86_64-pc-windows-gnu`）のコンパイルチェックを実行するための実現可能性を調査し、以下の観点でmarkdown形式でまとめてください。
     1. 利用可能なツール（`cargo check --target`, `cross`, `cargo-xwin`など）とその簡単な説明。
     2. 各ツールのGitHub Actions Linux Runnerでの設定方法と制約。
     3. GitHub Copilot Coding AgentがTDDを通じてWindowsビルドエラーを自律的に修正できる可能性と、そのためのプロンプト指示方法の検討。
     4. 最終的な推奨されるアプローチと、そのための初期ワークフローの草案（もし実現可能であれば）。

     確認事項: 調査にあたり、Rustのクロスコンパイルに関する公式ドキュメントや関連コミュニティの議論を参照し、最新かつ信頼性の高い情報を収集してください。また、既存の`.github/workflows/`内のファイルが参考になる可能性があります。

     期待する出力: 上記実行内容で指定された観点を含む、詳細な調査結果をmarkdown形式で出力してください。
     ```

2. [Issue #119](../issue-notes/119.md): server commandのうち、get interactive modeは不要になったので削除し、シンプル化する
   - 最初の小さな一歩: サーバーコマンド定義である`src/ipc/protocol.rs`から`GetInteractiveMode`エントリを削除し、コンパイルエラーが発生する箇所を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ipc/protocol.rs, src/server/command_handler.rs, src/client/core.rs, src/client/mod.rs, src/tests/**/*.rs, tests/**/*.rs

     実行内容: `GetInteractiveMode`コマンドをシステム全体から完全に削除するための変更を適用してください。具体的には、
     1. `src/ipc/protocol.rs`から`GetInteractiveMode`コマンド定義を削除します。
     2. `src/server/command_handler.rs`で`GetInteractiveMode`を処理するロジックを削除します。
     3. `src/client/core.rs`や`src/client/mod.rs`など、クライアント側で`GetInteractiveMode`を使用している箇所があれば、その呼び出しと関連ロジックを削除または修正します。
     4. `src/tests/`および`tests/`配下の関連する単体テストや統合テストを削除または適切に修正し、機能削除を反映させます。

     確認事項: この変更が、他のサーバーコマンドやクライアント機能の動作に影響を与えないことを確認してください。また、削除される機能の痕跡がコードベースに残らないように徹底してください。

     期待する出力: `GetInteractiveMode`の削除によって変更された全ファイルの差分と、削除されたテストファイルの一覧、および主要な変更箇所の説明をmarkdown形式で出力してください。
     ```

3. [Issue #117](../issue-notes/117.md): client側のdemo interactive modeで、clientからserverへの送信ごとにフレーズ開始タイミングがブレる
   - 最初の小さな一歩: `ym2151 tone editor` (またはそれに相当するデモクライアント) の通常モードとインタラクティブモードで音が正しく鳴るかを確認し、もしタイミングのブレが発生する場合、その具体的な状況（再現手順、ブレの度合いなど）を詳細に記録する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/client/interactive.rs, src/demo_client_interactive.rs, src/audio/scheduler.rs

     実行内容: クライアント側のデモインタラクティブモード（`src/demo_client_interactive.rs`）において、JSONデータの送信タイミングとサーバー側でのスケジューリングの関係性を分析し、以下の観点から報告してください。
     1. JSONがクライアントから送信される際のタイムスタンプ処理（またはその欠如）。
     2. サーバーがJSONを受信し、未来オフセットを加算してスケジューリングするロジック（`src/audio/scheduler.rs`に関連）。
     3. このプロセスにおける名前付きパイプのI/O遅延が、フレーズ開始タイミングのブレに与える影響の可能性。
     4. ブレを可視化するための簡単なデバッグログの挿入案（例: クライアント送信直前とサーバー受信直後のタイムスタンプ記録）。

     確認事項: 既存の`play json`コマンドや他のモードとのスケジューリングロジックの比較を行い、`demo interactive mode`特有の問題点がないか確認してください。名前付きパイプの動作原理についても考慮に入れてください。

     期待する出力: 分析結果とデバッグログ挿入案をmarkdown形式で出力してください。特に、タイムスタンプがどのように扱われているか、どの部分で遅延が発生しうるかについて焦点を当ててください。
     ```

---
Generated at: 2025-12-02 07:02:08 JST
