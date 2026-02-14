// crates/dryad_bytecode/src/vm.rs
//! Máquina Virtual baseada em pilha para Dryad
//!
//! Esta VM executa bytecode de forma eficiente usando uma pilha de valores.
//! Suporta chamadas de função, closures, classes e coleta de lixo básica.

use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::value::{Function, Heap, HeapId, NativeFn, Object, Value};
use std::collections::HashMap;
use std::rc::Rc;

/// Resultado da interpretação
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpretResult {
    /// Execução bem-sucedida
    Ok,
    /// Erro de compilação
    CompileError,
    /// Erro em tempo de execução
    RuntimeError,
}

/// Frame de chamada para funções
#[derive(Debug)]
struct CallFrame {
    /// Função sendo executada
    function: Chunk,
    /// Instruction pointer (índice do próximo opcode)
    ip: usize,
    /// Posição base na pilha para este frame
    stack_start: usize,
}

/// Frame para tratamento de exceções (try/catch)
#[derive(Debug)]
struct TryFrame {
    /// Posição do catch handler
    catch_ip: usize,
    /// Posição do finally handler (se houver)
    finally_ip: Option<usize>,
    /// Posição inicial na pilha quando try começou
    stack_start: usize,
    /// Profundidade de frames quando try começou
    frame_depth: usize,
}

impl CallFrame {
    fn new(function: Chunk, stack_start: usize) -> Self {
        Self {
            function,
            ip: 0,
            stack_start,
        }
    }

    /// Retorna o próximo opcode e avança o IP
    fn read_op(&mut self) -> Option<&OpCode> {
        let op = self.function.get_op(self.ip);
        self.ip += 1;
        op
    }

    /// Retorna o opcode atual sem avançar
    fn peek_op(&self) -> Option<&OpCode> {
        self.function.get_op(self.ip)
    }

    /// Retorna a linha do opcode atual
    fn current_line(&self) -> Option<usize> {
        self.function.get_line(self.ip.saturating_sub(1))
    }

    /// Salta para um offset relativo
    fn jump(&mut self, offset: u16) {
        self.ip += offset as usize;
    }

    /// Salta para trás (para loops)
    fn loop_back(&mut self, offset: u16) {
        self.ip -= offset as usize;
    }
}

/// Máquina Virtual baseada em pilha
pub struct VM {
    /// Pilha de valores
    stack: Vec<Value>,
    /// Frames de chamada
    frames: Vec<CallFrame>,
    /// Variáveis globais
    globals: HashMap<String, Value>,
    /// Heap para objetos
    heap: Heap,
    /// Flag de debug
    debug_mode: bool,
    /// Limite máximo de recursão
    max_frames: usize,
    /// Frames de try/catch
    try_frames: Vec<TryFrame>,
}

