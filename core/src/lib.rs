//! # C++ Package Manager Core Library
//!
//! This library provides the core functionality for managing C++ package dependencies.
//! It includes modules for:
//! - Package management and dependency resolution
//! - Configuration handling
//! - Build system integration (CMake)
//! - Serialization and persistence
//!
//! ## Overview
//!
//! The package manager allows users to:
//! - Initialize new C++ projects with dependency management
//! - Search and add dependencies from GitHub repositories
//! - Manage version constraints for dependencies
//! - Build projects using CMake integration
//! - Serialize package configurations to YAML files
//!


/// Build system integration module
pub mod build;

/// Configuration management module
pub mod config;

/// Dependency management module
pub mod dependency;

/// Package management module
pub mod package;

/// Serialization and persistence module
pub mod serialization;
