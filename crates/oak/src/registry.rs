use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest;
use std::error::Error;
use colored::*;

use crate::core::config::GlobalConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryPackageInfo {
    pub version: String,
    pub gitUrl: String,
    pub tag: String,
    pub hash: Option<String>,
    pub dependencies: HashMap<String, String>,
}

pub async fn find_package(
    package_name: &str,
    version: Option<&str>,
    config: &GlobalConfig,
) -> Result<(RegistryPackageInfo, String), Box<dyn Error>> {
    println!("{}", format!("🔍 Procurando pacote '{}'...", package_name).cyan());

    let mut found_packages = Vec::new();
    let client = reqwest::Client::new();

    // Query all registries
    for (reg_name, reg_url) in &config.registries {
        if !reg_url.starts_with("https://") {
            println!("{}", format!("⚠️ AVISO: Registry '{}' usa conexão insegura ({}). Recomenda-se HTTPS.", reg_name, reg_url).yellow());
        }

        let url = if let Some(ver) = version {
            format!("{}/packages/{}/{}", reg_url, package_name, ver)
        } else {
            format!("{}/packages/{}", reg_url, package_name)
        };

        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                if let Ok(pkg_info) = resp.json::<RegistryPackageInfo>().await {
                    found_packages.push((reg_name.clone(), pkg_info));
                }
            }
        }
    }

    if found_packages.is_empty() {
        return Err(format!("Pacote '{}' não encontrado em nenhum registry.", package_name).into());
    }

    if found_packages.len() == 1 {
        let (reg_name, pkg) = found_packages.remove(0);
        println!("{}", format!("✓ Encontrado no registry '{}'", reg_name).green());
        return Ok((pkg, reg_name));
    }

    // Conflict resolution
    println!("{}", format!("⚠️ Conflito: Pacote '{}' encontrado em múltiplos registries:", package_name).yellow());

    let selections: Vec<String> = found_packages.iter()
        .map(|(reg, pkg)| format!("{} (v{}) - {}", reg, pkg.version, pkg.gitUrl))
        .collect();

    let selection = inquire::Select::new(
        "Selecione de qual registry instalar:",
        selections,
    ).prompt()?;

    // Find the selected package index
    let index = found_packages.iter().position(|(reg, pkg)| {
        format!("{} (v{}) - {}", reg, pkg.version, pkg.gitUrl) == selection
    }).unwrap();

    let (reg_name, pkg) = found_packages.remove(index);
    Ok((pkg, reg_name))
}
