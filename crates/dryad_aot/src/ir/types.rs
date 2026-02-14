// crates/dryad_aot/src/ir/types.rs
//! Tipos da IR
//!
//! Sistema de tipos de baixo nível para a representação intermediária.

/// Tipo de dados da IR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrType {
    /// Tipo vazio (void)
    Void,
    
    /// Inteiro de 8 bits
    I8,
    
    /// Inteiro de 16 bits
    I16,
    
    /// Inteiro de 32 bits
    I32,
    
    /// Inteiro de 64 bits
    I64,
    
    /// Ponto flutuante de 32 bits
    F32,
    
    /// Ponto flutuante de 64 bits
    F64,
    
    /// Booleano (1 bit, mas geralmente representado como i8)
    Bool,
    
    /// Ponteiro para um tipo
    Ptr(Box<IrType>),
    
    /// Array de tamanho fixo
    Array { elem: Box<IrType>, len: usize },
    
    /// Função (tipo de função)
    Function { params: Vec<IrType>, ret: Box<IrType> },
    
    /// Estrutura/struct
    Struct { fields: Vec<(String, IrType)> },
    
    /// União
    Union { variants: Vec<IrType> },
}

impl IrType {
    /// Retorna o tamanho em bytes do tipo
    pub fn size(&self) -> u32 {
        match self {
            IrType::Void => 0,
            IrType::I8 | IrType::Bool => 1,
            IrType::I16 => 2,
            IrType::I32 | IrType::F32 => 4,
            IrType::I64 | IrType::F64 | IrType::Ptr(_) => 8,
            IrType::Array { elem, len } => elem.size() * *len as u32,
            IrType::Struct { fields } => {
                // Alinhamento simples (não otimizado)
                fields.iter().map(|(_, t)| t.size()).sum()
            }
            IrType::Union { variants } => {
                variants.iter().map(|t| t.size()).max().unwrap_or(0)
            }
            IrType::Function { .. } => 8, // Ponteiro para função
        }
    }
    
    /// Retorna o alinhamento necessário
    pub fn align(&self) -> u32 {
        match self {
            IrType::Void => 1,
            IrType::I8 | IrType::Bool => 1,
            IrType::I16 => 2,
            IrType::I32 | IrType::F32 => 4,
            IrType::I64 | IrType::F64 | IrType::Ptr(_) | IrType::Function { .. } => 8,
            IrType::Array { elem, .. } => elem.align(),
            IrType::Struct { fields } => {
                fields.iter().map(|(_, t)| t.align()).max().unwrap_or(1)
            }
            IrType::Union { variants } => {
                variants.iter().map(|t| t.align()).max().unwrap_or(1)
            }
        }
    }
    
    /// Verifica se é um tipo inteiro
    pub fn is_integer(&self) -> bool {
        matches!(self, IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64)
    }
    
    /// Verifica se é um tipo float
    pub fn is_float(&self) -> bool {
        matches!(self, IrType::F32 | IrType::F64)
    }
    
    /// Verifica se é um ponteiro
    pub fn is_pointer(&self) -> bool {
        matches!(self, IrType::Ptr(_))
    }
}
