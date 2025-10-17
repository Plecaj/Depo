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

#[derive(Subcommand, PartialEq)]
enum Commands {
    Init,
    Add {
        name: String,
        url: String,
    },
    Delete {
        name: String
    },
    Install,
    Build,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let file_name = "package.yaml";

    if let Commands::Init = cli.command {
        if serialization::package_exists(file_name) {
            println!("Package file already exists");
        } else {
            let pkg = Package::create();
            serialization::save_package(&pkg, file_name)?;
            println!("Initialized new package");
        }
        return Ok(());
    }

    let mut pkg = match serialization::load_package(file_name) {
        Ok(pkg) => pkg,
        Err(PackageError::PackageNotFound) => {
            println!("Package file not found. Use the `init` command to create one.");
            return Ok(());
        }
        Err(e) => return Err(Box::new(e)),
    };

    match cli.command {
        Commands::Add { name, url } => {
            match pkg.add_dependency(&name, &url) {
                Ok(true) => println!("Added dependency: {} -> {}", name, url),
                Ok(false) => println!("Dependency '{}' already exists", name),
                Err(e) => eprintln!("Failed to add dependency '{}': {}", name, e),
            }
        }
        Commands::Delete { name } => {
            match pkg.remove_dependency(&name) {
                Ok(()) => println!("Deleted dependency: {}", name),
                Err(e) => eprintln!("Failed to delete dependency '{}': {}", name, e),
            }
        }
        Commands::Install => {
            for dep in &pkg.dependencies {
                match dep.install() {
                    Ok(_) => println!("Installed dependency '{}'", dep.name),
                    Err(e) => eprintln!("Failed to install dependency '{}': {}", dep.name, e),
                }
            }
        }
        Commands::Build => {
            for dep in &pkg.dependencies {
                match CMake::build_dependency(dep) {
                    Ok(_) => println!("Built dependency '{}'", dep.name),
                    Err(e) => eprintln!("Failed to build dependency '{}': {}", dep.name, e),
                }
            }
            CMake::generate_dependency_bridge(&pkg.dependencies)?;
        }
        // Init is being checked in if statement before match to avoid repetition
        // Because every other variant needs data stored inside package.yaml
        // We can load the data before match statement this way
        Commands::Init => {}
    }
    serialization::save_package(&pkg, file_name)?;
    Ok(())
}
