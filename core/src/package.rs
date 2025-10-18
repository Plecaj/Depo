use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::build::{BuildSystem, CMake};
use crate::dependency::{Dependency};
use crate::serialization;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub dependencies: Vec<Dependency>
}
impl Package {

    pub fn new() -> Package {
        Package {
         dependencies: Vec::new()
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
            ("per_page", "5")
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
                anyhow::bail!("GitHub API rate limit exceeded. Please add a GitHub token to .pkg.env file");
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
                    repo["html_url"].as_str()?,
                    None,
                ))
            })
            .collect();

        Ok(repos)
    }

    pub fn add_dependency(&mut self, dep: Dependency, working_dir: &str) -> anyhow::Result<()> {
        if self.is_dependency_existing(dep.name.as_str()) {
            anyhow::bail!("package already exists");
        }

        dep.install(&working_dir)?;
        self.dependencies.push(dep);
        Ok(())
    }

    pub fn remove_dependency(&mut self, name: &str, working_dir: &str) -> anyhow::Result<()> {
        if !self.is_dependency_existing(name) {
            anyhow::bail!("package doesnt exist");
        }
        self.dependencies.retain(|d| d.name != name);

        let dep_path = Path::new(working_dir).join("deps").join(name);
        if dep_path.exists() {
            fs::remove_dir_all(&dep_path)?;
        }

        CMake::generate_dependency_bridge(&self.dependencies, &working_dir)?;
        Ok(())
    }

    pub async fn get_available_versions(&self, name: &str) -> anyhow::Result<Vec<String>> {
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
            ("per_page", "1") 
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
                anyhow::bail!("GitHub API rate limit exceeded. Please add a GitHub token to .pkg.env file");
            }
            anyhow::bail!("GitHub API error: {}", error_text);
        }

        let data: Value = response.json().await?;
        let empty_vec = vec![];
        let repos = data["items"]
            .as_array()
            .unwrap_or(&empty_vec);

        if repos.is_empty() {
            anyhow::bail!("No repositories found for '{}'", name);
        }

        let repo = &repos[0];
        let repo_url = repo["clone_url"].as_str().unwrap_or("");
        
        let temp_path = format!("temp_check_{}_{}", name, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs());
        
        let repo_obj = git2::Repository::clone(repo_url, &temp_path)?;
        let dep = Dependency::new(
            repo["name"].as_str().unwrap_or(""),
            repo["full_name"].as_str().unwrap_or(""),
            repo_url,
            None,
        );
        
        let versions = dep.get_available_versions(&repo_obj)?;
        
        let _ = fs::remove_dir_all(&temp_path);
        
        Ok(versions)
    }

}