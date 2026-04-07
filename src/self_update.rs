use anyhow::{anyhow, Result};
use cat_self_update_lib::{check_remote_commit, self_update, CheckResult};

pub const BUILD_COMMIT_HASH: &str = env!("BUILD_COMMIT_HASH");
pub(crate) const REPOSITORY_URL: &str = env!("CARGO_PKG_REPOSITORY");
pub(crate) const MAIN_BRANCH: &str = "main";

pub fn check_for_updates() -> Result<CheckResult> {
    let (owner, repo) = repository_owner_and_name(REPOSITORY_URL)?;
    check_for_updates_with(owner, repo, BUILD_COMMIT_HASH, check_remote_commit)
}

pub fn run_self_update() -> Result<()> {
    let (owner, repo) = repository_owner_and_name(REPOSITORY_URL)?;
    run_self_update_with(owner, repo, self_update)
}

pub(crate) fn check_for_updates_with<F, E>(
    owner: &str,
    repo: &str,
    build_commit_hash: &str,
    checker: F,
) -> Result<CheckResult>
where
    F: FnOnce(&str, &str, &str, &str) -> std::result::Result<CheckResult, E>,
    E: std::fmt::Display,
{
    checker(owner, repo, MAIN_BRANCH, build_commit_hash)
        .map_err(|error| anyhow!("更新確認に失敗しました: {error}"))
}

pub(crate) fn run_self_update_with<F, E>(owner: &str, repo: &str, updater: F) -> Result<()>
where
    F: FnOnce(&str, &str, &[&str]) -> std::result::Result<(), E>,
    E: std::fmt::Display,
{
    updater(owner, repo, &[]).map_err(|error| anyhow!("更新に失敗しました: {error}"))
}

pub(crate) fn repository_owner_and_name(repository_url: &str) -> Result<(&str, &str)> {
    let repository_url = repository_url.trim().trim_end_matches('/');
    let repository_path = repository_url
        .strip_prefix("https://github.com/")
        .or_else(|| repository_url.strip_prefix("http://github.com/"))
        .ok_or_else(|| anyhow!("GitHubリポジトリURLの解析に失敗しました: {repository_url}"))?;

    let mut parts = repository_path.split('/');
    let owner = parts.next().filter(|value| !value.is_empty());
    let repo = parts
        .next()
        .map(|value| value.trim_end_matches(".git"))
        .filter(|value| !value.is_empty());

    match (owner, repo, parts.next()) {
        (Some(owner), Some(repo), None) => Ok((owner, repo)),
        _ => Err(anyhow!(
            "GitHubリポジトリURLの解析に失敗しました: {repository_url}"
        )),
    }
}
