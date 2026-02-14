use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use crate::core::config::{load_config, save_config, OakLock, ModuleConfig, load_global_config};
use crate::registry::find_package;
use crate::ui::*;
use crate::commands::lock::generate_lockfile;

// Recurse function needs to be split or handled carefully with async recursion
// use async_recursion::async_recursion; // Unused
use sha2::{Sha256, Digest};
use std::io::Read;

pub async fn install_command(package: Option<&str>, version: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;
    let global_config = load_global_config();

    let oak_modules_path = Path::new("oak_modules");
    if !oak_modules_path.exists() {
        fs::create_dir_all(oak_modules_path)?;
    }

    if let Some(pkg_name) = package {
        install_single_package(pkg_name, version, &mut config, &global_config, oak_modules_path).await?;
        save_config(&config)?;
    } else {
        // Instalar todas as dependências do oaklibs.json
        print_info("📦 Instalando todas as dependências listadas...");
        let deps = config.dependencies.clone();
        for (name, version) in deps {
            install_single_package(&name, Some(&version), &mut config, &global_config, oak_modules_path).await?;
        }
    }
    
    // Gerar lockfile ao final
    generate_lockfile()?;
    print_success("Instalação concluída.");

    Ok(())
}

async fn install_single_package(
    pkg_name: &str, 
    version: Option<&str>, 
    config: &mut crate::core::config::OakConfig,
    global_config: &crate::core::config::GlobalConfig,
    oak_modules_path: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    
    let (pkg_info, registry_source) = find_package(pkg_name, version, global_config).await?;
    
    print_info(&format!("⬇️ Baixando {}@{} de {}...", pkg_name, pkg_info.version, registry_source));

    // 2. Git Clone
    let pkg_dir = oak_modules_path.join(pkg_name);
    
    if pkg_dir.exists() {
        print_warning("Atualizando pacote existente...");
        fs::remove_dir_all(&pkg_dir)?;
    }

    let status = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--branch")
        .arg(&pkg_info.tag) 
        .arg(&pkg_info.gitUrl)
        .arg(&pkg_dir)
        .status()?;

    if !status.success() {
         print_warning(&format!("Falha ao clonar com tag '{}', tentando branch default...", pkg_info.tag));
         let status_retry = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg(&pkg_info.gitUrl)
            .arg(&pkg_dir)
            .status()?;
            
         if !status_retry.success() {
            return Err("Falha ao clonar repositório git".into());
         }
    }
    
    // Remover .git
    let git_dir = pkg_dir.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(git_dir).ok();
    }

    // 3. Validação de Checksum
    if let Some(expected_hash) = &pkg_info.hash {
        print_info("🛡️ Verificando integridade do pacote...");
        let calculated_hash = calculate_dir_hash(&pkg_dir)?;
        
        if calculated_hash != *expected_hash {
            print_error(&format!("🚨 ERRO DE SEGURANÇA: Checksum não coincide para o pacote '{}'!", pkg_name));
            print_error(&format!("   Esperado: {}", expected_hash));
            print_error(&format!("   Encontrado: {}", calculated_hash));
            
            // Cleanup on failure
            fs::remove_dir_all(&pkg_dir).ok();
            return Err("Abortando instalação devido a falha no checksum".into());
        }
        print_success("✅ Integridade verificada com sucesso.");
    } else {
        print_error(&format!("🚨 ERRO DE SEGURANÇA: O pacote '{}' não possui checksum registrado!", pkg_name));
        print_error("   Instalações sem checksum são altamente inseguras e desabilitadas por padrão.");
        print_error("   Use um registry que forneça hashes de integridade ou verifique o pacote manualmente.");
        
        // Cleanup on failure
        fs::remove_dir_all(&pkg_dir).ok();
        return Err("Abortando instalação devido a ausência de checksum".into());
    }

    print_success(&format!("Pacote '{}' instalado.", pkg_name));

    // Atualizar oaklibs.json
    config.dependencies.insert(pkg_name.to_string(), pkg_info.version);
    
    // Instalar dependências transitivas
    for (dep_name, dep_ver) in pkg_info.dependencies {
        print_info(&format!("🔄 Dependência transitiva: {}@{}", dep_name, dep_ver));
        // Recursão manual simples aqui, idealmente usaríamos um grafo de dependência
        Box::pin(install_single_package(&dep_name, Some(&dep_ver), config, global_config, oak_modules_path)).await?;
    }

    Ok(())
}

fn calculate_dir_hash(dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    let mut entries: Vec<PathBuf> = Vec::new();
    
    collect_files_recursive(dir, dir, &mut entries)?;
    entries.sort(); // Sort by relative path for stability
    
    for relative_path in entries {
        let full_path = dir.join(&relative_path);
        
        // Hash the path name to prevent collisions if content is same but path changes
        hasher.update(relative_path.to_string_lossy().as_bytes());
        
        let mut file = fs::File::open(full_path)?;
        let mut buffer = [0u8; 8192];
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 { break; }
            hasher.update(&buffer[..count]);
        }
    }
    
    let result = hasher.finalize();
    Ok(result.iter().map(|b| format!("{:02x}", b)).collect())
}

fn collect_files_recursive(base_dir: &Path, current_dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Ignore .git if it somehow still exists
            if path.file_name().map(|n| n == ".git").unwrap_or(false) {
                continue;
            }
            collect_files_recursive(base_dir, &path, files)?;
        } else {
            if let Ok(rel) = path.strip_prefix(base_dir) {
                files.push(rel.to_path_buf());
            }
        }
    }
    Ok(())
}
