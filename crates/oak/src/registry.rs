use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest;
use std::error::Error;
use colored::*;

use crate::core::config::GlobalConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryPackageInfo {
    pub version: String,
    pub gitUrl: String,
    pub tag: String,
    pub hash: Option<String>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionResponse {
    pub versions: Vec<RegistryPackageInfo>,
}

pub async fn find_package(
    package_name: &str,
    version_range: Option<&str>,
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

        // Se uma versão específica foi pedida, tentamos buscar direto.
        // Caso contrário, buscamos a lista de versões (ou a última disponível no registry)
        let url = if let Some(ver) = version_range {
            format!("{}/packages/{}/{}", reg_url, package_name, ver)
        } else {
            format!("{}/packages/{}", reg_url, package_name)
        };

        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                // Tenta desserializar como pacote único ou como lista dependendo da API
                if let Ok(pkg_info) = resp.json::<RegistryPackageInfo>().await {
                    found_packages.push((reg_name.clone(), pkg_info));
                }
            }
        }
    }

    if found_packages.is_empty() {
        return Err(format!("Pacote '{}' não encontrado em nenhum registry.", package_name).into());
    }

    // Se houver range de versão, aqui deveríamos filtrar se tivéssemos múltiplas versões.
    // Atualmente as APIs retornam a melhor correspondência simples ou falham.
    // Vou manter a lógica de seleção manual para conflitos entre registries diferentes.

    if found_packages.len() == 1 {
        let (reg_name, pkg) = found_packages.remove(0);
        println!("{}", format!("✓ Encontrado no registry '{}' (v{})", reg_name, pkg.version).green());
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
