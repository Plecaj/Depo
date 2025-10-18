use std::path::Path;
use git2::Repository;
use serde::{Serialize, Deserialize};
use semver::{Version, VersionReq};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency{
    pub name: String,
    pub full_name: String,
    pub url: String,
    pub version_constraint: Option<String>,
}


impl Dependency{
    pub fn new(name: &str, full_name: &str, url: &str, version_constraint: Option<String>) -> Dependency {
        Dependency {
            name: name.to_string(),
            full_name: full_name.to_string(),
            url: url.to_string(),
            version_constraint,
        }
    }

    pub fn create(name: &str, full_name: &str, url: &str) -> anyhow::Result<Dependency> {
        let dep = Dependency::new(name, full_name, url, None);
        let install_path = format!("deps/{}", name);
        let path = Path::new(&install_path);

        if path.exists() {
            anyhow::bail!("{} already exists", install_path);
        }

        Repository::clone(&url, &path)?;
        Ok(dep)
    }

    pub fn install(&self, working_dir: &str) -> anyhow::Result<()> {
        let install_path = Path::new(&working_dir).join("deps").join(&self.full_name);
        let path = Path::new(&install_path);

        if path.exists() {
            return Ok(());
        }

        let mut repo = Repository::clone(&self.url, &path)?;
        
        if let Some(ref constraint) = self.version_constraint {
            self.resolve_and_checkout(&mut repo, constraint)?;
        }

        Ok(())
    }

    fn resolve_and_checkout(&self, repo: &mut Repository, constraint: &str) -> anyhow::Result<()> {
        if let Ok(version_req) = VersionReq::parse(constraint) {
            let tags = self.get_matching_tags(repo, &version_req)?;
            if let Some(tag) = tags.first() {
                let commit = repo.revparse_single(tag)?.id();
                repo.checkout_tree(&repo.find_object(commit, None)?, None)?;
                repo.set_head_detached(commit)?;
                return Ok(());
            } else {
                anyhow::bail!("No version matching constraint '{}' found. Available versions: {}", 
                    constraint, 
                    self.get_available_versions(repo)?.join(", ")
                );
            }
        }

        if let Ok(branch_ref) = repo.find_branch(constraint, git2::BranchType::Local) {
            let commit = branch_ref.get().peel_to_commit()?.id();
            repo.checkout_tree(&repo.find_object(commit, None)?, None)?;
            repo.set_head_detached(commit)?;
            return Ok(());
        }

        if let Ok(branch_ref) = repo.find_branch(&format!("origin/{}", constraint), git2::BranchType::Remote) {
            let commit = branch_ref.get().peel_to_commit()?.id();
            repo.checkout_tree(&repo.find_object(commit, None)?, None)?;
            repo.set_head_detached(commit)?;
            return Ok(());
        }

        anyhow::bail!("Could not resolve constraint '{}'. No matching version, branch, or tag found.", constraint);
    }

    fn get_matching_tags(&self, repo: &Repository, version_req: &VersionReq) -> anyhow::Result<Vec<String>> {
        let mut tags = Vec::new();
        let tag_names = repo.tag_names(None)?;
        
        for tag_name in tag_names.iter().flatten() {
            let version_str = if tag_name.starts_with('v') {
                &tag_name[1..]
            } else {
                tag_name
            };
            
            if let Ok(version) = Version::parse(version_str) {
                if version_req.matches(&version) {
                    tags.push(tag_name.to_string());
                }
            }
        }
        
        tags.sort_by(|a, b| {
            let version_a = Version::parse(if a.starts_with('v') { &a[1..] } else { a }).unwrap_or_else(|_| Version::new(0, 0, 0));
            let version_b = Version::parse(if b.starts_with('v') { &b[1..] } else { b }).unwrap_or_else(|_| Version::new(0, 0, 0));
            version_b.cmp(&version_a)
        });
        
        Ok(tags)
    }

    pub fn get_available_versions(&self, repo: &Repository) -> anyhow::Result<Vec<String>> {
        let mut versions = Vec::new();
        let tag_names = repo.tag_names(None)?;
        
        for tag_name in tag_names.iter().flatten() {
            let version_str = if tag_name.starts_with('v') {
                &tag_name[1..]
            } else {
                tag_name
            };
            
            if Version::parse(version_str).is_ok() {
                versions.push(tag_name.to_string());
            }
        }
        
        versions.sort_by(|a, b| {
            let version_a = Version::parse(if a.starts_with('v') { &a[1..] } else { a }).unwrap_or_else(|_| Version::new(0, 0, 0));
            let version_b = Version::parse(if b.starts_with('v') { &b[1..] } else { b }).unwrap_or_else(|_| Version::new(0, 0, 0));
            version_b.cmp(&version_a)
        });
        
        Ok(versions)
    }

    pub fn validate_version_constraint(&self, constraint: &str) -> anyhow::Result<()> {
        if VersionReq::parse(constraint).is_ok() {
            return Ok(());
        }
        
        Ok(())
    }

}