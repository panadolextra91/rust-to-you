use crate::error::IntakeError;
use crate::cli::RepoRef;

/// A clone workspace that holds the temp dir and the git repository.
/// The lifecycle is managed by hygiene module to ensure cleanup on signal/panic.
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

    crate::repo::hygiene::register_live_temp(tmp.path().to_path_buf());

    // On clone failure we return before constructing CloneWorkspace, so its Drop
    // never runs — clear the slot here so it doesn't keep a stale path.
    let repo = match git2::Repository::clone(&url, tmp.path()) {
        Ok(repo) => repo,
        Err(_) => {
            crate::repo::hygiene::clear_live_temp();
            return Err(IntakeError::Network);
        }
    };

    Ok(CloneWorkspace {
        tmp,
        repo,
    })
}

impl Drop for CloneWorkspace {
    fn drop(&mut self) {
        crate::repo::hygiene::clear_live_temp();
    }
}
