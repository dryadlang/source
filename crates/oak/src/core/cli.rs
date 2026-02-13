use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oak")]
#[command(about = "Oak - Gestor de Pacotes para Dryad", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Inicializa um novo projeto Dryad
    Init {
        /// Nome do projeto
        name: String,
        /// Diretório para criar o projeto (opcional)
        #[arg(short, long)]
        path: Option<String>,
        /// Tipo de projeto (project ou library)
        #[arg(short, long, default_value = "project")]
        r#type: String,
    },
    /// Instala dependências do projeto
    Install {
        /// Nome do pacote para instalar
        package: Option<String>,
        /// Versão específica
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Executa scripts definidos no projeto
    Run {
        /// Nome do script para executar
        script: String,
    },
    /// Constrói o oaklock.json baseado nas dependências
    Lock,
    /// Executa um arquivo Dryad diretamente
    Exec {
        /// Caminho do arquivo .dryad para executar
        file: String,
        /// Apenas validar sintaxe sem executar
        #[arg(short, long)]
        validate: bool,
        /// Argumentos para passar ao programa
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// Gerencia registries de pacotes
    Registry {
        #[command(subcommand)]
        action: RegistryAction,
    },
}

#[derive(Subcommand)]
pub enum RegistryAction {
    /// Lista registries configurados
    List,
    /// Adiciona um novo registry
    Add {
        /// Nome do registry
        name: String,
        /// URL do registry
        url: String,
    },
    /// Remove um registry
    Remove {
        /// Nome do registry
        name: String,
    },
}
