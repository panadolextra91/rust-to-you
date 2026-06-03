use crate::app::session::InvestigationSession;
use crate::app::collect::collect;
use crate::error::IntakeError;
use crate::snapshot::RepoMetaState;

pub fn run(session: &InvestigationSession) -> Result<(), IntakeError> {
    println!(
        "🦀 Investigation opened: {}/{} — Case {}",
        session.repo.owner, session.repo.repo, session.case_id
    );

    match collect(session) {
        Ok(snapshot) => {
            println!("✅ Khám nghiệm thành công: {}/{}", snapshot.repo.owner, snapshot.repo.repo);
            println!("  Tuổi đời (ngày): {}", snapshot.history.repo_age_days);
            println!("  Tổng số commit: {}", snapshot.history.total_commits);
            println!("  Số lượng contributor: {}", snapshot.history.contributor_count);
            println!("  Bus factor: {}", snapshot.history.bus_factor);
            println!("  Số lượng branch: {}", snapshot.branches.branches.len());
            
            let langs: Vec<String> = snapshot.filesystem.languages.iter()
                .take(3)
                .map(|(l, p)| format!("{} ({:.1}%)", l, p))
                .collect();
            println!("  Ngôn ngữ chính: {}", langs.join(", "));
            
            println!("  Infra:");
            println!("    Docker: {}", snapshot.filesystem.infra.docker);
            println!("    GitHub Actions: {}", snapshot.filesystem.infra.github_actions);
            println!("    CI/CD (GitLab/Circle/Jenkins): {}", 
                snapshot.filesystem.infra.gitlab_ci || snapshot.filesystem.infra.circleci || snapshot.filesystem.infra.jenkins
            );
            println!("    Dependabot/Renovate: {}", snapshot.filesystem.infra.dependabot || snapshot.filesystem.infra.renovate);

            match &snapshot.metadata {
                RepoMetaState::Available(meta) => {
                    println!("  Stars: {}", meta.stargazers_count);
                    println!("  Forks: {}", meta.forks_count);
                }
                RepoMetaState::Unavailable => {
                    println!("  Stars: unknown");
                    println!("  Forks: unknown");
                }
            }
            
            if snapshot.history.window.capped {
                println!("  (based on last {} commits)", snapshot.history.window.scanned);
            }
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}
