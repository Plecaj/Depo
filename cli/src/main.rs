use clap::{Parser, Subcommand};
use core::{
    serialization,
    build::{CMake, BuildSystem},
    package::{Package, PackageError}
};

#[derive(Parser)]
#[command(name = "pkg")]
#[command(about = "A simple c++ package manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        name: String,
        url: String,
    },
    Install,
    Build,
}

fn with_package<F>(file_name: &str, f: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(&mut Package) -> Result<(), Box<dyn std::error::Error>>,
{
    let mut pkg = match serialization::load_package(file_name) {
        Ok(pkg) => pkg,
        Err(PackageError::PackageNotFound) => {
            println!("Package file not found. Use the `init` command to create one.");
            return Ok(());
        }
        Err(e) => return Err(Box::new(e)),
    };

    f(&mut pkg)?;
    serialization::save_package(&pkg, file_name)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let file_name = "package.yaml";

    match &cli.command {
        Commands::Init => {
            if serialization::package_exists(file_name) {
                println!("Package file already exists");
            } else {
                let pkg = crate::Package::create();
                serialization::save_package(&pkg, file_name)?;
                println!("Initialized new package");
            }
        }

        Commands::Add { name, url } => {
            with_package(file_name, |pkg| {
                match pkg.add_dependency(name, url) {
                    Ok(true) => println!("Added dependency: {} -> {}", name, url),
                    Ok(false) => println!("Dependency '{}' already exists", name),
                    Err(e) => eprintln!("Failed to add dependency '{}': {}", name, e),
                }
                Ok(())
            })?;
        }

        Commands::Install => {
            with_package(file_name, |pkg| {
                for dep in &pkg.dependencies {
                    match dep.install() {
                        Ok(_) => println!("Installed dependency '{}'", dep.name),
                        Err(e) => eprintln!("Failed to install dependency '{}': {}", dep.name, e),
                    }
                }
                Ok(())
            })?;
        }

        Commands::Build => {
            with_package(file_name, |pkg| {
                for dep in &pkg.dependencies {
                    match CMake::build_dependency(&dep) {
                        Ok(_) => println!("Built dependency '{}'", dep.name),
                        Err(e) => eprintln!("Failed to build dependency '{}': {}", dep.name, e),
                    }
                }
                Ok(())
            })?;
        }
    }
    Ok(())
}
