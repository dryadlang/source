use std::path::Path;
use std::fs;
use crate::core::config::{OakConfig, ProjectType, OakLock, ModuleConfig, save_config};
use crate::ui::*;

pub fn init_project(name: &str, path: Option<&str>, project_type: ProjectType) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = match path {
        Some(p) => Path::new(p),
        None => Path::new(name),
    };

    if project_dir.exists() {
        return Err(format!("Diretório '{}' já existe", project_dir.display()).into());
    }

    fs::create_dir_all(project_dir)?;

    let mut config = OakConfig::default_for_type(project_type.clone());
    config.name = name.to_string();

    let config_path = project_dir.join("oaklibs.json");
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;

    // Criar .gitignore padrão
    let gitignore_content = "oak_modules/\noaklock.json\n";
    fs::write(project_dir.join(".gitignore"), gitignore_content)?;

    // Criar main.dryad básico
    let main_content = match project_type {
        ProjectType::Project => format!(r#"// {}/main.dryad
// Projeto Dryad gerado pelo Oak

print("Hello World from {}!");
"#, name, name),
        ProjectType::Library => r#"// src/main.dryad
// Biblioteca Dryad

pub fn hello() {
    print("Hello from Library!");
}
"#.to_string(),
    };
    
    match project_type {
        ProjectType::Project => {
            fs::write(project_dir.join("main.dryad"), main_content)?;
        },
        ProjectType::Library => {
             fs::create_dir_all(project_dir.join("src"))?;
             fs::write(project_dir.join("src/main.dryad"), main_content)?;
        }
    }

    print_success(&format!("Projeto '{}' inicializado com sucesso em '{}'", name, project_dir.display()));
    
    match project_type {
        ProjectType::Project => {
            print_info("Para rodar:");
            println!("  cd {}", name);
            println!("  oak run start");
        },
        ProjectType::Library => {
             print_info("Para testar:");
             println!("  cd {}", name);
             println!("  oak run test");
        }
    }

    Ok(())
}
