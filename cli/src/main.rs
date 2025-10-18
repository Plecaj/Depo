use std::env;
use clap::{Parser, Subcommand};
use pkgcore::{
    serialization,
    build::{CMake, BuildSystem},
    package::{Package},
    config::Config,
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
        #[arg(short, long)]
        version: Option<String>,
    },
    Delete {
        name: String
    },
    Install,
    Build,
    List,
    Versions {
        name: String,
    },
    Token {
        #[command(subcommand)]
        action: TokenAction,
    },
}

#[derive(Subcommand, PartialEq)]
enum TokenAction {
    Set {
        token: String,
    },
    Check,
    Remove,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let file_name = "package.yaml";

    if let Commands::Init = cli.command {
        Package::init(file_name)?;
        return Ok(());
    }

    if let Commands::Token { .. } = cli.command {
        match cli.command {
            Commands::Token { action } => {
                match action {
                    TokenAction::Set { token } => {
                        match Config::create_env_file(&token) {
                            Ok(_) => println!("GitHub token saved successfully!"),
                            Err(e) => println!("Error saving token: {}", e),
                        }
                    },
                    TokenAction::Check => {
                        match Config::load() {
                            Ok(config) => {
                                if config.has_token() {
                                    println!("GitHub token is configured");
                                    let token = &config.github_token.unwrap();
                                    println!("Token: ...{}", &token[token.len().saturating_sub(8)..]);
                                } else {
                                    println!("No GitHub token found");
                                    println!("Use 'pkg token set <your_token>' to add one");
                                }
                            },
                            Err(e) => println!("Error loading config: {}", e),
                        }
                    },
                    TokenAction::Remove => {
                        match std::fs::remove_file(".pkg.env") {
                            Ok(_) => println!("GitHub token removed successfully!"),
                            Err(e) => println!("Error removing token: {}", e),
                        }
                    },
                }
            },
            _ => unreachable!(),
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
    let working_dir = env::current_dir()?;
    match cli.command {
        Commands::Add { name, version } => {
            let mut candidates = pkg.find_dependency(&name).await?;

            if candidates.is_empty() {
                println!("No dependencies found for '{}'", name);
                return Ok(());
            }

            let options: Vec<String> = candidates.iter().map(|c| c.full_name.clone()).collect();

            let selection = dialoguer::Select::new()
                .with_prompt("Select a dependency")
                .items(&options)
                .default(0)
                .interact()?;

            let mut chosen = candidates.remove(selection);
            
            if let Some(version_constraint) = version {
                chosen.version_constraint = Some(version_constraint);
            }
            
            pkg.add_dependency(chosen, &working_dir.to_str().unwrap())?;
        }
        Commands::Delete { name } => {
            match pkg.remove_dependency(&name, &working_dir.to_str().unwrap()) {
                Ok(()) => println!("Deleted dependency: {}", name),
                Err(e) => eprintln!("Failed to delete dependency '{}': {}", name, e),
            }
         }
        Commands::Install => {
            for dep in &pkg.dependencies {
                match dep.install(&working_dir.to_str().unwrap()) {
                    Ok(_) => println!("Installed dependency '{}'", dep.name),
                    Err(e) => eprintln!("Failed to install dependency '{}': {}", dep.name, e),
                }
            }
        }
        Commands::Build => {
            for dep in &pkg.dependencies {
                match CMake::build_dependency(dep, &working_dir.to_str().unwrap()) {
                    Ok(_) => println!("Built dependency '{}'", dep.name),
                    Err(e) => eprintln!("Failed to build dependency '{}': {}", dep.name, e),
                }
            }
            CMake::generate_dependency_bridge(&pkg.dependencies, &working_dir.to_str().unwrap())?;
        }
        Commands::List => {
            if pkg.dependencies.is_empty() {
                println!("No dependencies found.");
            } else {
                println!("Dependencies:");
                for dep in &pkg.dependencies {
                    let version_info = match &dep.version_constraint {
                        Some(constraint) => format!(" (version: {})", constraint),
                        None => " (latest)".to_string(),
                    };
                    println!("  {}{}", dep.name, version_info);
                }
            }
        }
        Commands::Versions { name } => {
            println!("Checking available versions for {}...", name);
            
            match pkg.get_available_versions(&name).await {
                Ok(versions) => {
                    if versions.is_empty() {
                        println!("No version tags found for {}", name);
                    } else {
                        println!("Available versions for {}:", name);
                        for version in versions {
                            println!("  {}", version);
                        }
                    }
                },
                Err(e) => println!("Error getting versions: {}", e),
            }
        }
        Commands::Token { .. } => {
            unreachable!("Token commands should be handled before this match")
        }
        Commands::Init => {
            unreachable!("Init command should be handled before this match")
        }
    }
    serialization::save_package(&pkg, file_name)?;
    Ok(())
}
