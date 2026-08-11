// crates/dryad_aot/src/linker/mod.rs
//! Linker
//!
//! Responsável por linkar arquivos objeto e criar executáveis.

use std::process::Command;

/// Linker externo (gcc, clang, etc.)
pub struct ExternalLinker {
    /// Comando do linker
    command: String,
    
    /// Flags adicionais
    flags: Vec<String>,
}

impl ExternalLinker {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            flags: Vec::new(),
        }
    }
    
    /// Adiciona uma flag
    pub fn add_flag(&mut self, flag: impl Into<String>) -> &mut Self {
        self.flags.push(flag.into());
        self
    }
    
    /// Linka arquivos objeto para criar executável
    pub fn link(&self, object_files: &[&str], output: &str, libraries: &[&str]) -> Result<(), String> {
        let mut cmd = Command::new(&self.command);
        
        // Arquivos objeto
        for obj in object_files {
            cmd.arg(obj);
        }
        
        // Output
        cmd.arg("-o").arg(output);
        
        // Bibliotecas
        for lib in libraries {
            cmd.arg(format!("-l{}", lib));
        }
        
        // Flags adicionais
        cmd.args(&self.flags);
        
        // Executar
        let result = cmd.output()
            .map_err(|e| format!("Erro ao executar linker: {}", e))?;
        
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(format!("Erro de linkagem: {}", stderr));
        }
        
        Ok(())
    }
}
