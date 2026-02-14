// crates/dryad_aot/src/ir/values.rs
//! Valores da IR
//!
//! Constantes e valores que podem ser usados nas instruções.

use super::IrType;

/// Valor da IR (operando de instruções)
#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    /// Constante
    Constant(IrConstant),
    
    /// Referência a um registrador
    Register(IrRegister),
    
    /// Referência a um global
    Global(u32),
    
    /// Referência a um label/bloco
    Label(super::BlockId),
}

/// Constante
#[derive(Debug, Clone, PartialEq)]
pub enum IrConstant {
    /// Inteiro de 8 bits
    I8(i8),
    
    /// Inteiro de 16 bits
    I16(i16),
    
    /// Inteiro de 32 bits
    I32(i32),
    
    /// Inteiro de 64 bits
    I64(i64),
    
    /// Ponto flutuante de 32 bits
    F32(f32),
    
    /// Ponto flutuante de 64 bits
    F64(f64),
    
    /// Booleano
    Bool(bool),
    
    /// String (para constantes)
    String(String),
    
    /// Nulo
    Null,
    
    /// Array de constantes
    Array(Vec<IrConstant>),
    
    /// Struct de constantes
    Struct(Vec<IrConstant>),
}

impl IrConstant {
    /// Retorna o tipo da constante
    pub fn get_type(&self) -> IrType {
        match self {
            IrConstant::I8(_) => IrType::I8,
            IrConstant::I16(_) => IrType::I16,
            IrConstant::I32(_) => IrType::I32,
            IrConstant::I64(_) => IrType::I64,
            IrConstant::F32(_) => IrType::F32,
            IrConstant::F64(_) => IrType::F64,
            IrConstant::Bool(_) => IrType::Bool,
            IrConstant::String(_) => IrType::Ptr(Box::new(IrType::I8)),
            IrConstant::Null => IrType::Ptr(Box::new(IrType::Void)),
            IrConstant::Array(elems) => {
                if let Some(first) = elems.first() {
                    IrType::Array { 
                        elem: Box::new(first.get_type()), 
                        len: elems.len() 
                    }
                } else {
                    IrType::Array { elem: Box::new(IrType::Void), len: 0 }
                }
            }
            IrConstant::Struct(_) => IrType::Struct { fields: vec![] },
        }
    }
    
    /// Converte para i64 (se possível)
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            IrConstant::I8(v) => Some(*v as i64),
            IrConstant::I16(v) => Some(*v as i64),
            IrConstant::I32(v) => Some(*v as i64),
            IrConstant::I64(v) => Some(*v),
            IrConstant::Bool(v) => Some(if *v { 1 } else { 0 }),
            _ => None,
        }
    }
    
    /// Converte para f64 (se possível)
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            IrConstant::F32(v) => Some(*v as f64),
            IrConstant::F64(v) => Some(*v),
            IrConstant::I8(v) => Some(*v as f64),
            IrConstant::I16(v) => Some(*v as f64),
            IrConstant::I32(v) => Some(*v as f64),
            IrConstant::I64(v) => Some(*v as f64),
            _ => None,
        }
    }
}

/// Referência a um registrador virtual
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrRegister {
    /// ID do registrador
    pub id: super::RegisterId,
    
    /// Tipo do valor no registrador
    pub ty: IrType,
}

impl IrRegister {
    pub fn new(id: super::RegisterId, ty: IrType) -> Self {
        Self { id, ty }
    }
}
