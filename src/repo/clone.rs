use crate::error::IntakeError;
use crate::cli::RepoRef;

/// A clone workspace that holds the temp dir and the git repository.
/// WARNING: callers must not std::process::exit() while a CloneWorkspace is alive,
/// because exit skips the Drop trait and will leak the temp directory.
pub struct CloneWorkspace {
    #[allow(dead_code)]
    tmp: tempfile::TempDir,
    pub repo: git2::Repository,
}

pub fn clone_repo(repo_ref: &RepoRef) -> Result<CloneWorkspace, IntakeError> {
    let url = format!("https://github.com/{}/{}.git", repo_ref.owner, repo_ref.repo);
    let tmp = tempfile::Builder::new()
        .prefix("rust-to-you-clone-")
        .tempdir()
        .map_err(|_| IntakeError::Network)?;

    let repo = git2::Repository::clone(&url, tmp.path())
        .map_err(|_| IntakeError::Network)?;

    Ok(CloneWorkspace {
        tmp,
        repo,
    })
}
