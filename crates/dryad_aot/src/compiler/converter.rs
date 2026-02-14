// crates/dryad_aot/src/compiler/converter.rs
//! Conversor Bytecode → IR
//!
//! Converte chunks de bytecode Dryad para módulos da IR.

use crate::ir::*;
use dryad_bytecode::{Chunk, OpCode, Value};
use std::collections::HashMap;

/// Conversor de Bytecode para IR
pub struct BytecodeToIrConverter {
    /// Módulo IR sendo construído
    module: IrModule,
    
    /// Função atual sendo convertida
    current_function: Option<IrFunction>,
    
    /// Bloco atual
    current_block: Option<BlockId>,
    
    /// Mapeamento de índice de pilha para registrador
    stack_map: HashMap<usize, RegisterId>,
    
    /// Profundidade atual da pilha
    stack_depth: usize,
    
    /// Próximo offset de variável local
    local_offset: i32,
}

impl BytecodeToIrConverter {
    pub fn new() -> Self {
        Self {
            module: IrModule::new("main"),
            current_function: None,
            current_block: None,
            stack_map: HashMap::new(),
            stack_depth: 0,
            local_offset: 0,
        }
    }
    
    /// Converte um chunk de bytecode para um módulo IR
    pub fn convert(&mut self, chunk: &Chunk) -> Result<IrModule, String> {
        // Criar função main
        let mut func = IrFunction::new("main", IrType::I32);
        let entry_block_id = self.module.new_block_id();
        let entry_block = IrBlock::new(entry_block_id);
        func.entry_block = entry_block_id;
        func.add_block(entry_block);
        
        self.current_function = Some(func);
        self.current_block = Some(entry_block_id);
        
        // Converter cada opcode
        let mut ip = 0;
        while ip < chunk.len() {
            if let Some(op) = chunk.get_op(ip) {
                self.convert_opcode(op, chunk)?;
                ip += 1;
            } else {
                break;
            }
        }
        
        // Adicionar return implícito
        let ret_reg = self.module.new_register();
        self.add_instruction(IrInstruction::LoadConst {
            dest: ret_reg,
            value: IrValue::Constant(IrConstant::I32(0)),
        });
        self.set_terminator(IrTerminator::Return(Some(ret_reg)));
        
        // Adicionar função ao módulo
        if let Some(func) = self.current_function.take() {
            self.module.add_function(func);
        }
        
        Ok(self.module.clone())
    }
    
    /// Converte um único opcode
    fn convert_opcode(&mut self, op: &OpCode, chunk: &Chunk) -> Result<(), String> {
        match op {
            OpCode::Constant(idx) => {
                let value = chunk.get_constant(*idx)
                    .ok_or("Constante inválida")?;
                let ir_value = self.convert_value(value)?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::LoadConst {
                    dest,
                    value: ir_value,
                });
            }
            
            OpCode::Nil => {
                let dest = self.push_register();
                self.add_instruction(IrInstruction::LoadConst {
                    dest,
                    value: IrValue::Constant(IrConstant::Null),
                });
            }
            
            OpCode::True => {
                let dest = self.push_register();
                self.add_instruction(IrInstruction::LoadConst {
                    dest,
                    value: IrValue::Constant(IrConstant::Bool(true)),
                });
            }
            
            OpCode::False => {
                let dest = self.push_register();
                self.add_instruction(IrInstruction::LoadConst {
                    dest,
                    value: IrValue::Constant(IrConstant::Bool(false)),
                });
            }
            
            OpCode::Add => {
                let rhs = self.pop_register()?;
                let lhs = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::Add { dest, lhs, rhs });
            }
            
            OpCode::Subtract => {
                let rhs = self.pop_register()?;
                let lhs = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::Sub { dest, lhs, rhs });
            }
            
            OpCode::Multiply => {
                let rhs = self.pop_register()?;
                let lhs = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::Mul { dest, lhs, rhs });
            }
            
            OpCode::Divide => {
                let rhs = self.pop_register()?;
                let lhs = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::Div { dest, lhs, rhs });
            }
            
            OpCode::Negate => {
                let src = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::Neg { dest, src });
            }
            
            OpCode::Equal => {
                let rhs = self.pop_register()?;
                let lhs = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::CmpEq { dest, lhs, rhs });
            }
            
            OpCode::Greater => {
                let rhs = self.pop_register()?;
                let lhs = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::CmpGt { dest, lhs, rhs });
            }
            
            OpCode::Less => {
                let rhs = self.pop_register()?;
                let lhs = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::CmpLt { dest, lhs, rhs });
            }
            
            OpCode::Not => {
                let src = self.pop_register()?;
                let dest = self.push_register();
                self.add_instruction(IrInstruction::Not { dest, src });
            }
            
            OpCode::Print => {
                let value = self.pop_register()?;
                // TODO: Implementar chamada a função de print do runtime
                // Por enquanto, apenas descarta o valor
            }
            
            OpCode::PrintLn => {
                let value = self.pop_register()?;
                // TODO: Implementar chamada a função de println do runtime
            }
            
            OpCode::Pop => {
                self.pop_register()?;
            }
            
            OpCode::Return => {
                let value = if self.stack_depth > 0 {
                    Some(self.pop_register()?)
                } else {
                    None
                };
                self.set_terminator(IrTerminator::Return(value));
            }
            
            _ => {
                // Outros opcodes ainda não implementados
                return Err(format!("Opcode não suportado: {:?}", op));
            }
        }
        
        Ok(())
    }
    
    /// Converte um valor do bytecode para valor da IR
    fn convert_value(&self, value: &Value) -> Result<IrValue, String> {
        let constant = match value {
            Value::Nil => IrConstant::Null,
            Value::Boolean(b) => IrConstant::Bool(*b),
            Value::Number(n) => IrConstant::F64(*n),
            Value::String(s) => IrConstant::String(s.clone()),
            _ => return Err(format!("Tipo de valor não suportado: {:?}", value)),
        };
        
        Ok(IrValue::Constant(constant))
    }
    
    /// Empilha um novo registrador
    fn push_register(&mut self) -> RegisterId {
        let reg = self.module.new_register();
        self.stack_map.insert(self.stack_depth, reg);
        self.stack_depth += 1;
        reg
    }
    
    /// Desempilha um registrador
    fn pop_register(&mut self) -> Result<RegisterId, String> {
        if self.stack_depth == 0 {
            return Err("Stack underflow".to_string());
        }
        self.stack_depth -= 1;
        self.stack_map.remove(&self.stack_depth)
            .ok_or("Registrador não encontrado na pilha".to_string())
    }
    
    /// Adiciona uma instrução ao bloco atual
    fn add_instruction(&mut self, instr: IrInstruction) {
        if let (Some(func), Some(block_id)) = (&mut self.current_function, self.current_block) {
            if let Some(block) = func.get_block_mut(block_id) {
                block.add_instruction(instr);
            }
        }
    }
    
    /// Define o terminador do bloco atual
    fn set_terminator(&mut self, terminator: IrTerminator) {
        if let (Some(func), Some(block_id)) = (&mut self.current_function, self.current_block) {
            if let Some(block) = func.get_block_mut(block_id) {
                block.set_terminator(terminator);
            }
        }
    }
}

impl Default for BytecodeToIrConverter {
    fn default() -> Self {
        Self::new()
    }
}
