use anyhow::{anyhow, Result};
use cat_self_update_lib::{check_remote_commit, self_update, CheckResult};

pub const BUILD_COMMIT_HASH: &str = env!("BUILD_COMMIT_HASH");
pub(crate) const REPO_OWNER: &str = "cat2151";
pub(crate) const REPO_NAME: &str = "ym2151-log-play-server";
pub(crate) const MAIN_BRANCH: &str = "main";

pub fn check_for_updates() -> Result<CheckResult> {
    check_for_updates_with(BUILD_COMMIT_HASH, check_remote_commit)
}

pub fn run_self_update() -> Result<()> {
    run_self_update_with(self_update)
}

pub(crate) fn check_for_updates_with<F, E>(
    build_commit_hash: &str,
    checker: F,
) -> Result<CheckResult>
where
    F: FnOnce(&str, &str, &str, &str) -> std::result::Result<CheckResult, E>,
    E: std::fmt::Display,
{
    checker(REPO_OWNER, REPO_NAME, MAIN_BRANCH, build_commit_hash)
        .map_err(|error| anyhow!("更新確認に失敗しました: {error}"))
}

pub(crate) fn run_self_update_with<F, E>(updater: F) -> Result<()>
where
    F: FnOnce(&str, &str, &[&str]) -> std::result::Result<(), E>,
    E: std::fmt::Display,
{
    updater(REPO_OWNER, REPO_NAME, &[]).map_err(|error| anyhow!("更新に失敗しました: {error}"))
}
