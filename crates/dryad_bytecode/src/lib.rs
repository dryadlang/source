// crates/dryad_bytecode/src/lib.rs
//! # Dryad Bytecode VM
//!
//! Máquina Virtual baseada em bytecode para a linguagem de programação Dryad.
//!
//! ## Estrutura
//!
//! - `opcode` - Definição dos opcodes da VM
//! - `value` - Sistema de tipos dinâmicos e heap
//! - `chunk` - Armazenamento de bytecode
//! - `vm` - Máquina Virtual principal
//! - `compiler` - Compilador AST -> Bytecode
//! - `debug` - Disassembler e utilitários de debug
//!
//! ## Exemplo de Uso
//!
//! ```rust,no_run
//! use dryad_bytecode::{Compiler, VM};
//! use dryad_parser::Parser;
//!
//! // Parse do código fonte
//! let source = "print 1 + 2;";
//! let program = Parser::parse(source).unwrap();
//!
//! // Compila para bytecode
//! let mut compiler = Compiler::new();
//! let chunk = compiler.compile(program).unwrap();
//!
//! // Executa na VM
//! let mut vm = VM::new();
//! vm.interpret(chunk);
//! ```

// Módulos internos
mod chunk;
mod compiler;
mod debug;
mod opcode;
mod value;
mod vm;

// Re-exportações públicas
pub use chunk::{Chunk, ChunkBuilder};
pub use compiler::Compiler;
pub use debug::{DebugChunk, Disassembler};
pub use opcode::{OpCode, OpCodeCategory};
pub use value::{Function, Heap, HeapId, NativeFn, Object, Value};
pub use vm::{InterpretResult, VM};

/// Versão da crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Verifica se o bytecode está habilitado
pub fn is_enabled() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use dryad_errors::SourceLocation;
    use dryad_parser::ast::{Expr, Literal, Program, Stmt};

    fn dummy_loc() -> SourceLocation {
        SourceLocation {
            line: 1,
            column: 1,
            file: None,
        }
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_basic_compilation() {
        let program = Program {
            statements: vec![Stmt::Expression(
                Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                    "+".to_string(),
                    Box::new(Expr::Literal(Literal::Number(2.0), dummy_loc())),
                    dummy_loc(),
                ),
                dummy_loc(),
            )],
        };

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(program);
        assert!(chunk.is_ok());
    }

    #[test]
    fn test_vm_execution() {
        let mut chunk = Chunk::new("test");
        chunk.push_op(OpCode::Constant(0), 1);
        chunk.push_op(OpCode::Return, 1);

        let mut vm = VM::new();
        let result = vm.interpret(chunk);
        assert_eq!(result, InterpretResult::Ok);
    }
}
