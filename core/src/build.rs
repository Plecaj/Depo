use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use crate::dependency::Dependency;


pub trait BuildSystem {
    fn build_dependency(pkg: &Dependency) -> anyhow::Result<()>;
    fn generate_dependency_bridge(deps: &[Dependency]) -> anyhow::Result<()>;
}
pub struct CMake;
impl BuildSystem for CMake {
    fn build_dependency(dep: &Dependency) -> anyhow::Result<()> {
        let dep_path = Path::new("deps").join(&dep.name);
        let cmake_file = dep_path.join("CMakeLists.txt");

        if !cmake_file.exists() {
            anyhow::bail!("Build file not found for {}, path: {}", dep.name, dep_path.display());
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
            .map_err(|e| anyhow::anyhow!("Failed to run CMake for {}: {}", dep.name, e))?;

        if !status.success() {
            anyhow::bail!("CMake build failed for {}", dep.name);
        }
        Ok(())
    }

    fn generate_dependency_bridge(deps: &[Dependency]) -> anyhow::Result<()>{
        let mut include_file = File::create("deps/CMakeIncludes.cmake")?;
        let mut links_file = File::create("deps/CMakeLinks.cmake")?;

        for dep in deps {
            let dep_path = Path::new("deps")
                .join(&dep.name)
                .to_string_lossy()
                .replace("\\", "/");

            writeln!(include_file, "add_subdirectory({})", dep_path)?;
            writeln!(include_file, "include_directories({}/include)", dep_path)?;
            writeln!(links_file, "target_link_libraries(main PRIVATE {})", dep.name)?;
        }

        Ok(())
    }
}