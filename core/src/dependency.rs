use std::path::Path;
use git2::Repository;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency{
    pub name: String,
    pub url: String,
}

#[derive(Debug, Error)]
pub enum DependencyError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Dependency already exists at path: {0}")]
    AlreadyExists(String),
}

impl Dependency{
    pub fn create(name: &str, url: &str) -> Result<Dependency, DependencyError> {
        let dep = Dependency{name: name.to_string(), url: url.to_string()};
        let install_path = format!("deps/{}", name);
        let path = Path::new(&install_path);

        if path.exists() {
            return Err(DependencyError::AlreadyExists(install_path));
        }

        Repository::clone(&url, &path)?;
        Ok(dep)
    }

    pub fn install(&self) -> Result<(), DependencyError> {
        let install_path = format!("deps/{}", self.name);
        let path = Path::new(&install_path);

        if path.exists() {
            return Ok(());
        }

        Repository::clone(&self.url, &path)?;
        Ok(())
    }

}