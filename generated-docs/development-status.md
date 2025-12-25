Last updated: 2025-12-26

# Development Status

## 現在のIssues
- Windows CIでのビルド/テスト ([Issue #145](../issue-notes/145.md)) が失敗しており、AgentによるWindowsコードのTDD不足 ([Issue #118](../issue-notes/118.md)) やハルシネーション ([Issue #138](../issue-notes/138.md)) が根本原因として指摘されています。
- コマンドライン引数のヘルプ表示で `--demo-interactive` が欠落しユーザーが混乱する問題 ([Issue #121](../issue-notes/121.md)) と、サーバーコマンド (`clear schedule` の廃止・統合、`get interactive mode` の削除) の改善提案 ([Issue #120](../issue-notes/120.md), [Issue #119](../issue-notes/119.md)) があります。
- クライアントのデモインタラクティブモードでのフレーズ開始タイミングのブレ ([Issue #117](../issue-notes/117.md)) も未解決の重要な課題です。

## 次の一手候補
1. WindowsビルドのコンパイルチェックをLinux CIで実施し、Agentによる品質保証を強化する [Issue #118](../issue-notes/118.md), [Issue #145](../issue-notes/145.md)
   - 最初の小さな一歩: Linux Runner上で `cargo check --target=x86_64-pc-windows-gnu` を実行するGitHub Actionsワークフロー `.github/workflows/check_windows_build_on_linux.yml` を新規作成し、Windows版のコンパイルチェックが可能か検証する。
   - Agent実行プロンプト:
     ```
     対象ファイル: .github/workflows/check_windows_build_on_linux.yml (新規作成)

     実行内容: [Issue #118](../issue-notes/118.md) の対策案に基づき、Linux Runnerで `cargo check --target=x86_64-pc-windows-gnu` を実行するGitHub Actionsワークフロー `.github/workflows/check_windows_build_on_linux.yml` を新規作成してください。このワークフローは、`build_windows.yml` の関連部分を参考に、WindowsターゲットのRustコードがLinux環境でコンパイルチェック可能か検証することを目的とします。

     確認事項: `setup_ci_environment.sh` がRustツールチェインやターゲットのインストールにどのように影響するかを確認し、`cargo check` が必要な環境を正しく設定できるかを検証してください。また、`build_windows.yml` などの既存のCIワークフローとの整合性を考慮してください。

     期待する出力: 新規作成する `.github/workflows/check_windows_build_on_linux.yml` ファイルの内容をMarkdownコードブロックで出力し、このワークフローがWindowsビルドのコンパイルチェックをLinux CI上で行うための手順と、そのワークフローが意図通りに動作することを確認するための簡単な説明を含めてください。
     ```

2. コマンドライン引数のヘルプメッセージに `--demo-interactive` オプションを表示させる [Issue #121](../issue-notes/121.md)
   - 最初の小さな一歩: `src/main.rs` および `src/client/interactive.rs` のコマンドライン引数定義を分析し、`--demo-interactive` オプションが `clap` クレートに正しく登録され、ヘルプメッセージ生成に含まれる設定になっているかを確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/main.rs, src/client/interactive.rs, src/client/mod.rs

     実行内容: [Issue #121](../issue-notes/121.md) で報告されている `--demo-interactive` オプションがヘルプメッセージに表示されない問題について、`src/main.rs` にあるコマンドライン引数定義箇所を調査してください。特に `clap` クレートの利用状況を確認し、`--demo-interactive` オプションが正しく定義され、ヘルプ出力に含まれるように設定されているかを分析してください。

     確認事項: 他の既存のコマンドラインオプションとの整合性、および `clap` のバージョンと利用方法に関するドキュメントを確認し、推奨される実装方法と逸脱がないかを検証してください。

     期待する出力: `src/main.rs` 内の `clap` の定義と、`--demo-interactive` がヘルプに表示されない原因、そしてその修正案をMarkdown形式で出力してください。修正案には具体的なコード変更の例を含めてください。
     ```

3. クライアントデモインタラクティブモードでのフレーズ開始タイミングのブレの原因を特定し、可視化する [Issue #117](../issue-notes/117.md)
   - 最初の小さな一歩: [Issue #117](../issue-notes/117.md) の結論に基づき、`src/demo_client_interactive.rs` と `src/demo_server_interactive.rs` を使用してデモ環境をセットアップし、通常モードとインタラクティブモードで音が鳴ることを確認する。その後、各モードでのフレーズ開始タイミングのブレを測定・可視化し、具体的な問題点として記録する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/client/interactive.rs, src/server/command_handler.rs, src/audio/scheduler.rs, src/demo_client_interactive.rs, src/demo_server_interactive.rs

     実行内容: [Issue #117](../issue-notes/117.md) に記載されている「client側のdemo interactive modeで、clientからserverへの送信ごとにフレーズ開始タイミングがブレる」問題について、`src/client/interactive.rs` からサーバー (`src/server/command_handler.rs`) へのJSON送信と、サーバー側のスケジューリング (`src/audio/scheduler.rs`) のロジックを分析してください。特に、`src/demo_client_interactive.rs` および `src/demo_server_interactive.rs` のデモ実装におけるタイムスタンプの扱い、IPC通信の遅延、およびスケジューリングオフセットの適用方法に注目し、ブレの原因となりうる箇所を特定してください。

     確認事項: クライアントとサーバー間のIPC通信 (`src/ipc/mod.rs` など) の特性、および既存のテストコード (`tests/interactive/mod.rs` など) を確認し、ブレの再現性や測定方法について考慮してください。デモを再現するために必要な具体的なコマンドも考慮に入れてください。

     期待する出力: 問題の原因となりうる箇所を特定し、その分析結果と、ブレを軽減するための具体的なコード修正の方向性をMarkdown形式で提案してください。可能であれば、修正後の期待される挙動についても言及してください。
     ```

---
Generated at: 2025-12-26 07:01:52 JST
