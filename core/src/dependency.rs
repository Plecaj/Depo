use std::path::Path;
use git2::Repository;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency{
    pub name: String,
    pub full_name: String,
    pub url: String,
}


impl Dependency{
    pub fn create(name: &str, full_name: &str, url: &str) -> anyhow::Result<Dependency> {
        let dep = Dependency{name: name.to_string(),full_name: full_name.to_string(), url: url.to_string()};
        let install_path = format!("deps/{}", name);
        let path = Path::new(&install_path);

        if path.exists() {
            anyhow::bail!("{} already exists", install_path);
        }

        Repository::clone(&url, &path)?;
        Ok(dep)
    }

    pub fn install(&self) -> anyhow::Result<()> {
        let install_path = format!("deps/{}", self.name);
        let path = Path::new(&install_path);

        if path.exists() {
            return Ok(());
        }

        Repository::clone(&self.url, &path)?;
        Ok(())
    }

}