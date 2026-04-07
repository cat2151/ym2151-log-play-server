# コミット前にやること

1. **コードフォーマット**: `cargo fmt` を実行して一貫したフォーマットを確保
2. **リンティング問題修正**: `cargo clippy` を実行して警告に対処
3. **ビルド成功**: `cargo build` (または `cargo build --release`) を実行
4. **テスト実行**: `cargo test` を実行して全テストが通ることを確認
5. **ドキュメント更新**: パブリックAPIを追加した場合、docコメントを更新

# 参考資料

- Nuked-OPM: https://github.com/nukeykt/Nuked-OPM
- YM2151仕様: Yamaha YM2151データシート

# userからの指示
- プルリクエスト
  - 日本語で書く
  - 作業報告は、プルリクエストのコメントに書く。document作成禁止
    - DRY原則に準拠し、「codeやbuild scriptと同じことを、documentに書いたせいで、そのdocumentが陳腐化してハルシネーションやuserレビューコスト増大や混乱ほか様々なトラブル原因になる」を防止する
    - なおissue-notes/は、userがissueごとの意図を記録する用途で使う
  - cat2151 のライブラリを Git 依存で参照するときは rev 固定しない
- test
  - Rustのunit testは、本体codeとは別ファイル（src/tests/配下）に書く。agentハルシネーションのリスクを下げる用。
  - test時は、test_client.logと、test_server.logも参考にすること。それをtest codeに含めてもよい。その場合はtest並列動作させず、clean upすること
  - 調査のbuild時は、`Get-Process | Where-Object {$_.ProcessName -eq "ym2151-log-play-server"} | Stop-Process -Force` して、exeのlockを解除してからbuildすること
  - 調査用に、rust-scriptを使ったscriptを書くときは、install-ym2151-tools.rsを参考に、依存クレートをscript内にコメント形式で記述すること。このときCargo.tomlファイルの変更は禁止する
  - test codeを書くとき、server起動系test codeでは、test_util_server_mutex.rsを利用して、他のtest codeと競合しないようmutex lockを取ること
