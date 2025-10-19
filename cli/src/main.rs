//! # C++ Package Manager CLI
//!
//! This is the command-line interface for the C++ package manager.
//! It provides commands for initializing projects, managing dependencies,
//! and building C++ projects with their dependencies.

use clap::{Parser, Subcommand};
use pkgcore::{
    build::{BuildSystem, CMake},
    config::Config,
    package::Package,
    serialization,
};
use std::env;

/// Main CLI structure for the C++ package manager
///
/// This structure defines the command-line interface using clap for argument parsing.
/// It supports various subcommands for package management operations.
#[derive(Parser)]
#[command(name = "pkg")]
#[command(about = "A simple c++ package manager", long_about = None)]
struct Cli {
    /// The command to execute
    #[command(subcommand)]
    command: Commands,
}

/// Available CLI commands for package management
#[derive(Subcommand, PartialEq)]
enum Commands {
    /// Initialize a new package in the current directory
    Init,
    /// Add a new dependency to the package
    Add {
        /// Name of the dependency to add
        name: String,
        /// Optional version constraint for the dependency
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Remove a dependency from the package
    Delete {
        /// Name of the dependency to remove
        name: String,
    },
    /// Install all dependencies
    Install,
    /// Update a specific dependency to its latest version
    Update {
        /// Name of the dependency to update
        name: String,
    },
    /// Build all dependencies
    Build,
    /// List all dependencies in the package
    List,
    /// Manage version constraints for dependencies
    Constraint {
        /// Name of the dependency to modify
        name: String,
        /// New version constraint to set
        #[arg(short, long)]
        new: Option<String>,
        /// Remove the existing version constraint
        #[arg(long)]
        remove: bool,
    },
    /// Manage GitHub API token configuration
    Token {
        /// Token management action to perform
        #[command(subcommand)]
        action: TokenAction,
    },
}

/// Available token management actions
#[derive(Subcommand, PartialEq)]
enum TokenAction {
    /// Set a new GitHub API token
    Set { 
        /// The GitHub personal access token
        token: String 
    },
    /// Check if a token is configured
    Check,
    /// Remove the configured token
    Remove,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let working_dir = env::current_dir()?;

    if let Commands::Init = cli.command {
        Package::init(&working_dir.to_str().unwrap())?;
        return Ok(());
    }

    if let Commands::Token { .. } = cli.command {
        match cli.command {
            Commands::Token { action } => match action {
                TokenAction::Set { token } => match Config::create_env_file(&token) {
                    Ok(_) => println!("GitHub token saved successfully!"),
                    Err(e) => println!("Error saving token: {}", e),
                },
                TokenAction::Check => match Config::load() {
                    Ok(config) => {
                        if config.has_token() {
                            println!("GitHub token is configured");
                            let token = &config.github_token.unwrap();
                            println!("Token: ...{}", &token[token.len().saturating_sub(8)..]);
                        } else {
                            println!("No GitHub token found");
                            println!("Use 'pkg token set <your_token>' to add one");
                        }
                    }
                    Err(e) => println!("Error loading config: {}", e),
                },
                TokenAction::Remove => match std::fs::remove_file(".pkg.env") {
                    Ok(_) => println!("GitHub token removed successfully!"),
                    Err(e) => println!("Error removing token: {}", e),
                },
            },
            _ => unreachable!(),
        }
        return Ok(());
    }
    let mut pkg = match serialization::load_package(&working_dir.to_str().unwrap()) {
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
            for dep in pkg.dependencies.iter_mut() {
                match dep.install(&working_dir.to_str().unwrap()) {
                    Ok(_) => println!("Installed dependency '{}'", dep.name),
                    Err(e) => eprintln!("Failed to install dependency '{}': {}", dep.name, e),
                }
            }
        }
        Commands::Update { name } => {
            match pkg.update_dependency(&name, &working_dir.to_str().unwrap()) {
                Ok(_) => println!("Dependency '{}' updated successfully!", name),
                Err(e) => eprintln!("Failed to update dependency '{}': {}", name, e),
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
                    println!("  {}@{}", dep.name, dep.version);
                }
            }
        }
        Commands::Constraint { name, new, remove } => {
            if remove {
                match pkg.remove_dependency_constraint(&name, &working_dir.to_str().unwrap()) {
                    Ok(_) => println!("Removed constraint for dependency '{}'", name),
                    Err(e) => eprintln!("Failed to remove constraint for dependency '{}': {}", name, e),
                }
            } else if let Some(new_constraint) = new {
                match pkg.modify_dependency_constraint(&name, &new_constraint, &working_dir.to_str().unwrap()) {
                    Ok(_) => println!(
                        "Dependency '{}' constraint updated to '{}'",
                        name, new_constraint
                    ),
                    Err(e) => eprintln!(
                        "Failed to update constraint for dependency '{}': {}",
                        name, e
                    ),
                }
            } else {
                eprintln!("Error: must provide either --new <constraint> or --remove");
            }
        }
        Commands::Token { .. } => {
            unreachable!("Token commands should be handled before this match")
        }
        Commands::Init => {
            unreachable!("Init command should be handled before this match")
        }
    }
    serialization::save_package(&pkg, working_dir.to_str().unwrap())?;
    Ok(())
}
