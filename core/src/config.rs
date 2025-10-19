//! # Configuration Management
//!
//! This module handles configuration loading and management for the package manager.
//! It supports loading configuration from environment files and managing GitHub API tokens.

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Configuration structure for the package manager
///
/// Contains settings and credentials needed for package management operations.
/// Currently supports GitHub API token configuration for enhanced API access.
pub struct Config {
    /// Optional GitHub API token for authenticated requests
    ///
    /// When provided, this token allows for higher rate limits and access to private repositories.
    /// The token should be a personal access token with appropriate permissions.
    pub github_token: Option<String>,
}

impl Config {
    /// Load configuration from environment files
    ///
    /// This method loads configuration from multiple sources in order of precedence:
    /// 1. `.env` file (if exists)
    /// 2. `.pkg.env` file (if exists)
    /// 3. Environment variables
    ///
    /// # Returns
    ///
    /// Returns a `Result<Config>` containing the loaded configuration or an error if loading fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pkgcore::config::Config;
    ///
    /// let config = Config::load()?;
    /// if config.has_token() {
    ///     println!("GitHub token is configured");
    /// }
    /// ```
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

    /// Create a new environment file with the provided GitHub token
    ///
    /// This method creates a `.pkg.env` file containing the GitHub token for future use.
    ///
    /// # Arguments
    ///
    /// * `token` - The GitHub personal access token to save
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` indicating success or failure of the operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pkgcore::config::Config;
    ///
    /// Config::create_env_file("ghp_your_token_here")?;
    /// ```
    pub fn create_env_file(token: &str) -> Result<()> {
        let env_content = format!("GITHUB_TOKEN={}\n", token);
        fs::write(".pkg.env", env_content)?;
        println!("GitHub token saved to .pkg.env");
        Ok(())
    }

    /// Check if a GitHub token is configured
    ///
    /// # Returns
    ///
    /// Returns `true` if a GitHub token is available, `false` otherwise.
    pub fn has_token(&self) -> bool {
        self.github_token.is_some()
    }

    /// Get the authorization header for GitHub API requests
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the Bearer token header if a token is configured,
    /// or `None` if no token is available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pkgcore::config::Config;
    ///
    /// let config = Config::load()?;
    /// if let Some(auth_header) = config.get_auth_header() {
    ///     // Use auth_header in HTTP requests
    /// }
    /// ```
    pub fn get_auth_header(&self) -> Option<String> {
        self.github_token
            .as_ref()
            .map(|token| format!("Bearer {}", token))
    }
}
