use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use dirs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OakConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    pub main: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub scripts: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Project,
    Library,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OakLock {
    pub modules: HashMap<String, ModuleConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleConfig {
    pub paths: HashMap<String, String>,
}

impl Default for OakConfig {
    fn default() -> Self {
        Self::default_for_type(ProjectType::Project)
    }
}

impl OakConfig {
    pub fn default_for_type(project_type: ProjectType) -> Self {
        let mut scripts = HashMap::new();
        
        match project_type {
            ProjectType::Project => {
                scripts.insert("start".to_string(), "oak exec main.dryad".to_string());
                scripts.insert("test".to_string(), "oak exec tests/test.dryad".to_string());
                
                OakConfig {
                    name: "meu-projeto".to_string(),
                    version: "0.1.0".to_string(),
                    description: None,
                    author: None,
                    license: Some("MIT".to_string()),
                    project_type: ProjectType::Project,
                    main: Some("main.dryad".to_string()),
                    dependencies: HashMap::new(),
                    scripts,
                }
            }
            ProjectType::Library => {
                scripts.insert("test".to_string(), "oak exec tests/test.dryad".to_string());
                
                OakConfig {
                    name: "minha-biblioteca".to_string(),
                    version: "0.1.0".to_string(),
                    description: None,
                    author: None,
                    license: Some("MIT".to_string()),
                    project_type: ProjectType::Library,
                    main: Some("src/main.dryad".to_string()),
                    dependencies: HashMap::new(),
                    scripts,
                }
            }
        }
    }
}

impl Default for OakLock {
    fn default() -> Self {
        OakLock {
            modules: HashMap::new(),
        }
    }
}

pub fn load_config() -> Result<OakConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("oaklibs.json")?;
    Ok(serde_json::from_str(&content)?)
}

pub fn save_config(config: &OakConfig) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write("oaklibs.json", json)?;
    Ok(())
}

// Global Registry Config
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalConfig {
    pub registries: HashMap<String, String>,
    pub default_registry: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        let mut registries = HashMap::new();
        registries.insert("official".to_string(), "http://localhost:4000/api".to_string());
        
        GlobalConfig {
            registries,
            default_registry: "official".to_string(),
        }
    }
}

pub fn load_global_config() -> GlobalConfig {
    let config_path = get_global_config_path();
    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }
    GlobalConfig::default()
}

pub fn save_global_config(config: &GlobalConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_global_config_path();
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)?;
    fs::write(config_path, json)?;
    Ok(())
}

fn get_global_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".oak");
    path.push("config.json");
    path
}
