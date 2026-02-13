use std::path::{Path, PathBuf};
use std::fs;
use dryad_runtime::resolver::ModuleResolver;
use dryad_errors::DryadError;
use serde_json::Value as JsonValue;

/// Resolver de módulos que suporta o gerenciador de pacotes Oak
pub struct OakModuleResolver;

impl OakModuleResolver {
    fn resolve_oak_module(&self, module_alias: &str) -> Result<PathBuf, DryadError> {
        // Tentar carregar oaklock.json
        let oaklock_path = PathBuf::from("oaklock.json");
        
        if !oaklock_path.exists() {
            return Err(DryadError::new(3005, &format!(
                "oaklock.json não encontrado. Não é possível resolver módulo '{}'", 
                module_alias
            )));
        }
        
        let oaklock_content = fs::read_to_string(&oaklock_path)
            .map_err(|e| DryadError::new(3006, &format!("Erro ao ler oaklock.json: {}", e)))?;
        
        let oaklock: JsonValue = serde_json::from_str(&oaklock_content)
            .map_err(|e| DryadError::new(3007, &format!("Erro ao parsear oaklock.json: {}", e)))?;
        
        // Parsear alias do tipo "pacote/módulo" ou "pacote/subdir/módulo"
        // Exemplos: "matematica-utils/matematica", "greenleaf/math/arrays"
        let parts: Vec<&str> = module_alias.split('/').collect();
        if parts.is_empty() {
            return Err(DryadError::new(3008, &format!(
                "Alias de módulo inválido: '{}'. Esperado formato 'pacote/módulo' ou 'pacote/subdir/módulo'", 
                module_alias
            )));
        }
        
        let package_name = parts[0];
        // O resto é o caminho do módulo dentro do pacote
        let module_path_parts = &parts[1..];
        let module_name = module_path_parts.join("/");
        
        // Procurar no oaklock.json
        let modules = oaklock.get("modules")
            .ok_or_else(|| DryadError::new(3009, "Seção 'modules' não encontrada no oaklock.json"))?;
        
        let package = modules.get(package_name)
            .ok_or_else(|| DryadError::new(3010, &format!("Pacote '{}' não encontrado no oaklock.json", package_name)))?;
        
        let paths = package.get("paths")
            .ok_or_else(|| DryadError::new(3011, &format!("Seção 'paths' não encontrada para pacote '{}'", package_name)))?;
        
        let module_path = paths.get(&module_name)
            .ok_or_else(|| DryadError::new(3012, &format!("Módulo '{}' não encontrado no pacote '{}'", module_name, package_name)))?
            .as_str()
            .ok_or_else(|| DryadError::new(3013, &format!("Caminho inválido para módulo '{}/{}'", package_name, module_name)))?;
        
        Ok(PathBuf::from(module_path))
    }
}

impl ModuleResolver for OakModuleResolver {
    fn resolve(&self, module_path: &str, current_path: Option<&Path>) -> Result<PathBuf, DryadError> {
        if module_path.starts_with("./") || module_path.starts_with("../") {
            // Caminho relativo - delegar para lógica padrão de sistema de arquivos
            if let Some(current_file) = current_path {
                let base_dir = current_file.parent()
                    .ok_or_else(|| DryadError::new(3004, "Não é possível determinar diretório base"))?;
                Ok(base_dir.join(module_path))
            } else {
                // Se não há arquivo atual, usar diretório de trabalho
                Ok(PathBuf::from(module_path))
            }
        } else if module_path.starts_with("@/") {
            // Caminho absoluto do projeto
            let relative_path = &module_path[2..]; // Remove "@/"
            Ok(PathBuf::from(relative_path))
        } else {
            // Tentativa de usar Oak (oaklock.json)
            self.resolve_oak_module(module_path)
        }
    }
}
