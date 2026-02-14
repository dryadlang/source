// crates/dryad_bytecode/src/chunk.rs
//! Armazenamento de bytecode em chunks
//!
//! Este módulo implementa a estrutura Chunk que armazena bytecode,
//! constantes e informações de debug (números de linha).

use crate::opcode::OpCode;
use crate::value::Value;

/// Um chunk de bytecode
///
/// Representa uma unidade de código compilado, contendo:
/// - Código (vetor de opcodes)
/// - Constantes (tabela de valores)
/// - Linhas (mapeamento opcode -> linha no código fonte)
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Vetor de opcodes
    pub code: Vec<OpCode>,
    /// Tabela de constantes
    pub constants: Vec<Value>,
    /// Mapeamento de índice de opcode para linha no código fonte
    pub lines: Vec<usize>,
    /// Nome do chunk (para debug)
    pub name: String,
}

impl Chunk {
    /// Cria um novo chunk vazio
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            name: name.into(),
        }
    }

    /// Cria um chunk vazio sem nome
    pub fn empty() -> Self {
        Self::new("<anonymous>")
    }

    /// Adiciona um opcode ao chunk
    ///
    /// # Arguments
    /// * `op` - O opcode a ser adicionado
    /// * `line` - O número da linha no código fonte
    pub fn push_op(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }

    /// Adiciona uma constante e retorna seu índice (8 bits)
    ///
    /// Se a tabela de constantes estiver cheia (mais de 256 constantes),
    /// retornará um erro.
    pub fn add_constant(&mut self, value: Value) -> Result<u8, String> {
        let idx = self.constants.len();
        if idx > u8::MAX as usize {
            Err("Tabela de constantes cheia (máximo 256)".to_string())
        } else {
            self.constants.push(value);
            Ok(idx as u8)
        }
    }

    /// Adiciona uma constante e retorna seu índice (16 bits)
    ///
    /// Usado quando a tabela tem mais de 256 constantes.
    pub fn add_constant_long(&mut self, value: Value) -> Result<u16, String> {
        let idx = self.constants.len();
        if idx > u16::MAX as usize {
            Err("Tabela de constantes cheia (máximo 65536)".to_string())
        } else {
            self.constants.push(value);
            Ok(idx as u16)
        }
    }

    /// Retorna o número de opcodes no chunk
    pub fn len(&self) -> usize {
        self.code.len()
    }

    /// Verifica se o chunk está vazio
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    /// Retorna o opcode na posição especificada
    pub fn get_op(&self, index: usize) -> Option<&OpCode> {
        self.code.get(index)
    }

    /// Retorna o número da linha para o opcode na posição especificada
    pub fn get_line(&self, index: usize) -> Option<usize> {
        self.lines.get(index).copied()
    }

    /// Retorna uma constante pelo índice
    pub fn get_constant(&self, index: u8) -> Option<&Value> {
        self.constants.get(index as usize)
    }

    /// Retorna uma constante pelo índice longo (16 bits)
    pub fn get_constant_long(&self, index: u16) -> Option<&Value> {
        self.constants.get(index as usize)
    }

    /// Retorna o número de constantes
    pub fn constant_count(&self) -> usize {
        self.constants.len()
    }

    /// Limpa o chunk, removendo todos os opcodes e constantes
    pub fn clear(&mut self) {
        self.code.clear();
        self.constants.clear();
        self.lines.clear();
    }

    /// Retorna a capacidade atual do vetor de código
    pub fn capacity(&self) -> usize {
        self.code.capacity()
    }

    /// Reserva espaço para opcodes adicionais
    pub fn reserve(&mut self, additional: usize) {
        self.code.reserve(additional);
        self.lines.reserve(additional);
    }

    /// Verifica se há espaço para mais constantes (8 bits)
    pub fn has_constant_space(&self) -> bool {
        self.constants.len() <= u8::MAX as usize
    }

    /// Retorna o índice atual (próximo índice disponível)
    pub fn current_index(&self) -> usize {
        self.code.len()
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::empty()
    }
}

/// Builder para facilitar a construção de chunks
pub struct ChunkBuilder {
    chunk: Chunk,
}

impl ChunkBuilder {
    /// Cria um novo builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            chunk: Chunk::new(name),
        }
    }

    /// Adiciona um opcode
    pub fn op(mut self, opcode: OpCode, line: usize) -> Self {
        self.chunk.push_op(opcode, line);
        self
    }

    /// Adiciona uma constante e retorna seu índice
    pub fn constant(mut self, value: Value) -> Result<(Self, u8), String> {
        let idx = self.chunk.add_constant(value)?;
        Ok((self, idx))
    }

    /// Finaliza e retorna o chunk
    pub fn build(self) -> Chunk {
        self.chunk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_chunk() {
        let chunk = Chunk::empty();
        assert!(chunk.is_empty());
        assert_eq!(chunk.len(), 0);
    }

    #[test]
    fn test_push_op() {
        let mut chunk = Chunk::empty();
        chunk.push_op(OpCode::Add, 1);
        chunk.push_op(OpCode::Subtract, 2);

        assert_eq!(chunk.len(), 2);
        assert_eq!(chunk.get_line(0), Some(1));
        assert_eq!(chunk.get_line(1), Some(2));
    }

    #[test]
    fn test_add_constant() {
        let mut chunk = Chunk::empty();
        let idx1 = chunk.add_constant(Value::Number(1.0)).unwrap();
        let idx2 = chunk.add_constant(Value::String("hello".to_string())).unwrap();

        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(chunk.get_constant(0), Some(&Value::Number(1.0)));
        assert_eq!(chunk.get_constant(1), Some(&Value::String("hello".to_string())));
    }

    #[test]
    fn test_chunk_builder() {
        let chunk = ChunkBuilder::new("test")
            .op(OpCode::Add, 1)
            .op(OpCode::Subtract, 1)
            .build();

        assert_eq!(chunk.len(), 2);
        assert_eq!(chunk.name, "test");
    }
}
