/// Tipos de erro para o runtime do Dryad
use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// Erro de I/O (leitura/escrita de arquivos, console, etc.)
    IoError(String),
    
    /// Erro de argumentos (número ou tipo incorreto)
    ArgumentError(String),
    
    /// Erro de tipo (conversão ou operação inválida)
    TypeError(String),
    
    /// Erro de rede (HTTP, TCP, UDP, WebSocket)
    NetworkError(String),
    
    /// Erro de sistema (variáveis de ambiente, processos, etc.)
    SystemError(String),
    
    /// Erro de criptografia
    CryptoError(String),
    
    /// Erro de heap (alocação ou referência inválida)
    HeapError(String),
    
    /// Erro genérico
    Generic(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::IoError(msg) => write!(f, "Erro de I/O: {}", msg),
            RuntimeError::ArgumentError(msg) => write!(f, "Erro de argumentos: {}", msg),
            RuntimeError::TypeError(msg) => write!(f, "Erro de tipo: {}", msg),
            RuntimeError::NetworkError(msg) => write!(f, "Erro de rede: {}", msg),
            RuntimeError::SystemError(msg) => write!(f, "Erro de sistema: {}", msg),
            RuntimeError::CryptoError(msg) => write!(f, "Erro de criptografia: {}", msg),
            RuntimeError::HeapError(msg) => write!(f, "Erro de heap: {}", msg),
            RuntimeError::Generic(msg) => write!(f, "Erro: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError::IoError(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for RuntimeError {
    fn from(error: std::num::ParseFloatError) -> Self {
        RuntimeError::TypeError(format!("Erro ao converter para número: {}", error))
    }
}

impl From<std::num::ParseIntError> for RuntimeError {
    fn from(error: std::num::ParseIntError) -> Self {
        RuntimeError::TypeError(format!("Erro ao converter para inteiro: {}", error))
    }
}
