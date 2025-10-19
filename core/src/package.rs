use crate::build::{BuildSystem, CMake};
use crate::config::Config;
use crate::dependency::Dependency;
use crate::serialization;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub dependencies: Vec<Dependency>,
}
impl Package {
    pub fn new() -> Package {
        Package {
            dependencies: Vec::new(),
        }
    }

    pub fn init(path: &str) -> anyhow::Result<Package> {
        if serialization::package_exists(&path) {
            anyhow::bail!("package already exists");
        } else {
            let pkg = Package::new();
            serialization::save_package(&pkg, &path)?;
            Ok(pkg)
        }
    }

    pub fn is_dependency_existing(&self, name: &str) -> bool {
        self.dependencies.iter().any(|d| d.name == name)
    }

    pub async fn find_dependency(&self, name: &str) -> anyhow::Result<Vec<Dependency>> {
        use reqwest::Client;
        use serde_json::Value;

        let config = Config::load()?;
        let client = Client::new();
        let api_url = "https://api.github.com/search/repositories";
        let query = format!("{} language:C++", name);
        let params = [
            ("q", query.as_str()),
            ("sort", "stars"),
            ("order", "desc"),
            ("per_page", "5"),
        ];

        let mut request = client
            .get(api_url)
            .header("User-Agent", "rust-client")
            .query(&params);

        if let Some(auth_header) = config.get_auth_header() {
            request = request.header("Authorization", auth_header);
        }

        let response = request.send().await?;

        if response.status() == 403 {
            let error_text = response.text().await?;
            if error_text.contains("rate limit") {
                anyhow::bail!(
                    "GitHub API rate limit exceeded. Please add a GitHub token to .pkg.env file"
                );
            }
            anyhow::bail!("GitHub API error: {}", error_text);
        }

        let data: Value = response.json().await?;
        let repos: Vec<Dependency> = data["items"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|repo| {
                Some(Dependency::new(
                    repo["name"].as_str()?,
                    repo["full_name"].as_str()?,
                    repo["clone_url"].as_str()?,
                    None,
                    "",
                ))
            })
            .collect();

        Ok(repos)
    }

    pub fn add_dependency(&mut self, mut dep: Dependency, working_dir: &str) -> anyhow::Result<()> {
        if self.is_dependency_existing(dep.name.as_str()) {
            anyhow::bail!("package already exists");
        }

        dep.install(&working_dir)?;
        self.dependencies.push(dep);
        serialization::save_package(&self, working_dir)?;
        Ok(())
    }

    pub fn remove_dependency(&mut self, name: &str, working_dir: &str) -> anyhow::Result<()> {
        let dep_opt = self.dependencies.iter().find(|d| d.name == name).cloned();
        if dep_opt.is_none() {
            anyhow::bail!("dependency '{}' not found", name);
        }
        let dep = dep_opt.unwrap();

        self.dependencies.retain(|d| d.name != name);

        let dep_dir = format!("{}@{}", dep.name, dep.version);
        let dep_path = Path::new(working_dir).join("deps").join(dep_dir);

        if dep_path.exists() {
            fs::remove_dir_all(&dep_path)?;
        }

        CMake::generate_dependency_bridge(&self.dependencies, working_dir)?;
        serialization::save_package(&self, working_dir)?;
        Ok(())
    }

    pub fn update_dependency(&mut self, name: &str, working_dir: &str) -> anyhow::Result<()> {
        let dep = self
            .dependencies
            .iter_mut()
            .find(|d| d.name == name)
            .ok_or_else(|| anyhow::anyhow!("Dependency '{}' not found", name))?;

        let latest = dep.find_latest_matching_version()?;

        if latest == dep.version {
            return Ok(());
        }
        
        dep.version = latest;
        dep.install(working_dir)?;

        serialization::save_package(&self, working_dir)?;

        Ok(())
    }

    pub fn modify_dependency_constraint(
        &mut self,
        name: &str,
        new_constraint: &str,
        working_dir: &str,
    ) -> anyhow::Result<()> {
        let dep = self
            .dependencies
            .iter_mut()
            .find(|d| d.name == name)
            .ok_or_else(|| anyhow::anyhow!("Dependency '{}' not found", name))?;

        dep.validate_version_constraint(new_constraint)?;
        dep.version_constraint = Some(new_constraint.to_string());

        self.update_dependency(name, working_dir)?;
        Ok(())
    }
}
