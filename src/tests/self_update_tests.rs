use crate::self_update::{
    check_for_updates_with, run_self_update_with, BUILD_COMMIT_HASH, MAIN_BRANCH, REPO_NAME,
    REPO_OWNER,
};
use cat_self_update_lib::compare_hashes;

#[test]
fn test_check_for_updates_uses_expected_repository_metadata() {
    let result = check_for_updates_with("embedded-hash", |owner, repo, branch, embedded_hash| {
        assert_eq!(owner, REPO_OWNER);
        assert_eq!(repo, REPO_NAME);
        assert_eq!(branch, MAIN_BRANCH);
        assert_eq!(embedded_hash, "embedded-hash");
        Ok::<_, std::io::Error>(compare_hashes(embedded_hash, "remote-hash"))
    })
    .expect("check should succeed");

    assert!(!result.is_up_to_date());
    assert_eq!(result.embedded_hash, "embedded-hash");
    assert_eq!(result.remote_hash, "remote-hash");
}

#[test]
fn test_check_for_updates_propagates_checker_error() {
    let error =
        check_for_updates_with("embedded-hash", |_owner, _repo, _branch, _embedded_hash| {
            Err::<cat_self_update_lib::CheckResult, _>(std::io::Error::other("network failure"))
        })
        .expect_err("check should fail");

    assert!(error.to_string().contains("更新確認に失敗しました"));
    assert!(error.to_string().contains("network failure"));
}

#[test]
fn test_run_self_update_uses_expected_repository_metadata() {
    run_self_update_with(|owner, repo, bins| {
        assert_eq!(owner, REPO_OWNER);
        assert_eq!(repo, REPO_NAME);
        assert!(bins.is_empty());
        Ok::<_, std::io::Error>(())
    })
    .expect("update should succeed");
}

#[test]
fn test_run_self_update_propagates_updater_error() {
    let error = run_self_update_with(|_owner, _repo, _bins| {
        Err::<(), _>(std::io::Error::other("install failure"))
    })
    .expect_err("update should fail");

    assert!(error.to_string().contains("更新に失敗しました"));
    assert!(error.to_string().contains("install failure"));
}

#[test]
fn test_build_commit_hash_is_available() {
    assert!(!BUILD_COMMIT_HASH.is_empty());
}
