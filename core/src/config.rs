use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct Config {
    pub github_token: Option<String>,
}

impl Config {
    pub fn load() -> Result<Config> {
        if Path::new(".env").exists() {
            dotenv::dotenv()?;
        }

        if Path::new(".pkg.env").exists() {
            dotenv::from_filename(".pkg.env")?;
        }

        let github_token = std::env::var("GITHUB_TOKEN").ok();

        Ok(Config { github_token })
    }

    pub fn create_env_file(token: &str) -> Result<()> {
        let env_content = format!("GITHUB_TOKEN={}\n", token);
        fs::write(".pkg.env", env_content)?;
        println!("GitHub token saved to .pkg.env");
        Ok(())
    }

    pub fn has_token(&self) -> bool {
        self.github_token.is_some()
    }

    pub fn get_auth_header(&self) -> Option<String> {
        self.github_token.as_ref().map(|token| format!("Bearer {}", token))
    }
}
