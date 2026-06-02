use clap::Parser;
use tokio;

mod core;
mod registry;
mod commands;
mod ui;

use crate::core::cli::{Cli, Commands, RegistryAction};
use crate::core::config;
use commands::*;
use ui::*;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { name, path, r#type } => {
            let project_type = match r#type.to_lowercase().as_str() {
                "library" => config::ProjectType::Library,
                "project" => config::ProjectType::Project,
                _ => {
                    print_error("Tipo de projeto inválido. Use 'project' ou 'library'");
                    std::process::exit(1);
                }
            };
            init::init_project(&name, path.as_deref(), project_type)
        }
        Commands::Install { package, version } => {
            install::install_command(package.as_deref(), version.as_deref()).await
        }
        Commands::Run { script } => {
            run::run_script(&script)
        }
        Commands::Lock => {
            lock::generate_lockfile()
        }
        Commands::Exec { file, validate, args } => {
            exec::execute_dryad_file(&file, &args, validate)
        }
        Commands::Registry { action } => {
            match action {
                RegistryAction::List => {
                    commands::registry::registry_list();
                    Ok(())
                },
                RegistryAction::Add { name, url } => {
                    commands::registry::registry_add(&name, &url)
                },
                RegistryAction::Remove { name } => {
                    commands::registry::registry_remove(&name)
                }
            }
        }
    };

    if let Err(e) = result {
        print_error(&format!("{}", e));
        std::process::exit(1);
    }
}
