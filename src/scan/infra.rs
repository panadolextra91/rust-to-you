use std::path::Path;
use crate::snapshot::InfraFootprints;

pub fn detect_infra(root: &Path) -> InfraFootprints {
    InfraFootprints {
        docker: detect_docker(root),
        terraform: detect_terraform(root),
        github_actions: detect_github_actions(root),
        gitlab_ci: detect_gitlab_ci(root),
        circleci: detect_circleci(root),
        jenkins: detect_jenkins(root),
        dependabot: detect_dependabot(root),
        renovate: detect_renovate(root),
    }
}

fn detect_docker(root: &Path) -> bool {
    root.join("Dockerfile").exists()
        || root.join("docker-compose.yml").exists()
        || root.join("docker-compose.yaml").exists()
}

fn detect_terraform(root: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "tf" {
                    return true;
                }
            }
        }
    }
    false
}

fn detect_github_actions(root: &Path) -> bool {
    let workflows = root.join(".github").join("workflows");
    if let Ok(entries) = std::fs::read_dir(workflows) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "yml" || ext == "yaml" {
                    return true;
                }
            }
        }
    }
    false
}

fn detect_gitlab_ci(root: &Path) -> bool {
    root.join(".gitlab-ci.yml").exists()
}

fn detect_circleci(root: &Path) -> bool {
    root.join(".circleci").join("config.yml").exists()
}

fn detect_jenkins(root: &Path) -> bool {
    root.join("Jenkinsfile").exists()
}

fn detect_dependabot(root: &Path) -> bool {
    root.join(".github").join("dependabot.yml").exists()
}

fn detect_renovate(root: &Path) -> bool {
    root.join("renovate.json").exists()
        || root.join(".github").join("renovate.json").exists()
        || root.join(".renovaterc").exists()
        || root.join(".renovaterc.json").exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        let tmp = tempfile::Builder::new().prefix("infra-").tempdir().unwrap();
        
        std::fs::write(tmp.path().join("Dockerfile"), "").unwrap();
        std::fs::write(tmp.path().join("main.tf"), "").unwrap();
        
        std::fs::create_dir_all(tmp.path().join(".github").join("workflows")).unwrap();
        std::fs::write(tmp.path().join(".github").join("workflows").join("ci.yml"), "").unwrap();
        
        std::fs::write(tmp.path().join(".gitlab-ci.yml"), "").unwrap();
        std::fs::create_dir_all(tmp.path().join(".circleci")).unwrap();
        std::fs::write(tmp.path().join(".circleci").join("config.yml"), "").unwrap();
        std::fs::write(tmp.path().join("Jenkinsfile"), "").unwrap();
        std::fs::write(tmp.path().join(".github").join("dependabot.yml"), "").unwrap();
        std::fs::write(tmp.path().join("renovate.json"), "").unwrap();
        
        let footprints = detect_infra(tmp.path());
        assert!(footprints.docker);
        assert!(footprints.terraform);
        assert!(footprints.github_actions);
        assert!(footprints.gitlab_ci);
        assert!(footprints.circleci);
        assert!(footprints.jenkins);
        assert!(footprints.dependabot);
        assert!(footprints.renovate);
        
        let empty_tmp = tempfile::Builder::new().prefix("empty-").tempdir().unwrap();
        let empty = detect_infra(empty_tmp.path());
        assert!(!empty.docker);
        assert!(!empty.terraform);
        assert!(!empty.github_actions);
        assert!(!empty.gitlab_ci);
        assert!(!empty.circleci);
        assert!(!empty.jenkins);
        assert!(!empty.dependabot);
        assert!(!empty.renovate);
    }
}
