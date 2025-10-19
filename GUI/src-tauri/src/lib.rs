use pkgcore::build::{BuildSystem, CMake};
use pkgcore::dependency::Dependency;
use pkgcore::package::Package;
use pkgcore::serialization;

#[tauri::command]
fn init(path: &str) -> Result<Package, String> {
    Package::init(path).map_err(|e| e.to_string())
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
    pkg.add_dependency(dep, &path).map_err(|e| e.to_string())?;
    serialization::save_package(&pkg, &path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn delete_dependency(path: &str, name: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    pkg.remove_dependency(name, &path).map_err(|e| e.to_string())?;
    serialization::save_package(&pkg, &path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn install_dependencies(path: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    for dep in pkg.dependencies.iter_mut() {
        dep.install(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn update_dependency(path: &str, name: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    pkg.update_dependency(name, path).map_err(|e| e.to_string())?;
    serialization::save_package(&pkg, path).map_err(|e| e.to_string())?;
    Ok(())
}
#[tauri::command]
fn modify_dependency_constraint(path: &str, name: &str, new_constraint: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    pkg.modify_dependency_constraint(name, new_constraint, path)
        .map_err(|e| e.to_string())?;
    serialization::save_package(&pkg, path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn remove_dependency_constraint(path: &str, name: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    pkg.remove_dependency_constraint(name, path)
        .map_err(|e| e.to_string())?;
    serialization::save_package(&pkg, path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn build_dependencies(path: &str) -> Result<(), String> {
    let mut pkg = serialization::load_package(path).map_err(|e| e.to_string())?;
    for dep in pkg.dependencies.iter_mut() {
        CMake::build_dependency(dep, &path).map_err(|e| e.to_string())?;
    }
    CMake::generate_dependency_bridge(&pkg.dependencies, &path).map_err(|e| e.to_string())?;
    Ok(())
}
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
