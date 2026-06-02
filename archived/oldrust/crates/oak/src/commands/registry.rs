use crate::core::config::{load_global_config, save_global_config};
use crate::ui::*;

pub fn registry_list() {
    let config = load_global_config();
    println!("Registries Configurados:");
    for (name, url) in config.registries {
        if name == config.default_registry {
             println!("  * {} -> {}", name, url);
        } else {
             println!("    {} -> {}", name, url);
        }
    }
}

pub fn registry_add(name: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_global_config();
    if config.registries.contains_key(name) {
        return Err(format!("Registry '{}' já existe.", name).into());
    }
    config.registries.insert(name.to_string(), url.to_string());
    save_global_config(&config)?;
    print_success(&format!("Registry '{}' adicionado.", name));
    Ok(())
}

pub fn registry_remove(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_global_config();
    if !config.registries.contains_key(name) {
        return Err(format!("Registry '{}' não encontrado.", name).into());
    }
    if name == config.default_registry {
         return Err("Não é possível remover o registry padrão.".into());
    }
    config.registries.remove(name);
    save_global_config(&config)?;
    print_success(&format!("Registry '{}' removido.", name));
    Ok(())
}
