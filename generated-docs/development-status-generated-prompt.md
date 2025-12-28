Last updated: 2025-12-29

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
- .github/scripts/parse_nextest_junit.py
- .github/scripts/test_generate_test_failure_issue.py
- .github/scripts/test_parse_nextest_junit.py
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
- issue-notes/146.md
- issue-notes/148.md
- issue-notes/150.md
- issue-notes/152.md
- issue-notes/154.md
- issue-notes/156.md
- issue-notes/158.md
- issue-notes/161.md
- issue-notes/165.md
- issue-notes/167.md
- issue-notes/169.md
- issue-notes/173.md
- issue-notes/178.md
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
- src/tests/command_handler_tests.rs
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
## [Issue #179](../issue-notes/179.md): test failed
### ym2151-log-play-server::client_test::client_integration_tests::test_client_send_json

**エラー**: スレッド 'client_integration_tests::test_client_send_json' (4896) が tests\client_test.rs:62:9 でパニックしました

```
スレッド 'client_integration_tests::test_client_send_json' (4896) が tests\client_test.rs:62:9 で...
ラベル: ci, windows, auto-generated
--- issue-notes/179.md の内容 ---

```markdown

```

## [Issue #178](../issue-notes/178.md): demoがヨレないようになっているか、Windows実機で動作確認する
[issue-notes/178.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/178.md)

...
ラベル: 
--- issue-notes/178.md の内容 ---

```markdown
# issue demoがヨレないようになっているか、Windows実機で動作確認する #178
[issues #178](https://github.com/cat2151/ym2151-log-play-server/issues/178)



```

## [Issue #138](../issue-notes/138.md): （様子見中）PR 137 のagentの挙動（初手の対策案が誤っており、userがより深く分析させたら正しい対策案に到達した）はハルシネーションの可能性がある。対策案を洗い出して整理する
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

## [Issue #118](../issue-notes/118.md): （様子見中）agentがPRしたWindows用codeが、TDDされていないためハルシネーション検知と修正がされずビルドが通らない
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
- CIで日次でWindows Runnerによるtestを行わせるようにした
- 様子見中
- もしこれで運用がまわるようなら、OKとしてcloseする考え

```

## ドキュメントで言及されているファイルの内容
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
- CIで日次でWindows Runnerによるtestを行わせるようにした
- 様子見中
- もしこれで運用がまわるようなら、OKとしてcloseする考え

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

### issue-notes/178.md
```md
{% raw %}
# issue demoがヨレないようになっているか、Windows実機で動作確認する #178
[issues #178](https://github.com/cat2151/ym2151-log-play-server/issues/178)



{% endraw %}
```

### tests/client_test.rs
```rs
{% raw %}
//! Integration tests for the client module
//!
//! These tests verify that the client can send commands to a mock server using the binary protocol.

#![cfg(windows)]

mod test_util_server_mutex;

mod client_integration_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_play_server::ipc::pipe_windows::NamedPipe;
    use ym2151_log_play_server::ipc::protocol::{Command, Response};

    // Import test utilities for sequential server tests
    use super::test_util_server_mutex::server_test_lock;

    /// Helper to clean up pipe before test
    fn cleanup_pipe() {
        // On Windows, pipes are automatically cleaned up when all handles are closed
        thread::sleep(Duration::from_millis(50));
    }

    #[test]
    fn test_client_send_json() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Start a mock server in a separate thread
        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a PlayJson command with JSON data
            match cmd {
                Command::PlayJson { data } => {
                    // Verify the JSON structure
                    assert!(data.get("events").is_some());
                }
                _ => panic!("Expected PlayJson command"),
            }

            // Send OK response in binary format
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        // Give server time to start and create the pipe
        thread::sleep(Duration::from_millis(200));

        // Send JSON data from client
        let json_data = r#"{"events": [{"time": 0, "addr": "0x08", "data": "0x00"}]}"#;
        let result = ym2151_log_play_server::client::send_json(json_data);
        assert!(result.is_ok());

        // Wait for server to finish
        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_stop_playback() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a Stop command
            assert!(matches!(cmd, Command::Stop));

            // Send OK response in binary format
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::stop_playback();
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_shutdown_server() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a Shutdown command
            assert!(matches!(cmd, Command::Shutdown));

            // Send OK response in binary format
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::shutdown_server();
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_no_server() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Try to send a command when no server is running
        // This should fail with a connection error
        let result = ym2151_log_play_server::client::stop_playback();
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Failed to connect to server")
                || err_msg.contains("The system cannot find the file specified")
                || err_msg.contains("No such file or directory")
        );
    }
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
ccbe799 Add issue note for #178 [auto]
fcef1b7 force installにし、依存クレートのupdateにそれを反映できるようにした
1dd07e3 Merge pull request #177 from cat2151/copilot/fix-demo-interactive-mode-timing
d9e2ce5 Add ASAP mode support: detect first event time 0.0 for immediate playback
602db80 Fix timing jitter in interactive mode by using audio stream time instead of wall-clock time
4274a36 Initial plan
4cebd0e Update issue notes for ym2151 tone editor #117
d87ff6e Merge pull request #176 from cat2151/copilot/fix-windows-cross-compilation
a1f078d Fix Windows GNU cross-compilation errors: remove duplicate imports and update test API usage
af7fc66 Initial plan

### 変更されたファイル:
install-ym2151-tools.rs
issue-notes/117.md
issue-notes/173.md
issue-notes/178.md
src/server/command_handler.rs
src/server/mod.rs
src/tests/command_handler_tests.rs
src/tests/mod.rs


---
Generated at: 2025-12-29 07:01:46 JST
