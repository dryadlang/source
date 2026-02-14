// crates/dryad_bytecode/src/value.rs
//! Sistema de tipos dinâmicos da VM Dryad
//!
//! Este módulo implementa os valores que a VM pode manipular.
//! Suporta tipos primitivos e referências a objetos gerenciados.

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// Um valor na máquina virtual
///
/// Este enum representa todos os tipos de valores que podem existir
/// na pilha da VM ou em variáveis.
#[derive(Debug, Clone)]
pub enum Value {
    /// Valor nulo
    Nil,
    /// Valor booleano
    Boolean(bool),
    /// Número de ponto flutuante (f64)
    Number(f64),
    /// String
    String(String),
    /// Referência a um objeto no heap
    Object(HeapId),
    /// Função (função definida pelo usuário)
    Function(Rc<Function>),
    /// Native function (função nativa do runtime)
    NativeFunction(NativeFn),
}

/// Uma função nativa
pub type NativeFn = fn(&[Value]) -> Result<Value, String>;

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            (Value::NativeFunction(a), Value::NativeFunction(b)) => std::ptr::eq(a, b),
            _ => false,
        }
    }
}

/// Identificador único para objetos no heap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapId(pub u64);

/// Um objeto gerenciado no heap
#[derive(Debug, Clone)]
pub enum Object {
    /// Instância de classe
    Instance {
        class_name: String,
        fields: HashMap<String, Value>,
    },
    /// Definição de classe
    Class {
        name: String,
        methods: HashMap<String, Rc<Function>>,
        superclass: Option<HeapId>,
    },
    /// Array dinâmico
    Array(Vec<Value>),
    /// Tupla imutável
    Tuple(Vec<Value>),
    /// Função/Closure
    Closure(Rc<Function>, Vec<Value>), // função + upvalues
    /// HashMap (objeto literal)
    Map(HashMap<String, Value>),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        // Dois objetos são iguais apenas se forem o mesmo objeto (mesmo HeapId)
        // Isso é verificado externamente através dos HeapId
        false
    }
}

/// Representa uma função
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
    pub upvalue_count: usize,
}

/// Chunk de bytecode (importado de chunk.rs)
use crate::chunk::Chunk;

