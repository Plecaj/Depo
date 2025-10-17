use crate::package::{Package, PackageError};
use std::fs;
use std::path::Path;

pub fn package_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn load_package(path: &str) -> Result<Package, PackageError> {
    if !package_exists(path) {
        return Err(PackageError::PackageNotFound);
    }

    let content = fs::read_to_string(path)?;
    let package: Package = serde_yaml::from_str(&content)?;
    Ok(package)
}

pub fn save_package(package: &Package, path: &str) -> Result<(), PackageError> {
    let yaml_str = serde_yaml::to_string(package)?;
    fs::write(path, yaml_str)?;
    Ok(())
}