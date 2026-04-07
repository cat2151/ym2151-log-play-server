fn main() {
    cc::Build::new()
        .file("call_opm_clock_64times.c")
        .flag("-fwrapv")
        .compile("opm");

    println!("cargo:rerun-if-changed=call_opm_clock_64times.c");
    println!("cargo:rerun-if-changed=opm.c");
    println!("cargo:rerun-if-changed=opm.h");

    let hash = git_output(&["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_COMMIT_HASH={hash}");

    if let Some(head_path) = git_path("HEAD") {
        println!("cargo:rerun-if-changed={}", head_path.display());

        if let Ok(head) = std::fs::read_to_string(&head_path) {
            if let Some(ref_path_str) = head.trim().strip_prefix("ref: ") {
                let ref_path = std::path::Path::new(ref_path_str);
                if !ref_path.as_os_str().is_empty()
                    && ref_path
                        .components()
                        .all(|component| matches!(component, std::path::Component::Normal(_)))
                {
                    if let Some(ref_watch_path) =
                        git_path(ref_path_str).filter(|path| is_path_within_git_dir(path))
                    {
                        println!("cargo:rerun-if-changed={}", ref_watch_path.display());
                    }
                }
            }
        }
    }

    if let Some(packed_refs_path) = git_path("packed-refs") {
        println!("cargo:rerun-if-changed={}", packed_refs_path.display());
    }
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = std::process::Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty())
}

fn git_path(path: &str) -> Option<std::path::PathBuf> {
    git_output(&["rev-parse", "--git-path", path]).map(std::path::PathBuf::from)
}

fn git_dir() -> Option<std::path::PathBuf> {
    git_output(&["rev-parse", "--absolute-git-dir"]).map(std::path::PathBuf::from)
}

fn is_path_within_git_dir(path: &std::path::Path) -> bool {
    let canonical_path = std::fs::canonicalize(path).ok();
    let canonical_git_dir = git_dir().and_then(|dir| std::fs::canonicalize(dir).ok());

    match (canonical_path, canonical_git_dir) {
        (Some(canonical_path_value), Some(canonical_git_dir_value)) => {
            canonical_path_value.starts_with(canonical_git_dir_value)
        }
        _ => false,
    }
}
