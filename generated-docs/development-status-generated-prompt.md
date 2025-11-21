Last updated: 2025-11-22

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
- install-ym2151-tools.rs
- issue-notes/100.md
- issue-notes/101.md
- issue-notes/102.md
- issue-notes/103.md
- issue-notes/107.md
- issue-notes/110.md
- issue-notes/111.md
- issue-notes/112.md
- issue-notes/113.md
- issue-notes/114.md
- issue-notes/34.md
- issue-notes/36.md
- issue-notes/38.md
- issue-notes/40.md
- issue-notes/42.md
- issue-notes/44.md
- issue-notes/46.md
- issue-notes/48.md
- issue-notes/50.md
- issue-notes/52.md
- issue-notes/54.md
- issue-notes/56.md
- issue-notes/60.md
- issue-notes/62.md
- issue-notes/64.md
- issue-notes/66.md
- issue-notes/68.md
- issue-notes/70.md
- issue-notes/72.md
- issue-notes/74.md
- issue-notes/76.md
- issue-notes/78.md
- issue-notes/80.md
- issue-notes/82.md
- issue-notes/84.md
- issue-notes/86.md
- issue-notes/88.md
- issue-notes/90.md
- issue-notes/91.md
- issue-notes/94.md
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
## [Issue #113](../issue-notes/113.md): issue 103 「Nuked-OPMスレッドへの Windows MMCSS Pro Audio の実装と、 cpal audio_thread_priority フィーチャー有効化、を併用し、オーディオドロップアウトのリスクを低減させる」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー
[issue-notes/113.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/113.md)

...
ラベル: 
--- issue-notes/113.md の内容 ---

```markdown
# issue issue 103 「Nuked-OPMスレッドへの Windows MMCSS Pro Audio の実装と、 cpal audio_thread_priority フィーチャー有効化、を併用し、オーディオドロップアウトのリスクを低減させる」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #113
[issues #113](https://github.com/cat2151/ym2151-log-play-server/issues/113)



```

## [Issue #112](../issue-notes/112.md): issue 100 の「同一時刻レジスタ書き込み時の2sample delay を最終段でのみ行う」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー
[issue-notes/112.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/112.md)

...
ラベル: 
--- issue-notes/112.md の内容 ---

```markdown
# issue issue 100 の「同一時刻レジスタ書き込み時の2sample delay を最終段でのみ行う」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #112
[issues #112](https://github.com/cat2151/ym2151-log-play-server/issues/112)



```

## [Issue #111](../issue-notes/111.md): issue 101 の「最終段でのevent listのデータ形式は、addr data pairとする」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー
[issue-notes/111.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/111.md)

...
ラベル: 
--- issue-notes/111.md の内容 ---

```markdown
# issue issue 101 の「最終段でのevent listのデータ形式は、addr data pairとする #」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #111
[issues #111](https://github.com/cat2151/ym2151-log-play-server/issues/111)



```

## [Issue #110](../issue-notes/110.md): issue 102 の「JSONフォーマットを f64 seconds に統一する」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー
[issue-notes/110.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/110.md)

...
ラベル: 
--- issue-notes/110.md の内容 ---

```markdown
# issue issue 102 の「JSONフォーマットを f64 seconds に統一する」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #110
[issues #110](https://github.com/cat2151/ym2151-log-play-server/issues/110)



```

