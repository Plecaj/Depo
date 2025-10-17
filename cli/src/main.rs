use clap::{Parser, Subcommand};
use core::{serialization, package};

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let file_name = "package.yaml";

    match &cli.command {
        Commands::Init => {
            if serialization::package_exists(file_name) {
                println!("Package file already exists");
            } else {
                let pkg = crate::package::Package::create();
                serialization::save_package(&pkg, file_name)?;
                println!("Initialized new package");
            }
        }

        Commands::Add { name, url } => {
            match serialization::load_package(file_name) {
                Ok(mut pkg) => {
                    match pkg.add_dependency(name, url) {
                        Ok(true) => println!("Added dependency: {} -> {}", name, url),
                        Ok(false) => println!("Dependency '{}' already exists", name),
                        Err(e) => eprintln!("Failed to add dependency '{}': {}", name, e),
                    }
                    serialization::save_package(&pkg, file_name)?;
                }
                Err(package::PackageError::PackageNotFound) => {
                    println!("Package file not found. Use the `init` command to create one.");
                }
                Err(e) => return Err(Box::new(e)),
            }
        }

        Commands::Install => {
            match serialization::load_package(file_name) {
                Ok(pkg) => match pkg.install_dependencies() {
                    Ok(_) => println!("All dependencies installed successfully"),
                    Err(errors) => {
                        eprintln!("Some dependencies failed to install:");
                        for (name, err) in errors {
                            eprintln!("{} -> {}", name, err);
                        }
                    }
                },
                Err(package::PackageError::PackageNotFound) => {
                    println!("Package file not found. Use the `init` command to create one");
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
    }
    Ok(())
}
