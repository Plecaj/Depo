# Depo
A simple C++ package manager that helps you manage dependencies for your C++ projects. This tool provides both a command-line interface (CLI) and a graphical user interface (GUI) for managing C++ packages from GitHub repositories.

## Features

- 🚀 **Easy Project Initialization** - Initialize new C++ projects with dependency management
- 📦 **Dependency Management** - Add, remove, update, and manage C++ dependencies
- 🔍 **GitHub Integration** - Search and install packages directly from GitHub repositories
- 🏗️ **CMake Integration** - Automatic CMake configuration for building dependencies
- 🎨 **Modern GUI** -  graphical interface built with Tauri and React
- ⚡ **Fast CLI** - command-line interface for automation and scripting
- 🔒 **Version Constraints** - Manage version requirements for your dependencies


## Prerequisites

### For CLI Usage
- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/) (for cloning repositories)
- [CMake](https://cmake.org/) (for building dependencies)

### For GUI Usage
- All CLI prerequisites
- [Node.js](https://nodejs.org/) (v16 or later)
- [npm](https://www.npmjs.com/) (comes with Node.js)

## Installation

### Building from Source

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd Depo
   ```

2. **Build the CLI:**
   ```bash
   cargo build --release
   ```

3. **Install the CLI:**
   ```bash
   cargo install --path cli
   ```

## GUI Usage

The GUI provides a user-friendly interface for managing C++ packages with the same functionality as the CLI.

### Running the GUI

#### Development Mode
```bash
cd GUI
npm install
npm run tauri dev
```
The application will open automatically

#### Production Build
```bash
cd GUI
npm install
npm run tauri build
```

The built application will be available in `GUI/src-tauri/target/release/`.

### GUI Features

- **Project Selection** - Choose your C++ project directory
- **Dependency Management** - Add, remove, and update dependencies through a visual interface
- **Build Management** - Install and build dependencies with one click
- **Settings** - Configure GitHub tokens and other settings
- **Real-time Updates** - See dependency changes immediately

### GUI Workflow

1. **Launch the application** using the development or production build
2. **Select a project** by choosing a directory containing a package file
3. **Manage dependencies** using the intuitive interface:
   - Add new dependencies by searching GitHub repositories
   - Remove unwanted dependencies
   - Update existing dependencies
   - Modify version constraints
4. **Build your project** with the integrated build tools


## CLI Usage

The CLI tool is called `depo_cli` and provides various commands for managing C++ packages.

### Getting Help

- Check the command help: `depo_cli --help`
- For specific command help: `depo_cli <command> --help`
- Review the generated package configuration files
- Check the console output for detailed error messages



### Basic Commands

#### Initialize a New Project
```bash
depo_cli init
```
Creates a new package configuration file in the current directory.

#### Add Dependencies
```bash
# Add a dependency (interactive selection)
depo_cli add <dependency-name>

# Add with specific version constraint
depo_cli add <dependency-name> --version <version>
```

#### Remove Dependencies
```bash
depo_cli delete <dependency-name>
```

#### Install Dependencies
```bash
depo_cli install
```
Downloads and installs all dependencies listed in your package file.

#### Build Dependencies
```bash
depo_cli build
```
Builds all dependencies using CMake and generates necessary bridge files.

#### List Dependencies
```bash
depo_cli list
```
Shows all currently installed dependencies.

#### Update Dependencies
```bash
depo_cli update <dependency-name>
```

#### Manage Version Constraints
```bash
# Set a new version constraint
depo_cli constraint <dependency-name> --new <version-constraint>

# Remove version constraint
depo_cli constraint <dependency-name> --remove
```

### GitHub Token Configuration

For accessing private repositories or increasing API rate limits, configure a GitHub personal access token:

```bash
# Set GitHub token
depo_cli token set <your-github-token>

# Check if token is configured
depo_cli token check

# Remove token
depo_cli token remove
```

## Package Configuration

The package manager uses a YAML configuration file to track dependencies and project settings. This file is automatically created when you run `pkg init`.

### Example Package File Structure

```yaml
name: "my-project"
version: "1.0.0"
dependencies:
  - name: "nlohmann/json"
    version: "v3.11.2"
    url: "https://github.com/nlohmann/json"
    version_constraint: "v3.11.2"
  - name: "catchorg/Catch2"
    version: "v3.0.0"
    url: "https://github.com/catchorg/Catch2"
    version_constraint: "v3.0.0"
```

## CMake Integration

The package manager automatically generates CMake configuration files to integrate your dependencies into your build system. After running `pkg build`, you can include the generated files in your CMakeLists.txt:

```cmake
cmake_minimum_required(VERSION 3.10)
project(TestProject)

include(deps/CMakeIncludes.cmake)

add_executable(main src/main.cpp)

include(deps/CMakeLinks.cmake)
```

Those 2 include lines are required to add in order to work, CMakeIncludes.cmake before adding exe, CMakeLinks.cmake after!!

## License

This project is licensed under the terms specified in the LICENSE file.

---

**Happy coding with C++ Package Manager!** 🚀
