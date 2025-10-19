//! # Serialization and Persistence
//!
//! This module handles serialization and persistence of package configurations.
//! It provides functions for saving and loading package data to/from YAML files.

use crate::package::Package;
use anyhow::bail;
use std::fs;
use std::path::Path;

/// Check if a package configuration file exists in the given directory
///
/// # Arguments
///
/// * `path` - The directory path to check for a package configuration
///
/// # Returns
///
/// Returns `true` if a `package.yaml` file exists in the directory, `false` otherwise.
pub fn package_exists(path: &str) -> bool {
    Path::new(path).join("package.yaml").exists()
}

/// Load a package configuration from a YAML file
///
/// # Arguments
///
/// * `path` - The directory path containing the package configuration file
///
/// # Returns
///
/// Returns a `Result<Package>` containing the loaded package or an error if loading fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The package file does not exist
/// - The file cannot be read
/// - The YAML content cannot be parsed into a Package structure
pub fn load_package(path: &str) -> anyhow::Result<Package> {
    let file = Path::new(path).join("package.yaml");
    if !package_exists(path) {
        bail!("Package file not found at path: {}", path);
    }

    let content = fs::read_to_string(file)?;
    let package: Package = serde_yaml::from_str(&content)?;
    Ok(package)
}

/// Save a package configuration to a YAML file
///
/// # Arguments
///
/// * `package` - The package to save
/// * `path` - The directory path where the package configuration should be saved
///
/// # Returns
///
/// Returns a `Result<()>` indicating success or failure of the save operation.
///
/// # Errors
///
/// This function will return an error if:
/// - The package cannot be serialized to YAML
/// - The file cannot be written to the specified path
pub fn save_package(package: &Package, path: &str) -> anyhow::Result<()> {
    let file = Path::new(path).join("package.yaml");
    let yaml_str = serde_yaml::to_string(package)?;
    fs::write(file, yaml_str)?;
    Ok(())
}
