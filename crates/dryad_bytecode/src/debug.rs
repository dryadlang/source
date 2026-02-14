// crates/dryad_bytecode/src/debug.rs
//! Utilitários de debug para bytecode
//!
//! Este módulo implementa um disassembler para visualizar bytecode
//! de forma legível, útil para debug e desenvolvimento.

use crate::chunk::Chunk;
use crate::opcode::OpCode;

/// Disassembler para chunks de bytecode
pub struct Disassembler;

impl Disassembler {
    /// Disassembla um chunk completo
    pub fn disassemble(chunk: &Chunk, name: &str) {
        println!("== {} ==", name);
        println!("Constants:");
        for (i, constant) in chunk.constants.iter().enumerate() {
            println!("  [{:4}] {:?}", i, constant);
        }
        println!();
        println!("Bytecode:");

        let mut offset = 0;
        while offset < chunk.len() {
            offset = Self::disassemble_instruction(chunk, offset);
        }
    }

    /// Disassembla uma única instrução
    pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        print!("{:04} ", offset);

        // Mostra linha ou |
        if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset.saturating_sub(1)) {
            print!("   | ");
        } else {
            print!("{:4} ", chunk.get_line(offset).unwrap_or(0));
        }

        if let Some(op) = chunk.get_op(offset) {
            Self::print_instruction(chunk, op, offset)
        } else {
            println!("<invalid>");
            offset + 1
        }
    }

    fn print_instruction(chunk: &Chunk, op: &OpCode, offset: usize) -> usize {
        match op {
            // Constantes
            OpCode::Constant(idx) => {
                Self::print_constant_instruction("CONSTANT", chunk, *idx as u16, offset)
            }
            OpCode::ConstantLong(idx) => {
                Self::print_constant_instruction("CONSTANT_LONG", chunk, *idx, offset)
            }
            OpCode::Nil => Self::print_simple_instruction("NIL", offset),
            OpCode::True => Self::print_simple_instruction("TRUE", offset),
            OpCode::False => Self::print_simple_instruction("FALSE", offset),

            // Aritmética
            OpCode::Add => Self::print_simple_instruction("ADD", offset),
            OpCode::Subtract => Self::print_simple_instruction("SUBTRACT", offset),
            OpCode::Multiply => Self::print_simple_instruction("MULTIPLY", offset),
            OpCode::Divide => Self::print_simple_instruction("DIVIDE", offset),
            OpCode::Modulo => Self::print_simple_instruction("MODULO", offset),
            OpCode::Negate => Self::print_simple_instruction("NEGATE", offset),

            // Comparações
            OpCode::Equal => Self::print_simple_instruction("EQUAL", offset),
            OpCode::Greater => Self::print_simple_instruction("GREATER", offset),
            OpCode::Less => Self::print_simple_instruction("LESS", offset),
            OpCode::GreaterEqual => Self::print_simple_instruction("GREATER_EQUAL", offset),
            OpCode::LessEqual => Self::print_simple_instruction("LESS_EQUAL", offset),

            // Lógicas
            OpCode::Not => Self::print_simple_instruction("NOT", offset),
            OpCode::And => Self::print_simple_instruction("AND", offset),
            OpCode::Or => Self::print_simple_instruction("OR", offset),

            // Bitwise
            OpCode::BitAnd => Self::print_simple_instruction("BIT_AND", offset),
            OpCode::BitOr => Self::print_simple_instruction("BIT_OR", offset),
            OpCode::BitXor => Self::print_simple_instruction("BIT_XOR", offset),
            OpCode::BitNot => Self::print_simple_instruction("BIT_NOT", offset),
            OpCode::ShiftLeft => Self::print_simple_instruction("SHIFT_LEFT", offset),
            OpCode::ShiftRight => Self::print_simple_instruction("SHIFT_RIGHT", offset),

            // Variáveis
            OpCode::DefineGlobal(idx) => {
                Self::print_byte_instruction("DEFINE_GLOBAL", *idx, offset)
            }
            OpCode::GetGlobal(idx) => Self::print_byte_instruction("GET_GLOBAL", *idx, offset),
            OpCode::SetGlobal(idx) => Self::print_byte_instruction("SET_GLOBAL", *idx, offset),
            OpCode::GetLocal(idx) => Self::print_byte_instruction("GET_LOCAL", *idx, offset),
            OpCode::SetLocal(idx) => Self::print_byte_instruction("SET_LOCAL", *idx, offset),

            // Controle de fluxo
            OpCode::Jump(offset_val) => Self::print_jump_instruction("JUMP", *offset_val, offset),
            OpCode::JumpIfFalse(offset_val) => {
                Self::print_jump_instruction("JUMP_IF_FALSE", *offset_val, offset)
            }
            OpCode::JumpIfTrue(offset_val) => {
                Self::print_jump_instruction("JUMP_IF_TRUE", *offset_val, offset)
            }
            OpCode::Loop(offset_val) => {
                Self::print_loop_instruction("LOOP", *offset_val, offset)
            }
            OpCode::Break => Self::print_simple_instruction("BREAK", offset),
            OpCode::Continue => Self::print_simple_instruction("CONTINUE", offset),

            // Funções
            OpCode::Call(arg_count) => {
                Self::print_byte_instruction("CALL", *arg_count, offset)
            }
            OpCode::Return => Self::print_simple_instruction("RETURN", offset),
            OpCode::Closure(idx) => {
                Self::print_byte_instruction("CLOSURE", *idx, offset)
            }
            OpCode::GetUpvalue(idx) => {
                Self::print_byte_instruction("GET_UPVALUE", *idx, offset)
            }
            OpCode::SetUpvalue(idx) => {
                Self::print_byte_instruction("SET_UPVALUE", *idx, offset)
            }
            OpCode::CloseUpvalue => Self::print_simple_instruction("CLOSE_UPVALUE", offset),

            // Objetos
            OpCode::Class(idx) => Self::print_byte_instruction("CLASS", *idx, offset),
            OpCode::Method(idx) => Self::print_byte_instruction("METHOD", *idx, offset),
            OpCode::Invoke(arg_count) => {
                Self::print_byte_instruction("INVOKE", *arg_count, offset)
            }
            OpCode::GetProperty(idx) => {
                Self::print_byte_instruction("GET_PROPERTY", *idx, offset)
            }
            OpCode::SetProperty(idx) => {
                Self::print_byte_instruction("SET_PROPERTY", *idx, offset)
            }
            OpCode::This => Self::print_simple_instruction("THIS", offset),
            OpCode::Super(idx) => Self::print_byte_instruction("SUPER", *idx, offset),

            // Coleções
            OpCode::Array(count) => Self::print_short_instruction("ARRAY", *count, offset),
            OpCode::Index => Self::print_simple_instruction("INDEX", offset),
            OpCode::SetIndex => Self::print_simple_instruction("SET_INDEX", offset),
            OpCode::Tuple(count) => Self::print_byte_instruction("TUPLE", *count, offset),
            OpCode::TupleAccess(idx) => {
                Self::print_byte_instruction("TUPLE_ACCESS", *idx, offset)
            }

            // Pilha
            OpCode::Pop => Self::print_simple_instruction("POP", offset),
            OpCode::PopN(count) => Self::print_byte_instruction("POP_N", *count, offset),
            OpCode::Dup => Self::print_simple_instruction("DUP", offset),
            OpCode::DupN(n) => Self::print_byte_instruction("DUP_N", *n, offset),
            OpCode::Swap => Self::print_simple_instruction("SWAP", offset),

            // I/O
            OpCode::Print => Self::print_simple_instruction("PRINT", offset),
            OpCode::PrintLn => Self::print_simple_instruction("PRINT_LN", offset),
            OpCode::Nop => Self::print_simple_instruction("NOP", offset),
            OpCode::Halt => Self::print_simple_instruction("HALT", offset),
        }
    }

    fn print_simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn print_byte_instruction(name: &str, byte: u8, offset: usize) -> usize {
        println!("{:16} {:4}", name, byte);
        offset + 2
    }

    fn print_short_instruction(name: &str, short: u16, offset: usize) -> usize {
        println!("{:16} {:6}", name, short);
        offset + 3
    }

    fn print_constant_instruction(
        name: &str,
        chunk: &Chunk,
        idx: u16,
        offset: usize,
    ) -> usize {
        let constant = chunk.get_constant_long(idx);
        print!("{:16} {:4} '", name, idx);
        if let Some(val) = constant {
            print!("{}", val);
        } else {
            print!("<invalid>");
        }
        println!("'");
        offset + if idx <= u8::MAX as u16 { 2 } else { 3 }
    }

    fn print_jump_instruction(name: &str, jump_offset: u16, offset: usize) -> usize {
        let target = offset + 3 + jump_offset as usize;
        println!("{:16} {:4} -> {}", name, jump_offset, target);
        offset + 3
    }

    fn print_loop_instruction(name: &str, loop_offset: u16, offset: usize) -> usize {
        let target = offset + 3 - loop_offset as usize;
        println!("{:16} {:4} -> {}", name, loop_offset, target);
        offset + 3
    }
}

/// Trait para facilitar o debug de chunks
pub trait DebugChunk {
    /// Disassembla o chunk
    fn disassemble(&self, name: &str);
    /// Disassembla uma instrução específica
    fn disassemble_instruction(&self, offset: usize) -> usize;
}

impl DebugChunk for Chunk {
    fn disassemble(&self, name: &str) {
        Disassembler::disassemble(self, name);
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        Disassembler::disassemble_instruction(self, offset)
    }
}
