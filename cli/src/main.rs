use clap::{Parser, Subcommand};
use core::{
    serialization,
    build::{CMake, BuildSystem},
    package::{Package}
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
    },
    Delete {
        name: String
    },
    Install,
    Build,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        Err(e) => {
            if e.to_string().contains("Package file not found") {
                println!("Package file not found. Use the `init` command to create one.");
                return Ok(());
            }
            return Err(e.into());
        }
    };

    match cli.command {
        Commands::Add { name } => {
            let mut candidates = pkg.find_dependency(&name).await?;

            if candidates.is_empty() {
                println!("No dependencies found for '{}'", name);
                return Ok(());
            }

            let options: Vec<String> = candidates.iter().map(|c| c.name.clone()).collect();

            let selection = dialoguer::Select::new()
                .with_prompt("Select a dependency")
                .items(&options)
                .default(0)
                .interact()?;

            let chosen = candidates.remove(selection);
            pkg.add_dependency(chosen)?;
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
