use std::fs;
use serde::{Deserialize, Serialize};
use crate::dependency::{Dependency, DependencyError};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub dependencies: Vec<Dependency>
}

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("Package file not found")]
    PackageNotFound,

    #[error("Dependency not found")]
    DependencyNotFound(String),

    #[error("Dependency error: {0}")]
    DependencyError(#[from] DependencyError),

    #[error("YAML serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_yaml::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
impl Package {

    pub fn create() -> Package {
        Package{
            dependencies: vec![]
        }
    }

    pub fn add_dependency(&mut self, name: &str,  url: &str) -> Result<bool, PackageError> {
        if self.dependencies.iter().any(|d| d.name == name) {
            return Ok(false);
        }

        let dep = Dependency::create(&name, &url)?;
        self.dependencies.push(dep);

        Ok(true)
    }

    pub fn remove_dependency(&mut self, name: &str) -> Result<(), PackageError> {
        if !self.dependencies.iter().any(|d| d.name == name) {
            return Err(PackageError::DependencyNotFound(format!("Dependency: {} not found", name)));
        }
        self.dependencies.retain(|d| d.name != name);
        fs::remove_dir_all(format!("deps/{}", name))?;
        Ok(())
    }

}