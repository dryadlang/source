use std::process::Command;
use crate::core::config::load_config;
use crate::ui::*;

pub fn run_script(script_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    
    if let Some(command) = config.scripts.get(script_name) {
        print_info(&format!("🚀 Executando script '{}': {}", script_name, command));
        
        #[cfg(target_os = "windows")]
        let status = Command::new("cmd")
            .args(["/C", command])
            .status()?;

        #[cfg(not(target_os = "windows"))]
        let status = Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()?;
            
        if !status.success() {
            return Err(format!("Script falhou com código: {}", status).into());
        }
        Ok(())
    } else {
        Err(format!("Script '{}' não encontrado em oaklibs.json", script_name).into())
    }
}
