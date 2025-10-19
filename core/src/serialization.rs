use crate::package::Package;
use anyhow::bail;
use std::fs;
use std::path::Path;

pub fn package_exists(path: &str) -> bool {
    Path::new(path).join("package.yaml").exists()
}
pub fn load_package(path: &str) -> anyhow::Result<Package> {
    let file = Path::new(path).join("package.yaml");
    if !package_exists(path) {
        bail!("Package file not found at path: {}", path);
    }

    let content = fs::read_to_string(file)?;
    let package: Package = serde_yaml::from_str(&content)?;
    Ok(package)
}

pub fn save_package(package: &Package, path: &str) -> anyhow::Result<()> {
    let file = Path::new(path).join("package.yaml");
    let yaml_str = serde_yaml::to_string(package)?;
    fs::write(file, yaml_str)?;
    Ok(())
}
