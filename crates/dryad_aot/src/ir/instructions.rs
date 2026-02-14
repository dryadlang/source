// crates/dryad_aot/src/ir/instructions.rs
//! Instruções da IR
//!
//! Instruções de baixo nível independentes de arquitetura específica.

use super::{BlockId, IrType, IrValue, RegisterId};

/// Instrução da IR
#[derive(Debug, Clone, PartialEq)]
pub enum IrInstruction {
    // ============================================
    // Movimentação de dados
    // ============================================
    /// Carrega uma constante em um registrador
    /// LoadConst dest, constant
    LoadConst { dest: RegisterId, value: IrValue },
    
    /// Move valor entre registradores
    /// Move dest, src
    Move { dest: RegisterId, src: RegisterId },
    
    /// Carrega da memória
    /// Load dest, ptr
    Load { dest: RegisterId, ptr: RegisterId },
    
    /// Armazena na memória
    /// Store ptr, value
    Store { ptr: RegisterId, value: RegisterId },
    
    /// Carrega endereço de um global
    /// LoadGlobal dest, global_id
    LoadGlobal { dest: RegisterId, global_id: u32 },
    
    /// Carrega endereço de um local (stack offset)
    /// LoadLocal dest, offset
    LoadLocal { dest: RegisterId, offset: i32 },

    // ============================================
    // Aritmética
    // ============================================
    /// Adição: dest = lhs + rhs
    Add { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Subtração: dest = lhs - rhs
    Sub { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Multiplicação: dest = lhs * rhs
    Mul { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Divisão: dest = lhs / rhs
    Div { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Módulo: dest = lhs % rhs
    Mod { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Negação: dest = -src
    Neg { dest: RegisterId, src: RegisterId },

    // ============================================
    // Comparação
    // ============================================
    /// Comparação igual: dest = (lhs == rhs)
    CmpEq { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Comparação diferente: dest = (lhs != rhs)
    CmpNe { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Comparação menor: dest = (lhs < rhs)
    CmpLt { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Comparação menor ou igual: dest = (lhs <= rhs)
    CmpLe { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Comparação maior: dest = (lhs > rhs)
    CmpGt { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Comparação maior ou igual: dest = (lhs >= rhs)
    CmpGe { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },

    // ============================================
    // Lógica e Bitwise
    // ============================================
    /// AND lógico/bitwise: dest = lhs & rhs
    And { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// OR lógico/bitwise: dest = lhs | rhs
    Or { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// XOR: dest = lhs ^ rhs
    Xor { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// NOT: dest = !src
    Not { dest: RegisterId, src: RegisterId },
    
    /// Shift left: dest = lhs << rhs
    Shl { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },
    
    /// Shift right: dest = lhs >> rhs
    Shr { dest: RegisterId, lhs: RegisterId, rhs: RegisterId },

    // ============================================
    // Controle de fluxo
    // ============================================
    /// Salto incondicional para um bloco
    Jump(BlockId),
    
    /// Salto condicional
    /// Branch cond, then_block, else_block
    Branch { 
        cond: RegisterId, 
        then_block: BlockId, 
        else_block: BlockId 
    },
    
    /// Retorna de uma função
    Return(Option<RegisterId>),
    
    /// Chama uma função
    Call { 
        dest: Option<RegisterId>, 
        func: u32, 
        args: Vec<RegisterId> 
    },
    
    /// Chama uma função via ponteiro
    CallIndirect { 
        dest: Option<RegisterId>, 
        ptr: RegisterId, 
        args: Vec<RegisterId> 
    },

    // ============================================
    // Alocação e memória
    // ============================================
    /// Aloca memória no stack
    /// StackAlloc dest, size, align
    StackAlloc { dest: RegisterId, size: u32, align: u32 },
    
    /// Aloca memória no heap
    /// HeapAlloc dest, size
    HeapAlloc { dest: RegisterId, size: RegisterId },
    
    /// Libera memória do heap
    /// HeapFree ptr
    HeapFree { ptr: RegisterId },

    // ============================================
    // Exceções
    // ============================================
    /// Lança uma exceção
    Throw { value: RegisterId },
    
    /// Inicia bloco try
    TryBegin { catch_block: BlockId, finally_block: Option<BlockId> },
    
    /// Termina bloco try
    TryEnd,

    // ============================================
    // Misc
    // ============================================
    /// Phi node (para SSA)
    Phi { dest: RegisterId, incoming: Vec<(RegisterId, BlockId)> },
    
    /// Nop (não faz nada)
    Nop,
    
    /// Debug break
    DebugBreak,
    
    /// Marcação de posição no código fonte
    DebugLoc { line: u32, column: u32 },
}

/// Bloco básico da IR
#[derive(Debug, Clone)]
pub struct IrBlock {
    /// ID do bloco
    pub id: BlockId,
    
    /// Instruções do bloco
    pub instructions: Vec<IrInstruction>,
    
    /// Terminador do bloco (salto, ret, etc.)
    pub terminator: IrTerminator,
}

impl IrBlock {
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            terminator: IrTerminator::Unreachable,
        }
    }
    
    pub fn add_instruction(&mut self, instr: IrInstruction) {
        self.instructions.push(instr);
    }
    
    pub fn set_terminator(&mut self, terminator: IrTerminator) {
        self.terminator = terminator;
    }
}

/// Terminador de bloco
#[derive(Debug, Clone, PartialEq)]
pub enum IrTerminator {
    /// Salto para outro bloco
    Jump(BlockId),
    
    /// Branch condicional
    Branch { cond: RegisterId, then_block: BlockId, else_block: BlockId },
    
    /// Retorno
    Return(Option<RegisterId>),
    
    /// Inalcançável
    Unreachable,
    
    /// Lança exceção
    Throw(RegisterId),
}