impl VM {
    /// Cria uma nova VM
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(256),
            frames: Vec::new(),
            globals: HashMap::new(),
            heap: Heap::new(),
            debug_mode: false,
            max_frames: 1000,
            try_frames: Vec::new(),
        }
    }

    /// Define o modo de debug
    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug_mode = debug;
    }

    /// Define o limite máximo de frames
    pub fn set_max_frames(&mut self, max: usize) {
        self.max_frames = max;
    }

    /// Interpreta um chunk de bytecode
    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.reset();
        self.frames.push(CallFrame::new(chunk, 0));

        match self.run() {
            Ok(_) => InterpretResult::Ok,
            Err(err) => {
                self.runtime_error(&err);
                InterpretResult::RuntimeError
            }
        }
    }

    /// Reseta o estado da VM
    fn reset(&mut self) {
        self.stack.clear();
        self.frames.clear();
    }

    /// Loop principal de execução
    fn run(&mut self) -> Result<(), String> {
        while let Some(frame) = self.frames.last_mut() {
            // Debug: mostra estado atual
            if self.debug_mode {
                self.debug_stack();
                if let Some(op) = frame.peek_op() {
                    self.debug_instruction(frame, op);
                }
            }

            // Executa o próximo opcode
            if let Some(op) = frame.read_op() {
                match self.execute_op(op, frame)? {
                    ExecutionControl::Continue => {}
                    ExecutionControl::Return => {
                        // Retorna da função atual
                        self.frames.pop();
                    }
                    ExecutionControl::Break => {
                        // TODO: Implementar break
                        return Err("Break não implementado".to_string());
                    }
                    ExecutionControl::ContinueLoop => {
                        // TODO: Implementar continue
                        return Err("Continue não implementado".to_string());
                    }
                }
            } else {
                // Fim do chunk
                break;
            }
        }

        Ok(())
    }

    /// Executa um único opcode
    fn execute_op(&mut self, op: &OpCode, frame: &mut CallFrame) -> Result<ExecutionControl, String> {
        match op {
            // ============================================
            // Constantes
            // ============================================
            OpCode::Constant(idx) => {
                let value = frame
                    .function
                    .get_constant(*idx)
                    .ok_or("Índice de constante inválido")?
                    .clone();
                self.push(value);
            }

            OpCode::ConstantLong(idx) => {
                let value = frame
                    .function
                    .get_constant_long(*idx)
                    .ok_or("Índice de constante longo inválido")?
                    .clone();
                self.push(value);
            }

            OpCode::Nil => self.push(Value::Nil),
            OpCode::True => self.push(Value::Boolean(true)),
            OpCode::False => self.push(Value::Boolean(false)),

            // ============================================
            // Aritmética
            // ============================================
            OpCode::Add => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.add(&b)?);
            }

            OpCode::Subtract => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.subtract(&b)?);
            }

            OpCode::Multiply => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.multiply(&b)?);
            }

            OpCode::Divide => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.divide(&b)?);
            }

            OpCode::Modulo => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.modulo(&b)?);
            }

            OpCode::Negate => {
                let a = self.pop()?;
                self.push(a.negate()?);
            }

            // ============================================
            // Comparações
            // ============================================
            OpCode::Equal => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a == b));
            }

            OpCode::Greater => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.greater(&b)?);
            }

            OpCode::Less => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.less(&b)?);
            }

            OpCode::GreaterEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.greater_equal(&b)?);
            }

            OpCode::LessEqual => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.less_equal(&b)?);
            }

            // ============================================
            // Lógicas
            // ============================================
            OpCode::Not => {
                let a = self.pop()?;
                self.push(Value::Boolean(!a.is_truthy()));
            }

            OpCode::And => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a.is_truthy() && b.is_truthy()));
            }

            OpCode::Or => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(Value::Boolean(a.is_truthy() || b.is_truthy()));
            }

            // ============================================
            // Bitwise
            // ============================================
            OpCode::BitAnd => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bit_and(&b)?);
            }

            OpCode::BitOr => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bit_or(&b)?);
            }

            OpCode::BitXor => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.bit_xor(&b)?);
            }

            OpCode::BitNot => {
                let a = self.pop()?;
                self.push(a.bit_not()?);
            }

            OpCode::ShiftLeft => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.shift_left(&b)?);
            }

            OpCode::ShiftRight => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(a.shift_right(&b)?);
            }

            // ============================================
            // Variáveis
            // ============================================
            OpCode::DefineGlobal(idx) => {
                let name = frame
                    .function
                    .get_constant(*idx)
                    .ok_or("Índice de constante inválido")?
                    .to_string();
                let value = self.pop()?;
                self.globals.insert(name, value);
            }

            OpCode::GetGlobal(idx) => {
                let name = frame
                    .function
                    .get_constant(*idx)
                    .ok_or("Índice de constante inválido")?
                    .to_string();
                let value = self
                    .globals
                    .get(&name)
                    .ok_or_else(|| format!("Variável '{}' não definida", name))?
                    .clone();
                self.push(value);
            }

            OpCode::SetGlobal(idx) => {
                let name = frame
                    .function
                    .get_constant(*idx)
                    .ok_or("Índice de constante inválido")?
                    .to_string();
                let value = self.peek(0)?.clone();

                if self.globals.contains_key(&name) {
                    self.globals.insert(name, value);
                } else {
                    return Err(format!("Variável '{}' não definida", name));
                }
            }

            OpCode::GetLocal(idx) => {
                let idx = *idx as usize;
                let value = self.stack[frame.stack_start + idx].clone();
                self.push(value);
            }

            OpCode::SetLocal(idx) => {
                let idx = *idx as usize;
                let value = self.peek(0)?.clone();
                self.stack[frame.stack_start + idx] = value;
            }

            // ============================================
            // Controle de Fluxo
            // ============================================
            OpCode::Jump(offset) => {
                frame.jump(*offset);
            }

            OpCode::JumpIfFalse(offset) => {
                if !self.peek(0)?.is_truthy() {
                    frame.jump(*offset);
                }
            }

            OpCode::JumpIfTrue(offset) => {
                if self.peek(0)?.is_truthy() {
                    frame.jump(*offset);
                }
            }

            OpCode::Loop(offset) => {
                frame.loop_back(*offset);
            }

            OpCode::Break => {
                return Ok(ExecutionControl::Break);
            }

            OpCode::Continue => {
                return Ok(ExecutionControl::ContinueLoop);
            }

            // ============================================
            // Funções
            // ============================================
            OpCode::Call(arg_count) => {
                let callee = self.peek(*arg_count as usize)?;
                
                match callee {
                    Value::Function(function) => {
                        self.call_function(Rc::clone(function), *arg_count)?;
                    }
                    Value::NativeFunction(native_fn) => {
                        self.call_native(*native_fn, *arg_count)?;
                    }
                    _ => {
                        return Err(format!("Não é possível chamar '{}'", callee.type_name()));
                    }
                }
            }

            OpCode::Return => {
                // Pega o valor de retorno
                let result = self.pop()?;
                
                // Remove argumentos e função da pilha
                let frame = self.frames.pop().ok_or("Não há frame para retornar")?;
                while self.stack.len() > frame.stack_start {
                    self.stack.pop();
                }
                
                // Empilha o resultado
                self.push(result);
                
                return Ok(ExecutionControl::Return);
            }

            OpCode::Closure(_idx) => {
                // TODO: Implementar criação de closure
                return Err("Closure não implementado".to_string());
            }

            // ============================================
            // Objetos
            // ============================================
            OpCode::Class(idx) => {
                let class_name = match self.read_constant(*idx) {
                    Some(Value::String(name)) => name.clone(),
                    _ => return Err("Nome da classe inválido".to_string()),
                };

                // Cria a classe no heap
                let class_id = self.heap.allocate(Object::Class {
                    name: class_name,
                    methods: std::collections::HashMap::new(),
                    superclass: None,
                });

                self.push(Value::Object(class_id));
            }

            OpCode::Method(idx) => {
                let method_name = match self.read_constant(*idx) {
                    Some(Value::String(name)) => name.clone(),
                    _ => return Err("Nome do método inválido".to_string()),
                };

                // Pega a função do método (topo da pilha)
                let method = self.pop()?;
                
                // Pega a classe (abaixo da função)
                let class_value = self.peek(0)?;
                
                if let Value::Object(class_id) = class_value {
                    if let Some(obj) = self.heap.get(*class_id) {
                        let mut obj_ref = obj.borrow_mut();
                        if let Object::Class { methods, .. } = &mut *obj_ref {
                            if let Value::Function(func) = method {
                                methods.insert(method_name, std::rc::Rc::clone(&func));
                            }
                        }
                    }
                }
            }

            OpCode::Invoke(arg_count) => {
                // Pega o método da instância
                let method_name_value = self.pop()?;
                let method_name = match method_name_value {
                    Value::String(name) => name,
                    _ => return Err("Nome do método inválido".to_string()),
                };

                // Pega a instância
                let instance = self.peek(*arg_count as usize)?;
                
                if let Value::Object(instance_id) = instance {
                    if let Some(obj) = self.heap.get(*instance_id) {
                        let obj_ref = obj.borrow();
                        if let Object::Instance { class_name, fields: _ } = &*obj_ref {
                            // Busca o método na classe
                            if let Some(class_value) = self.globals.get(class_name) {
                                if let Value::Object(class_id) = class_value {
                                    if let Some(class_obj) = self.heap.get(*class_id) {
                                        let class_ref = class_obj.borrow();
                                        if let Object::Class { methods, .. } = &*class_ref {
                                            if let Some(method) = methods.get(&method_name) {
                                                // Chama o método
                                                drop(class_ref);
                                                drop(obj_ref);
                                                self.call_function(std::rc::Rc::clone(method), *arg_count)?;
                                            } else {
                                                return Err(format!("Método '{}' não encontrado na classe '{}'", method_name, class_name));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    return Err("Apenas instâncias podem ter métodos".to_string());
                }
            }

            OpCode::GetProperty(idx) => {
                let prop_name = match self.read_constant(*idx) {
                    Some(Value::String(name)) => name.clone(),
                    _ => return Err("Nome da propriedade inválido".to_string()),
                };

                let object = self.pop()?;
                
                if let Value::Object(object_id) = object {
                    if let Some(obj) = self.heap.get(object_id) {
                        let obj_ref = obj.borrow();
                        match &*obj_ref {
                            Object::Instance { fields, .. } => {
                                // Procura no objeto
                                if let Some(value) = fields.get(&prop_name) {
                                    self.push(value.clone());
                                } else {
                                    self.push(Value::Nil);
                                }
                            }
                            Object::Map(map) => {
                                if let Some(value) = map.get(&prop_name) {
                                    self.push(value.clone());
                                } else {
                                    self.push(Value::Nil);
                                }
                            }
                            _ => return Err("Objeto não suporta propriedades".to_string()),
                        }
                    } else {
                        return Err("Objeto inválido no heap".to_string());
                    }
                } else {
                    return Err("Apenas objetos têm propriedades".to_string());
                }
            }

            OpCode::SetProperty(idx) => {
                let prop_name = match self.read_constant(*idx) {
                    Some(Value::String(name)) => name.clone(),
                    _ => return Err("Nome da propriedade inválido".to_string()),
                };

                let value = self.pop()?;
                let object = self.pop()?;
                
                if let Value::Object(object_id) = object {
                    if let Some(obj) = self.heap.get(object_id) {
                        let mut obj_ref = obj.borrow_mut();
                        match &mut *obj_ref {
                            Object::Instance { fields, .. } => {
                                fields.insert(prop_name, value.clone());
                            }
                            Object::Map(map) => {
                                map.insert(prop_name, value.clone());
                            }
                            _ => return Err("Objeto não suporta propriedades".to_string()),
                        }
                    } else {
                        return Err("Objeto inválido no heap".to_string());
                    }
                } else {
                    return Err("Apenas objetos têm propriedades".to_string());
                }
                
                self.push(value);
            }

            OpCode::This => {
                // 'this' é sempre a primeira variável local no frame atual
                if let Some(frame) = self.frames.last() {
                    let this_value = self.stack[frame.stack_start].clone();
                    self.push(this_value);
                } else {
                    return Err("'this' fora de método".to_string());
                }
            }

            OpCode::Super(_idx) => {
                // TODO: Implementar herança completa
                return Err("'super' não implementado".to_string());
            }

            // ============================================
            // Coleções
            // ============================================
            OpCode::Array(count) => {
                // Desempilha 'count' elementos e cria um array
                let mut elements = Vec::with_capacity(*count as usize);
                for _ in 0..*count {
                    elements.push(self.pop()?);
                }
                elements.reverse();  // Ordem correta
                
                // Aloca no heap
                let array_id = self.heap.allocate(Object::Array(elements));
                self.push(Value::Object(array_id));
            }

            OpCode::Index => {
                let index_val = self.pop()?;
                let collection = self.pop()?;
                
                let index = match index_val {
                    Value::Number(n) => n as usize,
                    _ => return Err("Índice deve ser um número".to_string()),
                };
                
                match collection {
                    Value::Object(id) => {
                        if let Some(obj) = self.heap.get(id) {
                            let obj_ref = obj.borrow();
                            match &*obj_ref {
                                Object::Array(arr) => {
                                    if index >= arr.len() {
                                        return Err(format!("Índice {} fora dos limites do array (tamanho: {})", index, arr.len()));
                                    }
                                    self.push(arr[index].clone());
                                }
                                Object::Tuple(tup) => {
                                    if index >= tup.len() {
                                        return Err(format!("Índice {} fora dos limites do tuple (tamanho: {})", index, tup.len()));
                                    }
                                    self.push(tup[index].clone());
                                }
                                Object::Map(map) => {
                                    // Para mapas, o índice é uma string
                                    let key = match index_val {
                                        Value::Number(n) => n.to_string(),
                                        Value::String(s) => s,
                                        _ => return Err("Chave de mapa deve ser string ou número".to_string()),
                                    };
                                    match map.get(&key) {
                                        Some(val) => self.push(val.clone()),
                                        None => self.push(Value::Nil),
                                    }
                                }
                                _ => return Err("Não é possível indexar este objeto".to_string()),
                            }
                        } else {
                            return Err("Objeto inválido no heap".to_string());
                        }
                    }
                    _ => return Err("Apenas arrays, tuples e mapas podem ser indexados".to_string()),
                }
            }

            OpCode::SetIndex => {
                let value = self.pop()?;
                let index_val = self.pop()?;
                let collection = self.pop()?;
                
                let index = match index_val {
                    Value::Number(n) => n as usize,
                    _ => return Err("Índice deve ser um número".to_string()),
                };
                
                match collection {
                    Value::Object(id) => {
                        if let Some(obj) = self.heap.get(id) {
                            let mut obj_ref = obj.borrow_mut();
                            match &mut *obj_ref {
                                Object::Array(arr) => {
                                    if index >= arr.len() {
                                        return Err(format!("Índice {} fora dos limites do array (tamanho: {})", index, arr.len()));
                                    }
                                    arr[index] = value;
                                }
                                Object::Map(map) => {
                                    let key = match index_val {
                                        Value::Number(n) => n.to_string(),
                                        Value::String(s) => s,
                                        _ => return Err("Chave de mapa deve ser string ou número".to_string()),
                                    };
                                    map.insert(key, value);
                                }
                                _ => return Err("Não é possível modificar índice deste objeto".to_string()),
                            }
                        } else {
                            return Err("Objeto inválido no heap".to_string());
                        }
                    }
                    _ => return Err("Apenas arrays e mapas podem ter índices modificados".to_string()),
                }
                
                // Empilha o valor atribuído (para permitir encadeamento: a[0] = b[1] = 2)
                self.push(value);
            }

            OpCode::Tuple(count) => {
                // Desempilha 'count' elementos e cria um tuple
                let mut elements = Vec::with_capacity(*count as usize);
                for _ in 0..*count {
                    elements.push(self.pop()?);
                }
                elements.reverse();  // Ordem correta
                
                // Aloca no heap
                let tuple_id = self.heap.allocate(Object::Tuple(elements));
                self.push(Value::Object(tuple_id));
            }

            OpCode::TupleAccess(idx) => {
                let tuple = self.pop()?;
                let index = *idx as usize;
                
                match tuple {
                    Value::Object(id) => {
                        if let Some(obj) = self.heap.get(id) {
                            let obj_ref = obj.borrow();
                            match &*obj_ref {
                                Object::Tuple(tup) => {
                                    if index >= tup.len() {
                                        return Err(format!("Índice {} fora dos limites do tuple (tamanho: {})", index, tup.len()));
                                    }
                                    self.push(tup[index].clone());
                                }
                                _ => return Err("Apenas tuples podem usar TupleAccess".to_string()),
                            }
                        } else {
                            return Err("Objeto inválido no heap".to_string());
                        }
                    }
                    _ => return Err("Apenas tuples podem ser acessados com TupleAccess".to_string()),
                }
            }

            // ============================================
            // Manipulação de Pilha
            // ============================================
            OpCode::Pop => {
                self.pop()?;
            }

            OpCode::PopN(n) => {
                for _ in 0..*n {
                    self.pop()?;
                }
            }

            OpCode::Dup => {
                let value = self.peek(0)?.clone();
                self.push(value);
            }

            OpCode::DupN(n) => {
                let idx = self.stack.len() - 1 - *n as usize;
                let value = self.stack.get(idx).ok_or("Índice inválido na pilha")?.clone();
                self.push(value);
            }

            OpCode::Swap => {
                let len = self.stack.len();
                if len < 2 {
                    return Err("Pilha com menos de 2 elementos".to_string());
                }
                self.stack.swap(len - 1, len - 2);
            }

            // ============================================
            // I/O e Debug
            // ============================================
            OpCode::Print => {
                let value = self.pop()?;
                print!("{}", value);
            }

            OpCode::PrintLn => {
                let value = self.pop()?;
                println!("{}", value);
            }

            OpCode::Nop => {
                // Não faz nada
            }

            OpCode::Halt => {
                return Ok(ExecutionControl::Return);
            }

            // ============================================
            // Exceções
            // ============================================
            OpCode::TryBegin(catch_offset, finally_offset) => {
                // Empilha informação de try frame
                self.try_frames.push(TryFrame {
                    catch_ip: frame.ip + *catch_offset as usize,
                    finally_ip: if *finally_offset > 0 { Some(frame.ip + *finally_offset as usize) } else { None },
                    stack_start: self.stack.len(),
                    frame_depth: self.frames.len(),
                });
            }

            OpCode::TryEnd => {
                // Remove o try frame
                self.try_frames.pop();
            }

            OpCode::Throw => {
                let exception = self.pop()?;
                return self.handle_exception(exception, frame);
            }

            OpCode::NewException(msg_idx) => {
                let msg = match self.read_constant(*msg_idx) {
                    Some(Value::String(s)) => s.clone(),
                    _ => "Exceção desconhecida".to_string(),
                };
                
                // Cria objeto de exceção
                let mut fields = std::collections::HashMap::new();
                fields.insert("message".to_string(), Value::String(msg));
                fields.insert("type".to_string(), Value::String("Exception".to_string()));
                
                let exception_id = self.heap.allocate(Object::Instance {
                    class_name: "Exception".to_string(),
                    fields,
                });
                
                self.push(Value::Object(exception_id));
            }

            OpCode::Catch(var_idx) => {
                // A exceção já está no topo da pilha
                // Vamos apenas associar à variável (o compilador já criou a variável local)
                // Não precisamos fazer nada aqui, pois o Catch é seguido de uma atribuição
            }

            // ============================================
            // Upvalues (não implementados)
            // ============================================
            OpCode::GetUpvalue(_)
            | OpCode::SetUpvalue(_)
            | OpCode::CloseUpvalue => {
                return Err("Upvalues não implementados".to_string());
            }
        }

        Ok(ExecutionControl::Continue)
    }

    // ============================================
    // Operações com Pilha
    // ============================================

    /// Empilha um valor
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /// Desempilha um valor
    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Pilha vazia".to_string())
    }

    /// Olha o valor na pilha sem remover
    fn peek(&self, distance: usize) -> Result<&Value, String> {
        let idx = self.stack.len().saturating_sub(1 + distance);
        self.stack.get(idx).ok_or_else(|| "Pilha vazia".to_string())
    }

    // ============================================
    // Chamadas de Função
    // ============================================

    /// Chama uma função definida pelo usuário
    fn call_function(&mut self, function: Rc<Function>, arg_count: u8) -> Result<(), String> {
        // Verifica número de argumentos
        if function.arity != arg_count as usize {
            return Err(format!(
                "Função {} espera {} argumentos, mas recebeu {}",
                function.name, function.arity, arg_count
            ));
        }

        // Verifica limite de recursão
        if self.frames.len() >= self.max_frames {
            return Err("Stack overflow: muitas chamadas recursivas".to_string());
        }

        // Calcula onde os argumentos começam na pilha
        let stack_start = self.stack.len() - arg_count as usize - 1; // -1 para a função

        // Cria novo frame
        self.frames.push(CallFrame::new(function.chunk.clone(), stack_start));

        Ok(())
    }

    /// Chama uma função nativa
    fn call_native(&mut self, native_fn: NativeFn, arg_count: u8) -> Result<(), String> {
        // Pega os argumentos da pilha
        let mut args = Vec::new();
        for _ in 0..arg_count {
            args.push(self.pop()?);
        }
        args.reverse();  // Reverte para ordem correta

        // Remove a função da pilha
        self.pop()?;

        // Chama a função nativa
        let result = native_fn(&args)?;

        // Empilha o resultado
        self.push(result);

        Ok(())
    }

    // ============================================
    // Tratamento de Exceções
    // ============================================

    /// Trata uma exceção lançada
    fn handle_exception(&mut self, exception: Value, current_frame: &mut CallFrame) -> Result<ExecutionControl, String> {
        // Procura um try frame que possa lidar com a exceção
        while let Some(try_frame) = self.try_frames.pop() {
            // Restaura o estado da pilha
            while self.stack.len() > try_frame.stack_start {
                self.stack.pop();
            }

            // Restaura os frames de chamada
            while self.frames.len() > try_frame.frame_depth {
                self.frames.pop();
            }

            // Empilha a exceção para o catch
            self.push(exception.clone());

            // Define o IP para o catch handler
            current_frame.ip = try_frame.catch_ip;

            return Ok(ExecutionControl::Continue);
        }

        // Se não encontrou handler, propaga o erro
        Err(format!("Exceção não capturada: {:?}", exception))
    }

    // ============================================
    // Debug
    // ============================================

    /// Mostra o estado atual da pilha
    fn debug_stack(&self) {
        print!("          ");
        for value in &self.stack {
            print!("[ {} ]", value);
        }
        println!();
    }

    /// Mostra a instrução atual
    fn debug_instruction(&self, frame: &CallFrame, op: &OpCode) {
        let line = frame.current_line().unwrap_or(0);
        print!("{:04} {:4} {:?}\n", frame.ip - 1, line, op);
    }

    /// Reporta um erro em tempo de execução
    fn runtime_error(&self, message: &str) {
        eprintln!("Erro em tempo de execução: {}", message);

        // Mostra stack trace
        for (i, frame) in self.frames.iter().enumerate() {
            if let Some(line) = frame.current_line() {
                eprintln!("  [{}] linha {} em {}", i, line, frame.function.name);
            }
        }
    }

    /// Retorna o valor no topo da pilha (para testes)
    pub fn peek_top(&self) -> Option<&Value> {
        self.stack.last()
    }

    /// Retorna o tamanho atual da pilha
    pub fn stack_size(&self) -> usize {
        self.stack.len()
    }

    /// Retorna o número de frames
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Define uma variável global diretamente
    pub fn define_global(&mut self, name: String, value: Value) {
        self.globals.insert(name, value);
    }

    /// Obtém uma variável global
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.globals.get(name)
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

/// Controle de execução retornado por opcodes
enum ExecutionControl {
    /// Continua execução normal
    Continue,
    /// Retorna da função atual
    Return,
    /// Sai de um loop (break)
    Break,
    /// Continua para próxima iteração (continue)
    ContinueLoop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        let mut chunk = Chunk::new("test");
        chunk.push_op(OpCode::Constant(0), 1);
        chunk.push_op(OpCode::Constant(1), 1);
        chunk.push_op(OpCode::Add, 1);
        chunk.push_op(OpCode::Return, 1);

        let mut vm = VM::new();
        let result = vm.interpret(chunk);
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn test_stack_underflow() {
        let mut chunk = Chunk::new("test");
        chunk.push_op(OpCode::Pop, 1);

        let mut vm = VM::new();
        let result = vm.interpret(chunk);
        assert_eq!(result, InterpretResult::RuntimeError);
    }
}
