use std::fs;
use serde::{Deserialize, Serialize};
use crate::build::{BuildSystem, CMake};
use crate::dependency::{Dependency};
use crate::serialization;

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

    pub fn init(file: &str) -> anyhow::Result<Package> {
        if serialization::package_exists(&file) {
            anyhow::bail!("package already exists");
        } else {
            let pkg = Package::new();
            serialization::save_package(&pkg, &file)?;
            Ok(pkg)
        }
    }

    pub fn is_dependency_existing(&self, name: &str) -> bool {
        self.dependencies.iter().any(|d| d.name == name)
    }

    pub async fn find_dependency(&self, name: &str) -> anyhow::Result<Vec<Dependency>> {
        use reqwest::Client;
        use serde_json::Value;

        let client = Client::new();
        let api_url = "https://api.github.com/search/repositories";
        let query = format!("{} language:C++", name);
        let params = [
            ("q", query.as_str()),
            ("sort", "stars"),
            ("order", "desc"),
            ("per_page", "5")
        ];

        let response = client
            .get(api_url)
            .header("User-Agent", "rust-client")
            .query(&params)
            .send()
            .await?;

        let data: Value = response.json().await?;
        let repos: Vec<Dependency> = data["items"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|repo| {
                Some(Dependency {
                    name: repo["full_name"].as_str()?.to_string(),
                    url: repo["html_url"].as_str()?.to_string(),
                })
            })
            .collect();

        Ok(repos)
    }

    pub fn add_dependency(&mut self, dep: Dependency) -> anyhow::Result<()> {
        if self.is_dependency_existing(dep.name.as_str()) {
            anyhow::bail!("package already exists");
        }

        dep.install()?;
        self.dependencies.push(dep);
        Ok(())
    }

    pub fn remove_dependency(&mut self, name: &str) -> anyhow::Result<()> {
        if !self.is_dependency_existing(name) {
            anyhow::bail!("package doesnt exist");
        }
        self.dependencies.retain(|d| d.name != name);
        fs::remove_dir_all(format!("deps/{}", name))?;
        CMake::generate_dependency_bridge(&self.dependencies)?;
        Ok(())
    }

}