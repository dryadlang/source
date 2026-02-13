use std::path::Path;
use std::fs;
use std::collections::HashMap;
use crate::core::config::{OakLock, ModuleConfig};
use crate::ui::*;

pub fn generate_lockfile() -> Result<(), Box<dyn std::error::Error>> {
    print_info("🔐 Gerando oaklock.json...");
    let oak_modules_dir = Path::new("oak_modules");
    if !oak_modules_dir.exists() {
         return Ok(());
    }

    let mut lock = OakLock::default();

    // Ler diretórios em oak_modules
    for entry in fs::read_dir(oak_modules_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
             if let Some(pkg_name) = path.file_name().and_then(|n| n.to_str()) {
                 let mut module_config = ModuleConfig {
                     paths: HashMap::new(),
                 };

                 // Indexar arquivos .dryad
                 index_package_files(&path, pkg_name, &path, &mut module_config.paths)?;
                 
                 lock.modules.insert(pkg_name.to_string(), module_config);
             }
        }
    }

    let json = serde_json::to_string_pretty(&lock)?;
    fs::write("oaklock.json", json)?;
    print_success("oaklock.json atualizado.");
    Ok(())
}

fn index_package_files(base_pkg_path: &Path, _pkg_name: &str, current_dir: &Path, paths: &mut HashMap<String, String>) -> std::io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            index_package_files(base_pkg_path, _pkg_name, &path, paths)?;
        } else if let Some(ext) = path.extension() {
            if ext == "dryad" {
                // Calcular alias relativo
                let relative_path = path.strip_prefix(base_pkg_path.parent().unwrap()).unwrap();
                let relative_str = relative_path.to_string_lossy().replace("\\", "/");
                
                // Remover extensão .dryad
                let alias = relative_str.trim_end_matches(".dryad").to_string();
                
                // Caminho físico (relativo à raiz do projeto)
                let physical_path = format!("./{}", path.to_string_lossy().replace("\\", "/"));
                
                paths.insert(alias, physical_path);
            }
        }
    }
    Ok(())
}
