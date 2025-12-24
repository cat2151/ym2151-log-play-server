Last updated: 2025-12-25

# 開発状況生成プロンプト（開発者向け）

## 生成するもの：
- 現在openされているissuesを3行で要約する
- 次の一手の候補を3つlistする
- 次の一手の候補3つそれぞれについて、極力小さく分解して、その最初の小さな一歩を書く

## 生成しないもの：
- 「今日のissue目標」などuserに提案するもの
  - ハルシネーションの温床なので生成しない
- ハルシネーションしそうなものは生成しない（例、無価値なtaskや新issueを勝手に妄想してそれをuserに提案する等）
- プロジェクト構造情報（来訪者向け情報のため、別ファイルで管理）

## 「Agent実行プロンプト」生成ガイドライン：
「Agent実行プロンプト」作成時は以下の要素を必ず含めてください：

### 必須要素
1. **対象ファイル**: 分析/編集する具体的なファイルパス
2. **実行内容**: 具体的な分析や変更内容（「分析してください」ではなく「XXXファイルのYYY機能を分析し、ZZZの観点でmarkdown形式で出力してください」）
3. **確認事項**: 変更前に確認すべき依存関係や制約
4. **期待する出力**: markdown形式での結果や、具体的なファイル変更

### Agent実行プロンプト例

**良い例（上記「必須要素」4項目を含む具体的なプロンプト形式）**:
```
対象ファイル: `.github/workflows/translate-readme.yml`と`.github/workflows/call-translate-readme.yml`

実行内容: 対象ファイルについて、外部プロジェクトから利用する際に必要な設定項目を洗い出し、以下の観点から分析してください：
1) 必須入力パラメータ（target-branch等）
2) 必須シークレット（GEMINI_API_KEY）
3) ファイル配置の前提条件（README.ja.mdの存在）
4) 外部プロジェクトでの利用時に必要な追加設定

確認事項: 作業前に既存のworkflowファイルとの依存関係、および他のREADME関連ファイルとの整合性を確認してください。

期待する出力: 外部プロジェクトがこの`call-translate-readme.yml`を導入する際の手順書をmarkdown形式で生成してください。具体的には：必須パラメータの設定方法、シークレットの登録手順、前提条件の確認項目を含めてください。
```

**避けるべき例**:
- callgraphについて調べてください
- ワークフローを分析してください
- issue-noteの処理フローを確認してください

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Development Status

