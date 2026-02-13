use std::path::{Path, PathBuf};
use dryad_errors::DryadError;
use std::fs;

/// Trait para resolver caminhos de módulos
/// 
/// Permite desacoplar a lógica de resolução de módulos do interpretador,
/// facilitando a integração com diferentes gerenciadores de pacotes (Oak, NPM, etc).
pub trait ModuleResolver: Send + Sync {
    /// Resolve o caminho de importação para um caminho físico
    /// 
    /// # Argumentos
    /// * `module_path` - O caminho ou alias do módulo a ser importado (ex: "./foo", "pkg/mod")
    /// * `current_path` - O caminho do arquivo atual que está fazendo a importação (se houver)
    fn resolve(&self, module_path: &str, current_path: Option<&Path>) -> Result<PathBuf, DryadError>;
}

/// Implementação padrão que resolve apenas caminhos relativos e absolutos locais
pub struct FileSystemResolver;

impl ModuleResolver for FileSystemResolver {
    fn resolve(&self, module_path: &str, current_path: Option<&Path>) -> Result<PathBuf, DryadError> {
        if module_path.starts_with("./") || module_path.starts_with("../") {
            // Caminho relativo
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
            // Para FileSystemResolver, qualquer outra coisa é um erro
            // pois ele não sabe lidar com aliases de pacotes
            Err(DryadError::new(3008, &format!(
                "FileSystemResolver não suporta o alias '{}'. Configure um resolver de pacotes (ex: Oak).", 
                module_path
            )))
        }
    }
}
