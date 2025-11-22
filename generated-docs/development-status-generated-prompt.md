Last updated: 2025-11-23

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
- .cargo/config.toml
- .editorconfig
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
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
- .github/workflows/build_windows.yml
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-translate-readme.yml
- .gitignore
- Cargo.lock
- Cargo.toml
- LICENSE
- README.ja.md
- README.md
- _codeql_detected_source_root
- _config.yml
- build.rs
- generated-docs/project-overview-generated-prompt.md
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

## [Issue #96](../issue-notes/96.md): インタラクティブモードで音が鳴らない
[issue-notes/96.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/96.md)

...
ラベル: 
--- issue-notes/96.md の内容 ---

```markdown
# issue インタラクティブモードで音が鳴らない #96
[issues #96](https://github.com/cat2151/ym2151-log-play-server/issues/96)

# 課題
- 事象がわかりづらい
# 対策
- 切り分ける
    - サーバ側だけで動かして切り分ける
        - サーバ側だけで動くdemo interactive modeを作る
            - 別issue参照
# close条件
- 別issueの
    - [issues #98](https://github.com/cat2151/ym2151-log-play-server/issues/98) client側のdemo interactive modeで音が崩れる
    - が解決すること
- ym2151 tone editorでインタラクティブモードを使って音が鳴ること
    - ※今はym2151 tone editor側は、デフォルトは、
        - 安全優先で、非インタラクティブモードで運用している。
            - optionでインタラクティブモードで動かすと鳴らない、という状況

```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/17.md
```md
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

```

### .github/actions-tmp/issue-notes/7.md
```md
# issue issue note生成できるかのtest用 #7
[issues #7](https://github.com/cat2151/github-actions/issues/7)

- 生成できた
- closeとする

```

### issue-notes/117.md
```md
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

### issue-notes/96.md
```md
# issue インタラクティブモードで音が鳴らない #96
[issues #96](https://github.com/cat2151/ym2151-log-play-server/issues/96)

# 課題
- 事象がわかりづらい
# 対策
- 切り分ける
    - サーバ側だけで動かして切り分ける
        - サーバ側だけで動くdemo interactive modeを作る
            - 別issue参照
# close条件
- 別issueの
    - [issues #98](https://github.com/cat2151/ym2151-log-play-server/issues/98) client側のdemo interactive modeで音が崩れる
    - が解決すること
- ym2151 tone editorでインタラクティブモードを使って音が鳴ること
    - ※今はym2151 tone editor側は、デフォルトは、
        - 安全優先で、非インタラクティブモードで運用している。
            - optionでインタラクティブモードで動かすと鳴らない、という状況

```

## 最近の変更（過去7日間）
### コミット履歴:
6d3b94a server、エラー「Not in interactive mode」のときは、stateもエラーメッセージに含むようにした
0836da1 log改善。client側もprintするものはlogにも出すようにした。これまではprintしかなくてlogとprintを交互に見る必要があって混乱を招いていた
20d0abd main.rsについて、エラーはlogにも出して、logを読みやすくするようにした。これまでは画面にしか出ないエラーがあり、logをみてもわからず混乱を招いていた。なおコマンドライン引数エラーはシンプル優先でeprintlnのままとした
8ecf2b0 log改善。print時も時刻を出すようにした。logにはprint内容も含むようにした。
a54814a printについて[デバッグ]という文言が規則性なくついており混乱を招いていたので、削除
19fbeba clientがserverにインタラクティブモード切り替えを送信したとき、serverの切り替え完了まで待つようにした
42a7f9c clientが、serverからインタラクティブモードか？を取得できるようにした
b8fff62 fix #98, Document countermeasures for issue #100
951348e Document analysis for issue #117 timing problems
e74f93d Add issue note for #117 [auto]

### 変更されたファイル:
src/audio/generator.rs
src/audio/player.rs
src/audio/stream.rs
src/client/config.rs
src/client/core.rs
src/client/interactive.rs
src/client/mod.rs
src/client/server.rs
src/debug_wav.rs
src/demo_client_interactive.rs
src/demo_server_interactive.rs
src/demo_server_non_interactive.rs
src/ipc/protocol.rs
src/logging.rs
src/main.rs
src/mmcss.rs
src/player.rs
src/server/command_handler.rs
src/server/connection.rs
src/server/mod.rs
src/server/playback.rs
src/tests/client_tests.rs
src/tests/logging_tests.rs
tests/logging_test.rs


---
Generated at: 2025-11-23 07:01:45 JST
