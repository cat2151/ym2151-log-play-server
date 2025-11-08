//! エラーハンドリングテスト - Phase 7
//!
//! サーバーとクライアント間でのエラー状況の適切な処理を検証

// Windows専用のテスト実装がここに配置されます
// 現在はUnix専用のテストを無効化しています

#[cfg(not(windows))]
#[test]
fn error_tests_require_windows() {
    println!("ℹ️  Error tests are designed for Windows platforms");
}