## [Issue #98](../issue-notes/98.md): client側のdemo interactive modeで音が崩れる
[issue-notes/98.md](https://github.com/cat2151/ym2151-log-play-server/blob/main/issue-notes/98.md)

...
ラベル: 
--- issue-notes/98.md の内容 ---

```markdown
# issue client側のdemo interactive modeで音が崩れる #98
[issues #98](https://github.com/cat2151/ym2151-log-play-server/issues/98)

# 事象は？
- ドレドレドレ...が期待値
- 実際はドレミレミレ
# server verboseを起動して、これを実行、の確認結果は？
- 問題発見
    - 同一sampleにレジスタ書き込みがあった
        - ※ログでこれを見つける = 致命的バグが発生しているという証拠
        - これはドの音が鳴らなかったことの説明になる
        - 分析、原因は、2sample delayをどの段で入れるかの考慮漏れ、対策は後述
# どうする？
- 別issueで切り分けて対処する
    - [issues #100](https://github.com/cat2151/ym2151-log-play-server/issues/100) 最終段で規定sampleぶんのdelayを入れて、
        - レジスタ書き込みが同一sample時刻に連続しないようにする
    - 今まではあちこちの段でまちまちに入れていたので、
        - こうやって漏れが発生して、
        - 根の深いトラブルの原因になっていた
# close条件は？
- 音が崩れず、期待値どおりに、ドレドレドレ…と鳴ること

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
### .github/actions-tmp/issue-notes/10.md
```md
# issue callgraph を他projectから使いやすくする #10
[issues #10](https://github.com/cat2151/github-actions/issues/10)

# ブレインストーミング
- 洗い出し
    - 他projectから使う場合の問題を洗い出す、今見えている範囲で、手早く、このnoteに可視化する
    - 洗い出したものは、一部は別issueに切り分ける
- close条件
    - [x] まずは4つそれぞれを個別のdirに切り分けてtest greenとなること、とするつもり
        - 別issueに切り分けるつもり
- 切り分け
    - 別dirに切り分ける
        - [x] 課題、`codeql-queries/` が `.github/` 配下にある。対策、`.github_automation/callgraph/codeql-queries/` とする
        - [x] 課題、scriptも、`.github/`配下にある。対策、移動する
        - 方法、agentを試し、ハルシネーションで時間が取られるなら人力に切り替える
- test
    - local WSL + act でtestする
- 名前
    - [x] 課題、名前 enhanced が不要。対策、名前から enhanced を削除してymlなどもそれぞれ同期して修正すべし
- docs
    - [x] call導入手順を書く

# 状況
- 実際に他project tonejs-mml-to-json リポジトリにて使うことができている
    - その際に発生した運用ミスは、
        - call導入手順のメンテを行ったので、改善された、と判断する

# closeとする

```

### .github/actions-tmp/issue-notes/11.md
```md
# issue translate を他projectから使いやすくする #11
[issues #11](https://github.com/cat2151/github-actions/issues/11)

# ブレインストーミング
- 課題、個別dirへの移動が必要。
    - scripts
- 課題、promptをハードコーディングでなく、promptsに切り出す。
    - さらに、呼び出し元ymlから任意のpromptsを指定できるようにする。
- 済、課題、README以外のtranslateも可能にするか検討する
    - 対策、シンプル優先でREADME決め打ちにする
        - 理由、README以外の用途となると、複数ファイルをどうGemini APIにわたすか？等、仕様が爆発的にふくらんでいくリスクがある
        - README以外の用途が明確でないうちは、README決め打ちにするほうがよい
- docs
    - call導入手順を書く

# 状況
- 上記のうち、別dirへの切り分け等は実施済みのはず
- どうする？
    - それをここに可視化する。

```

### .github/actions-tmp/issue-notes/12.md
```md
# issue project-summary を他projectから使いやすくする #12
[issues #12](https://github.com/cat2151/github-actions/issues/12)

# 保留、別projectでの検証待ちのもの
- promptsをcall側ymlで指定可能にする
  - 保留の理由
    - YAGNI原則
      - 現状の共通workflow側のpromptsで問題ないうちは、保留とする
        - そのままで使える可能性が高い見込み
      - 検証が必要
      - 別promptsを実際に書く必要が出たときに、追加実装をする
# 課題、 docs/ をメンテする
- 対象は、 daily-summary-setup.md
- call-daily-project-summary.yml の導入手順を書く
- どうする？
  - 次の日次バッチでagent用promptを生成させる
- 結果
  - 生成させた
  - 導入手順をメンテさせた
  - 人力でさらにメンテした
  - これでOKと判断する。
  - あとは必要に応じてissue起票すればよい、今すぐのissue起票は不要（YAGNI原則）、と判断する

# closeとする

```

### .github/actions-tmp/issue-notes/13.md
```md
# issue issue-note を他projectから使いやすくする #13
[issues #13](https://github.com/cat2151/github-actions/issues/13)

- docs
    - call導入手順を書く

```

### .github/actions-tmp/issue-notes/2.md
```md
# issue GitHub Actions「関数コールグラフhtmlビジュアライズ生成」を共通ワークフロー化する #2
[issues #2](https://github.com/cat2151/github-actions/issues/2)


# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-callgraph_enhanced.yml
まずplanしてください
```

# 結果
- indent
    - linter？がindentのエラーを出しているがyml内容は見た感じOK
    - テキストエディタとagentの相性問題と判断する
    - 別のテキストエディタでsaveしなおし、テキストエディタをreload
    - indentのエラーは解消した
- LLMレビュー
    - agent以外の複数のLLMにレビューさせる
    - prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューしてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 共通ワークフロー

# GitHub Actions Reusable Workflow for Call Graph Generation
name: Generate Call Graph

# TODO Windowsネイティブでのtestをしていた名残が残っているので、今後整理していく。今はWSL act でtestしており、Windowsネイティブ環境依存問題が解決した
#  ChatGPTにレビューさせるとそこそこ有用そうな提案が得られたので、今後それをやる予定
#  agentに自己チェックさせる手も、セカンドオピニオンとして選択肢に入れておく

on:
  workflow_call:

jobs:
  check-commits:
    runs-on: ubuntu-latest
    outputs:
      should-run: ${{ steps.check.outputs.should-run }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 50 # 過去のコミットを取得

      - name: Check for user commits in last 24 hours
        id: check
        run: |
          node .github/scripts/callgraph_enhanced/check-commits.cjs

  generate-callgraph:
    needs: check-commits
    if: needs.check-commits.outputs.should-run == 'true'
    runs-on: ubuntu-latest
    permissions:
      contents: write
      security-events: write
      actions: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Set Git identity
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

      - name: Remove old CodeQL packages cache
        run: rm -rf ~/.codeql/packages

      - name: Check Node.js version
        run: |
          node .github/scripts/callgraph_enhanced/check-node-version.cjs

      - name: Install CodeQL CLI
        run: |
          wget https://github.com/github/codeql-cli-binaries/releases/download/v2.22.1/codeql-linux64.zip
          unzip codeql-linux64.zip
          sudo mv codeql /opt/codeql
          echo "/opt/codeql" >> $GITHUB_PATH

      - name: Install CodeQL query packs
        run: |
          /opt/codeql/codeql pack install .github/codeql-queries

      - name: Check CodeQL exists
        run: |
          node .github/scripts/callgraph_enhanced/check-codeql-exists.cjs

      - name: Verify CodeQL Configuration
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs verify-config

      - name: Remove existing CodeQL DB (if any)
        run: |
          rm -rf codeql-db

      - name: Perform CodeQL Analysis
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs analyze

      - name: Check CodeQL Analysis Results
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs check-results

      - name: Debug CodeQL execution
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs debug

      - name: Wait for CodeQL results
        run: |
          node -e "setTimeout(()=>{}, 10000)"

      - name: Find and process CodeQL results
        run: |
          node .github/scripts/callgraph_enhanced/find-process-results.cjs

      - name: Generate HTML graph
        run: |
          node .github/scripts/callgraph_enhanced/generate-html-graph.cjs

      - name: Copy files to generated-docs and commit results
        run: |
          node .github/scripts/callgraph_enhanced/copy-commit-results.cjs

--- 呼び出し元
# 呼び出し元ワークフロー: call-callgraph_enhanced.yml
name: Call Call Graph Enhanced

on:
  schedule:
    # 毎日午前5時(JST) = UTC 20:00前日
    - cron: '0 20 * * *'
  workflow_dispatch:

jobs:
  call-callgraph-enhanced:
    # uses: cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
    uses: ./.github/workflows/callgraph_enhanced.yml # ローカルでのテスト用
```

# レビュー結果OKと判断する
- レビュー結果を人力でレビューした形になった

# test
- #4 同様にローカル WSL + act でtestする
- エラー。userのtest設計ミス。
  - scriptの挙動 : src/ がある前提
  - 今回の共通ワークフローのリポジトリ : src/ がない
  - 今回testで実現したいこと
    - 仮のソースでよいので、関数コールグラフを生成させる
  - 対策
    - src/ にダミーを配置する
- test green
  - ただしcommit pushはしてないので、html内容が0件NG、といったケースの検知はできない
  - もしそうなったら別issueとしよう

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする
- もしhtml内容が0件NG、などになったら、別issueとするつもり

```

### .github/actions-tmp/issue-notes/3.md
```md
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

```

### .github/actions-tmp/issue-notes/8.md
```md
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

```

### issue-notes/110.md
```md
# issue issue 102 の「JSONフォーマットを f64 seconds に統一する」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #110
[issues #110](https://github.com/cat2151/ym2151-log-play-server/issues/110)



```

### issue-notes/111.md
```md
# issue issue 101 の「最終段でのevent listのデータ形式は、addr data pairとする #」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #111
[issues #111](https://github.com/cat2151/ym2151-log-play-server/issues/111)



```

### issue-notes/112.md
```md
# issue issue 100 の「同一時刻レジスタ書き込み時の2sample delay を最終段でのみ行う」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #112
[issues #112](https://github.com/cat2151/ym2151-log-play-server/issues/112)



```

### issue-notes/113.md
```md
# issue issue 103 「Nuked-OPMスレッドへの Windows MMCSS Pro Audio の実装と、 cpal audio_thread_priority フィーチャー有効化、を併用し、オーディオドロップアウトのリスクを低減させる」に実装漏れと実装誤りがあり、codeとtest codeがビルドエラー #113
[issues #113](https://github.com/cat2151/ym2151-log-play-server/issues/113)



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

### issue-notes/98.md
```md
# issue client側のdemo interactive modeで音が崩れる #98
[issues #98](https://github.com/cat2151/ym2151-log-play-server/issues/98)

# 事象は？
- ドレドレドレ...が期待値
- 実際はドレミレミレ
# server verboseを起動して、これを実行、の確認結果は？
- 問題発見
    - 同一sampleにレジスタ書き込みがあった
        - ※ログでこれを見つける = 致命的バグが発生しているという証拠
        - これはドの音が鳴らなかったことの説明になる
        - 分析、原因は、2sample delayをどの段で入れるかの考慮漏れ、対策は後述
# どうする？
- 別issueで切り分けて対処する
    - [issues #100](https://github.com/cat2151/ym2151-log-play-server/issues/100) 最終段で規定sampleぶんのdelayを入れて、
        - レジスタ書き込みが同一sample時刻に連続しないようにする
    - 今まではあちこちの段でまちまちに入れていたので、
        - こうやって漏れが発生して、
        - 根の深いトラブルの原因になっていた
# close条件は？
- 音が崩れず、期待値どおりに、ドレドレドレ…と鳴ること

```

## 最近の変更（過去7日間）
### コミット履歴:
dac0301 今なにをinstall中かわかりやすくした
faf1330 demo client interactiveで2回目以降の音が崩れないよう修正
2b97c0c demo client interactiveを実態どおりにrename
fbcf59e Clippy warning対処した
5b8673f audio.rsをリファクタリングしてソース分割した。あわせてcargo fmtをかけた
c9fc96d pipe_windows.rsをリファクタリング。code規模が大きくなりハルシネーションリスクが高まっていたので、ソース分割した
4a34533 player.rsをリファクタリングし、test codeを他同様にtests/に移動することで、ハルシネーション対策とした
8f2e361 #103 をlocal windowsでagentに修正させた
543782c buildとClippyのwarningが出ていたのでagentに修正させた
18a8bd0 Enhance Windows build workflow with caching

### 変更されたファイル:
install-ym2151-tools.rs
src/audio.rs
src/audio/buffers.rs
src/audio/commands.rs
src/audio/generator.rs
src/audio/mod.rs
src/audio/player.rs
src/audio/scheduler.rs
src/audio/stream.rs
src/client/core.rs
src/client/interactive.rs
src/client/server.rs
src/demo_client_interactive.rs
src/demo_server_interactive.rs
src/demo_server_non_interactive.rs
src/ipc/mod.rs
src/ipc/pipe_windows.rs
src/ipc/windows/mod.rs
src/ipc/windows/pipe_factory.rs
src/ipc/windows/pipe_handle.rs
src/ipc/windows/pipe_reader.rs
src/ipc/windows/pipe_writer.rs
src/ipc/windows/test_logging.rs
src/lib.rs
src/main.rs
src/mmcss.rs
src/player.rs
src/server/command_handler.rs
src/server/connection.rs
src/tests/client_tests.rs
src/tests/demo_server_interactive_tests.rs
src/tests/demo_server_non_interactive_tests.rs
src/tests/events_tests.rs
src/tests/mmcss_tests.rs
src/tests/mod.rs
src/tests/play_json_interactive_tests.rs
src/tests/player_tests.rs
src/tests/wav_writer_tests.rs
tests/client_test.rs
tests/events_processing_test.rs
tests/feature_demonstration_test.rs
tests/integration_test.rs
tests/interactive/mod.rs
tests/interactive/mode_test.rs
tests/interactive/play_json_test.rs
tests/interactive/shared_mutex.rs
tests/interactive/step_by_step_test.rs
tests/server_basic_test.rs


---
Generated at: 2025-11-22 07:01:49 JST
