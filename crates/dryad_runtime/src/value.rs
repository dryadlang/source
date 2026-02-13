use dryad_parser::ast::{Stmt, Expr, Visibility};
use std::collections::HashMap;
use crate::heap::HeapId;

#[derive(Debug, Clone)]
pub enum FlowControl {
    Return(Value),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Array(HeapId),
    Tuple(HeapId),
    Exception(String),
    Function {
        name: String,
        params: Vec<String>,
        body: Stmt,
    },
    AsyncFunction {
        name: String,
        params: Vec<String>,
        body: Stmt,
    },
    ThreadFunction {
        name: String,
        params: Vec<String>,
        body: Stmt,
    },
    Lambda(HeapId),
    Thread {
        id: u64,
        is_running: bool,
    },
    Mutex {
        id: u64,
        locked: bool,
    },
    Promise {
        id: u64,
        resolved: bool,
        value: Option<Box<Value>>,
    },
    Class(HeapId),
    Instance(HeapId),
    Object(HeapId),
}

#[derive(Debug, Clone)]
pub struct ObjectMethod {
    pub params: Vec<String>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct ClassMethod {
    pub visibility: Visibility,
    pub is_static: bool,
    pub params: Vec<String>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct ClassProperty {
    pub visibility: Visibility,
    pub is_static: bool,
    pub default_value: Option<Value>,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Bool(b) => format!("{}", b),
            Value::Null => "null".to_string(),
            Value::Array(id) => format!("[Array (heap:{})]", id),
            Value::Tuple(id) => format!("(Tuple (heap:{}))", id),
            Value::Exception(msg) => format!("Exception: {}", msg),
            Value::Function { name, .. } => format!("function {}", name),
            Value::AsyncFunction { name, .. } => format!("async function {}", name),
            Value::ThreadFunction { name, .. } => format!("thread function {}", name),
            Value::Lambda(id) => format!("(Lambda (heap:{}))", id),
            Value::Thread { id, is_running } => {
                format!("Thread(id: {}, running: {})", id, is_running)
            }
            Value::Mutex { id, locked } => {
                format!("Mutex(id: {}, locked: {})", id, locked)
            }
            Value::Promise { id, resolved, .. } => {
                format!("Promise(id: {}, resolved: {})", id, resolved)
            }
            Value::Class(id) => format!("class (heap:{})", id),
            Value::Instance(id) => format!("instance (heap:{})", id),
            Value::Object(id) => format!("object (heap:{})", id),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(_) | Value::Tuple(_) | Value::Lambda(_) | 
            Value::Class(_) | Value::Instance(_) | Value::Object(_) => true,
            Value::Exception(_) => false,
            Value::Function { .. } | Value::AsyncFunction { .. } | Value::ThreadFunction { .. } => true,
            Value::Thread { is_running, .. } => *is_running,
            Value::Mutex { .. } => true,
            Value::Promise { resolved, .. } => *resolved,
        }
    }
}
