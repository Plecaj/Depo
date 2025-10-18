use crate::package::{Package};
use std::fs;
use std::path::Path;
use anyhow::bail;

pub fn package_exists(path: &str) -> bool {
    Path::new(path).join("package.yaml").exists()
}
pub fn load_package(path: &str) -> anyhow::Result<Package> {
    let file = path.to_owned() + "/package.json";
    if !package_exists(file.as_str()) {
        bail!("Package file not found at path: {}", path);
    }

    let content = fs::read_to_string(file)?;
    let package: Package = serde_yaml::from_str(&content)?;
    Ok(package)
}

pub fn save_package(package: &Package, path: &str) -> anyhow::Result<()> {
    let file = path.to_owned() + "/package.json";
    let yaml_str = serde_yaml::to_string(package)?;
    fs::write(file, yaml_str)?;
    Ok(())
}