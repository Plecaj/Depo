use pkgcore::build::{BuildSystem, CMake};
use pkgcore::dependency::Dependency;
use pkgcore::package::Package;
use pkgcore::serialization;

#[tauri::command]
fn init(file: &str) -> Result<Package, String> {
    Package::init(file).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_project_deps(path: &str) -> Result<Vec<Dependency>, String> {
    let pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    Ok(pkg.dependencies)
}

#[tauri::command]
async fn find_dependency(path: &str, name: &str) -> Result<Vec<Dependency>, String> {
    let pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    pkg.find_dependency(name).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn add_dependency(path: &str, dep: Dependency) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    pkg.add_dependency(dep).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_dependency(path: &str, name: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    pkg.remove_dependency(name).map_err(|e| e.to_string())
}

#[tauri::command]
fn install_dependencies(path: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    for dep in pkg.dependencies.iter_mut() {
        dep.install().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn build_dependencies(path: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    for dep in pkg.dependencies.iter_mut() {
        CMake::build_dependency(dep).map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init,
            get_project_deps,
            find_dependency,
            add_dependency,
            delete_dependency,
            install_dependencies,
            build_dependencies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
