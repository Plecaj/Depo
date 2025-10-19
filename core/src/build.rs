//! # Build System Integration
//!
//! This module provides build system integration for C++ dependencies.
//! Currently supports CMake build system with plans for additional build systems.

use crate::dependency::Dependency;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Trait defining the interface for build system implementations
///
/// This trait allows for different build systems to be supported by the package manager.
/// Each build system implementation must provide methods for building dependencies and
/// generating integration files.
pub trait BuildSystem {
    /// Build a specific dependency using the build system
    ///
    /// # Arguments
    ///
    /// * `pkg` - The dependency to build
    /// * `working_dir` - The working directory where the dependency is installed
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` indicating success or failure of the build.
    fn build_dependency(pkg: &Dependency, working_dir: &str) -> anyhow::Result<()>;
    
    /// Generate bridge files to integrate dependencies with the main project
    ///
    /// # Arguments
    ///
    /// * `deps` - List of dependencies to integrate
    /// * `working_dir` - The working directory where bridge files should be generated
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` indicating success or failure of bridge generation.
    fn generate_dependency_bridge(deps: &[Dependency], working_dir: &str) -> anyhow::Result<()>;
}

/// CMake build system implementation
///
/// This struct provides CMake-specific implementation of the BuildSystem trait.
/// It handles building dependencies using CMake and generating CMake integration files.
pub struct CMake;
impl BuildSystem for CMake {
    fn build_dependency(dep: &Dependency, working_dir: &str) -> anyhow::Result<()> {
        let dep_path = Path::new(working_dir).join("deps").join(&dep.name);
        let cmake_file = dep_path.join("CMakeLists.txt");

        if !cmake_file.exists() {
            anyhow::bail!(
                "Build file not found for {}, path: {}",
                dep.name,
                cmake_file.display()
            );
        }

        let build_dir = dep_path.join("build");
        fs::create_dir_all(&build_dir)?;

        let status = Command::new("cmake")
            .arg("..")
            .current_dir(&build_dir)
            .status()
            .map_err(|e| anyhow::anyhow!("Failed to run CMake for {}: {}", dep.name, e))?;

        if !status.success() {
            anyhow::bail!("CMake configure failed for {}", dep.name);
        }

        let status = Command::new("cmake")
            .arg("--build")
            .arg(".")
            .current_dir(&build_dir)
            .status()
            .map_err(|e| anyhow::anyhow!("Failed to build {}: {}", dep.name, e))?;

        if !status.success() {
            anyhow::bail!("CMake build failed for {}", dep.name);
        }

        Ok(())
    }

    fn generate_dependency_bridge(deps: &[Dependency], working_dir: &str) -> anyhow::Result<()> {
        let deps_dir = Path::new(working_dir).join("deps");
        fs::create_dir_all(&deps_dir)?;

        let include_path = deps_dir.join("CMakeIncludes.cmake");
        let links_path = deps_dir.join("CMakeLinks.cmake");

        let mut include_file = File::create(&include_path)?;
        let mut links_file = File::create(&links_path)?;

        for dep in deps {
            let dep_path = deps_dir.join(&dep.name);
            let dep_path_str = dep_path.to_string_lossy().replace("\\", "/");

            writeln!(include_file, "add_subdirectory({})", dep_path_str)?;
            writeln!(
                include_file,
                "include_directories({}/include)",
                dep_path_str
            )?;
            writeln!(
                links_file,
                "target_link_libraries(main PRIVATE {})",
                dep.name
            )?;
        }

        Ok(())
    }
}
