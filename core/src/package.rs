//! # Package Management
//!
//! This module provides the core package management functionality for C++ projects.
//! It handles dependency resolution, package initialization, and dependency operations.

use crate::build::{BuildSystem, CMake};
use crate::config::Config;
use crate::dependency::Dependency;
use crate::serialization;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents a C++ package with its dependencies
///
/// A Package contains a collection of dependencies that are managed together.
/// It provides methods for adding, removing, updating, and searching dependencies.
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    /// List of dependencies managed by this package
    pub dependencies: Vec<Dependency>,
}
impl Package {
    /// Create a new empty package
    ///
    /// # Returns
    ///
    /// Returns a new `Package` instance with an empty dependencies list.
    pub fn new() -> Package {
        Package {
            dependencies: Vec::new(),
        }
    }

    /// Initialize a new package in the specified directory
    ///
    /// This method creates a new package configuration file (`package.yaml`) in the given directory.
    /// If a package already exists in the directory, this method will return an error.
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path where the package should be initialized
    ///
    /// # Returns
    ///
    /// Returns a `Result<Package>` containing the newly created package or an error if initialization fails.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - A package already exists in the specified directory
    /// - The directory cannot be written to
    /// - File I/O operations fail
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pkgcore::package::Package;
    ///
    /// let package = Package::init("./my_project")?;
    /// ```
    pub fn init(path: &str) -> anyhow::Result<Package> {
        if serialization::package_exists(&path) {
            anyhow::bail!("package already exists");
        } else {
            let pkg = Package::new();
            serialization::save_package(&pkg, &path)?;
            Ok(pkg)
        }
    }

    /// Check if a dependency with the given name already exists
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the dependency to check for
    ///
    /// # Returns
    ///
    /// Returns `true` if a dependency with the given name exists, `false` otherwise.
    pub fn is_dependency_existing(&self, name: &str) -> bool {
        self.dependencies.iter().any(|d| d.name == name)
    }

    /// Search for dependencies on GitHub
    ///
    /// This method searches GitHub repositories for C++ libraries matching the given name.
    /// It uses the GitHub API to find repositories with the specified name and C++ language tag.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to search for in GitHub repositories
    ///
    /// # Returns
    ///
    /// Returns a `Result<Vec<Dependency>>` containing a list of matching dependencies or an error if the search fails.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - GitHub API rate limit is exceeded (suggests adding a GitHub token)
    /// - Network requests fail
    /// - API responses cannot be parsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pkgcore::package::Package;
    ///
    /// let package = Package::new();
    /// let dependencies = package.find_dependency("json").await?;
    /// for dep in dependencies {
    ///     println!("Found: {}", dep.full_name);
    /// }
    /// ```
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

    /// Add a new dependency to the package
    ///
    /// This method adds a dependency to the package, installs it to the local filesystem,
    /// and saves the updated package configuration.
    ///
    /// # Arguments
    ///
    /// * `dep` - The dependency to add
    /// * `working_dir` - The working directory where dependencies are installed
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` indicating success or failure of the operation.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - A dependency with the same name already exists
    /// - The dependency installation fails
    /// - The package configuration cannot be saved
    pub fn add_dependency(&mut self, mut dep: Dependency, working_dir: &str) -> anyhow::Result<()> {
        if self.is_dependency_existing(dep.name.as_str()) {
            anyhow::bail!("package already exists");
        }

        dep.install(&working_dir)?;
        self.dependencies.push(dep);
        serialization::save_package(&self, working_dir)?;
        Ok(())
    }

    /// Remove a dependency from the package
    ///
    /// This method removes a dependency from the package, deletes its local installation,
    /// regenerates the CMake bridge files, and saves the updated package configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the dependency to remove
    /// * `working_dir` - The working directory where dependencies are installed
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` indicating success or failure of the operation.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The dependency is not found in the package
    /// - File system operations fail
    /// - CMake bridge generation fails
    /// - Package configuration cannot be saved
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
        let index = self
            .dependencies
            .iter()
            .position(|d| d.name == name)
            .ok_or_else(|| anyhow::anyhow!("Dependency '{}' not found", name))?;

        let dep = &mut self.dependencies[index];
        let old_version = dep.version.clone();

        let latest = dep.find_latest_matching_version()?;

        if latest == old_version {
            println!("Dependency '{}' is already up to date (version {}).", name, latest);
            return Ok(());
        }

        let old_dep_path = {
            let old_dir_name = format!("{}@{}", dep.name, old_version);
            Path::new(working_dir).join("deps").join(old_dir_name)
        };

        if old_dep_path.exists() {
            std::fs::remove_dir_all(&old_dep_path)?;
        }

        dep.version = latest.clone();
        dep.install(working_dir)?;

        CMake::generate_dependency_bridge(&self.dependencies, working_dir)?;
        serialization::save_package(self, working_dir)?;

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

        serialization::save_package(&self, working_dir)?;
        Ok(())
    }
    pub fn remove_dependency_constraint(
        &mut self,
        name: &str,
        working_dir: &str,
    ) -> anyhow::Result<()> {
        let dep = self
            .dependencies
            .iter_mut()
            .find(|d| d.name == name)
            .ok_or_else(|| anyhow::anyhow!("Dependency '{}' not found", name))?;

        if dep.version_constraint.is_none() {
            anyhow::bail!("Dependency '{}' has no constraint to remove", name);
        }

        dep.version_constraint = None;

        serialization::save_package(&self, working_dir)?;
        Ok(())
    }
}
