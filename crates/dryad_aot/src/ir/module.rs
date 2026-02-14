// crates/dryad_aot/src/ir/module.rs
//! Módulo da IR
//!
//! Representa um módulo completo com funções, globais e metadados.

use super::{BlockId, IrBlock, IrInstruction, IrType, IrValue};
use std::collections::HashMap;

/// Módulo IR completo
#[derive(Debug, Clone)]
pub struct IrModule {
    /// Nome do módulo
    pub name: String,
    
    /// Funções do módulo
    pub functions: Vec<IrFunction>,
    
    /// Variáveis globais
    pub globals: Vec<IrGlobal>,
    
    /// Metadados
    pub metadata: HashMap<String, String>,
    
    /// Contador de registradores (para gerar IDs únicos)
    pub next_register_id: u32,
    
    /// Contador de blocos
    pub next_block_id: u32,
}

impl IrModule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::new(),
            globals: Vec::new(),
            metadata: HashMap::new(),
            next_register_id: 0,
            next_block_id: 0,
        }
    }
    
    /// Adiciona uma função ao módulo
    pub fn add_function(&mut self, func: IrFunction) -> u32 {
        let id = self.functions.len() as u32;
        self.functions.push(func);
        id
    }
    
    /// Adiciona uma global ao módulo
    pub fn add_global(&mut self, global: IrGlobal) -> u32 {
        let id = self.globals.len() as u32;
        self.globals.push(global);
        id
    }
    
    /// Gera um novo ID de registrador
    pub fn new_register(&mut self) -> super::RegisterId {
        let id = self.next_register_id;
        self.next_register_id += 1;
        id
    }
    
    /// Gera um novo ID de bloco
    pub fn new_block_id(&mut self) -> BlockId {
        let id = self.next_block_id;
        self.next_block_id += 1;
        id
    }
    
    /// Obtém uma função pelo ID
    pub fn get_function(&self, id: u32) -> Option<&IrFunction> {
        self.functions.get(id as usize)
    }
    
    /// Obtém uma função pelo ID (mutable)
    pub fn get_function_mut(&mut self, id: u32) -> Option<&mut IrFunction> {
        self.functions.get_mut(id as usize)
    }
}

/// Função na IR
#[derive(Debug, Clone)]
pub struct IrFunction {
    /// Nome da função
    pub name: String,
    
    /// Parâmetros (registradores de entrada)
    pub params: Vec<(super::RegisterId, IrType)>,
    
    /// Tipo de retorno
    pub return_type: IrType,
    
    /// Blocos básicos
    pub blocks: Vec<IrBlock>,
    
    /// Bloco de entrada
    pub entry_block: BlockId,
    
    /// Variáveis locais (stack allocations)
    pub locals: Vec<IrLocal>,
    
    /// Se é uma função externa (importada)
    pub is_external: bool,
    
    /// Se é uma função exportada
    pub is_exported: bool,
}

impl IrFunction {
    pub fn new(name: impl Into<String>, return_type: IrType) -> Self {
        Self {
            name: name.into(),
            params: Vec::new(),
            return_type,
            blocks: Vec::new(),
            entry_block: 0,
            locals: Vec::new(),
            is_external: false,
            is_exported: false,
        }
    }
    
    /// Adiciona um parâmetro
    pub fn add_param(&mut self, reg: super::RegisterId, ty: IrType) {
        self.params.push((reg, ty));
    }
    
    /// Adiciona um bloco
    pub fn add_block(&mut self, block: IrBlock) {
        self.blocks.push(block);
    }
    
    /// Obtém um bloco pelo ID
    pub fn get_block(&self, id: BlockId) -> Option<&IrBlock> {
        self.blocks.iter().find(|b| b.id == id)
    }
    
    /// Obtém um bloco pelo ID (mutable)
    pub fn get_block_mut(&mut self, id: BlockId) -> Option<&mut IrBlock> {
        self.blocks.iter_mut().find(|b| b.id == id)
    }
    
    /// Adiciona uma variável local
    pub fn add_local(&mut self, name: impl Into<String>, ty: IrType, offset: i32) {
        self.locals.push(IrLocal {
            name: name.into(),
            ty,
            offset,
        });
    }
}

/// Variável local (stack allocation)
#[derive(Debug, Clone)]
pub struct IrLocal {
    /// Nome da variável
    pub name: String,
    
    /// Tipo
    pub ty: IrType,
    
    /// Offset do stack pointer
    pub offset: i32,
}

/// Variável global
#[derive(Debug, Clone)]
pub struct IrGlobal {
    /// Nome da global
    pub name: String,
    
    /// Tipo
    pub ty: IrType,
    
    /// Valor inicial (se houver)
    pub initializer: Option<IrValue>,
    
    /// Se é mutável
    pub is_mutable: bool,
    
    /// Se é exportada
    pub is_exported: bool,
}

impl IrGlobal {
    pub fn new(name: impl Into<String>, ty: IrType) -> Self {
        Self {
            name: name.into(),
            ty,
            initializer: None,
            is_mutable: true,
            is_exported: false,
        }
    }
    
    pub fn with_initializer(mut self, value: IrValue) -> Self {
        self.initializer = Some(value);
        self
    }
    
    pub fn immutable(mut self) -> Self {
        self.is_mutable = false;
        self
    }
    
    pub fn exported(mut self) -> Self {
        self.is_exported = true;
        self
    }
}
