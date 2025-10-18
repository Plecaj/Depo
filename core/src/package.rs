use std::fs;
use serde::{Deserialize, Serialize};
use crate::dependency::{Dependency};
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub dependencies: Vec<Dependency>
}
impl Package {

    pub fn create() -> Package {
        Package{
            dependencies: vec![]
        }
    }

    pub fn is_dependency_existing(&self, name: &str) -> anyhow::Result<()> {
        if self.dependencies.iter().any(|d| d.name == name) {
            anyhow::bail!("Dependency already exists: {}", name);
        }
        return Ok(());
    }

    pub async fn find_dependency(&mut self, name: &str) -> anyhow::Result<Vec<Dependency>> {
        use reqwest::Client;
        use serde_json::Value;

        self.is_dependency_existing(name)?;

        let client = Client::new();
        let api_url = "https://api.github.com/search/repositories";
        let params = [("q", name), ("sort", "stars"), ("order", "desc"), ("per_page", "5")];

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
        self.is_dependency_existing(dep.name.as_str())?;
        self.dependencies.push(dep);
        Ok(())
    }

    pub fn remove_dependency(&mut self, name: &str) -> anyhow::Result<()> {
        self.is_dependency_existing(name)?;
        self.dependencies.retain(|d| d.name != name);
        fs::remove_dir_all(format!("deps/{}", name))?;
        Ok(())
    }

}