## 現在のIssues
[以下の形式で3行でオープン中のissuesを要約。issue番号を必ず書く]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 次の一手候補
1. [候補1のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

2. [候補2のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

3. [候補3のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```
```


# 開発状況情報
- 以下の開発状況情報を参考にしてください。
- Issue番号を記載する際は、必ず [Issue #番号](../issue-notes/番号.md) の形式でMarkdownリンクとして記載してください。

## プロジェクトのファイル一覧
- .config/nextest.toml
- .editorconfig
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
- .github/actions-tmp/.github/workflows/rust-windows-check.yml
- .github/actions-tmp/.github/workflows/translate-readme.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/callgraph.ql
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/codeql-pack.lock.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/qlpack.yml
- .github/actions-tmp/.github_automation/callgraph/config/example.json
- .github/actions-tmp/.github_automation/callgraph/docs/callgraph.md
- .github/actions-tmp/.github_automation/callgraph/presets/callgraph.js
- .github/actions-tmp/.github_automation/callgraph/presets/style.css
- .github/actions-tmp/.github_automation/callgraph/scripts/analyze-codeql.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/callgraph-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-codeql-exists.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-node-version.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/common-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/copy-commit-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/extract-sarif-info.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/find-process-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generate-html-graph.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generateHTML.cjs
- .github/actions-tmp/.github_automation/check_recent_human_commit/scripts/check-recent-human-commit.cjs
- .github/actions-tmp/.github_automation/project_summary/docs/daily-summary-setup.md
- .github/actions-tmp/.github_automation/project_summary/prompts/development-status-prompt.md
- .github/actions-tmp/.github_automation/project_summary/prompts/project-overview-prompt.md
- .github/actions-tmp/.github_automation/project_summary/scripts/ProjectSummaryCoordinator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/GitUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/IssueTracker.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/generate-project-summary.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/CodeAnalyzer.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectAnalysisOrchestrator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataCollector.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataFormatter.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectOverviewGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/BaseGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/FileSystemUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/ProjectFileUtils.cjs
- .github/actions-tmp/.github_automation/translate/docs/TRANSLATION_SETUP.md
- .github/actions-tmp/.github_automation/translate/scripts/translate-readme.cjs
- .github/actions-tmp/.gitignore
- .github/actions-tmp/.vscode/settings.json
- .github/actions-tmp/LICENSE
- .github/actions-tmp/README.ja.md
- .github/actions-tmp/README.md
- .github/actions-tmp/_config.yml
- .github/actions-tmp/generated-docs/callgraph.html
- .github/actions-tmp/generated-docs/callgraph.js
- .github/actions-tmp/generated-docs/development-status-generated-prompt.md
- .github/actions-tmp/generated-docs/development-status.md
- .github/actions-tmp/generated-docs/project-overview-generated-prompt.md
- .github/actions-tmp/generated-docs/project-overview.md
- .github/actions-tmp/generated-docs/style.css
- .github/actions-tmp/googled947dc864c270e07.html
- .github/actions-tmp/issue-notes/10.md
- .github/actions-tmp/issue-notes/11.md
- .github/actions-tmp/issue-notes/12.md
- .github/actions-tmp/issue-notes/13.md
- .github/actions-tmp/issue-notes/14.md
- .github/actions-tmp/issue-notes/15.md
- .github/actions-tmp/issue-notes/16.md
- .github/actions-tmp/issue-notes/17.md
- .github/actions-tmp/issue-notes/18.md
- .github/actions-tmp/issue-notes/19.md
- .github/actions-tmp/issue-notes/2.md
- .github/actions-tmp/issue-notes/20.md
- .github/actions-tmp/issue-notes/21.md
- .github/actions-tmp/issue-notes/22.md
- .github/actions-tmp/issue-notes/23.md
- .github/actions-tmp/issue-notes/24.md
- .github/actions-tmp/issue-notes/25.md
- .github/actions-tmp/issue-notes/26.md
- .github/actions-tmp/issue-notes/27.md
- .github/actions-tmp/issue-notes/28.md
- .github/actions-tmp/issue-notes/29.md
- .github/actions-tmp/issue-notes/3.md
- .github/actions-tmp/issue-notes/30.md
- .github/actions-tmp/issue-notes/4.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
- .github/scripts/README.md
- .github/scripts/generate_test_failure_issue.py
- .github/scripts/test_generate_test_failure_issue.py
- .github/workflows/build_windows.yml
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-rust-windows-check.yml
- .github/workflows/call-translate-readme.yml
- .gitignore
- .vscode/extensions.json
- .vscode/settings.json
- Cargo.lock
- Cargo.toml
- LICENSE
- README.ja.md
- README.md
- _codeql_detected_source_root
- _config.yml
- build.rs
- googled947dc864c270e07.html
- install-ym2151-tools.rs
- issue-notes/100.md
- issue-notes/101.md
- issue-notes/102.md
- issue-notes/110.md
- issue-notes/111.md
- issue-notes/112.md
- issue-notes/113.md
- issue-notes/116.md
- issue-notes/117.md
- issue-notes/118.md
- issue-notes/119.md
- issue-notes/120.md
- issue-notes/121.md
- issue-notes/122.md
- issue-notes/123.md
- issue-notes/124.md
- issue-notes/128.md
- issue-notes/130.md
- issue-notes/132.md
- issue-notes/134.md
- issue-notes/138.md
- issue-notes/141.md
- issue-notes/143.md
- issue-notes/96.md
- issue-notes/97.md
- issue-notes/98.md
- issue-notes/99.md
- opm.c
- opm.h
- output_ym2151.json
- setup_ci_environment.sh
- src/audio/buffers.rs
- src/audio/commands.rs
- src/audio/generator.rs
- src/audio/mod.rs
- src/audio/player.rs
- src/audio/scheduler.rs
- src/audio/stream.rs
- src/audio_config.rs
- src/client/config.rs
- src/client/core.rs
- src/client/interactive.rs
- src/client/json.rs
- src/client/mod.rs
- src/client/server.rs
- src/debug_wav.rs
- src/demo_client_interactive.rs
- src/demo_server_interactive.rs
- src/demo_server_non_interactive.rs
- src/events.rs
- src/ipc/mod.rs
- src/ipc/pipe_windows.rs
- src/ipc/protocol.rs
- src/ipc/windows/mod.rs
- src/ipc/windows/pipe_factory.rs
- src/ipc/windows/pipe_handle.rs
- src/ipc/windows/pipe_reader.rs
- src/ipc/windows/pipe_writer.rs
- src/ipc/windows/test_logging.rs
- src/lib.rs
- src/logging.rs
- src/main.rs
- src/mmcss.rs
- src/opm.rs
- src/opm_ffi.rs
- src/player.rs
- src/resampler.rs
- src/scheduler.rs
- src/server/command_handler.rs
- src/server/connection.rs
- src/server/mod.rs
- src/server/playback.rs
- src/server/state.rs
- src/tests/audio_tests.rs
- src/tests/client_tests.rs
- src/tests/debug_wav_tests.rs
- src/tests/demo_server_interactive_tests.rs
- src/tests/demo_server_non_interactive_tests.rs
- src/tests/events_tests.rs
- src/tests/ipc_pipe_windows_tests.rs
- src/tests/ipc_protocol_tests.rs
- src/tests/logging_tests.rs
- src/tests/mmcss_tests.rs
- src/tests/mod.rs
- src/tests/opm_ffi_tests.rs
- src/tests/opm_tests.rs
- src/tests/play_json_interactive_tests.rs
- src/tests/player_tests.rs
- src/tests/resampler_tests.rs
- src/tests/scheduler_tests.rs
- src/tests/server_tests.rs
- src/tests/wav_writer_tests.rs
- src/wav_writer.rs
- tests/audio/audio_playback_test.rs
- tests/audio/audio_sound_test.rs
- tests/audio/mod.rs
- tests/clear_schedule_test.rs
- tests/cli_integration_test.rs
- tests/client_json_test.rs
- tests/client_test.rs
- tests/client_verbose_test.rs
- tests/debug_wav_test.rs
- tests/duration_test.rs
- tests/ensure_server_ready_test.rs
- tests/events_processing_test.rs
- tests/feature_demonstration_test.rs
- tests/fixtures/complex.json
- tests/fixtures/simple.json
- tests/integration_test.rs
- tests/interactive/mod.rs
- tests/interactive/mode_test.rs
- tests/interactive/play_json_test.rs
- tests/interactive/shared_mutex.rs
- tests/interactive/step_by_step_test.rs
- tests/interactive_tests.rs
- tests/ipc_pipe_test.rs
- tests/logging_test.rs
- tests/server_basic_test.rs
- tests/server_integration_test.rs
- tests/tail_generation_test.rs
- tests/test_util_server_mutex.rs

## 現在のオープンIssues
## [Issue #144](../issue-notes/144.md): Add Gemini AI translation of test errors to Windows CI failure issues
When Windows CI tests fail, auto-generated GitHub issues now include AI-translated Japanese summaries at the top to reduce cognitive load for Japanese developers.

## Changes

### Python Script (`generate_test_failure_issue.py`)
- Added `translate_error_messages_with_gemini()` using Gemini 1.5 Flash...
ラベル: 
--- issue-notes/144.md の内容 ---

```markdown

```

## [Issue #143](../issue-notes/143.md): build_windows.ymlでtest failed時のissue生成機能について、issue先頭に、geminiを利用してtestエラーメッセージ群を日本語訳したものを追加し、userの認知負荷を下げる
[issue-notes/143.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/143.md)

...
ラベル: 
--- issue-notes/143.md の内容 ---

```markdown
# issue build_windows.ymlでtest failed時のissue生成機能について、issue先頭に、geminiを利用してtestエラーメッセージ群を日本語訳したものを追加し、userの認知負荷を下げる #143
[issues #143](https://github.com/cat2151/ym2151-log-play-server/issues/143)



```

## [Issue #138](../issue-notes/138.md): PR 137 のagentの挙動（初手の対策案が誤っており、userがより深く分析させたら正しい対策案に到達した）はハルシネーションの可能性がある。対策案を洗い出して整理する
[issue-notes/138.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/138.md)

...
ラベル: 
--- issue-notes/138.md の内容 ---

```markdown
# issue PR 137 のagentの挙動（初手の対策案が誤っており、userがより深く分析させたら正しい対策案に到達した）はハルシネーションの可能性がある。対策案を洗い出して整理する #138
[issues #138](https://github.com/cat2151/ym2151-log-play-server/issues/138)

# 何が困るの？
- PR 137はラッキーだっただけ
- もっと深刻な潜在的な「アーキテクチャ誤り、仕様誤り、バグ」をagentが生成してしまうリスクがある
- つまり大きな開発コスト増大リスクがある

# 対策案は？
- 様子見。例えば、あと2回同様の「agentがハルシネーション的誤り。しかもuserもうっかり素通りさせるところだった」が発生したら、さらに検討する
- CIエラーログの縮小。今回50KB超のサイズである。エラー部分だけにして縮小できるか検討する。
  - 課題、見込みが低そう。agentは結局CIログを全量readしにいきそう。
- ひとまず様子見とする

```

## [Issue #121](../issue-notes/121.md): コマンドライン引数の表示パターンが2パターンあり（help時、不明なオプション時）、どちらも--demo-interactiveが表示されず、userが混乱する
[issue-notes/121.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/121.md)

...
ラベル: 
--- issue-notes/121.md の内容 ---

```markdown
# issue コマンドライン引数の表示パターンが2パターンあり（help時、不明なオプション時）、どちらも--demo-interactiveが表示されず、userが混乱する #121
[issues #121](https://github.com/cat2151/ym2151-log-play-server/issues/121)



```

## [Issue #120](../issue-notes/120.md): server commandのうち、clear scheduleを廃止し、play json interactiveはデフォルトでwith clear scheduleにする（そのjsonの先頭sample時刻より未来のscheduleだけ削除する。キーリピート問題対策用）
[issue-notes/120.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/120.md)

...
ラベル: 
--- issue-notes/120.md の内容 ---

```markdown
# issue server commandのうち、clear scheduleを廃止し、play json with clear scheduleにする（そのjsonのsample時刻より過去のscheduleだけ削除する） #120
[issues #120](https://github.com/cat2151/ym2151-log-play-server/issues/120)



```

## [Issue #119](../issue-notes/119.md): server commandのうち、get interactive modeは不要になったので削除し、シンプル化する
[issue-notes/119.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/119.md)

...
ラベル: 
--- issue-notes/119.md の内容 ---

```markdown
# issue server commandのうち、get interactive modeは不要になったので削除し、シンプル化する #119
[issues #119](https://github.com/cat2151/ym2151-log-play-server/issues/119)



```

## [Issue #118](../issue-notes/118.md): agentがPRしたWindows用codeが、TDDされていないためハルシネーション検知と修正がされずビルドが通らない
[issue-notes/118.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/118.md)

...
ラベル: 
--- issue-notes/118.md の内容 ---

```markdown
# issue agentがPRしたWindows用codeが、TDDされていないためハルシネーション検知と修正がされずビルドが通らない #118
[issues #118](https://github.com/cat2151/ym2151-log-play-server/issues/118)

# 何が困るの？
- 開発体験が悪い
  - ほかのprojectの事例
    - PRをいくつか、検証なしで受け入れ
    - 結果、そのまま使える
      - 分析、TDDで品質担保されており、ハルシネーションはほぼない
        - Windows用codeがないマルチプラットフォームprojectなので、agentがTDDしておりcode品質が高い
      - 開発体験が良い
  - このprojectの事例
    - PRをいくつか、検証なしで受け入れ
    - 結果、ビルドが通らない
      - 複数のPRすべてがハルシネーション
      - userが修正時、複数PRのバグが混ざっており切り分けコストがかかる
      - 開発体験が悪い
    - 分析、このprojectのWindows用codeの品質は低い
      - 規模が大きくなってくるにつれ、指数関数的に品質低下が起こっている感触がある
      - ハルシネーションがどんどん増えている

# 対策案
- cargo check target ～gnu
  - WSLで動作確認済み
  - GitHub Copilot Coding Agentでも実施できる可能性がある

# 方法の案
## まずGitHub Actions
- 上記をworkflow作成し、GitHub Actions Linux Runnerで動作確認する
- logでcargo check finished目視確認または、より効率的な確認

# 草稿
- ゴール
    - GitHubのLinux Runner上での GitHub Copilot Coding Agent によるTDDにおいて、以下をPRコメントに書くこと
        - ※大前提。GitHub Linux Runner上での話である。現状、GitHub Copilot Coding Agent はそれしか使えないので。Windows Runnerは使えない。
        - 最低限、Rustのコンパイルチェックで、
            - Windows版コンパイルがエラーとwarningのない状態
            - （code、unit test、統合testすべて）
            - をTDDで実現できるか？
            - その方法は？
                - cargo check target gnuを使う？
                - crossを使う？
                - cargo-xwinを使う？
                - ほかに方法は？
            - agentへのprompt指示だけで実現可能か？
                - そのpromptは？
            - これをweb調査してまとめること
        - もしどうしようもないなら、
            - 一つのPRにつき毎回、userがWindows版の手動ビルドを
                - するしかない？
                - ※今回、3つのPRを「userがWindows版の手動ビルドをせず」受け入れた結果、
                    - 3つともハルシネーションによる認識誤りによるバグや実装漏れがあった
                    - ビルドが通らない、testがfailed、
                        - ビルドを通した以降もバグっている、
                            - という低品質codeだった
                - つまりGitHub Copilot Coding Agentの自律的なactionではどうにもならない？
                    - ※もしCI/CDでGitHub Actionsで、Windows版のコンパイルが通るかチェックしたところで、それをagentが自律的にactionしてTDDで修正できない、というフローなら、手間をかけてやる意味が薄い
                        - それは結局、運用として、userが手動でそれをチェックしてlocalでagentをkickする、がマストになってしまい、userの手間がかかる点では大差ないので
            - これをweb調査してまとめること

# 状況
- 検討中

```

## [Issue #117](../issue-notes/117.md): client側のdemo interactive modeで、clientからserverへの送信ごとにフレーズ開始タイミングがブレる
[issue-notes/117.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/117.md)

...
ラベル: 
--- issue-notes/117.md の内容 ---

```markdown
# issue client側のdemo interactive modeで、clientからserverへの送信ごとにフレーズ開始タイミングがブレる #117
[issues #117](https://github.com/cat2151/ym2151-log-play-server/issues/117)

# 原因分析
- 時刻指定に原因がある考え
    - clientは時刻指定せずにjson送信してる
        - 開始time 0.0 のjson
    - 受信したサーバー側は、
        - 未来オフセットを加算して、スケジューリングしてる
    - よって名前付きパイプのブレでモタる
# どうする？
- 分析
    - 切り分け
        - client demo interactiveは、モタらないシーケンス演奏をしたいdemoである
        - tone editorは、最速で音を変更したい
            - まず、今の各モードが鳴るところまで持っていく
                - で、どれくらい使ってて問題が出るか？を可視化する
                    - これが重要
- 結論
    - ym2151 tone editorにおいて以下を確認する
        - 通常モードとインタラクティブモードで音が鳴ること
        - 問題あれば、どのような問題があるか？をissueに可視化すること


```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/17.md
```md
{% raw %}
# issue development-status が生成したmdに誤りがある。issue-note へのlinkがURL誤りで、404となってしまう #17
[issues #17](https://github.com/cat2151/github-actions/issues/17)

# 事例
- 生成したmdのURL：
    - https://github.com/cat2151/github-actions/blob/main/generated-docs/development-status.md
- そのmdをGitHub上でdecodeして閲覧したときのURL、404である：
    - https://github.com/cat2151/github-actions/blob/main/generated-docs/issue-notes/16.md
- そのmdに実際に含まれるURL：
    - issue-notes/16.md
- あるべきURL：
    - https://github.com/cat2151/github-actions/blob/main/issue-notes/16.md
- あるべきURLがmdにどう含まれているべきか：
    - ../issue-notes/16.md

# どうする？
- 案
    - promptを修正する
    - promptの場所は：
        - .github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
    - 備考、cjs内にpromptがハードコーディングされており、promptをメンテしづらいので別途対処する : [issues #18](https://github.com/cat2151/github-actions/issues/18)

# 結果
- agentにpromptを投げた
    - ※promptは、development-statusで生成したもの
- レビューした
    - agentがフルパスで実装した、ことがわかった
- userが分析し、 ../ のほうが適切と判断した
    - ※「事例」コーナーを、あわせて修正した
- そのように指示してagentに修正させた
- testする

# 結果
- test green
- closeする

{% endraw %}
```

### .github/actions-tmp/issue-notes/18.md
```md
{% raw %}
# issue DevelopmentStatusGenerator.cjs 内に、Geminiに与えるpromptがハードコーディングされてしまっている #18
[issues #18](https://github.com/cat2151/github-actions/issues/18)

# 何が困るの？
- project把握しづらい。どこにpromptが書いてあるのか、把握しづらい。
- prompts/ にほかのpromptがあるため、方針がブレていると、読みづらい。
- 備忘、いくらテンプレートリテラルとプレースホルダーで密結合しているからとはいえ、ハードコーディングはNG。
    - それらはreplaceを使う等で楽に切り出しできるので。

# 問題のcjsの場所は？
- ファイルパス : .github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
- 関数 : generateDevelopmentStatus

# 結果
- Geminiに生成させたpromptを、agentに投げて、リファクタリングさせてみた
- ハルシネーションした。使い物にならなかった
- 人力でやる

# 結果
- test green

# closeとする


{% endraw %}
```

### .github/actions-tmp/issue-notes/19.md
```md
{% raw %}
# issue project-summary の development-status 生成時、issue-notes/ 配下のmdファイルの内容を参照させる #19
[issues #19](https://github.com/cat2151/github-actions/issues/19)

# 何が困るの？
- issue解決に向けての次の一手の内容が実態に即していないことが多い。

# 対策案
- issue-notes/ 配下のmdファイルの内容を参照させる

# 備考
- さらにmd内に書かれているfileも、project内をcjsに検索させて添付させると、よりGeminiの生成品質が向上する可能性がある。
    - [issues #20](https://github.com/cat2151/github-actions/issues/20)
- さらにproject overviewでGeminiがまとめたmdも、Geminiに与えると、よりGeminiの生成品質が向上する可能性がある。
    - [issues #21](https://github.com/cat2151/github-actions/issues/21)
- さらに、Geminiに与えたpromptをfileにしてcommit pushしておくと、デバッグに役立つ可能性がある。
    - [issues #22](https://github.com/cat2151/github-actions/issues/22)

# close条件
- issues #22 がcloseされること。
- commitされたpromptを確認し、issue-notes/ 配下のmdファイルがpromptに添付されていること、が確認できること。

# 状況
- 課題、実装したがtestができていない
- 対策、issues #22 が実装されれば、testができる
- 対策、issues #22 のcloseを待つ

# 状況
- issues #22 がcloseされた
- testできるようになった
- commitされたpromptを確認した。issue-notes/ 配下のmdファイルがpromptに添付されていること、が確認できた

# closeする

{% endraw %}
```

### .github/actions-tmp/issue-notes/20.md
```md
{% raw %}
# issue project-summary の development-status 生成時、issue-notes/ 配下のmdにファイル名が書いてあれば、そのファイル内容もpromptに添付、を試す #20
[issues #20](https://github.com/cat2151/github-actions/issues/20)

# 何が困るの？
- Geminiに次の一手を生成させるとき、cjsの内容も添付したほうが、生成品質が改善できる可能性がある。

# 案
## outputのimage
- promptが言及するfilename、について、そのfileの内容もすべてpromptに含める。
    - 軸は、projectのfilename一覧である。
        - 一覧それぞれのfilenameについて、promptで言及されているものをfile内容埋め込み、とする。
- 方向性
    - シンプルで明確なルール、曖昧さのないルールで、メンテを楽にすることを優先する
    - 余分なファイルが出てしまうが割り切ってOKとし、欠落リスクを減らせることを優先する
- 備考
    - 曖昧でメンテが必要な「documentからのfilename抽出」をやめ、
        - かわりに、逆に、「今のprojectにあるfileすべてのうち、promptで言及されているもの」を軸とする
## 実現方法の案
- project全体について、filenameと、filepath配列（複数ありうる）、のmapを取得する。そういう関数Aをまず実装する。
    - filepathは、agentが扱えるよう、github上のworkの絶対pathではなく、projectRootからの相対パス表記とする。
- そして、そのfilenameにmatchするfilepath配列について、filepathとファイル内容を記したmarkdown文字列を返却、という関数Bを実装する。
- さらに、Geminiにわたすpromptについて、前述の関数Aのfilenameそれぞれについて、prompt内を検索し、filenameが存在する場合は、そのfilenameについて、関数Bを用いてmarkdown文字列を取得する。そうして得られたmarkdown文字列群を返却する、という関数Cを実装する。
- さらに、promptの末尾に書いてあるプレースホルダー「`${file_contents}`」を、関数Cの結果で置き換える、という関数Dを実装する。
- 実際には、Geminiにわたすpromptのプレースホルダー展開は、2回にわたる必要がある。1回目でissues-note内容をpromptに埋め込む。2回目でそのpromptに対して関数Dを適用する。
## 備忘
- 上記は、agentにplanさせてレビューし、context不足と感じたら上記をメンテ、というサイクルで書いた。

# どうする？
- 上記をagentに投げる。documentやtestについてのplanもしてくるかもしれないがそこは時間の都合で省略して実施させるつもり。
- 投げた、実装させた、レビューして人力リファクタリングした
- testする

# 結果
- バグ
    - この20.mdにあるプレースホルダーが置換されてしまっている
    - issue-notesで言及されていないfileまで添付されてしまっている
- 分析
    - この20.mdにあるプレースホルダーが置換されてしまっている
        - 原因
            - 20.mdにあるプレースホルダーまで置換対象としてしまっていたため。
            - prompt全体のプレースホルダーを置換対象としてしまっていたため。
            - issue-notesを埋め込んだあとでの、プレースホルダー処理だったので、
                - 20.md が置換対象となってしまったため。
        - 対策案
            - プレースホルダーはすべて、「行頭と行末で囲まれている」ときだけ置換対象とする。
                - つまり文中やcode中のプレースホルダーは置換対象外とする。
            - さらに、2つ以上プレースホルダーが出たら想定外なので早期エラー終了させ、検知させる。
    - issue-notesで言及されていないfileまで添付されてしまっている
        - 原因
            - promptに、既にprojectの全file listが書き込まれたあとなので、
                - issue-noteで言及されていなくても、
                - promptの全file listを対象に検索してしまっている
        - 対策案の候補
            - プレースホルダー置換の順番を変更し、全file listは最後に置換する
            - file添付の対象を変更し、promptでなく、issue-notesとする
                - これが範囲が絞られているので安全である、と考える
        - 備忘
            - 全fileの対象は、リモートリポジトリ側のfileなので、secretsの心配はないし、実際に検索して確認済み

# どうする？
- agent半分、人力が半分（agentがハルシネーションでソース破壊したので、関数切り分けしたり、リファクタリングしたり）。
- で実装した。
- testする

# 結果
- test green

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/21.md
```md
{% raw %}
# issue project-summary の development-status 生成時、project-overviewが生成済みのproject-overview.mdもpromptに添付、を試す #21
[issues #21](https://github.com/cat2151/github-actions/issues/21)

# 何が困るの？
- project-overview.mdがpromptに添付されていたほうが、Geminiの生成品質が改善できる可能性がある。
    - メリットは、ファイル一覧、関数一覧、をGeminiにわたせること

# 検討事項
- 課題、その一覧に付記されている「ファイルや関数の要約」は、Geminiが「ファイル名や関数名を元に生成しただけ」で、「ファイル内容や関数内容を参照せずに生成した」可能性が高い
    - 対策、project-overview.mdに依存しない。
        - 方法、新規関数をagentに実装させる
            - 新規関数で、ファイル一覧と関数一覧を生成する
        - 根拠、そのほうが、シンプルに目的を達成できる可能性が高そう。
        - 根拠、project-overview.mdだと、不具合として.github 配下のymlがlistに含まれておらず、ymlに関するissue、に関する生成、をするとき不具合の可能性がありそう。そういった、別機能の不具合に影響されがち。
- 課題、早期に実施したほうが毎日好影響が出る可能性がある
    - 対策、上記検討事項の対処は後回しにして、先に実装してみる
    - agentに投げる
- 課題、ProjectSummaryCoordinator をみたところ、並列処理されている
    - なので、project-overview.mdを参照したいときに、まだ生成されていない、という可能性が高い
    - 対策、前述の、新規関数で、ファイル一覧と関数一覧を生成させる

# agentに投げるための整理
- 編集対象ファイル
    - prompt
        - .github_automation/project_summary/prompts/development-status-prompt.md
        - 編集内容
            - projectのファイル一覧を埋め込む用の、プレースホルダーを追加する
    - source
        - .github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
        - 編集内容
            - projectのファイル一覧を生成する関数、を実装し、
            - それを前述のプレースホルダーに埋め込む

# agentに投げて実装させた

# test結果
- 以下が不要
    - .git/
    - node_modules/

# どうする？
- agentに上記を変更させた
- testする

# 結果
- test greenとなった

# まとめ
- issueのtitleからは仕様変更した。
    - projectのfile一覧をpromptに含める、とした。
    - そのほうがpromptとして、よい生成結果が期待できる、と判断した。
- test greenとなった

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/3.md
```md
{% raw %}
# issue GitHub Actions「issue note生成」を共通ワークフロー化する #3
[issues #3](https://github.com/cat2151/github-actions/issues/3)

- 前提
  - userは、translateのworkflowを共通ワークフロー化し、動くようにしたので、共通ワークフロー化の知識がある
  - よって、今回は、promptでplanさせ、そのplanをuserがレビューし、planの改善を指示できるので、ハルシネーションのリスクを下げることができる

- prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlはまだ共通ワークフロー化されず、
呼び出し元からコピーされたままの状態です。
このymlファイル名のまま、
中身を共通ワークフロー化してください。
呼び出し元ymlは別途書きますので気にしないでください。
まずplanしてください
```

- plan 生成結果
```
共通ワークフロー化のためのplanは以下の通りです。

1. 目的
現在の issue-note.yml を「共通ワークフロー」として再設計し、他リポジトリや他ワークフローから workflow_call で呼び出せるようにする。
2. 主な変更点
on: トリガーを workflow_call に変更（直接イベントで起動しない）。
必要な入力（inputs）やシークレット（secrets）を workflow_call セクションで定義。
github.event.issue などのイベント依存部分を、inputs経由で受け取る形に修正。
既存の処理（issueノート生成、コミット、issue body更新）はそのまま活かすが、パラメータ化する。
3. 具体的な修正内容
on: issues: → on: workflow_call: に変更。
inputs で必要な値（例: issue_title, issue_number, issue_body, issue_url など）を受け取る。
スクリプト内の ${{ github.event.issue.xxx }} を ${{ inputs.xxx }} に置換。
github-script ステップも同様にinputsを参照。
必要に応じて secrets も workflow_call で受け取れるようにする。
4. ファイル名・配置
ファイル名はそのまま issue-note.yml。
配置場所は workflows でOK（共通ワークフローとしてもこの場所で良い）。
5. 呼び出し元への影響
呼び出し元ワークフローで uses: <repo>/.github/workflows/issue-note.yml@main のように参照し、inputsを渡す形になる。
このplanで進めます。次に、実際の共通ワークフロー化のためのyml修正案を作成しますか？
```

- userがplanレビューし、OKを出した
- agentに生成結果ymlをレビューさせた、OKとなった
  - userはこのレビューは認知負荷が高いのでskipとする（単純な細かい間違い探し作業になるので、それはagentに任せた）
-  エラー。呼び出し元ymlのエラーがあり修正。共通ワークフローのエラー指摘があり、共通ワークフロー側の修正が必要となった
```
原因の分析：

呼び出し元（tonejs-mml-to-jsonのissue-note.yml）は、reusable workflow（cat2151/github-actions/.github/workflows/issue-note.yml@main）をusesで呼び出し、inputsをwithで渡しています。
共通ワークフロー（reusable workflow）は inputs を workflow_call で受け取り、各ステップで ${{ inputs.xxx }} で参照しています。
しかし、エラーが発生したのは actions/github-script@v7 の script 内です。
エラーの本質的な原因：

actions/github-script の script: で ${{ inputs.issue_number }} のような式を直接 JavaScript テンプレートリテラル内で使っていますが、これはサポートされていません。
script: の中は JavaScript であり、YAMLの ${{ ... }} は展開されません（または意図通りに展開されません）。
そのため、inputs 変数は script 内の JavaScript で未定義となり、ReferenceError になります。
正しい書き方：

actions/github-script の script: でワークフローの inputs を使いたい場合は、with: で input を environment 変数や script の引数として渡し、JavaScript で process.env から取得するか、github-script の input 機能を使う必要があります。
例（修正版）：

このように、inputs の値を env で渡し、script 内では process.env で参照してください。

まとめ：

actions/github-script の script 内で ${{ inputs.xxx }} を直接使うと ReferenceError になる
env: で値を渡し、process.env で参照するのが正しい
修正が必要です。修正版のワークフローが必要な場合はお知らせください。
```

- test green
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/4.md
```md
{% raw %}
# issue GitHub Actions「project概要生成」を共通ワークフロー化する #4
[issues #4](https://github.com/cat2151/github-actions/issues/4)

# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/daily-project-summary.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-daily-project-summary.yml
まずplanしてください
```

# 結果、あちこちハルシネーションのあるymlが生成された
- agentの挙動があからさまにハルシネーション
    - インデントが修正できない、「失敗した」という
    - 構文誤りを認識できない
- 人力で修正した

# このagentによるセルフレビューが信頼できないため、別のLLMによるセカンドオピニオンを試す
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 呼び出し元

name: Call Daily Project Summary

on:
  schedule:
    # 日本時間 07:00 (UTC 22:00 前日)
    - cron: '0 22 * * *'
  workflow_dispatch:

jobs:
  call-daily-project-summary:
    uses: cat2151/github-actions/.github/workflows/daily-project-summary.yml
    secrets:
      GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}

--- 共通ワークフロー
name: Daily Project Summary
on:
  workflow_call:

jobs:
  generate-summary:
    runs-on: ubuntu-latest

    permissions:
      contents: write
      issues: read
      pull-requests: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          fetch-depth: 0  # 履歴を取得するため

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: |
          # 一時的なディレクトリで依存関係をインストール
          mkdir -p /tmp/summary-deps
          cd /tmp/summary-deps
          npm init -y
          npm install @google/generative-ai @octokit/rest
          # generated-docsディレクトリを作成
          mkdir -p $GITHUB_WORKSPACE/generated-docs

      - name: Generate project summary
        env:
          GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          GITHUB_REPOSITORY: ${{ github.repository }}
          NODE_PATH: /tmp/summary-deps/node_modules
        run: |
          node .github/scripts/generate-project-summary.cjs

      - name: Check for generated summaries
        id: check_summaries
        run: |
          if [ -f "generated-docs/project-overview.md" ] && [ -f "generated-docs/development-status.md" ]; then
            echo "summaries_generated=true" >> $GITHUB_OUTPUT
          else
            echo "summaries_generated=false" >> $GITHUB_OUTPUT
          fi

      - name: Commit and push summaries
        if: steps.check_summaries.outputs.summaries_generated == 'true'
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          # package.jsonの変更のみリセット（generated-docsは保持）
          git restore package.json 2>/dev/null || true
          # サマリーファイルのみを追加
          git add generated-docs/project-overview.md
          git add generated-docs/development-status.md
          git commit -m "Update project summaries (overview & development status)"
          git push

      - name: Summary generation result
        run: |
          if [ "${{ steps.check_summaries.outputs.summaries_generated }}" == "true" ]; then
            echo "✅ Project summaries updated successfully"
            echo "📊 Generated: project-overview.md & development-status.md"
          else
            echo "ℹ️ No summaries generated (likely no user commits in the last 24 hours)"
          fi
```

# 上記promptで、2つのLLMにレビューさせ、合格した

# 細部を、先行する2つのymlを参照に手直しした

# ローカルtestをしてからcommitできるとよい。方法を検討する
- ローカルtestのメリット
    - 素早く修正のサイクルをまわせる
    - ムダにgit historyを汚さない
        - これまでの事例：「実装したつもり」「エラー。修正したつもり」「エラー。修正したつもり」...（以降エラー多数）
- 方法
    - ※検討、WSL + act を環境構築済みである。test可能であると判断する
    - 呼び出し元のURLをコメントアウトし、相対パス記述にする
    - ※備考、テスト成功すると結果がcommit pushされる。それでよしとする
- 結果
    - OK
    - secretsを簡略化できるか試した、できなかった、現状のsecrets記述が今わかっている範囲でベストと判断する
    - OK

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/7.md
```md
{% raw %}
# issue issue note生成できるかのtest用 #7
[issues #7](https://github.com/cat2151/github-actions/issues/7)

- 生成できた
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/8.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズ生成の対象ソースファイルを、呼び出し元ymlで指定できるようにする #8
[issues #8](https://github.com/cat2151/github-actions/issues/8)

# これまでの課題
- 以下が決め打ちになっていた
```
  const allowedFiles = [
    'src/main.js',
    'src/mml2json.js',
    'src/play.js'
  ];
```

# 対策
- 呼び出し元ymlで指定できるようにする

# agent
- agentにやらせることができれば楽なので、初手agentを試した
- 失敗
    - ハルシネーションしてscriptを大量破壊した
- 分析
    - 修正対象scriptはagentが生成したもの
    - 低品質な生成結果でありソースが巨大
    - ハルシネーションで破壊されやすいソース
    - AIの生成したソースは、必ずしもAIフレンドリーではない

# 人力リファクタリング
- 低品質コードを、最低限agentが扱えて、ハルシネーションによる大量破壊を防止できる内容、にする
- 手短にやる
    - そもそもビジュアライズは、agentに雑に指示してやらせたもので、
    - 今後別のビジュアライザを選ぶ可能性も高い
    - 今ここで手間をかけすぎてコンコルド効果（サンクコストバイアス）を増やすのは、project群をトータルで俯瞰して見たとき、損
- 対象
    - allowedFiles のあるソース
        - callgraph-utils.cjs
            - たかだか300行未満のソースである
            - この程度でハルシネーションされるのは予想外
            - やむなし、リファクタリングでソース分割を進める

# agentに修正させる
## prompt
```
allowedFilesを引数で受け取るようにしたいです。
ないならエラー。
最終的に呼び出し元すべてに波及して修正したいです。

呼び出し元をたどってエントリポイントも見つけて、
エントリポイントにおいては、
引数で受け取ったjsonファイル名 allowedFiles.js から
jsonファイル allowedFiles.jsonの内容をreadして
変数 allowedFilesに格納、
後続処理に引き渡す、としたいです。

まずplanしてください。
planにおいては、修正対象のソースファイル名と関数名を、呼び出し元を遡ってすべて特定し、listしてください。
```

# 修正が順調にできた
- コマンドライン引数から受け取る作りになっていなかったので、そこだけ指示して修正させた
- yml側は人力で修正した

# 他のリポジトリから呼び出した場合にバグらないよう修正する
- 気付いた
    - 共通ワークフローとして他のリポジトリから使った場合はバグるはず。
        - ymlから、共通ワークフロー側リポジトリのcheckoutが漏れているので。
- 他のyml同様に修正する
- あわせて全体にymlをリファクタリングし、修正しやすくし、今後のyml読み書きの学びにしやすくする

# local WSL + act : test green

# closeとする
- もし生成されたhtmlがNGの場合は、別issueとするつもり

{% endraw %}
```

### .github/actions-tmp/issue-notes/9.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズが0件なので、原因を可視化する #9
[issues #9](https://github.com/cat2151/github-actions/issues/9)

# agentに修正させたり、人力で修正したりした
- agentがハルシネーションし、いろいろ根の深いバグにつながる、エラー隠蔽などを仕込んでいたため、検知が遅れた
- 詳しくはcommit logを参照のこと
- WSL + actの環境を少し変更、act起動時のコマンドライン引数を変更し、generated-docsをmountする（ほかはデフォルト挙動であるcpだけにする）ことで、デバッグ情報をコンテナ外に出力できるようにし、デバッグを効率化した

# test green

# closeとする

{% endraw %}
```

### .github/scripts/generate_test_failure_issue.py
```py
{% raw %}
#!/usr/bin/env python3
"""
Generate issue body text for CI test failures.

This script generates the issue body for GitHub issues created when
Windows CI tests fail or time out.
"""

import argparse
import sys
from typing import Optional


def generate_issue_body(
    status_ja: str,
    total_tests: str,
    passed: str,
    failed: str,
    timed_out: str,
    failed_tests_categorized: str,
    workflow: str,
    job: str,
    run_id: str,
    run_attempt: str,
    ref: str,
    commit: str,
    server_url: str,
    repository: str,
    error_log: Optional[str] = None,
) -> str:
    """
    Generate the issue body text for a test failure.
    
    Args:
        status_ja: Status in Japanese (e.g., "失敗" or "タイムアウトによりキャンセル")
        total_tests: Total number of tests run
        passed: Number of passed tests
        failed: Number of failed tests
        timed_out: Number of timed out tests
        failed_tests_categorized: Categorized list of failed tests (markdown formatted)
        workflow: GitHub workflow name
        job: GitHub job name
        run_id: GitHub run ID
        run_attempt: GitHub run attempt number
        ref: GitHub ref (branch/tag)
        commit: GitHub commit SHA
        server_url: GitHub server URL
        repository: GitHub repository (owner/repo)
        error_log: Optional detailed error log
    
    Returns:
        The formatted issue body text
    """
    
    # Build the main sections
    sections = []
    
    # Header
    sections.append("Windows CI でビルドまたはテストに失敗しました。")
    sections.append("")
    sections.append(f"**ステータス**: {status_ja}")
    sections.append("")
    
    # Test Summary
    sections.append("## 失敗テストサマリー")
    sections.append("")
    sections.append(f"**総テスト数**: {total_tests}")
    sections.append(f"**成功**: {passed}")
    sections.append(f"**失敗**: {failed}")
    sections.append(f"**タイムアウト**: {timed_out}")
    sections.append("")
    
    # Failed Tests List
    sections.append("### 失敗したテスト一覧")
    sections.append(failed_tests_categorized)
    sections.append("")
    
    # Log Link
    sections.append("## ログへのリンク")
    sections.append(f"{server_url}/{repository}/actions/runs/{run_id}")
    sections.append("")
    
    # Details
    sections.append("## 詳細")
    sections.append(f"- Workflow: {workflow}")
    sections.append(f"- Job: {job}")
    sections.append(f"- Run ID: {run_id}")
    sections.append(f"- Run Attempt: {run_attempt}")
    sections.append(f"- Ref: {ref}")
    sections.append(f"- Commit: {commit}")
    sections.append("")
    
    # Detailed Error Log (if provided)
    if error_log and error_log.strip():
        sections.append("## 詳細なエラーログ")
        sections.append("<details>")
        sections.append("<summary>クリックして展開</summary>")
        sections.append("")
        sections.append("```")
        sections.append(error_log)
        sections.append("```")
        sections.append("")
        sections.append("</details>")
        sections.append("")
    
    # Artifacts
    sections.append("## アーティファクト")
    sections.append("完全なログは上記リンクの「Artifacts」セクションから `test-logs` をダウンロードしてください。")
    
    return "\n".join(sections)


def main():
    """Main entry point for the script."""
    parser = argparse.ArgumentParser(
        description="Generate issue body text for CI test failures"
    )
    
    parser.add_argument(
        "--status-ja",
        required=True,
        help="Status in Japanese (e.g., '失敗' or 'タイムアウトによりキャンセル')"
    )
    parser.add_argument(
        "--total-tests",
        required=True,
        help="Total number of tests run"
    )
    parser.add_argument(
        "--passed",
        required=True,
        help="Number of passed tests"
    )
    parser.add_argument(
        "--failed",
        required=True,
        help="Number of failed tests"
    )
    parser.add_argument(
        "--timed-out",
        required=True,
        help="Number of timed out tests"
    )
    parser.add_argument(
        "--failed-tests-categorized",
        required=True,
        help="Categorized list of failed tests (markdown formatted)"
    )
    parser.add_argument(
        "--workflow",
        required=True,
        help="GitHub workflow name"
    )
    parser.add_argument(
        "--job",
        required=True,
        help="GitHub job name"
    )
    parser.add_argument(
        "--run-id",
        required=True,
        help="GitHub run ID"
    )
    parser.add_argument(
        "--run-attempt",
        required=True,
        help="GitHub run attempt number"
    )
    parser.add_argument(
        "--ref",
        required=True,
        help="GitHub ref (branch/tag)"
    )
    parser.add_argument(
        "--commit",
        required=True,
        help="GitHub commit SHA"
    )
    parser.add_argument(
        "--server-url",
        required=True,
        help="GitHub server URL"
    )
    parser.add_argument(
        "--repository",
        required=True,
        help="GitHub repository (owner/repo)"
    )
    parser.add_argument(
        "--error-log",
        default="",
        help="Optional detailed error log"
    )
    
    args = parser.parse_args()
    
    issue_body = generate_issue_body(
        status_ja=args.status_ja,
        total_tests=args.total_tests,
        passed=args.passed,
        failed=args.failed,
        timed_out=args.timed_out,
        failed_tests_categorized=args.failed_tests_categorized,
        workflow=args.workflow,
        job=args.job,
        run_id=args.run_id,
        run_attempt=args.run_attempt,
        ref=args.ref,
        commit=args.commit,
        server_url=args.server_url,
        repository=args.repository,
        error_log=args.error_log,
    )
    
    print(issue_body)
    return 0


if __name__ == "__main__":
    sys.exit(main())

{% endraw %}
```

### .github/workflows/build_windows.yml
```yml
{% raw %}
name: Windows CI

on:
  workflow_dispatch:

jobs:
  build-windows:
    runs-on: windows-latest
    timeout-minutes: 30
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry/index
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo dependencies
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry/cache
          key: ${{ runner.os }}-cargo-deps-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache target directory
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Build code
        run: cargo build --locked --verbose

      - name: Install cargo-nextest
        uses: taiki-e/install-action@nextest

      - name: Run tests with nextest
        id: test
        # 15分でタイムアウト (ジョブ全体の30分タイムアウトより短く設定)
        # テストがハングした場合でも、issue作成などの後処理を確実に実行するため
        timeout-minutes: 15
        continue-on-error: true
        run: |
          # nextest: 設定ファイル(.config/nextest.toml)でタイムアウト、fail-fast、failure-outputを設定済み
          cargo nextest run 2>&1 | Tee-Object -FilePath test_output.log
          exit $LASTEXITCODE

      - name: Parse test results
        if: always() && (steps.test.outcome == 'failure' || steps.test.outcome == 'cancelled' || steps.test.outcome == 'timed_out')
        id: test_summary
        shell: pwsh
        run: |
          # テスト結果を解析して構造化データを抽出
          if (Test-Path test_output.log) {
            $content = Get-Content -Path test_output.log -Raw -ErrorAction SilentlyContinue
            
            # 統計情報を抽出（nextest summary行から）
            $total = "不明"
            $passed = "不明"
            $failed = "不明"
            $timedOut = "不明"
            
            if ($content -match 'Summary.*?(\d+) tests run: (\d+) passed, (\d+) failed(?:, (\d+) timed out)?') {
              $total = $matches[1]
              $passed = $matches[2]
              $failed = $matches[3]
              $timedOut = if ($matches[4]) { $matches[4] } else { "0" }
            }
            
            # 失敗したテスト名を抽出
            $failedTestsList = @()
            $failMatches = [regex]::Matches($content, 'FAIL\s+(?:\[[\d.]+s\])?\s+\([\d/]+\)\s+([\w:]+(?:::[\w:]+)*)')
            foreach ($match in $failMatches) {
              $testName = $match.Groups[1].Value
              if ($testName -and $failedTestsList -notcontains $testName) {
                $failedTestsList += $testName
              }
            }
            
            # タイムアウトしたテストを抽出
            $timeoutMatches = [regex]::Matches($content, 'TIMEOUT\s+\[[\d.]+s\]\s+\([\d/]+\)\s+([\w:]+(?:::[\w:]+)*)')
            foreach ($match in $timeoutMatches) {
              $testName = $match.Groups[1].Value
              if ($testName -and $failedTestsList -notcontains $testName) {
                $failedTestsList += "$testName (タイムアウト)"
              }
            }
            
            # テストをカテゴリ別に分類
            $categorized = @{
              "Pipe Tests" = @()
              "CLI Integration Tests" = @()
              "Client Integration Tests" = @()
              "Interactive Mode Tests" = @()
              "Server Integration Tests" = @()
              "その他" = @()
            }
            
            foreach ($test in $failedTestsList) {
              if ($test -match 'pipe.*test|test.*pipe') {
                $categorized["Pipe Tests"] += $test
              } elseif ($test -match 'cli_integration') {
                $categorized["CLI Integration Tests"] += $test
              } elseif ($test -match 'client.*test|test.*client') {
                $categorized["Client Integration Tests"] += $test
              } elseif ($test -match 'interactive') {
                $categorized["Interactive Mode Tests"] += $test
              } elseif ($test -match 'server.*test|test.*server') {
                $categorized["Server Integration Tests"] += $test
              } else {
                $categorized["その他"] += $test
              }
            }
            
            # カテゴリ別リストを作成
            $categorizedList = ""
            foreach ($category in $categorized.Keys | Sort-Object) {
              $tests = $categorized[$category]
              if ($tests.Count -gt 0) {
                $categorizedList += "`n#### $category ($($tests.Count)件)`n"
                foreach ($test in $tests) {
                  $categorizedList += "- $test`n"
                }
              }
            }
            
            # GITHUB_OUTPUTに出力
            "total_tests=$total" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            "passed=$passed" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            "failed=$failed" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            "timed_out=$timedOut" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            
            "failed_tests_categorized<<EOF" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            $categorizedList | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            "EOF" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
          }

      - name: Capture test failure summary
        if: always() && (steps.test.outcome == 'failure' || steps.test.outcome == 'cancelled' || steps.test.outcome == 'timed_out')
        id: test_output
        shell: pwsh
        run: |
          # GitHub issue用にログを取得（最大65000文字）
          if (Test-Path test_output.log) {
            $content = Get-Content -Path test_output.log -Raw -ErrorAction SilentlyContinue
            if ($content -and $content.Length -gt 65000) {
              $content = $content.Substring($content.Length - 65000)
            }
            if ($content) {
              "log<<EOF" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
              $content | Out-File -FilePath $env:GITHUB_OUTPUT -Append
              "EOF" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            }
          }

      - name: Upload test log artifacts
        if: always() && (steps.test.outcome == 'failure' || steps.test.outcome == 'cancelled' || steps.test.outcome == 'timed_out')
        uses: actions/upload-artifact@v4
        with:
          name: test-logs
          path: test_output.log
          retention-days: 30

      - name: Determine failure status
        if: always() && (steps.test.outcome == 'failure' || steps.test.outcome == 'cancelled' || steps.test.outcome == 'timed_out')
        id: failure_status
        shell: pwsh
        run: |
          # cancelled: ユーザーによる手動キャンセル
          # timed_out: timeout-minutesによる自動タイムアウト
          # どちらも「タイムアウト」としてユーザーに報告
          if ("${{ steps.test.outcome }}" -eq "cancelled" -or "${{ steps.test.outcome }}" -eq "timed_out") {
            "status_en=timed out" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            "status_ja=タイムアウトによりキャンセル" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
          } else {
            "status_en=failed" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
            "status_ja=失敗" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
          }

      - name: Generate issue body
        if: always() && (steps.test.outcome == 'failure' || steps.test.outcome == 'cancelled' || steps.test.outcome == 'timed_out')
        id: issue_body
        shell: pwsh
        run: |
          # Python scriptを使用してissue本文を生成
          $issueBody = python3 .github/scripts/generate_test_failure_issue.py `
            --status-ja "${{ steps.failure_status.outputs.status_ja }}" `
            --total-tests "${{ steps.test_summary.outputs.total_tests }}" `
            --passed "${{ steps.test_summary.outputs.passed }}" `
            --failed "${{ steps.test_summary.outputs.failed }}" `
            --timed-out "${{ steps.test_summary.outputs.timed_out }}" `
            --failed-tests-categorized "${{ steps.test_summary.outputs.failed_tests_categorized }}" `
            --workflow "${{ github.workflow }}" `
            --job "${{ github.job }}" `
            --run-id "${{ github.run_id }}" `
            --run-attempt "${{ github.run_attempt }}" `
            --ref "${{ github.ref }}" `
            --commit "${{ github.sha }}" `
            --server-url "${{ github.server_url }}" `
            --repository "${{ github.repository }}" `
            --error-log "${{ steps.test_output.outputs.log }}"
          
          # GITHUB_OUTPUTに出力
          "body<<EOF" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
          $issueBody | Out-File -FilePath $env:GITHUB_OUTPUT -Append
          "EOF" | Out-File -FilePath $env:GITHUB_OUTPUT -Append

      - name: Create issue on failure
        if: always() && (steps.test.outcome == 'failure' || steps.test.outcome == 'cancelled' || steps.test.outcome == 'timed_out')
        uses: dacbd/create-issue-action@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          title: "[CI] Windows build or test ${{ steps.failure_status.outputs.status_en }}"
          body: ${{ steps.issue_body.outputs.body }}
          labels: "ci,windows,auto-generated"

{% endraw %}
```

### issue-notes/117.md
```md
{% raw %}
# issue client側のdemo interactive modeで、clientからserverへの送信ごとにフレーズ開始タイミングがブレる #117
[issues #117](https://github.com/cat2151/ym2151-log-play-server/issues/117)

# 原因分析
- 時刻指定に原因がある考え
    - clientは時刻指定せずにjson送信してる
        - 開始time 0.0 のjson
    - 受信したサーバー側は、
        - 未来オフセットを加算して、スケジューリングしてる
    - よって名前付きパイプのブレでモタる
# どうする？
- 分析
    - 切り分け
        - client demo interactiveは、モタらないシーケンス演奏をしたいdemoである
        - tone editorは、最速で音を変更したい
            - まず、今の各モードが鳴るところまで持っていく
                - で、どれくらい使ってて問題が出るか？を可視化する
                    - これが重要
- 結論
    - ym2151 tone editorにおいて以下を確認する
        - 通常モードとインタラクティブモードで音が鳴ること
        - 問題あれば、どのような問題があるか？をissueに可視化すること


{% endraw %}
```

### issue-notes/118.md
```md
{% raw %}
# issue agentがPRしたWindows用codeが、TDDされていないためハルシネーション検知と修正がされずビルドが通らない #118
[issues #118](https://github.com/cat2151/ym2151-log-play-server/issues/118)

# 何が困るの？
- 開発体験が悪い
  - ほかのprojectの事例
    - PRをいくつか、検証なしで受け入れ
    - 結果、そのまま使える
      - 分析、TDDで品質担保されており、ハルシネーションはほぼない
        - Windows用codeがないマルチプラットフォームprojectなので、agentがTDDしておりcode品質が高い
      - 開発体験が良い
  - このprojectの事例
    - PRをいくつか、検証なしで受け入れ
    - 結果、ビルドが通らない
      - 複数のPRすべてがハルシネーション
      - userが修正時、複数PRのバグが混ざっており切り分けコストがかかる
      - 開発体験が悪い
    - 分析、このprojectのWindows用codeの品質は低い
      - 規模が大きくなってくるにつれ、指数関数的に品質低下が起こっている感触がある
      - ハルシネーションがどんどん増えている

# 対策案
- cargo check target ～gnu
  - WSLで動作確認済み
  - GitHub Copilot Coding Agentでも実施できる可能性がある

# 方法の案
## まずGitHub Actions
- 上記をworkflow作成し、GitHub Actions Linux Runnerで動作確認する
- logでcargo check finished目視確認または、より効率的な確認

# 草稿
- ゴール
    - GitHubのLinux Runner上での GitHub Copilot Coding Agent によるTDDにおいて、以下をPRコメントに書くこと
        - ※大前提。GitHub Linux Runner上での話である。現状、GitHub Copilot Coding Agent はそれしか使えないので。Windows Runnerは使えない。
        - 最低限、Rustのコンパイルチェックで、
            - Windows版コンパイルがエラーとwarningのない状態
            - （code、unit test、統合testすべて）
            - をTDDで実現できるか？
            - その方法は？
                - cargo check target gnuを使う？
                - crossを使う？
                - cargo-xwinを使う？
                - ほかに方法は？
            - agentへのprompt指示だけで実現可能か？
                - そのpromptは？
            - これをweb調査してまとめること
        - もしどうしようもないなら、
            - 一つのPRにつき毎回、userがWindows版の手動ビルドを
                - するしかない？
                - ※今回、3つのPRを「userがWindows版の手動ビルドをせず」受け入れた結果、
                    - 3つともハルシネーションによる認識誤りによるバグや実装漏れがあった
                    - ビルドが通らない、testがfailed、
                        - ビルドを通した以降もバグっている、
                            - という低品質codeだった
                - つまりGitHub Copilot Coding Agentの自律的なactionではどうにもならない？
                    - ※もしCI/CDでGitHub Actionsで、Windows版のコンパイルが通るかチェックしたところで、それをagentが自律的にactionしてTDDで修正できない、というフローなら、手間をかけてやる意味が薄い
                        - それは結局、運用として、userが手動でそれをチェックしてlocalでagentをkickする、がマストになってしまい、userの手間がかかる点では大差ないので
            - これをweb調査してまとめること

# 状況
- 検討中

{% endraw %}
```

### issue-notes/119.md
```md
{% raw %}
# issue server commandのうち、get interactive modeは不要になったので削除し、シンプル化する #119
[issues #119](https://github.com/cat2151/ym2151-log-play-server/issues/119)



{% endraw %}
```

### issue-notes/120.md
```md
{% raw %}
# issue server commandのうち、clear scheduleを廃止し、play json with clear scheduleにする（そのjsonのsample時刻より過去のscheduleだけ削除する） #120
[issues #120](https://github.com/cat2151/ym2151-log-play-server/issues/120)



{% endraw %}
```

### issue-notes/121.md
```md
{% raw %}
# issue コマンドライン引数の表示パターンが2パターンあり（help時、不明なオプション時）、どちらも--demo-interactiveが表示されず、userが混乱する #121
[issues #121](https://github.com/cat2151/ym2151-log-play-server/issues/121)



{% endraw %}
```

### issue-notes/138.md
```md
{% raw %}
# issue PR 137 のagentの挙動（初手の対策案が誤っており、userがより深く分析させたら正しい対策案に到達した）はハルシネーションの可能性がある。対策案を洗い出して整理する #138
[issues #138](https://github.com/cat2151/ym2151-log-play-server/issues/138)

# 何が困るの？
- PR 137はラッキーだっただけ
- もっと深刻な潜在的な「アーキテクチャ誤り、仕様誤り、バグ」をagentが生成してしまうリスクがある
- つまり大きな開発コスト増大リスクがある

# 対策案は？
- 様子見。例えば、あと2回同様の「agentがハルシネーション的誤り。しかもuserもうっかり素通りさせるところだった」が発生したら、さらに検討する
- CIエラーログの縮小。今回50KB超のサイズである。エラー部分だけにして縮小できるか検討する。
  - 課題、見込みが低そう。agentは結局CIログを全量readしにいきそう。
- ひとまず様子見とする

{% endraw %}
```

### issue-notes/143.md
```md
{% raw %}
# issue build_windows.ymlでtest failed時のissue生成機能について、issue先頭に、geminiを利用してtestエラーメッセージ群を日本語訳したものを追加し、userの認知負荷を下げる #143
[issues #143](https://github.com/cat2151/ym2151-log-play-server/issues/143)



{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
952e137 Merge pull request #142 from cat2151/copilot/extract-issue-text-generation
c078a87 Add issue note for #143 [auto]
6964680 Add unit tests and documentation for issue generation script
8089618 Extract issue text generation to Python script
b3005fc Initial plan
ffb7dad Add issue note for #141 [auto]
4ed9b1d fix: testでフリーズしていたのを修正。名前付きパイプへの接続を使用後に閉じてから次のtestをするよう修正した
df7815f fix: test_client_option_without_argument_fails - remove stderr message assertion
695b13d fix: test_server_shutdown_without_server_fails - remove stderr message assertion
614447e fix: test_client_without_server_fails - remove stderr message assertion

### 変更されたファイル:
.github/scripts/README.md
.github/scripts/generate_test_failure_issue.py
.github/scripts/test_generate_test_failure_issue.py
.github/workflows/build_windows.yml
.gitignore
issue-notes/141.md
issue-notes/143.md
tests/cli_integration_test.rs
tests/server_integration_test.rs


---
Generated at: 2025-12-25 07:01:34 JST
