use std::fs;
use git2::Repository;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::Context;
use tempfile::TempDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub full_name: String,
    pub url: String,
    pub version_constraint: Option<String>,
    pub version: String,
}

impl Dependency {
    pub fn new(
        name: &str,
        full_name: &str,
        url: &str,
        version_constraint: Option<String>,
        version: &str,
    ) -> Dependency
    {
        Dependency {
            name: name.to_string(),
            full_name: full_name.to_string(),
            url: url.to_string(),
            version_constraint,
            version: version.to_string(),
        }
    }

    pub fn install(&mut self, working_dir: &str) -> anyhow::Result<()> {
        let deps_dir = Path::new(working_dir).join("deps");
        fs::create_dir_all(&deps_dir)?;

        let temp_path = deps_dir.join(format!("{}@temp", self.name));
        let final_path = self.get_final_path(&deps_dir);

        if final_path.exists() {
            let repo = Repository::open(&final_path)?;
            self.version = self.detect_checked_out_version(&repo)?;
            return Ok(());
        }

        self.cleanup_path(&temp_path)?;
        let mut repo = self.clone_repo(&temp_path)?;
        self.apply_version_constraint(&mut repo)?;
        self.version = self.detect_checked_out_version(&repo)?;
        drop(repo);
        std::thread::sleep(std::time::Duration::from_millis(200));

        let versioned_path = deps_dir.join(format!("{}@{}", self.name, self.version));
        self.move_to_final_path(&temp_path, &versioned_path)?;

        Ok(())
    }

    pub fn find_latest_matching_version(&self) -> anyhow::Result<String> {
        let temp_dir = TempDir::new()
            .context("Failed to create temporary directory")?;
        let temp_path = temp_dir.path();

        let repo = Repository::clone(&self.url, temp_path)
            .context("Failed to clone repository")?;

        let mut versions: Vec<Version> = vec![];

        for tag_name in repo.tag_names(None)?.iter().flatten() {
            let version_str = tag_name.trim_start_matches('v');
            if let Ok(v) = Version::parse(version_str) {
                versions.push(v);
            }
        }

        if versions.is_empty() {
            anyhow::bail!("No semantic version tags found for '{}'", self.name);
        }

        versions.sort_by(|a, b| b.cmp(a));

        if let Some(constraint) = &self.version_constraint {
            let req = VersionReq::parse(constraint)?;
            for v in &versions {
                if req.matches(v) {
                    return Ok(v.to_string());
                }
            }
            anyhow::bail!(
            "No versions of '{}' match constraint '{}'",
            self.name,
            constraint
        );
        }

        Ok(versions.first().unwrap().to_string())
    }

    fn detect_checked_out_version(&self, repo: &Repository) -> anyhow::Result<String> {
        let head = repo.head()?.peel_to_commit()?;
        let head_oid = head.id();

        let tag_names = repo.tag_names(None)?;
        for tag_name in tag_names.iter().flatten() {
            if let Ok(tag_ref) = repo.revparse_single(tag_name) {
                if let Ok(tag_commit) = tag_ref.peel_to_commit() {
                    if tag_commit.id() == head_oid {
                        return Ok(tag_name.to_string());
                    }
                }
            }
        }
        Ok(format!("{}", &head_oid.to_string()[..7]))
    }

    fn get_final_path(&self, deps_dir: &Path) -> PathBuf {
        if self.version.is_empty() {
            deps_dir.join(format!("{}@temp", self.name))
        } else {
            deps_dir.join(format!("{}@{}", self.name, self.version))
        }
    }

    fn cleanup_path(&self, path: &Path) -> anyhow::Result<()> {
        if path.exists() {
            fs::remove_dir_all(path)
                .map_err(|e| anyhow::anyhow!("Failed to remove old temp dir '{}': {}", path.display(), e))?;
        }
        Ok(())
    }

    fn clone_repo(&self, dest: &Path) -> anyhow::Result<Repository> {
        Repository::clone(&self.url, dest)
            .map_err(|e| anyhow::anyhow!("Failed to clone '{}' into '{}': {}", self.url, dest.display(), e))
    }
    fn apply_version_constraint(&self, repo: &mut Repository) -> anyhow::Result<()> {
        if let Some(ref constraint) = self.version_constraint {
            self.resolve_and_checkout(repo, constraint)?;
        }
        Ok(())
    }

    fn move_to_final_path(&self, src: &Path, dst: &Path) -> anyhow::Result<()> {
        if !src.exists() {
            anyhow::bail!("Temp path '{}' not found", src.display());
        }

        if let Err(e) = fs::rename(src, dst) {
            eprintln!("Rename failed: {} → {}: {}", src.display(), dst.display(), e);
            eprintln!("Falling back to recursive copy...");

            copy_dir_all(src, dst)?;
            fs::remove_dir_all(src)?;
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
                anyhow::bail!("No version matching constraint '{}' found", constraint);
            }
        }

        if let Ok(branch_ref) = repo.find_branch(constraint, git2::BranchType::Local) {
            let commit = branch_ref.get().peel_to_commit()?.id();
            repo.checkout_tree(&repo.find_object(commit, None)?, None)?;
            repo.set_head_detached(commit)?;
            return Ok(());
        }

        if let Ok(branch_ref) =
            repo.find_branch(&format!("origin/{}", constraint), git2::BranchType::Remote)
        {
            let commit = branch_ref.get().peel_to_commit()?.id();
            repo.checkout_tree(&repo.find_object(commit, None)?, None)?;
            repo.set_head_detached(commit)?;
            return Ok(());
        }

        anyhow::bail!(
            "Could not resolve constraint '{}'. No matching version, branch, or tag found.",
            constraint
        );
    }

    fn get_matching_tags(
        &self,
        repo: &Repository,
        version_req: &VersionReq,
    ) -> anyhow::Result<Vec<String>>
    {
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
            let version_a = Version::parse(if a.starts_with('v') { &a[1..] } else { a })
                .unwrap_or_else(|_| Version::new(0, 0, 0));
            let version_b = Version::parse(if b.starts_with('v') { &b[1..] } else { b })
                .unwrap_or_else(|_| Version::new(0, 0, 0));
            version_b.cmp(&version_a)
        });

        Ok(tags)
    }
    pub fn validate_version_constraint(&self, constraint: &str) -> anyhow::Result<()> {
        if !VersionReq::parse(constraint).is_ok() {
            anyhow::bail!("Invalid version constraint: {}", constraint);
        }

        Ok(())
    }
}


fn copy_dir_all(src: &Path, dst: &Path) -> anyhow::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else if ty.is_file() {
            fs::copy(entry.path(), &target)?;
        }
    }

    Ok(())
}