impl Value {
    /// Verifica se o valor é "truthy" (verdadeiro em contextos booleanos)
    ///
    /// Regras:
    /// - Nil é falso
    /// - Boolean: retorna o próprio valor
    /// - Number: 0 é falso, outros são verdadeiros
    /// - String: vazia é falsa, outras são verdadeiras
    /// - Object: sempre verdadeiro
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Object(_) => true,
        }
    }

    /// Retorna true se o valor é nil
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    /// Retorna true se o valor é um número
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Retorna true se o valor é uma string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Tenta obter o valor como número
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Tenta obter o valor como string
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Tenta obter o valor como booleano
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    // ============================================
    // Operações Aritméticas
    // ============================================

    /// Adição: suporta Number + Number e String + String
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(format!(
                "Não é possível adicionar {} com {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Subtração (apenas números)
    pub fn subtract(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(format!(
                "Não é possível subtrair {} de {}",
                other.type_name(),
                self.type_name()
            )),
        }
    }

    /// Multiplicação (apenas números)
    pub fn multiply(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(format!(
                "Não é possível multiplicar {} com {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Divisão (apenas números)
    pub fn divide(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(_), Value::Number(b)) if *b == 0.0 => {
                Err("Divisão por zero".to_string())
            }
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err(format!(
                "Não é possível dividir {} por {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Módulo (apenas números)
    pub fn modulo(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            _ => Err(format!(
                "Não é possível calcular módulo de {} com {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Negação unária (apenas números)
    pub fn negate(&self) -> Result<Value, String> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(format!(
                "Não é possível negar {}",
                self.type_name()
            )),
        }
    }

    // ============================================
    // Operações de Comparação
    // ============================================

    /// Maior que (apenas números)
    pub fn greater(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
            _ => Err(format!(
                "Não é possível comparar {} com {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Menor que (apenas números)
    pub fn less(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
            _ => Err(format!(
                "Não é possível comparar {} com {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Maior ou igual (apenas números)
    pub fn greater_equal(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            _ => Err(format!(
                "Não é possível comparar {} com {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Menor ou igual (apenas números)
    pub fn less_equal(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            _ => Err(format!(
                "Não é possível comparar {} com {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    // ============================================
    // Operações Bitwise
    // ============================================

    /// Converte para i64 para operações bitwise
    fn as_i64(&self) -> Option<i64> {
        self.as_number().map(|n| n as i64)
    }

    /// AND bitwise
    pub fn bit_and(&self, other: &Value) -> Result<Value, String> {
        match (self.as_i64(), other.as_i64()) {
            (Some(a), Some(b)) => Ok(Value::Number((a & b) as f64)),
            _ => Err(format!(
                "Operação bitwise AND requer números inteiros"
            )),
        }
    }

    /// OR bitwise
    pub fn bit_or(&self, other: &Value) -> Result<Value, String> {
        match (self.as_i64(), other.as_i64()) {
            (Some(a), Some(b)) => Ok(Value::Number((a | b) as f64)),
            _ => Err(format!(
                "Operação bitwise OR requer números inteiros"
            )),
        }
    }

    /// XOR bitwise
    pub fn bit_xor(&self, other: &Value) -> Result<Value, String> {
        match (self.as_i64(), other.as_i64()) {
            (Some(a), Some(b)) => Ok(Value::Number((a ^ b) as f64)),
            _ => Err(format!(
                "Operação bitwise XOR requer números inteiros"
            )),
        }
    }

    /// NOT bitwise
    pub fn bit_not(&self) -> Result<Value, String> {
        match self.as_i64() {
            Some(a) => Ok(Value::Number((!a) as f64)),
            _ => Err(format!(
                "Operação bitwise NOT requer número inteiro"
            )),
        }
    }

    /// Shift left
    pub fn shift_left(&self, other: &Value) -> Result<Value, String> {
        match (self.as_i64(), other.as_i64()) {
            (Some(a), Some(b)) => Ok(Value::Number((a << b) as f64)),
            _ => Err(format!(
                "Operação shift left requer números inteiros"
            )),
        }
    }

    /// Shift right
    pub fn shift_right(&self, other: &Value) -> Result<Value, String> {
        match (self.as_i64(), other.as_i64()) {
            (Some(a), Some(b)) => Ok(Value::Number((a >> b) as f64)),
            _ => Err(format!(
                "Operação shift right requer números inteiros"
            )),
        }
    }

    // ============================================
    // Utilitários
    // ============================================

    /// Retorna o nome do tipo
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Boolean(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Object(_) => "object",
            Value::Function(_) => "function",
            Value::NativeFunction(_) => "native function",
        }
    }

    /// Converte o valor para string (para debug/impressão)
    pub fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => {
                // Formatação especial para números inteiros
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Object(_) => "[object]".to_string(),
            Value::Function(f) => format!("<fn {}>", f.name),
            Value::NativeFunction(_) => "<native fn>".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Gerenciador de heap para objetos
pub struct Heap {
    objects: HashMap<HeapId, Rc<RefCell<Object>>>,
    next_id: u64,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 1,
        }
    }

    /// Aloca um novo objeto no heap
    pub fn allocate(&mut self, object: Object) -> HeapId {
        let id = HeapId(self.next_id);
        self.next_id += 1;
        self.objects.insert(id, Rc::new(RefCell::new(object)));
        id
    }

    /// Obtém uma referência a um objeto
    pub fn get(&self, id: HeapId) -> Option<Rc<RefCell<Object>>> {
        self.objects.get(&id).cloned()
    }

    /// Libera um objeto do heap
    pub fn deallocate(&mut self, id: HeapId) {
        self.objects.remove(&id);
    }

    /// Retorna o número de objetos no heap
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
