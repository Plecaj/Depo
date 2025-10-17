use std::fs::File;
use thiserror::Error;
use std::{fs, io};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::dependency::Dependency;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Build file not found at {0}")]
    BuildFileNotFound(PathBuf),

    #[error("Build process failed: {0}")]
    BuildProcessFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("Unknown build error: {0}")]
    Other(String),
}

pub trait BuildSystem {
    fn build_dependency(pkg: &Dependency) -> Result<(), BuildError>;
}
pub struct CMake;
impl BuildSystem for CMake {
    fn build_dependency(dep: &Dependency) -> Result<(), BuildError> {
        let dep_path = Path::new("deps").join(&dep.name);
        let cmake_file = dep_path.join("CMakeLists.txt");

        if !cmake_file.exists() {
            return Err(BuildError::BuildFileNotFound(dep_path.to_path_buf()));
        }

        let build_dir = dep_path.join("build");
        fs::create_dir_all(&build_dir)?;

        let status = Command::new("cmake")
            .arg("..") // source directory
            .current_dir(&build_dir)
            .status()
            .map_err(|e| BuildError::BuildProcessFailed(e.to_string()))?;

        if !status.success() {
            return Err(BuildError::BuildProcessFailed(format!(
                "CMake configure failed for {}",
                dep.name
            )));
        }

        let status = Command::new("cmake")
            .arg("--build")
            .arg(".")
            .current_dir(&build_dir)
            .status()
            .map_err(|e| BuildError::BuildProcessFailed(e.to_string()))?;

        if !status.success() {
            return Err(BuildError::BuildProcessFailed(format!(
                "CMake build failed for {}",
                dep.name
            )));
        }

        let mut file = File::options()
            .create(true)
            .append(true)
            .open("deps/CMakeIncludes.cmake")?;

        let dep_str = dep_path.to_string_lossy().replace("\\", "/");
        writeln!(file, "add_subdirectory({})", dep_str)?;
        writeln!(file, "include_directories({}/include)", dep_str)?;

        let mut links_file = File::options()
            .create(true)
            .append(true)
            .open("deps/CMakeLinks.cmake")?;

        writeln!(links_file, "target_link_libraries(main PRIVATE {})", dep.name)?;
        
        Ok(())
    }
}