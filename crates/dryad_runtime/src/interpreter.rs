// crates/dryad_runtime/src/interpreter.rs
use crate::debug::{DebugCommand, DebugEvent, ExecutionMode, SharedDebugState};
use crate::environment::Environment;
use crate::heap::{Heap, HeapId, ManagedObject};
use crate::native_modules::NativeModuleManager;
use crate::native_registry::NativeRegistry;
pub use crate::value::{
    ClassGetter, ClassMethod, ClassProperty, ClassSetter, FlowControl, ObjectMethod, Value,
};
use dryad_errors::{DryadError, SourceLocation, StackFrame, StackTrace};
use dryad_parser::ast::{
    ClassMember, Expr, ImportKind, InterfaceMember, Literal, MatchArm, ObjectProperty, Pattern,
    Program, Stmt, Visibility,
};
use serde_json::{self, Value as JsonValue};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

// Type alias for compatibility with native modules
pub type RuntimeValue = Value;

pub struct Interpreter {
    pub env: Environment,
    pub heap: Heap,
    pub native_registry: NativeRegistry,
    pub debug_state: Option<SharedDebugState>,

    current_file_path: Option<PathBuf>,
    next_thread_id: u64,
    next_mutex_id: u64,
    next_promise_id: u64,
    threads: HashMap<u64, std::thread::JoinHandle<Result<Value, DryadError>>>,
    mutexes: HashMap<u64, std::sync::Arc<std::sync::Mutex<()>>>,
    pending_promises: HashMap<
        u64,
        Pin<
            Box<
                dyn std::future::Future<Output = Result<Value, crate::errors::RuntimeError>> + Send,
            >,
        >,
    >,
    current_stack_trace: StackTrace,
    resolver: Box<dyn crate::resolver::ModuleResolver>,
    call_depth: usize,
    compile_mode: bool,
    jit_mode: bool,
}

const MAX_RECURSION_DEPTH: usize = 1000;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
            heap: Heap::new(),
            native_registry: NativeRegistry::new(),
            debug_state: None,
            current_file_path: None,
            next_thread_id: 1,
            next_mutex_id: 1,
            next_promise_id: 1,
            threads: HashMap::new(),
            mutexes: HashMap::new(),
            pending_promises: HashMap::new(),
            current_stack_trace: StackTrace::new(),
            resolver: Box::new(crate::resolver::FileSystemResolver),
            call_depth: 0,
            compile_mode: false,
            jit_mode: false,
        }
    }

    pub fn set_current_file(&mut self, file_path: PathBuf) {
        self.current_file_path = Some(file_path);
    }

    pub fn set_resolver(&mut self, resolver: Box<dyn crate::resolver::ModuleResolver>) {
        self.resolver = resolver;
    }

    pub fn set_allow_unsafe(&mut self, allow: bool) {
        self.native_registry.manager.set_allow_unsafe(allow);
    }

    pub fn set_allow_exec(&mut self, allow: bool) {
        self.native_registry.manager.set_allow_exec(allow);
    }

    pub fn set_sandbox_root(&mut self, root: std::path::PathBuf) {
        self.native_registry.manager.set_sandbox_root(root);
    }

    pub fn set_compile_mode(&mut self, compile: bool) {
        self.compile_mode = compile;
    }

    pub fn set_jit_mode(&mut self, jit: bool) {
        self.jit_mode = jit;
    }

    pub fn execute(&mut self, program: &Program) -> Result<String, DryadError> {
        // Adicionar frame inicial do programa principal
        let main_location = SourceLocation {
            file: self.current_file_path.clone(),
            line: 1,
            column: 1,
            position: 0,
            source_line: Some("<main>".to_string()),
        };
        self.current_stack_trace
            .push_frame(StackFrame::new("<main>".to_string(), main_location));

        let mut last_value = Value::Null;

        for statement in &program.statements {
            last_value = self.execute_statement(statement)?;
        }

        // Remover frame ao final
        self.current_stack_trace.frames.pop();

        Ok(last_value.to_string())
    }

    pub fn execute_and_return_value(&mut self, program: &Program) -> Result<Value, DryadError> {
        let mut last_value = Value::Null;

        for statement in &program.statements {
            last_value = self.execute_statement(statement)?;

            // Opcionalmente aciona o GC entre statements
            // self.collect_garbage();
        }

        Ok(last_value)
    }

    pub fn collect_garbage(&mut self) {
        let roots = self.collect_roots();
        self.heap.collect(&roots);
    }

    fn maybe_collect_garbage(&mut self) {
        if self.heap.should_collect() {
            self.collect_garbage();
        }
    }

    fn collect_roots(&self) -> Vec<HeapId> {
        let mut roots = Vec::new();

        // 1. Variáveis globais/locais atuais
        for val in self.env.variables.values() {
            self.collect_value_roots(val, &mut roots);
        }

        // 2. Constantes
        for val in self.env.constants.values() {
            self.collect_value_roots(val, &mut roots);
        }

        // 3. Classes (definições)
        for val in self.env.classes.values() {
            self.collect_value_roots(val, &mut roots);
        }

        // 4. Módulos importados
        for module in self.env.imported_modules.values() {
            for val in module.values() {
                self.collect_value_roots(val, &mut roots);
            }
        }

        // 5. Instância atual (this)
        if let Some(val) = &self.env.current_instance {
            self.collect_value_roots(val, &mut roots);
        }

        // 6. Variáveis em frames de chamadas anteriores
        for env in &self.env.call_stack_vars {
            for val in env.values() {
                self.collect_value_roots(val, &mut roots);
            }
        }

        roots
    }

    fn collect_value_roots(&self, val: &Value, roots: &mut Vec<HeapId>) {
        match val {
            Value::Array(id)
            | Value::Tuple(id)
            | Value::Lambda(id)
            | Value::Class(id)
            | Value::Instance(id)
            | Value::Object(id) => {
                roots.push(*id);
            }
            Value::Promise {
                value: Some(inner), ..
            } => {
                self.collect_value_roots(inner, roots);
            }
            _ => {}
        }
    }

    pub fn set_debug_state(&mut self, state: SharedDebugState) {
        self.debug_state = Some(state);
    }

    fn check_debug_hooks(&mut self, location: &SourceLocation) -> Result<(), DryadError> {
        let debug_state_arc = if let Some(state) = &self.debug_state {
            state.clone()
        } else {
            return Ok(());
        };

        let file_path = location
            .file
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let line = location.line;

        let mut should_pause = false;
        {
            let mut state = debug_state_arc.lock().unwrap();
            if state.should_pause(&file_path, line) {
                state.execution_mode = ExecutionMode::Paused;
                should_pause = true;
            }
        }

        if should_pause {
            {
                let mut state = debug_state_arc.lock().unwrap();
                let event = if state.execution_mode == ExecutionMode::Stepping {
                    DebugEvent::StepComplete {
                        file: file_path.clone(),
                        line,
                    }
                } else {
                    DebugEvent::BreakpointHit {
                        file: file_path.clone(),
                        line,
                    }
                };
                state.event_queue.push(event);
                state.execution_mode = ExecutionMode::Paused;
            }

            // Loop de espera por comando
            loop {
                let command = {
                    let mut state = debug_state_arc.lock().unwrap();
                    state.command_queue.pop()
                };

                if let Some(cmd) = command {
                    match cmd {
                        DebugCommand::Continue => {
                            let mut state = debug_state_arc.lock().unwrap();
                            state.execution_mode = ExecutionMode::Running;
                            break;
                        }
                        DebugCommand::Step => {
                            let mut state = debug_state_arc.lock().unwrap();
                            state.execution_mode = ExecutionMode::Stepping;
                            break;
                        }
                        DebugCommand::GetVariables => {
                            let vars = self.get_debug_variables();
                            let mut state = debug_state_arc.lock().unwrap();
                            state.event_queue.push(DebugEvent::Variables(vars));
                        }
                        DebugCommand::GetHeap => {
                            let heap = self.get_debug_heap();
                            let mut state = debug_state_arc.lock().unwrap();
                            state.event_queue.push(DebugEvent::Heap(heap));
                        }
                        DebugCommand::Pause => {
                            // Já está pausado
                        }
                        _ => {}
                    }
                }

                // Dorme um pouco para não consumir CPU
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        Ok(())
    }

    pub fn get_debug_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        for (name, val) in &self.env.variables {
            let name_str: String = name.clone();
            let val_str: String = val.to_string();
            vars.insert(name_str, val_str);
        }
        for (name, val) in &self.env.constants {
            let name_str: String = name.clone();
            let val_str: String = val.to_string();
            vars.insert(name_str, val_str);
        }
        vars
    }

    pub fn get_debug_heap(&self) -> Vec<String> {
        // Implementação simplificada para o debugger
        vec![format!("Heap size: {} objects", self.heap.heap_size())]
    }

    // Método helper para criar erros runtime com stack trace atual
    fn runtime_error(&self, code: u16, message: &str) -> DryadError {
        let location = self
            .current_stack_trace
            .frames
            .last()
            .map(|frame| frame.location.clone())
            .unwrap_or_else(SourceLocation::unknown);

        DryadError::Runtime {
            code,
            message: message.to_string(),
            location,
            stack_trace: self.current_stack_trace.clone(),
            debug_context: None,
        }
    }

    // Método antigo mantido para compatibilidade com testes existentes
    pub fn eval(&mut self, expr: &Expr) -> Result<String, DryadError> {
        let value = self.evaluate(expr)?;
        Ok(value.to_string())
    }

    pub fn execute_statement(&mut self, stmt: &Stmt) -> Result<Value, DryadError> {
        // Obter localização do statement
        let location = match stmt {
            Stmt::Expression(_, loc) => loc,
            Stmt::VarDeclaration(_, _, _, loc) => loc,
            Stmt::ConstDeclaration(_, _, _, loc) => loc,
            Stmt::Assignment(_, _, loc) => loc,
            Stmt::PropertyAssignment(_, _, _, loc) => loc,
            Stmt::IndexAssignment(_, _, _, loc) => loc,
            Stmt::Block(_, loc) => loc,
            Stmt::If(_, _, loc) => loc,
            Stmt::IfElse(_, _, _, loc) => loc,
            Stmt::While(_, _, loc) => loc,
            Stmt::DoWhile(_, _, loc) => loc,
            Stmt::Break(loc) => loc,
            Stmt::Continue(loc) => loc,
            Stmt::For(_, _, _, _, loc) => loc,
            Stmt::ForEach(_, _, _, loc) => loc,
            Stmt::Try(_, _, _, loc) => loc,
            Stmt::Throw(_, loc) => loc,
            Stmt::FunctionDeclaration { location, .. } => location,
            Stmt::ThreadFunctionDeclaration { location, .. } => location,
            Stmt::ClassDeclaration(_, _, _, _, loc) => loc,
            Stmt::InterfaceDeclaration(_, _, loc) => loc,
            Stmt::Return(_, loc) => loc,
            Stmt::NativeDirective(_, loc) => loc,
            Stmt::Export(_, loc) => loc,
            Stmt::Use(_, loc) => loc,
            Stmt::Import(_, _, loc) => loc,
        };

        // Hook de depuração
        self.check_debug_hooks(location)?;

        // Proteção contra recursão infinita
        self.call_depth += 1;
        if self.call_depth > MAX_RECURSION_DEPTH {
            self.call_depth -= 1;
            return Err(DryadError::runtime(
                3001,
                &format!(
                    "Limite de recursão excedido ({}). Verifique se há recursão infinita.",
                    MAX_RECURSION_DEPTH
                ),
                location.clone(),
                self.current_stack_trace.clone(),
            ));
        }

        let result = match stmt {
            Stmt::NativeDirective(module_name, _) => {
                match self.native_registry.manager.activate_category(module_name) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => Err(DryadError::runtime(
                        6001,
                        &e,
                        location.clone(),
                        self.current_stack_trace.clone(),
                    )),
                }
            }
            Stmt::Expression(expr, _) => self.evaluate(expr),
            Stmt::VarDeclaration(pattern, _, initializer, _) => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Null,
                };

                let mut bindings = HashMap::new();
                if self.match_pattern(&value, pattern, &mut bindings) {
                    for (name, val) in bindings {
                        self.env.variables.insert(name, val);
                    }
                    Ok(Value::Null)
                } else {
                    Err(self.runtime_error(3035, "Padrão de desestruturação não corresponde ao valor na declaração de variável"))
                }
            }
            Stmt::ConstDeclaration(pattern, _, expr, _) => {
                let value = self.evaluate(expr)?;
                let mut bindings = HashMap::new();

                if self.match_pattern(&value, pattern, &mut bindings) {
                    for (name, val) in bindings {
                        if self.env.constants.contains_key(&name) {
                            return Err(self.runtime_error(
                                3002,
                                &format!("Constante '{}' já foi declarada", name),
                            ));
                        }
                        self.env.constants.insert(name, val);
                    }
                    Ok(Value::Null)
                } else {
                    Err(self.runtime_error(3036, "Padrão de desestruturação não corresponde ao valor na declaração de constante"))
                }
            }
            Stmt::Assignment(pattern, expr, _) => {
                let value = self.evaluate(expr)?;
                let mut bindings = HashMap::new();

                if self.match_pattern(&value, pattern, &mut bindings) {
                    for (name, val) in bindings {
                        // Verifica se não está tentando modificar uma constante
                        if self.env.constants.contains_key(&name) {
                            return Err(self.runtime_error(
                                3011,
                                &format!("Não é possível modificar a constante '{}'", name),
                            ));
                        }

                        // Na desestruturação por atribuição, poderíamos permitir criar variáveis novas
                        // mas segui o padrão atual de exigir declaração prévia para variáveis simples.
                        // Para desestruturação, fazemos o mesmo.
                        if !self.env.variables.contains_key(&name) {
                            return Err(self.runtime_error(
                                3001,
                                &format!("Variável '{}' não foi declarada", name),
                            ));
                        }

                        self.env.variables.insert(name, val);
                    }
                    Ok(value)
                } else {
                    Err(self.runtime_error(
                        3037,
                        "Padrão de desestruturação não corresponde ao valor na atribuição",
                    ))
                }
            }
            Stmt::PropertyAssignment(object_expr, property_name, value_expr, _) => {
                let value = self.evaluate(value_expr)?;
                let object = self.evaluate(object_expr)?;

                match object {
                    Value::Instance(instance_id) => {
                        // Get class info to check for setter
                        let class_name = {
                            let heap_obj = self.heap.get(instance_id).ok_or_else(|| {
                                DryadError::new(3100, "Heap error: Instance reference not found")
                            })?;
                            if let ManagedObject::Instance { class_name, .. } = heap_obj {
                                class_name.clone()
                            } else {
                                return Err(DryadError::new(3101, "Heap error: Expected Instance"));
                            }
                        };

                        // Check for setter in class
                        let setter_to_run = if let Some(Value::Class(cid)) = self.env.classes.get(&class_name) {
                            if let Some(class_obj) = self.heap.get(*cid) {
                                if let ManagedObject::Class { setters, .. } = class_obj {
                                    setters.get(property_name).cloned()
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if let Some(setter) = setter_to_run {
                            match setter.visibility {
                                Visibility::Private => {
                                    return Err(DryadError::new(
                                        3029,
                                        &format!("Setter '{}' é privado", property_name),
                                    ));
                                }
                                _ => {
                                    // Execute setter with 'this' and parameter
                                    let mut setter_env = self.env.clone();
                                    setter_env.variables.insert("this".to_string(), object.clone());
                                    setter_env.variables.insert(setter.param.clone(), value.clone());
                                    let prev_env = std::mem::replace(&mut self.env, setter_env);
                                    let _ = self.execute_statement(&setter.body);
                                    self.env = prev_env;
                                    return Ok(value);
                                }
                            }
                        }

                        // No setter found, assign directly to instance
                        let heap_obj = self.heap.get_mut(instance_id).ok_or_else(|| {
                            DryadError::new(3100, "Heap error: Instance reference not found")
                        })?;
                        
                        if let ManagedObject::Instance { properties, .. } = heap_obj {
                            properties.insert(property_name.clone(), value.clone());
                            Ok(value)
                        } else {
                            Err(DryadError::new(3101, "Heap error: Expected Instance"))
                        }
                    }
                    Value::Object(id) => {
                        let heap_obj = self.heap.get_mut(id).ok_or_else(|| {
                            DryadError::new(3100, "Heap error: Object reference not found")
                        })?;
                        
                        if let ManagedObject::Object { properties, .. } = heap_obj {
                            properties.insert(property_name.clone(), value.clone());
                            Ok(value)
                        } else {
                            Err(DryadError::new(3101, "Heap error: Expected Object"))
                        }
                    }
                    _ => Err(self.runtime_error(3034, "Tentativa de atribuir propriedade a valor que não é uma instância ou objeto"))
                }
            }
            Stmt::IndexAssignment(array_expr, index_expr, value_expr, _) => {
                let value = self.evaluate(value_expr)?;
                let index_value = self.evaluate(index_expr)?;

                // Handle different types of indices
                let result = self.execute_index_assignment(array_expr, index_value, value)?;
                Ok(result)
            }
            Stmt::Block(statements, _) => self.execute_block(statements),
            Stmt::If(condition, then_stmt, _) => {
                let condition_value = self.evaluate(condition)?;
                if self.is_truthy(&condition_value) {
                    self.execute_statement(then_stmt)
                } else {
                    Ok(Value::Null)
                }
            }
            Stmt::IfElse(condition, then_stmt, else_stmt, _) => {
                let condition_value = self.evaluate(condition)?;
                if self.is_truthy(&condition_value) {
                    self.execute_statement(then_stmt)
                } else {
                    self.execute_statement(else_stmt)
                }
            }
            Stmt::While(condition, body, _) => {
                let mut last_value = Value::Null;

                loop {
                    let condition_value = self.evaluate(condition)?;
                    if !self.is_truthy(&condition_value) {
                        break;
                    }

                    // Execute o corpo do loop
                    match self.execute_statement(body) {
                        Ok(value) => last_value = value,
                        Err(err) => {
                            // Verifica se é break ou continue
                            if err.code() == 3010 {
                                // Break
                                break;
                            } else if err.code() == 3011 {
                                // Continue
                                continue;
                            } else {
                                return Err(err);
                            }
                        }
                    }
                }

                Ok(last_value)
            }
            Stmt::DoWhile(body, condition, _) => {
                let mut last_value = Value::Null;

                // Do-while executa o corpo pelo menos uma vez
                loop {
                    // Execute o corpo do loop primeiro
                    match self.execute_statement(body) {
                        Ok(value) => last_value = value,
                        Err(err) => {
                            // Verifica se é break ou continue
                            if err.code() == 3010 {
                                // Break
                                break;
                            } else if err.code() == 3011 { // Continue
                                 // No continue, ainda precisa avaliar a condição
                            } else {
                                return Err(err);
                            }
                        }
                    }

                    // Avalia a condição após executar o corpo
                    let condition_value = self.evaluate(condition)?;
                    if !self.is_truthy(&condition_value) {
                        break;
                    }
                }

                Ok(last_value)
            }
            Stmt::Break(_) => Err(DryadError::new(3010, "break")),
            Stmt::Continue(_) => Err(DryadError::new(3011, "continue")),
            Stmt::For(init, condition, update, body, _) => {
                self.execute_for_loop(init, condition, update, body)
            }
            Stmt::ForEach(pattern, iterable, body, _) => {
                self.execute_foreach_loop(pattern, iterable, body)
            }
            Stmt::Try(try_block, catch_clause, finally_block, _) => {
                self.execute_try_catch_finally(try_block, catch_clause, finally_block)
            }
            Stmt::Throw(expr, location) => {
                let value = self.evaluate(expr)?;
                let exception_msg = match value {
                    Value::String(s) => s,
                    _ => value.to_string(),
                };
                Err(DryadError::Runtime {
                    code: 3020,
                    message: exception_msg,
                    location: location.clone(),
                    stack_trace: self.current_stack_trace.clone(),
                    debug_context: None,
                })
            }
            Stmt::FunctionDeclaration {
                name,
                params,
                body,
                is_async,
                ..
            } => {
                let params_vec: Vec<String> = params.iter().map(|(p, _)| p.clone()).collect();
                if *is_async {
                    let async_function = Value::AsyncFunction {
                        name: name.clone(),
                        params: params_vec,
                        body: (**body).clone(),
                    };
                    self.env.variables.insert(name.clone(), async_function);
                } else {
                    let function = Value::Function {
                        name: name.clone(),
                        params: params_vec,
                        body: (**body).clone(),
                    };
                    self.env.variables.insert(name.clone(), function);
                }
                Ok(Value::Null)
            }
            Stmt::ThreadFunctionDeclaration {
                name, params, body, ..
            } => {
                let params_vec: Vec<String> = params.iter().map(|(p, _)| p.clone()).collect();
                let thread_function = Value::ThreadFunction {
                    name: name.clone(),
                    params: params_vec,
                    body: (**body).clone(),
                };
                self.env.variables.insert(name.clone(), thread_function);
                Ok(Value::Null)
            }
            Stmt::ClassDeclaration(name, parent, interfaces, members, _) => {
                let mut methods = HashMap::new();
                let mut properties = HashMap::new();
                let mut getters = HashMap::new();
                let mut setters = HashMap::new();

                // Process class members
                for member in members {
                    match member {
                        ClassMember::Method {
                            visibility,
                            is_static,
                            is_async: _,
                            name: method_name,
                            params,
                            body,
                            ..
                        } => {
                            let params_vec: Vec<String> =
                                params.iter().map(|(p, _)| p.clone()).collect();
                            let method = ClassMethod {
                                visibility: visibility.clone(),
                                is_static: *is_static,
                                params: params_vec,
                                body: *(*body).clone(),
                            };
                            methods.insert(method_name.clone(), method);
                        }
                        ClassMember::Property(
                            visibility,
                            is_static,
                            prop_name,
                            _,
                            default_value,
                        ) => {
                            let default_val = match default_value {
                                Some(expr) => Some(self.evaluate(&expr)?),
                                None => None,
                            };
                            let property = ClassProperty {
                                visibility: visibility.clone(),
                                is_static: *is_static,
                                default_value: default_val,
                            };
                            properties.insert(prop_name.clone(), property);
                        }
                        ClassMember::Getter {
                            visibility,
                            name: getter_name,
                            body,
                        } => {
                            let getter = ClassGetter {
                                visibility: visibility.clone(),
                                name: getter_name.clone(),
                                body: *(*body).clone(),
                            };
                            getters.insert(getter_name.clone(), getter);
                        }
                        ClassMember::Setter {
                            visibility,
                            name: setter_name,
                            param,
                            body,
                        } => {
                            let setter = ClassSetter {
                                visibility: visibility.clone(),
                                name: setter_name.clone(),
                                param: param.clone(),
                                body: *(*body).clone(),
                            };
                            setters.insert(setter_name.clone(), setter);
                        }
                    }
                }

                // Verify interfaces are implemented
                for interface_name in interfaces {
                    if !self.env.interfaces.contains_key(interface_name) {
                        return Err(DryadError::new(
                            3102,
                            &format!("Interface '{}' não encontrada", interface_name),
                        ));
                    }

                    // Verify class implements all interface methods
                    let interface = self.env.interfaces.get(interface_name).unwrap();
                    for interface_method in interface {
                        let method_name = match interface_method {
                            InterfaceMember::Method(m) => &m.name,
                        };

                        if !methods.contains_key(method_name) {
                            return Err(DryadError::new(
                                3103,
                                &format!(
                                    "Classe '{}' deve implementar o método '{}' da interface '{}'",
                                    name, method_name, interface_name
                                ),
                            ));
                        }
                    }
                }

                let managed_class = ManagedObject::Class {
                    name: name.clone(),
                    parent: parent.clone(),
                    interfaces: interfaces.clone(),
                    methods,
                    properties,
                    getters,
                    setters,
                };
                let class_id = self.heap.allocate(managed_class);
                self.maybe_collect_garbage();
                let class = Value::Class(class_id);

                self.env.classes.insert(name.clone(), class.clone());
                self.env.variables.insert(name.clone(), class); // Também disponível como variável
                Ok(Value::Null)
            }
            Stmt::InterfaceDeclaration(name, members, _) => {
                // Register interface in environment
                self.env.interfaces.insert(name.clone(), members.clone());
                Ok(Value::Null)
            }
            Stmt::Return(expr, _) => {
                let value = match expr {
                    Some(e) => self.evaluate(e)?,
                    None => Value::Null,
                };
                // Use uma convenção específica para distinguir returns de outros erros
                match value {
                    Value::Number(n) => Err(DryadError::new(3021, &format!("RETURN_NUMBER:{}", n))),
                    Value::String(s) => Err(DryadError::new(3021, &format!("RETURN_STRING:{}", s))),
                    Value::Bool(b) => Err(DryadError::new(3021, &format!("RETURN_BOOL:{}", b))),
                    Value::Null => Err(DryadError::new(3021, "RETURN_NULL")),
                    Value::Array(_)
                    | Value::Tuple(_)
                    | Value::Lambda(_)
                    | Value::Class(_)
                    | Value::Instance(_)
                    | Value::Object(_)
                    | Value::Exception(_)
                    | Value::Function { .. }
                    | Value::AsyncFunction { .. }
                    | Value::ThreadFunction { .. }
                    | Value::Thread { .. }
                    | Value::Mutex { .. }
                    | Value::Promise { .. } => Err(DryadError::new(
                        3021,
                        &format!("RETURN_OTHER:{}", value.to_string()),
                    )),
                }
            }
            Stmt::Export(stmt, _) => {
                // Por enquanto, simplesmente executa o statement interno
                // Em uma implementação completa, isto seria registrado como exportação
                self.execute_statement(stmt)
            }
            Stmt::Use(module_path, _) => {
                // Importa o módulo especificado
                self.import_module(module_path)
            }
            Stmt::Import(kind, module_path, _) => {
                // Importa o módulo com diferentes estratégias
                self.import_module_with_kind(kind, module_path)
            }
        };

        self.call_depth -= 1;
        result
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        match expr {
            Expr::Literal(literal, _) => self.eval_literal(literal),
            Expr::Variable(name, _) => self.eval_variable(name),
            Expr::Binary(left, operator, right, _) => self.eval_binary(left, operator, right),
            Expr::Unary(operator, operand, _) => self.eval_unary(operator, operand),
            Expr::Call(func_expr, args, location) => self.eval_call(func_expr, args, location),
            Expr::PostIncrement(expr, _) => self.eval_post_increment(expr),
            Expr::PostDecrement(expr, _) => self.eval_post_decrement(expr),
            Expr::PreIncrement(expr, _) => self.eval_pre_increment(expr),
            Expr::PreDecrement(expr, _) => self.eval_pre_decrement(expr),
            Expr::Array(elements, _) => self.eval_array(elements),
            Expr::Tuple(elements, _) => self.eval_tuple(elements),
            Expr::Index(array_expr, index_expr, _) => self.eval_index(array_expr, index_expr),
            Expr::TupleAccess(tuple_expr, index, _) => self.eval_tuple_access(tuple_expr, *index),
            Expr::Lambda { params, body, .. } => {
                let params_vec: Vec<String> = params.iter().map(|(p, _)| p.clone()).collect();
                let managed_lambda = ManagedObject::Lambda {
                    params: params_vec,
                    body: *body.clone(),
                    closure: self.env.variables.clone(), // Captura o escopo atual
                };
                let lambda_id = self.heap.allocate(managed_lambda);
                self.maybe_collect_garbage();
                Ok(Value::Lambda(lambda_id))
            }
            Expr::This(_) => {
                if let Some(instance) = &self.env.current_instance {
                    Ok(instance.clone())
                } else {
                    Err(DryadError::new(
                        3022,
                        "'this' usado fora do contexto de uma instância",
                    ))
                }
            }
            Expr::Super(_) => {
                // Para implementar super, precisaríamos do contexto da classe pai
                // Por agora, retorna erro
                Err(DryadError::new(3023, "'super' ainda não implementado"))
            }
            Expr::MethodCall(object_expr, method_name, args, _) => {
                self.eval_method_call(object_expr, method_name, args)
            }
            Expr::PropertyAccess(object_expr, property_name, _) => {
                self.eval_property_access(object_expr, property_name)
            }
            Expr::ClassInstantiation(class_name, args, location) => {
                self.eval_class_instantiation(class_name, args, location)
            }
            Expr::ObjectLiteral(properties, _) => self.eval_object_literal(properties),
            Expr::Match(target, arms, location) => self.eval_match(target, arms, location),
            Expr::Await(expr, _) => self.eval_await(expr),
            Expr::ThreadCall(func_expr, args, _) => self.eval_thread_call(func_expr, args),
            Expr::MutexCreation(_) => self.eval_mutex_creation(),
        }
    }

    fn eval_literal(&self, literal: &Literal) -> Result<Value, DryadError> {
        match literal {
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Null => Ok(Value::Null),
        }
    }

    fn eval_variable(&self, name: &str) -> Result<Value, DryadError> {
        // Primeiro verifica nas constantes
        if let Some(value) = self.env.constants.get(name) {
            return Ok(value.clone());
        }

        // Depois verifica nas variáveis
        self.env
            .variables
            .get(name)
            .cloned()
            .ok_or_else(|| self.runtime_error(3001, &format!("Variável '{}' não definida", name)))
    }

    fn eval_binary(
        &mut self,
        left: &Expr,
        operator: &str,
        right: &Expr,
    ) -> Result<Value, DryadError> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match operator {
            "+" => self.add_values(left_val, right_val),
            "-" => self.subtract_values(left_val, right_val),
            "*" => self.multiply_values(left_val, right_val),
            "/" => self.divide_values(left_val, right_val),
            "%" => self.modulo_values(left_val, right_val),
            "**" => self.power_values(left_val, right_val),
            "^^" => self.nth_root_values(left_val, right_val),
            "%%" => self.safe_modulo_values(left_val, right_val),
            "##" => self.power_of_ten_values(left_val, right_val),
            // Operadores bitwise
            "&" => self.bitwise_and_values(left_val, right_val),
            "|" => self.bitwise_or_values(left_val, right_val),
            "^" => self.bitwise_xor_values(left_val, right_val),
            // Operadores de shift
            "<<" => self.left_shift_values(left_val, right_val),
            ">>" => self.right_shift_values(left_val, right_val),
            "<<<" => self.symmetric_left_shift_values(left_val, right_val),
            ">>>" => self.symmetric_right_shift_values(left_val, right_val),
            // Operadores de comparação
            "==" => Ok(Value::Bool(self.values_equal(&left_val, &right_val))),
            "!=" => Ok(Value::Bool(!self.values_equal(&left_val, &right_val))),
            "<" => self.compare_values(left_val, right_val, |a, b| a < b),
            ">" => self.compare_values(left_val, right_val, |a, b| a > b),
            "<=" => self.compare_values(left_val, right_val, |a, b| a <= b),
            ">=" => self.compare_values(left_val, right_val, |a, b| a >= b),
            "&&" => Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy())),
            "||" => Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy())),
            "!" => Ok(Value::Bool(!right_val.is_truthy())), // Unário
            _ => Err(DryadError::new(
                3002,
                &format!("Operador desconhecido: {}", operator),
            )),
        }
    }

    fn eval_call(
        &mut self,
        func_expr: &Expr,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        // Se a expressão da função é uma variável simples, usar o caminho otimizado
        if let Expr::Variable(name, _) = func_expr {
            return self.eval_call_by_name(name, args, location);
        }

        // Para expressões complexas (como lambdas imediatas), avaliar a expressão primeiro
        let function_value = self.evaluate(func_expr)?;

        match function_value {
            Value::Function { name, params, body } => {
                self.call_user_function(name, params, body, args, location)
            }
            Value::Lambda(id) => {
                let heap_obj = self.heap.get(id).cloned().ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Lambda reference not found")
                })?;

                if let ManagedObject::Lambda {
                    params,
                    body,
                    closure,
                } = heap_obj
                {
                    self.call_lambda(params, body, closure, args, location)
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Lambda"))
                }
            }
            _ => Err(DryadError::new(3003, "Expressão não é uma função")),
        }
    }

    fn eval_call_by_name(
        &mut self,
        name: &str,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        // Primeiro verificar se é uma classe (para instanciação)
        if self.env.classes.contains_key(name) {
            return self.eval_class_instantiation(name, args, location);
        }

        // Segundo verificar se é uma função nativa do novo sistema modular
        if let Some(native_func) = self.native_registry.manager.get_function(name) {
            // Avaliar argumentos primeiro
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(self.evaluate(arg)?);
            }
            // Chama a função nativa
            return native_func(&arg_values, &self.native_registry.manager, &mut self.heap)
                .map_err(|e| {
                    DryadError::new(3005, &format!("Erro na função nativa '{}': {}", name, e))
                });
        }

        // Terceiro verificar se é uma função nativa assíncrona
        if let Some(async_native_func) = self.native_registry.manager.get_async_function(name) {
            // Avaliar argumentos primeiro
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(self.evaluate(arg)?);
            }

            let promise_id = self.next_promise_id;
            self.next_promise_id += 1;

            // Chama a função nativa assíncrona para obter o Future
            let future =
                async_native_func(arg_values, &self.native_registry.manager, &mut self.heap);

            // Armazena o future para ser resolvido depois no await
            self.pending_promises.insert(promise_id, future);

            return Ok(Value::Promise {
                id: promise_id,
                resolved: false,
                value: None,
            });
        }

        match name {
            _ => {
                // Verificar se é uma função definida pelo usuário
                if let Some(function_value) = self.env.variables.get(name).cloned() {
                    match function_value {
                        Value::Function {
                            name: _,
                            params,
                            body,
                        } => {
                            self.call_user_function(name.to_string(), params, body, args, location)
                        }
                        Value::Lambda(id) => {
                            let heap_obj = self.heap.get(id).ok_or_else(|| {
                                DryadError::new(3100, "Heap error: Lambda reference not found")
                            })?;
                            if let ManagedObject::Lambda {
                                params,
                                body,
                                closure,
                            } = heap_obj
                            {
                                self.call_lambda(
                                    params.clone(),
                                    body.clone(),
                                    closure.clone(),
                                    args,
                                    location,
                                )
                            } else {
                                Err(DryadError::new(3101, "Heap error: Expected Lambda"))
                            }
                        }
                        _ => Err(DryadError::new(
                            3003,
                            &format!("'{}' não é uma função", name),
                        )),
                    }
                } else {
                    // Verificar se a função existe em uma categoria nativa inativa
                    if self
                        .native_registry
                        .manager
                        .is_function_in_inactive_category(name)
                    {
                        if let Some(category) =
                            self.native_registry.manager.find_function_category(name)
                        {
                            return Err(DryadError::runtime(
                                6001,
                                &format!(
                                    "Função nativa '{}' não está disponível. \
                                    Ative o módulo '{}' com a diretiva #<{}> antes de usar esta função.",
                                    name, category, category
                                ),
                                location.clone(),
                                self.current_stack_trace.clone(),
                            ));
                        }
                    }

                    Err(DryadError::new(
                        3003,
                        &format!("Função '{}' não definida", name),
                    ))
                }
            }
        }
    }

    fn call_user_function(
        &mut self,
        function_name: String,
        params: Vec<String>,
        body: Stmt,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate(arg)?);
        }
        self.call_user_function_values(function_name, params, body, arg_values, location)
    }

    fn call_user_function_values(
        &mut self,
        function_name: String,
        params: Vec<String>,
        body: Stmt,
        arg_values: Vec<Value>,
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        // Verificar limite de recursão
        self.call_depth += 1;
        if self.call_depth > MAX_RECURSION_DEPTH {
            self.call_depth -= 1;
            return Err(self.runtime_error(3040, "Stack overflow: limite de recursão excedido"));
        }

        // Verificar número de argumentos (allow extra arguments, JavaScript-style)
        if arg_values.len() < params.len() {
            self.call_depth -= 1;
            return Err(self.runtime_error(
                3004,
                &format!(
                    "Número incorreto de argumentos: esperado pelo menos {}, encontrado {}",
                    params.len(),
                    arg_values.len()
                ),
            ));
        }

        // Salvar estado atual das variáveis para escopo e GC roots
        self.env.call_stack_vars.push(self.env.variables.clone());

        // Push stack frame for function call
        let frame = StackFrame::new(function_name.clone(), location.clone());
        self.current_stack_trace.push_frame(frame);

        // Bind parameters
        for (i, param) in params.iter().enumerate() {
            self.env
                .variables
                .insert(param.clone(), arg_values[i].clone());
        }

        // Executar corpo da função
        let result = match self.execute_statement(&body) {
            Ok(value) => Ok(value),
            Err(err) => {
                // Verificar se é um retorno especial
                if err.code() == 3021 {
                    if err.message() == "RETURN_NULL" {
                        Ok(Value::Null)
                    } else if let Some(num_str) = err.message().strip_prefix("RETURN_NUMBER:") {
                        if let Ok(num) = num_str.parse::<f64>() {
                            Ok(Value::Number(num))
                        } else {
                            Ok(Value::Null)
                        }
                    } else if let Some(str_val) = err.message().strip_prefix("RETURN_STRING:") {
                        Ok(Value::String(str_val.to_string()))
                    } else if let Some(bool_str) = err.message().strip_prefix("RETURN_BOOL:") {
                        if let Ok(bool_val) = bool_str.parse::<bool>() {
                            Ok(Value::Bool(bool_val))
                        } else {
                            Ok(Value::Null)
                        }
                    } else if let Some(other_val) = err.message().strip_prefix("RETURN_OTHER:") {
                        Ok(Value::String(other_val.to_string()))
                    } else {
                        Ok(Value::Null)
                    }
                } else {
                    Err(err)
                }
            }
        };

        // Pop stack frame
        self.current_stack_trace.frames.pop();

        // Restaurar estado das variáveis
        if let Some(saved) = self.env.call_stack_vars.pop() {
            self.env.variables = saved;
        }

        self.call_depth -= 1;
        result
    }

    fn call_lambda(
        &mut self,
        params: Vec<String>,
        body: Expr,
        closure: HashMap<String, Value>,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate(arg)?);
        }
        self.call_lambda_values(params, body, closure, arg_values, location)
    }

    fn call_lambda_values(
        &mut self,
        params: Vec<String>,
        body: Expr,
        closure: HashMap<String, Value>,
        arg_values: Vec<Value>,
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        // Verificar limite de recursão
        self.call_depth += 1;
        if self.call_depth > MAX_RECURSION_DEPTH {
            self.call_depth -= 1;
            return Err(self.runtime_error(
                3040,
                "Stack overflow: limite de recursão excedido em lambda",
            ));
        }

        // Verificar número de argumentos (allow extra arguments, JavaScript-style)
        if arg_values.len() < params.len() {
            self.call_depth -= 1;
            return Err(DryadError::new(
                3004,
                &format!(
                    "Número incorreto de argumentos: esperado pelo menos {}, encontrado {}",
                    params.len(),
                    arg_values.len()
                ),
            ));
        }

        // Salvar estado atual das variáveis para escopo e GC roots
        self.env.call_stack_vars.push(self.env.variables.clone());

        // Restaurar o closure (escopo onde a lambda foi criada)
        self.env.variables = closure;

        // Criar parâmetros com os valores já avaliados
        for (i, param) in params.iter().enumerate() {
            self.env
                .variables
                .insert(param.clone(), arg_values[i].clone());
        }

        // Executar corpo da lambda (é uma expressão)
        let result = self.evaluate(&body);

        // Restaurar estado das variáveis original
        if let Some(saved) = self.env.call_stack_vars.pop() {
            self.env.variables = saved;
        }

        self.call_depth -= 1;
        result
    }

    fn add_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_string()))),
            (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_string(), b))),
            _ => Err(DryadError::new(
                3004,
                "Operação '+' inválida para estes tipos",
            )),
        }
    }

    fn subtract_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(DryadError::new(
                3005,
                "Operação '-' só é válida para números",
            )),
        }
    }

    fn multiply_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(DryadError::new(
                3006,
                "Operação '*' só é válida para números",
            )),
        }
    }

    fn divide_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(DryadError::new(3007, "Divisão por zero"))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(DryadError::new(
                3008,
                "Operação '/' só é válida para números",
            )),
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (Value::Lambda(a), Value::Lambda(b)) => a == b,
            (Value::Class(a), Value::Class(b)) => a == b,
            (Value::Instance(a), Value::Instance(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            _ => false,
        }
    }

    fn compare_values<F>(&self, left: Value, right: Value, op: F) -> Result<Value, DryadError>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(op(a, b))),
            _ => Err(DryadError::new(3009, "Comparação só é válida para números")),
        }
    }

    fn eval_unary(&mut self, operator: &str, operand: &Expr) -> Result<Value, DryadError> {
        let value = self.evaluate(operand)?;

        match operator {
            "-" => match value {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(DryadError::new(
                    3005,
                    "Operação '-' só é válida para números",
                )),
            },
            "!" => Ok(Value::Bool(!self.is_truthy(&value))),
            _ => Err(DryadError::new(
                3006,
                &format!("Operador unário '{}' desconhecido", operator),
            )),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(_)
            | Value::Tuple(_)
            | Value::Lambda(_)
            | Value::Class(_)
            | Value::Instance(_)
            | Value::Object(_) => true,
            Value::Exception(_) => false,
            Value::Function { .. } | Value::AsyncFunction { .. } | Value::ThreadFunction { .. } => {
                true
            }
            Value::Thread { is_running, .. } => *is_running,
            Value::Mutex { .. } => true,
            Value::Promise { resolved, .. } => *resolved,
        }
    }

    fn eval_post_increment(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para x++: retorna o valor atual e depois incrementa
        if let Expr::Variable(name, _) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    // Incrementa a variável
                    self.env
                        .variables
                        .insert(name.clone(), Value::Number(n + 1.0));
                    // Retorna o valor original
                    Ok(Value::Number(n))
                }
                _ => Err(DryadError::new(
                    3007,
                    "Operador ++ só é válido para números",
                )),
            }
        } else {
            Err(DryadError::new(
                3008,
                "Operador ++ só pode ser aplicado a variáveis",
            ))
        }
    }

    fn eval_post_decrement(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para x--: retorna o valor atual e depois decrementa
        if let Expr::Variable(name, _) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    // Decrementa a variável
                    self.env
                        .variables
                        .insert(name.clone(), Value::Number(n - 1.0));
                    // Retorna o valor original
                    Ok(Value::Number(n))
                }
                _ => Err(DryadError::new(
                    3009,
                    "Operador -- só é válido para números",
                )),
            }
        } else {
            Err(DryadError::new(
                3010,
                "Operador -- só pode ser aplicado a variáveis",
            ))
        }
    }

    fn eval_pre_increment(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para ++x: incrementa primeiro e retorna o novo valor
        if let Expr::Variable(name, _) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    let new_value = n + 1.0;
                    // Incrementa a variável
                    self.env
                        .variables
                        .insert(name.clone(), Value::Number(new_value));
                    // Retorna o novo valor
                    Ok(Value::Number(new_value))
                }
                _ => Err(DryadError::new(
                    3011,
                    "Operador ++ só é válido para números",
                )),
            }
        } else {
            Err(DryadError::new(
                3012,
                "Operador ++ só pode ser aplicado a variáveis",
            ))
        }
    }

    fn eval_pre_decrement(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para --x: decrementa primeiro e retorna o novo valor
        if let Expr::Variable(name, _) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    let new_value = n - 1.0;
                    // Decrementa a variável
                    self.env
                        .variables
                        .insert(name.clone(), Value::Number(new_value));
                    // Retorna o novo valor
                    Ok(Value::Number(new_value))
                }
                _ => Err(DryadError::new(
                    3013,
                    "Operador -- só é válido para números",
                )),
            }
        } else {
            Err(DryadError::new(
                3014,
                "Operador -- só pode ser aplicado a variáveis",
            ))
        }
    }

    fn modulo_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(DryadError::new(3015, "Divisão por zero no operador %"))
                } else {
                    Ok(Value::Number(a % b))
                }
            }
            _ => Err(DryadError::new(
                3016,
                "Operação '%' só é válida para números",
            )),
        }
    }

    fn power_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(b))),
            _ => Err(DryadError::new(
                3017,
                "Operação '**' só é válida para números",
            )),
        }
    }

    fn nth_root_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(DryadError::new(3020, "Raiz de índice zero não é válida"))
                } else {
                    // n-ésima raiz: a ^^ b = a^(1/b)
                    Ok(Value::Number(a.powf(1.0 / b)))
                }
            }
            _ => Err(DryadError::new(
                3021,
                "Operação '^^' só é válida para números",
            )),
        }
    }

    fn safe_modulo_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(DryadError::new(3022, "Divisão por zero no operador %%"))
                } else {
                    // Módulo seguro: sempre retorna valor positivo
                    let result = a % b.abs();
                    if result < 0.0 {
                        Ok(Value::Number(result + b.abs()))
                    } else {
                        Ok(Value::Number(result))
                    }
                }
            }
            _ => Err(DryadError::new(
                3023,
                "Operação '%%' só é válida para números",
            )),
        }
    }

    fn power_of_ten_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                // a ## b = a * 10^b
                Ok(Value::Number(a * 10.0_f64.powf(b)))
            }
            _ => Err(DryadError::new(
                3024,
                "Operação '##' só é válida para números",
            )),
        }
    }

    fn bitwise_and_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                let a_int = a as i64;
                let b_int = b as i64;
                Ok(Value::Number((a_int & b_int) as f64))
            }
            _ => Err(DryadError::new(
                3026,
                "Operação '&' só é válida para números",
            )),
        }
    }

    fn bitwise_or_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                let a_int = a as i64;
                let b_int = b as i64;
                Ok(Value::Number((a_int | b_int) as f64))
            }
            _ => Err(DryadError::new(
                3027,
                "Operação '|' só é válida para números",
            )),
        }
    }

    fn bitwise_xor_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                let a_int = a as i64;
                let b_int = b as i64;
                Ok(Value::Number((a_int ^ b_int) as f64))
            }
            _ => Err(DryadError::new(
                3028,
                "Operação '^' só é válida para números",
            )),
        }
    }

    fn left_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(
                        3029,
                        "Não é possível fazer shift com número negativo",
                    ))
                } else {
                    // Left shift: a << b = a * 2^b
                    let result = a * 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(
                3030,
                "Operação '<<' só é válida para números",
            )),
        }
    }

    fn right_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(
                        3031,
                        "Não é possível fazer shift com número negativo",
                    ))
                } else {
                    // Right shift: a >> b = a / 2^b
                    let result = a / 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(
                3032,
                "Operação '>>' só é válida para números",
            )),
        }
    }

    fn symmetric_left_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(
                        3033,
                        "Não é possível fazer shift com número negativo",
                    ))
                } else {
                    // Symmetric left shift: a <<< b = a * 2^b (igual ao left shift padrão)
                    let result = a * 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(
                3034,
                "Operação '<<<' só é válida para números",
            )),
        }
    }

    fn symmetric_right_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(
                        3035,
                        "Não é possível fazer shift com número negativo",
                    ))
                } else {
                    // Symmetric right shift: a >>> b = a / 2^b (igual ao right shift padrão)
                    let result = a / 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(
                3036,
                "Operação '>>>' só é válida para números",
            )),
        }
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<Value, DryadError> {
        // Backup das variáveis atuais para implementar escopo de bloco
        let backup_variables = self.env.variables.clone();

        // Track das variáveis declaradas no bloco (para shadow)
        let mut declared_in_block = std::collections::HashSet::new();

        let mut last_value = Value::Null;

        // Execute todas as declarações no bloco
        for stmt in statements {
            // Se é uma VarDeclaration, marca como declarada no bloco
            if let Stmt::VarDeclaration(name, _, _, _) = stmt {
                if let Some(var_name) = name.identifier_name() {
                    declared_in_block.insert(var_name.clone());
                }
            }
            last_value = self.execute_statement(stmt)?;
        }

        // Implementa escopo correto:
        // 1. Remove variáveis declaradas no bloco (shadow)
        // 2. Restaura variáveis que existiam antes e foram shadowed
        // 3. Mantém modificações de variáveis que já existiam (assignments)
        for var_name in declared_in_block {
            // Remove a variável declarada no bloco
            self.env.variables.remove(&var_name);

            // Se existia uma variável com o mesmo nome antes, restaura
            if let Some(original_value) = backup_variables.get(&var_name) {
                self.env.variables.insert(var_name, original_value.clone());
            }
        }

        Ok(last_value)
    }

    fn execute_for_loop(
        &mut self,
        init: &Option<Box<Stmt>>,
        condition: &Option<Expr>,
        update: &Option<Box<Stmt>>,
        body: &Box<Stmt>,
    ) -> Result<Value, DryadError> {
        // Executa inicialização se presente
        if let Some(init_stmt) = init {
            self.execute_statement(init_stmt)?;
        }

        let mut last_value = Value::Null;

        loop {
            // Verifica condição se presente
            if let Some(condition_expr) = condition {
                let condition_value = self.evaluate(condition_expr)?;
                if !self.is_truthy(&condition_value) {
                    break;
                }
            }

            // Executa corpo do loop
            match self.execute_statement(body) {
                Ok(value) => last_value = value,
                Err(err) if err.code() == 3010 => {
                    // Break statement
                    break;
                }
                Err(err) if err.code() == 3011 => {
                    // Continue statement - pula para update
                }
                Err(e) => return Err(e),
            }

            // Executa update se presente
            if let Some(update_stmt) = update {
                self.execute_statement(update_stmt)?;
            }
        }

        Ok(last_value)
    }

    fn execute_foreach_loop(
        &mut self,
        pattern: &Pattern,
        iterable: &Expr,
        body: &Box<Stmt>,
    ) -> Result<Value, DryadError> {
        // Extrai o nome da variável do pattern
        let var_name = match pattern {
            Pattern::Identifier(name) => name.as_str(),
            _ => {
                return Err(DryadError::new(
                    3101,
                    "ForEach loop only supports identifier patterns",
                ))
            }
        };

        // Avalia a expressão iterável
        let iterable_value = self.evaluate(iterable)?;

        // Salva o valor anterior da variável de iteração (se existir)
        let previous_value = self.env.variables.get(var_name).cloned();

        let mut last_value = Value::Null;

        // Itera sobre os elementos dependendo do tipo
        match iterable_value {
            Value::Array(id) => {
                let elements = if let Some(ManagedObject::Array(e)) = self.heap.get(id) {
                    e.clone()
                } else {
                    return Err(DryadError::new(3101, "Heap error: Expected Array"));
                };

                for element in elements {
                    // Define a variável de iteração
                    self.env.variables.insert(var_name.to_string(), element);

                    // Executa o corpo do loop
                    match self.execute_statement(body) {
                        Ok(value) => last_value = value,
                        Err(err) if err.code() == 3010 => {
                            // Break statement
                            break;
                        }
                        Err(err) if err.code() == 3011 => {
                            // Continue statement - continua para próximo elemento
                            continue;
                        }
                        Err(e) => {
                            // Restaura valor anterior antes de retornar erro
                            if let Some(prev_val) = previous_value {
                                self.env.variables.insert(var_name.to_string(), prev_val);
                            } else {
                                self.env.variables.remove(var_name);
                            }
                            return Err(e);
                        }
                    }
                }
            }
            Value::Tuple(id) => {
                let elements = if let Some(ManagedObject::Tuple(e)) = self.heap.get(id) {
                    e.clone()
                } else {
                    return Err(DryadError::new(3101, "Heap error: Expected Tuple"));
                };

                for element in elements {
                    // Define a variável de iteração
                    self.env.variables.insert(var_name.to_string(), element);

                    // Executa o corpo do loop
                    match self.execute_statement(body) {
                        Ok(value) => last_value = value,
                        Err(err) if err.code() == 3010 => {
                            // Break statement
                            break;
                        }
                        Err(err) if err.code() == 3011 => {
                            // Continue statement - continua para próximo elemento
                            continue;
                        }
                        Err(e) => {
                            // Restaura valor anterior antes de retornar erro
                            if let Some(prev_val) = previous_value {
                                self.env.variables.insert(var_name.to_string(), prev_val);
                            } else {
                                self.env.variables.remove(var_name);
                            }
                            return Err(e);
                        }
                    }
                }
            }
            Value::String(s) => {
                // Itera sobre caracteres da string
                for char in s.chars() {
                    let char_value = Value::String(char.to_string());
                    self.env.variables.insert(var_name.to_string(), char_value);

                    // Executa o corpo do loop
                    match self.execute_statement(body) {
                        Ok(value) => last_value = value,
                        Err(err) if err.code() == 3010 => {
                            // Break statement
                            break;
                        }
                        Err(err) if err.code() == 3011 => {
                            // Continue statement - continua para próximo caractere
                            continue;
                        }
                        Err(e) => {
                            // Restaura valor anterior antes de retornar erro
                            if let Some(prev_val) = previous_value {
                                self.env.variables.insert(var_name.to_string(), prev_val);
                            } else {
                                self.env.variables.remove(var_name);
                            }
                            return Err(e);
                        }
                    }
                }
            }
            Value::Number(_)
            | Value::Bool(_)
            | Value::Null
            | Value::Exception(_)
            | Value::Function { .. }
            | Value::AsyncFunction { .. }
            | Value::ThreadFunction { .. }
            | Value::Lambda(_)
            | Value::Thread { .. }
            | Value::Mutex { .. }
            | Value::Promise { .. }
            | Value::Class(_)
            | Value::Instance(_)
            | Value::Object(_) => {
                return Err(DryadError::new(
                    3030,
                    &format!("Valor não é iterável: {}", iterable_value.to_string()),
                ));
            }
        }

        // Restaura o valor anterior da variável (se existia)
        if let Some(prev_val) = previous_value {
            self.env.variables.insert(var_name.to_string(), prev_val);
        } else {
            self.env.variables.remove(var_name);
        }

        Ok(last_value)
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.env.variables.get(name).cloned()
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.env.variables.insert(name, value);
    }

    fn eval_array(&mut self, elements: &[Expr]) -> Result<Value, DryadError> {
        let mut values = Vec::new();

        for element in elements {
            let value = self.evaluate(element)?;
            values.push(value);
        }

        let array_id = self.heap.allocate(ManagedObject::Array(values));
        self.maybe_collect_garbage();
        Ok(Value::Array(array_id))
    }

    fn eval_tuple(&mut self, elements: &[Expr]) -> Result<Value, DryadError> {
        let mut values = Vec::new();

        for element in elements {
            let value = self.evaluate(element)?;
            values.push(value);
        }

        let tuple_id = self.heap.allocate(ManagedObject::Tuple(values));
        self.maybe_collect_garbage();
        Ok(Value::Tuple(tuple_id))
    }

    fn eval_index(&mut self, array_expr: &Expr, index_expr: &Expr) -> Result<Value, DryadError> {
        let array_value = self.evaluate(array_expr)?;
        let index_value = self.evaluate(index_expr)?;

        match array_value {
            Value::Array(id) => {
                let heap_obj = self.heap.get(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Array reference not found")
                })?;

                if let ManagedObject::Array(elements) = heap_obj {
                    // Array access requires numeric index
                    let index = match index_value {
                        Value::Number(n) => {
                            if n < 0.0 || n.fract() != 0.0 {
                                return Err(DryadError::new(
                                    3080,
                                    "Índice deve ser um número inteiro não negativo",
                                ));
                            }
                            n as usize
                        }
                        _ => {
                            return Err(DryadError::new(3081, "Índice de array deve ser um número"))
                        }
                    };

                    if index >= elements.len() {
                        return Err(DryadError::new(
                            3082,
                            &format!(
                                "Índice {} fora dos limites do array (tamanho: {})",
                                index,
                                elements.len()
                            ),
                        ));
                    }
                    Ok(elements[index].clone())
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Array"))
                }
            }
            Value::Object(id) => {
                let heap_obj = self.heap.get(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Object reference not found")
                })?;

                if let ManagedObject::Object { properties, .. } = heap_obj {
                    // Object access supports string keys (HashMap-like behavior)
                    let key = match index_value {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        _ => {
                            return Err(DryadError::new(
                                3084,
                                "Chave do objeto deve ser string ou número",
                            ))
                        }
                    };

                    match properties.get(&key) {
                        Some(value) => Ok(value.clone()),
                        None => Ok(Value::Null), // Return null for non-existent keys (like JavaScript)
                    }
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Object"))
                }
            }
            Value::Number(_)
            | Value::Bool(_)
            | Value::String(_)
            | Value::Null
            | Value::Tuple(_)
            | Value::Exception(_)
            | Value::Function { .. }
            | Value::AsyncFunction { .. }
            | Value::ThreadFunction { .. }
            | Value::Lambda { .. }
            | Value::Thread { .. }
            | Value::Mutex { .. }
            | Value::Promise { .. }
            | Value::Class { .. }
            | Value::Instance { .. } => Err(DryadError::new(
                3083,
                "Operador [] só pode ser usado em arrays e objetos",
            )),
        }
    }

    fn eval_tuple_access(&mut self, tuple_expr: &Expr, index: usize) -> Result<Value, DryadError> {
        let tuple_value = self.evaluate(tuple_expr)?;

        match tuple_value {
            Value::Tuple(id) => {
                let heap_obj = self.heap.get(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Tuple reference not found")
                })?;

                if let ManagedObject::Tuple(elements) = heap_obj {
                    if index >= elements.len() {
                        return Err(DryadError::new(
                            3084,
                            &format!(
                                "Índice {} fora dos limites da tupla (tamanho: {})",
                                index,
                                elements.len()
                            ),
                        ));
                    }
                    Ok(elements[index].clone())
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Tuple"))
                }
            }
            Value::Number(_)
            | Value::Bool(_)
            | Value::String(_)
            | Value::Null
            | Value::Array(_)
            | Value::Exception(_)
            | Value::Function { .. }
            | Value::AsyncFunction { .. }
            | Value::ThreadFunction { .. }
            | Value::Lambda { .. }
            | Value::Thread { .. }
            | Value::Mutex { .. }
            | Value::Promise { .. }
            | Value::Class { .. }
            | Value::Instance { .. }
            | Value::Object { .. } => Err(DryadError::new(
                3085,
                "Operador . só pode ser usado em tuplas",
            )),
        }
    }

    fn execute_try_catch_finally(
        &mut self,
        try_block: &Box<Stmt>,
        catch_clause: &Option<(String, Box<Stmt>)>,
        finally_block: &Option<Box<Stmt>>,
    ) -> Result<Value, DryadError> {
        let mut last_value = Value::Null;
        let mut exception_occurred = false;
        let mut caught_exception = None;

        // Execute try block
        match self.execute_statement(try_block) {
            Ok(value) => {
                last_value = value;
            }
            Err(err) => {
                exception_occurred = true;
                caught_exception = Some(err);
            }
        }

        // Execute catch block if exception occurred and catch clause exists
        if exception_occurred && catch_clause.is_some() {
            let (catch_var, catch_block) = catch_clause.as_ref().unwrap();
            let exception = caught_exception.as_ref().unwrap();

            // Store exception message in catch variable
            let exception_value = Value::Exception(exception.message().to_string());
            let old_value = self.env.variables.get(catch_var).cloned();
            self.env
                .variables
                .insert(catch_var.clone(), exception_value);

            // Execute catch block
            match self.execute_statement(catch_block) {
                Ok(value) => {
                    last_value = value;
                    exception_occurred = false; // Exception was handled
                    caught_exception = None;
                }
                Err(catch_err) => {
                    // New exception in catch block
                    caught_exception = Some(catch_err);
                }
            }

            // Restore old variable value or remove if it didn't exist
            match old_value {
                Some(old_val) => {
                    self.env.variables.insert(catch_var.clone(), old_val);
                }
                None => {
                    self.env.variables.remove(catch_var);
                }
            }
        }

        // Always execute finally block if it exists
        if let Some(finally_stmt) = finally_block {
            match self.execute_statement(finally_stmt) {
                Ok(value) => {
                    // If no exception is pending, use finally's value
                    if !exception_occurred {
                        last_value = value;
                    }
                }
                Err(finally_err) => {
                    // Finally block exception overrides any previous exception
                    return Err(finally_err);
                }
            }
        }

        // If exception wasn't caught, re-throw it
        if exception_occurred {
            if let Some(exception) = caught_exception {
                return Err(exception);
            }
        }

        Ok(last_value)
    }

    fn eval_method_call(
        &mut self,
        object_expr: &Expr,
        method_name: &str,
        args: &[Expr],
    ) -> Result<Value, DryadError> {
        self.call_depth += 1;
        if self.call_depth > MAX_RECURSION_DEPTH {
            self.call_depth -= 1;
            return Err(self.runtime_error(
                3040,
                "Stack overflow: limite de recursão excedido em chamada de método",
            ));
        }

        // Extract location from the object expression
        let location = match object_expr {
            Expr::Variable(_, loc)
            | Expr::Literal(_, loc)
            | Expr::Binary(_, _, _, loc)
            | Expr::Unary(_, _, loc)
            | Expr::Call(_, _, loc)
            | Expr::MethodCall(_, _, _, loc)
            | Expr::PropertyAccess(_, _, loc)
            | Expr::Index(_, _, loc)
            | Expr::Array(_, loc)
            | Expr::Tuple(_, loc)
            | Expr::TupleAccess(_, _, loc)
            | Expr::Lambda { location: loc, .. }
            | Expr::ObjectLiteral(_, loc)
            | Expr::PostIncrement(_, loc)
            | Expr::PostDecrement(_, loc)
            | Expr::PreIncrement(_, loc)
            | Expr::PreDecrement(_, loc)
            | Expr::ClassInstantiation(_, _, loc)
            | Expr::Await(_, loc)
            | Expr::ThreadCall(_, _, loc)
            | Expr::MutexCreation(loc)
            | Expr::This(loc)
            | Expr::Super(loc)
            | Expr::Match(_, _, loc) => loc,
        };
        let result = self.eval_method_call_internal(object_expr, method_name, args, location);
        self.call_depth -= 1;
        result
    }

    fn eval_method_call_internal(
        &mut self,
        object_expr: &Expr,
        method_name: &str,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        let object = self.evaluate(object_expr)?;

        match object {
            Value::Array(_) => self.eval_array_method(object_expr, method_name, args, location),
            Value::Class(id) => {
                let heap_obj = self.heap.get(id).cloned().ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Class reference not found")
                })?;

                if let ManagedObject::Class {
                    name: class_name,
                    methods,
                    ..
                } = heap_obj
                {
                    // Static method call on a class
                    if let Some(method) = methods.get(method_name) {
                        // Check if method is static
                        if !method.is_static {
                            return Err(DryadError::new(
                                3024,
                                &format!("Método '{}' não é estático", method_name),
                            ));
                        }

                        // Check visibility
                        match method.visibility {
                            Visibility::Private => {
                                return Err(DryadError::new(
                                    3024,
                                    &format!("Método '{}' é privado", method_name),
                                ));
                            }
                            _ => {}
                        }

                        // Evaluate arguments
                        let mut arg_values = Vec::new();
                        for arg in args {
                            arg_values.push(self.evaluate(arg)?);
                        }

                        if arg_values.len() != method.params.len() {
                            return Err(DryadError::new(
                                3025,
                                &format!(
                                    "Método '{}' espera {} argumentos, mas recebeu {}",
                                    method_name,
                                    method.params.len(),
                                    arg_values.len()
                                ),
                            ));
                        }

                        let saved_vars = self.env.variables.clone();
                        let saved_instance = self.env.current_instance.clone();
                        self.env.current_instance = None;

                        for (param, value) in method.params.iter().zip(arg_values.iter()) {
                            self.env.variables.insert(param.clone(), value.clone());
                        }

                        let result = match self.execute_statement(&method.body) {
                            Ok(value) => Ok(value),
                            Err(e) => {
                                if e.code() == 3021 {
                                    self.parse_return_value(e.message())
                                } else {
                                    Err(e)
                                }
                            }
                        };

                        self.env.variables = saved_vars;
                        self.env.current_instance = saved_instance;
                        result
                    } else {
                        Err(DryadError::new(
                            3026,
                            &format!(
                                "Método estático '{}' não encontrado na classe '{}'",
                                method_name, class_name
                            ),
                        ))
                    }
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Class"))
                }
            }
            Value::Instance(id) => {
                let heap_obj = self.heap.get(id).cloned().ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Instance reference not found")
                })?;

                if let ManagedObject::Instance {
                    class_name,
                    properties,
                } = heap_obj
                {
                    let class_name = class_name.clone();

                    if let Some(Value::Class(cid)) = self.env.classes.get(&class_name).cloned() {
                        let class_obj = self.heap.get(cid).cloned().ok_or_else(|| {
                            DryadError::new(3100, "Heap error: Inconsistent class reference")
                        })?;

                        if let ManagedObject::Class { methods, .. } = class_obj {
                            if let Some(method) = methods.get(method_name) {
                                match method.visibility {
                                    Visibility::Private => {
                                        return Err(DryadError::new(
                                            3024,
                                            &format!("Método '{}' é privado", method_name),
                                        ));
                                    }
                                    _ => {}
                                }

                                let mut arg_values = Vec::new();
                                for arg in args {
                                    arg_values.push(self.evaluate(arg)?);
                                }

                                if arg_values.len() != method.params.len() {
                                    return Err(DryadError::new(
                                        3025,
                                        &format!(
                                            "Método '{}' espera {} argumentos, mas recebeu {}",
                                            method_name,
                                            method.params.len(),
                                            arg_values.len()
                                        ),
                                    ));
                                }

                                let saved_vars = self.env.variables.clone();
                                let saved_instance = self.env.current_instance.clone();

                                self.env.current_instance = Some(Value::Instance(id));

                                for (param, value) in method.params.iter().zip(arg_values.iter()) {
                                    self.env.variables.insert(param.clone(), value.clone());
                                }

                                let result = match self.execute_statement(&method.body) {
                                    Ok(value) => Ok(value),
                                    Err(e) => {
                                        if e.code() == 3021 {
                                            self.parse_return_value(e.message())
                                        } else {
                                            Err(e)
                                        }
                                    }
                                };

                                self.env.variables = saved_vars;
                                self.env.current_instance = saved_instance;
                                result
                            } else {
                                Err(DryadError::new(
                                    3026,
                                    &format!(
                                        "Método '{}' não encontrado na classe '{}'",
                                        method_name, class_name
                                    ),
                                ))
                            }
                        } else {
                            Err(DryadError::new(
                                3101,
                                "Heap error: Expected Class definition",
                            ))
                        }
                    } else {
                        Err(DryadError::new(
                            3027,
                            &format!("Definição da classe '{}' não encontrada", class_name),
                        ))
                    }
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Instance"))
                }
            }
            Value::Object(id) => {
                let heap_obj = self.heap.get(id).cloned().ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Object reference not found")
                })?;

                if let ManagedObject::Object {
                    properties,
                    methods,
                } = heap_obj
                {
                    if let Some(method) = methods.get(method_name) {
                        let method = method.clone();
                        let properties = properties.clone();
                        let methods = methods.clone();

                        let mut arg_values = Vec::new();
                        for arg in args {
                            arg_values.push(self.evaluate(arg)?);
                        }

                        if arg_values.len() != method.params.len() {
                            return Err(DryadError::new(
                                3025,
                                &format!(
                                    "Método '{}' espera {} argumentos, mas recebeu {}",
                                    method_name,
                                    method.params.len(),
                                    arg_values.len()
                                ),
                            ));
                        }

                        let saved_vars = self.env.variables.clone();
                        let saved_instance = self.env.current_instance.clone();

                        self.env.current_instance = Some(Value::Object(id));

                        for (param, value) in method.params.iter().zip(arg_values.iter()) {
                            self.env.variables.insert(param.clone(), value.clone());
                        }

                        let result = match self.execute_statement(&method.body) {
                            Ok(value) => Ok(value),
                            Err(e) => {
                                if e.code() == 3021 {
                                    self.parse_return_value(e.message())
                                } else {
                                    Err(e)
                                }
                            }
                        };

                        self.env.variables = saved_vars;
                        self.env.current_instance = saved_instance;
                        result
                    } else if let Some(func_value) = properties.get(method_name) {
                        match func_value {
                            Value::Function { params, body, .. } => {
                                let mut arg_values = Vec::new();
                                for arg in args {
                                    arg_values.push(self.evaluate(arg)?);
                                }

                                if arg_values.len() != params.len() {
                                    return Err(DryadError::new(
                                        3025,
                                        &format!(
                                            "Função '{}' espera {} argumentos, mas recebeu {}",
                                            method_name,
                                            params.len(),
                                            arg_values.len()
                                        ),
                                    ));
                                }

                                let saved_vars = self.env.variables.clone();

                                for (param, value) in params.iter().zip(arg_values.iter()) {
                                    self.env.variables.insert(param.clone(), value.clone());
                                }

                                let result = match self.execute_statement(body) {
                                    Ok(value) => Ok(value),
                                    Err(e) => {
                                        if e.code() == 3021 {
                                            self.parse_return_value(e.message())
                                        } else {
                                            Err(e)
                                        }
                                    }
                                };

                                self.env.variables = saved_vars;
                                result
                            }
                            _ => Err(DryadError::new(
                                3026,
                                &format!("Propriedade '{}' não é uma função", method_name),
                            )),
                        }
                    } else {
                        Err(DryadError::new(
                            3026,
                            &format!("Método '{}' não encontrado no objeto", method_name),
                        ))
                    }
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Object"))
                }
            }
            _ => Err(DryadError::new(
                3028,
                "Tentativa de chamar método em valor que não é uma instância ou objeto",
            )),
        }
    }

    fn eval_property_access(
        &mut self,
        object_expr: &Expr,
        property_name: &str,
    ) -> Result<Value, DryadError> {
        let object = self.evaluate(object_expr)?;

        match object {
            Value::Class(id) => {
                let heap_obj = self.heap.get(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Class reference not found")
                })?;

                if let ManagedObject::Class {
                    name: class_name,
                    properties: class_props,
                    ..
                } = heap_obj
                {
                    // Static property access on a class
                    if let Some(class_prop) = class_props.get(property_name) {
                        // Check if property is static
                        if !class_prop.is_static {
                            return Err(DryadError::new(
                                3029,
                                &format!("Propriedade '{}' não é estática", property_name),
                            ));
                        }

                        // Check visibility (simplified - public only for now)
                        match class_prop.visibility {
                            Visibility::Private => {
                                return Err(DryadError::new(
                                    3029,
                                    &format!("Propriedade '{}' é privada", property_name),
                                ));
                            }
                            _ => {
                                if let Some(default_value) = &class_prop.default_value {
                                    return Ok(default_value.clone());
                                } else {
                                    return Ok(Value::Null);
                                }
                            }
                        }
                    } else {
                        Err(DryadError::new(
                            3030,
                            &format!(
                                "Propriedade estática '{}' não encontrada na classe '{}'",
                                property_name, class_name
                            ),
                        ))
                    }
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Class"))
                }
            }
            Value::Instance(id) => {
                let heap_obj = self.heap.get(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Instance reference not found")
                })?;

                if let ManagedObject::Instance {
                    class_name,
                    properties,
                } = heap_obj
                {
                    // First check instance properties
                    if let Some(value) = properties.get(property_name) {
                        return Ok(value.clone());
                    }

                    // Then check class properties and getters
                    let (class_props, getter_to_run) =
                        if let Some(Value::Class(cid)) = self.env.classes.get(class_name) {
                            if let Some(class_obj) = self.heap.get(*cid) {
                                if let ManagedObject::Class {
                                    properties: class_props,
                                    getters,
                                    ..
                                } = class_obj
                                {
                                    (
                                        Some(class_props.clone()),
                                        getters.get(property_name).cloned(),
                                    )
                                } else {
                                    (None, None)
                                }
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        };

                    // Check for getter first
                    if let Some(getter) = getter_to_run {
                        match getter.visibility {
                            Visibility::Private => {
                                return Err(DryadError::new(
                                    3029,
                                    &format!("Getter '{}' é privado", property_name),
                                ));
                            }
                            _ => {
                                // Execute getter with 'this' bound to instance
                                let mut getter_env = self.env.clone();
                                getter_env
                                    .variables
                                    .insert("this".to_string(), object.clone());
                                let prev_env = std::mem::replace(&mut self.env, getter_env);
                                let result = self.execute_statement(&getter.body);
                                self.env = prev_env;
                                return result;
                            }
                        }
                    }

                    // Then check class properties
                    if let Some(class_props) = class_props {
                        if let Some(class_prop) = class_props.get(property_name) {
                            match class_prop.visibility {
                                Visibility::Private => {
                                    return Err(DryadError::new(
                                        3029,
                                        &format!("Propriedade '{}' é privada", property_name),
                                    ));
                                }
                                _ => {
                                    if let Some(default_value) = &class_prop.default_value {
                                        return Ok(default_value.clone());
                                    } else {
                                        return Ok(Value::Null);
                                    }
                                }
                            }
                        }
                    }
                }

                Err(DryadError::new(
                    3030,
                    &format!("Propriedade '{}' não encontrada", property_name),
                ))
            }
            Value::Object(id) => {
                let heap_obj = self.heap.get(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Object reference not found")
                })?;

                if let ManagedObject::Object { properties, .. } = heap_obj {
                    // Check object literal properties
                    if let Some(value) = properties.get(property_name) {
                        Ok(value.clone())
                    } else {
                        Err(DryadError::new(
                            3030,
                            &format!("Propriedade '{}' não encontrada", property_name),
                        ))
                    }
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Object"))
                }
            }
            _ => Err(DryadError::new(
                3031,
                "Tentativa de acessar propriedade em valor que não é uma instância ou objeto",
            )),
        }
    }

    fn eval_class_instantiation(
        &mut self,
        class_name: &str,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        self.call_depth += 1;
        if self.call_depth > MAX_RECURSION_DEPTH {
            self.call_depth -= 1;
            return Err(self.runtime_error(
                3040,
                "Stack overflow: limite de recursão excedido em instanciação de classe",
            ));
        }

        let result = self.eval_class_instantiation_internal(class_name, args, location);
        self.call_depth -= 1;
        result
    }

    fn eval_class_instantiation_internal(
        &mut self,
        class_name: &str,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        // Check if it's a class call or regular function call
        if let Some(Value::Class(id)) = self.env.classes.get(class_name).cloned() {
            let class_obj = self
                .heap
                .get(id)
                .ok_or_else(|| DryadError::new(3100, "Heap error: Class reference not found"))?;

            if let ManagedObject::Class {
                methods,
                properties,
                ..
            } = class_obj
            {
                let methods = methods.clone();
                let properties = properties.clone();

                // It's a class instantiation
                let mut instance_properties = HashMap::new();

                // Initialize properties with default values
                for (prop_name, class_prop) in &properties {
                    if !class_prop.is_static {
                        if let Some(default_value) = &class_prop.default_value {
                            instance_properties.insert(prop_name.clone(), default_value.clone());
                        } else {
                            instance_properties.insert(prop_name.clone(), Value::Null);
                        }
                    }
                }

                let instance_id = self.heap.allocate(ManagedObject::Instance {
                    class_name: class_name.to_string(),
                    properties: instance_properties,
                });
                self.maybe_collect_garbage();
                let instance = Value::Instance(instance_id);

                // Call init method if it exists
                if let Some(init_method) = methods.get("init") {
                    // Evaluate arguments
                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(self.evaluate(arg)?);
                    }

                    // Check parameter count
                    if arg_values.len() != init_method.params.len() {
                        return Err(DryadError::new(
                            3032,
                            &format!(
                                "Construtor da classe '{}' espera {} argumentos, mas recebeu {}",
                                class_name,
                                init_method.params.len(),
                                arg_values.len()
                            ),
                        ));
                    }

                    // Save current state
                    self.env.call_stack_vars.push(self.env.variables.clone());
                    let saved_instance = self.env.current_instance.clone();

                    // Set up constructor context
                    self.env.current_instance = Some(instance.clone());

                    // Bind parameters
                    for (param, value) in init_method.params.iter().zip(arg_values.iter()) {
                        self.env.variables.insert(param.clone(), value.clone());
                    }

                    // Execute constructor
                    let _ = match self.execute_statement(&init_method.body) {
                        Ok(_) => {}
                        Err(e) => {
                            // Check if it's a return (constructors shouldn't return values, but handle it gracefully)
                            if e.code() != 3021 {
                                // Restore state before returning error
                                if let Some(saved) = self.env.call_stack_vars.pop() {
                                    self.env.variables = saved;
                                }
                                self.env.current_instance = saved_instance;
                                return Err(e);
                            }
                        }
                    };

                    // Restore state
                    if let Some(saved) = self.env.call_stack_vars.pop() {
                        self.env.variables = saved;
                    }
                    self.env.current_instance = saved_instance;
                } else if !args.is_empty() {
                    return Err(DryadError::new(
                        3033,
                        &format!(
                        "Classe '{}' não tem construtor 'init', mas argumentos foram fornecidos",
                        class_name
                    ),
                    ));
                }

                Ok(instance)
            } else {
                Err(DryadError::new(
                    3101,
                    "Heap error: Expected Class definition",
                ))
            }
        } else {
            // Not a class, treat as regular function call
            self.eval_call_by_name(class_name, args, location)
        }
    }

    fn parse_return_value(&self, error_message: &str) -> Result<Value, DryadError> {
        if error_message.starts_with("RETURN_NUMBER:") {
            let number_str = &error_message[14..];
            if let Ok(n) = number_str.parse::<f64>() {
                return Ok(Value::Number(n));
            }
        } else if error_message.starts_with("RETURN_STRING:") {
            let string_val = &error_message[14..];
            return Ok(Value::String(string_val.to_string()));
        } else if error_message.starts_with("RETURN_BOOL:") {
            let bool_str = &error_message[12..];
            if let Ok(b) = bool_str.parse::<bool>() {
                return Ok(Value::Bool(b));
            }
        } else if error_message == "RETURN_NULL" {
            return Ok(Value::Null);
        } else if error_message.starts_with("RETURN_OTHER:") {
            let value_str = &error_message[13..];
            return Ok(Value::String(value_str.to_string()));
        }

        // Se não conseguiu fazer parse do return, retorna o erro original
        Err(DryadError::new(
            3035,
            &format!("Erro ao processar return: {}", error_message),
        ))
    }

    fn eval_match(
        &mut self,
        target: &Expr,
        arms: &[MatchArm],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        let value = self.evaluate(target)?;

        for arm in arms {
            let mut bindings = HashMap::new();
            if self.match_pattern(&value, &arm.pattern, &mut bindings) {
                // Check guard if present
                let mut matches_guard = true;
                if let Some(guard) = &arm.guard {
                    // To evaluate the guard with the new bindings, we need to temporarily update our scope
                    let backup = self.env.variables.clone();
                    for (name, val) in &bindings {
                        self.env.variables.insert(name.clone(), val.clone());
                    }

                    let guard_result = self.evaluate(guard)?;
                    matches_guard = self.is_truthy(&guard_result);

                    self.env.variables = backup;
                }

                if matches_guard {
                    // Match confirmed! Execute body with bindings
                    let backup = self.env.variables.clone();
                    for (name, val) in bindings {
                        self.env.variables.insert(name, val);
                    }

                    let result = match &arm.body {
                        Stmt::Block(stmts, _) => self.execute_block(stmts),
                        _ => self.execute_statement(&arm.body),
                    };

                    self.env.variables = backup;
                    return result;
                }
            }
        }

        Err(DryadError::new(
            3100,
            &format!("Nenhum padrão corresponde ao valor: {}", value.to_string()),
        ))
    }

    fn match_pattern(
        &self,
        value: &Value,
        pattern: &Pattern,
        bindings: &mut HashMap<String, Value>,
    ) -> bool {
        match pattern {
            Pattern::Wildcard => true,
            Pattern::Identifier(name) => {
                bindings.insert(name.clone(), value.clone());
                true
            }
            Pattern::Literal(lit) => {
                let val = match lit {
                    Literal::Bool(b) => Value::Bool(*b),
                    Literal::Number(n) => Value::Number(*n),
                    Literal::String(s) => Value::String(s.clone()),
                    Literal::Null => Value::Null,
                };
                self.values_equal(value, &val)
            }
            Pattern::Array(patterns) => {
                if let Value::Array(id) = value {
                    if let Some(ManagedObject::Array(elements)) = self.heap.get(*id) {
                        if elements.len() != patterns.len() {
                            return false;
                        }
                        for (i, p) in patterns.iter().enumerate() {
                            if !self.match_pattern(&elements[i], p, bindings) {
                                return false;
                            }
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Pattern::Tuple(patterns) => {
                if let Value::Tuple(id) = value {
                    if let Some(ManagedObject::Tuple(elements)) = self.heap.get(*id) {
                        if elements.len() != patterns.len() {
                            return false;
                        }
                        for (i, p) in patterns.iter().enumerate() {
                            if !self.match_pattern(&elements[i], p, bindings) {
                                return false;
                            }
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Pattern::Object(patterns) => {
                if let Value::Object(id) = value {
                    if let Some(ManagedObject::Object { properties, .. }) = self.heap.get(*id) {
                        for (key, p) in patterns {
                            if let Some(val) = properties.get(key) {
                                if !self.match_pattern(val, p, bindings) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    fn eval_object_literal(&mut self, properties: &[ObjectProperty]) -> Result<Value, DryadError> {
        let mut object_properties = HashMap::new();
        let mut object_methods = HashMap::new();

        for property in properties {
            match property {
                ObjectProperty::Property(key, value_expr) => {
                    let value = self.evaluate(value_expr)?;
                    object_properties.insert(key.clone(), value);
                }
                ObjectProperty::Method {
                    name: key,
                    params,
                    body,
                    ..
                } => {
                    let params_vec: Vec<String> = params.iter().map(|(p, _)| p.clone()).collect();
                    let method = ObjectMethod {
                        params: params_vec,
                        body: *body.clone(),
                    };
                    object_methods.insert(key.clone(), method);
                }
            }
        }

        let obj_id = self.heap.allocate(ManagedObject::Object {
            properties: object_properties,
            methods: object_methods,
        });
        self.maybe_collect_garbage();
        Ok(Value::Object(obj_id))
    }

    fn eval_await(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        let value = self.evaluate(expr)?;
        match value {
            Value::Promise {
                id,
                resolved: true,
                value: Some(val),
            } => Ok(*val),
            Value::Promise {
                id,
                resolved: false,
                ..
            } => {
                // Tenta resolver se for uma promise nativa pendente
                if let Some(future) = self.pending_promises.remove(&id) {
                    // Executa o future sincronamente (bloqueando)
                    // Como o interpretador todo é síncrono, isso é aceitável por enquanto para integrar IO assíncrono
                    let handle = tokio::runtime::Handle::current();
                    let result = handle.block_on(future);

                    match result {
                        Ok(val) => Ok(val),
                        Err(e) => Err(DryadError::new(
                            3005,
                            &format!("Erro em operação assíncrona (Promise ID {}): {}", id, e),
                        )),
                    }
                } else {
                    Err(DryadError::new(4001, &format!("Promise (ID {}) ainda não foi resolvida e não é uma operação nativa pendente", id)))
                }
            }
            other_value => Ok(other_value), // Se não é uma promise, retorna o valor diretamente
        }
    }

    fn eval_thread_call(&mut self, func_expr: &Expr, args: &[Expr]) -> Result<Value, DryadError> {
        use std::thread;

        let function = self.evaluate(func_expr)?;
        let mut evaluated_args = Vec::new();

        for arg in args {
            evaluated_args.push(self.evaluate(arg)?);
        }

        match function {
            Value::Function { name, params, body }
            | Value::ThreadFunction { name, params, body } => {
                if params.len() != evaluated_args.len() {
                    return Err(DryadError::new(
                        4002,
                        &format!(
                            "Função '{}' espera {} argumentos, mas {} foram fornecidos",
                            name,
                            params.len(),
                            evaluated_args.len()
                        ),
                    ));
                }

                let thread_id = self.next_thread_id;
                self.next_thread_id += 1;

                // Cria um contexto isolado para a thread
                let mut thread_context = Self::new();

                // Passa os argumentos
                for (param, arg) in params.iter().zip(evaluated_args.iter()) {
                    thread_context
                        .env
                        .variables
                        .insert(param.clone(), arg.clone());
                }

                // Clona o body para mover para a thread
                let thread_body = body.clone();

                let handle = thread::spawn(move || -> Result<Value, DryadError> {
                    thread_context.execute_statement(&thread_body)
                });

                // Armazena o handle da thread
                self.threads.insert(thread_id, handle);

                Ok(Value::Thread {
                    id: thread_id,
                    is_running: true,
                })
            }
            _ => Err(DryadError::new(
                4003,
                "Expressão não é uma função válida para thread()",
            )),
        }
    }

    fn eval_mutex_creation(&mut self) -> Result<Value, DryadError> {
        use std::sync::{Arc, Mutex};

        let mutex_id = self.next_mutex_id;
        self.next_mutex_id += 1;

        let mutex = Arc::new(Mutex::new(()));
        self.mutexes.insert(mutex_id, mutex);

        Ok(Value::Mutex {
            id: mutex_id,
            locked: false,
        })
    }

    pub fn import_module(&mut self, module_path: &str) -> Result<Value, DryadError> {
        // 1. Resolver o caminho do módulo
        let resolved_path = self.resolve_module_path(module_path)?;

        // 2. Verificar se o módulo já foi importado
        if self
            .env
            .imported_modules
            .contains_key(&resolved_path.to_string_lossy().to_string())
        {
            return self.apply_imported_module(&resolved_path.to_string_lossy().to_string());
        }

        // 3. Ler o arquivo do módulo
        let source_code = fs::read_to_string(&resolved_path).map_err(|e| {
            DryadError::new(
                3001,
                &format!("Erro ao ler módulo '{}': {}", resolved_path.display(), e),
            )
        })?;

        // 4. Fazer lexing e parsing do módulo
        let mut lexer = dryad_lexer::lexer::Lexer::new(&source_code);
        let mut tokens = Vec::new();

        // Coletar todos os tokens
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    let is_eof = matches!(token.token, dryad_lexer::token::Token::Eof);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                Err(e) => {
                    return Err(DryadError::new(
                        3002,
                        &format!(
                            "Erro de lexing no módulo '{}': {:?}",
                            resolved_path.display(),
                            e
                        ),
                    ))
                }
            }
        }

        let mut parser = dryad_parser::parser::Parser::new(tokens);
        let program = parser.parse().map_err(|e| {
            DryadError::new(
                3003,
                &format!(
                    "Erro de parsing no módulo '{}': {:?}",
                    resolved_path.display(),
                    e
                ),
            )
        })?;

        // 5. Executar o módulo em um contexto separado e capturar exports
        let exported_symbols = self.execute_module_and_capture_exports(&program, &resolved_path)?;

        // 6. Armazenar os símbolos exportados
        let module_key = resolved_path.to_string_lossy().to_string();
        self.env
            .imported_modules
            .insert(module_key.clone(), exported_symbols);

        // 7. Aplicar as importações ao escopo atual
        self.apply_imported_module(&module_key)
    }

    fn resolve_module_path(&self, module_path: &str) -> Result<PathBuf, DryadError> {
        self.resolver
            .resolve(module_path, self.current_file_path.as_deref())
    }

    fn execute_module_and_capture_exports(
        &mut self,
        program: &Program,
        module_path: &PathBuf,
    ) -> Result<HashMap<String, Value>, DryadError> {
        // Salvar estado atual e registrar no call_stack_vars para GC
        let original_file_path = self.current_file_path.clone();
        self.env.call_stack_vars.push(self.env.variables.clone());
        let original_classes = self.env.classes.clone();

        // Definir contexto do módulo
        self.current_file_path = Some(module_path.clone());

        // Executar todas as declarações do módulo
        let mut exported_symbols = HashMap::new();

        for stmt in &program.statements {
            match stmt {
                Stmt::Export(exported_stmt, _) => {
                    // Executar a declaração exportada
                    self.execute_statement(exported_stmt)?;

                    // Capturar o símbolo exportado
                    match exported_stmt.as_ref() {
                        Stmt::VarDeclaration(name, _, _, _) => {
                            if let Some(var_name) = name.identifier_name() {
                                if let Some(value) = self.env.variables.get(var_name) {
                                    exported_symbols.insert(var_name.clone(), value.clone());
                                }
                            }
                        }
                        Stmt::FunctionDeclaration { name, .. } => {
                            if let Some(value) = self.env.variables.get(name) {
                                exported_symbols.insert(name.clone(), value.clone());
                            }
                        }
                        Stmt::ClassDeclaration(name, _, _, _, _) => {
                            if let Some(value) = self.env.classes.get(name) {
                                exported_symbols.insert(name.clone(), value.clone());
                            }
                        }
                        _ => {} // Outros tipos de export
                    }
                }
                _ => {
                    // Executar declarações normais (não exportadas)
                    self.execute_statement(stmt)?;
                }
            }
        }

        // Restaurar estado original
        self.current_file_path = original_file_path;
        if let Some(saved) = self.env.call_stack_vars.pop() {
            self.env.variables = saved;
        }
        self.env.classes = original_classes;

        Ok(exported_symbols)
    }

    fn apply_imported_module(&mut self, module_key: &str) -> Result<Value, DryadError> {
        if let Some(exported_symbols) = self.env.imported_modules.get(module_key) {
            // Aplicar todos os símbolos exportados ao escopo atual
            for (name, value) in exported_symbols {
                match value {
                    Value::Class(_) => {
                        // Classes vão para ambos os namespaces
                        self.env.classes.insert(name.clone(), value.clone());
                        self.env.variables.insert(name.clone(), value.clone()); // Também como variável para acesso estático
                    }
                    _ => {
                        // Variáveis e funções vão para o namespace de variáveis
                        self.env.variables.insert(name.clone(), value.clone());
                    }
                }
            }

            Ok(Value::Null)
        } else {
            Err(DryadError::new(
                3014,
                &format!(
                    "Módulo '{}' não encontrado nos módulos importados",
                    module_key
                ),
            ))
        }
    }

    pub fn import_module_with_kind(
        &mut self,
        kind: &ImportKind,
        module_path: &str,
    ) -> Result<Value, DryadError> {
        // 1. Resolver o caminho do módulo
        let resolved_path = self.resolve_module_path(module_path)?;

        // 2. Carregar/executar módulo se ainda não foi
        let module_key = resolved_path.to_string_lossy().to_string();
        if !self.env.imported_modules.contains_key(&module_key) {
            // Carregar e executar o módulo pela primeira vez
            let source_code = fs::read_to_string(&resolved_path).map_err(|e| {
                DryadError::new(
                    3001,
                    &format!("Erro ao ler módulo '{}': {}", resolved_path.display(), e),
                )
            })?;

            let mut lexer = dryad_lexer::lexer::Lexer::new(&source_code);
            let mut tokens = Vec::new();

            loop {
                match lexer.next_token() {
                    Ok(token) => {
                        let is_eof = matches!(token.token, dryad_lexer::token::Token::Eof);
                        tokens.push(token);
                        if is_eof {
                            break;
                        }
                    }
                    Err(e) => {
                        return Err(DryadError::new(
                            3002,
                            &format!(
                                "Erro de lexing no módulo '{}': {:?}",
                                resolved_path.display(),
                                e
                            ),
                        ))
                    }
                }
            }

            let mut parser = dryad_parser::parser::Parser::new(tokens);
            let program = parser.parse().map_err(|e| {
                DryadError::new(
                    3003,
                    &format!(
                        "Erro de parsing no módulo '{}': {:?}",
                        resolved_path.display(),
                        e
                    ),
                )
            })?;

            let exported_symbols =
                self.execute_module_and_capture_exports(&program, &resolved_path)?;
            self.env
                .imported_modules
                .insert(module_key.clone(), exported_symbols);
        }

        // 3. Aplicar importações de acordo com o tipo
        match kind {
            ImportKind::SideEffect => {
                // import "module"; - apenas executa o módulo, não importa símbolos
                Ok(Value::Null)
            }
            ImportKind::Named(names) => {
                // import { x, y } from "module"; - importa apenas símbolos específicos
                if let Some(exported_symbols) = self.env.imported_modules.get(&module_key) {
                    for name in names {
                        if let Some(value) = exported_symbols.get(name) {
                            match value {
                                Value::Class(_) => {
                                    self.env.classes.insert(name.clone(), value.clone());
                                    self.env.variables.insert(name.clone(), value.clone());
                                }
                                _ => {
                                    self.env.variables.insert(name.clone(), value.clone());
                                }
                            }
                        } else {
                            return Err(DryadError::new(
                                3015,
                                &format!(
                                    "Símbolo '{}' não encontrado nas exportações do módulo '{}'",
                                    name, module_key
                                ),
                            ));
                        }
                    }
                    Ok(Value::Null)
                } else {
                    Err(DryadError::new(
                        3014,
                        &format!("Módulo '{}' não encontrado", module_key),
                    ))
                }
            }
            ImportKind::Namespace(namespace) => {
                // import * as name from "module"; - importa tudo sob um namespace
                if let Some(exported_symbols) = self.env.imported_modules.get(&module_key) {
                    // Criar um objeto com todos os exports no heap
                    let obj_id = self.heap.allocate(ManagedObject::Object {
                        properties: exported_symbols.clone(),
                        methods: HashMap::new(),
                    });
                    let namespace_obj = Value::Object(obj_id);

                    self.env.variables.insert(namespace.clone(), namespace_obj);
                    Ok(Value::Null)
                } else {
                    Err(DryadError::new(
                        3014,
                        &format!("Módulo '{}' não encontrado", module_key),
                    ))
                }
            }
        }
    }

    // === MÉTODOS PARA NOVO SISTEMA DE MÓDULOS NATIVOS ===

    /// Ativa uma categoria de funções nativas através de diretiva #<categoria>
    pub fn activate_native_category(&mut self, category: &str) -> Result<(), String> {
        self.native_registry.manager.activate_category(category)
    }

    /// Desativa uma categoria de funções nativas
    pub fn deactivate_native_category(&mut self, category: &str) {
        self.native_registry.manager.deactivate_category(category);
    }

    /// Verifica se uma categoria está ativa
    pub fn is_native_category_active(&self, category: &str) -> bool {
        self.native_registry.manager.is_category_active(category)
    }

    /// Lista todas as categorias ativas
    pub fn list_active_native_categories(&self) -> Vec<String> {
        self.native_registry.manager.list_active_categories()
    }

    /// Lista todas as funções nativas ativas
    pub fn list_active_native_functions(&self) -> Vec<String> {
        self.native_registry.manager.list_active_functions()
    }

    fn execute_index_assignment(
        &mut self,
        array_expr: &Expr,
        index_value: Value,
        value: Value,
    ) -> Result<Value, DryadError> {
        let target = self.evaluate(array_expr)?;

        match target {
            Value::Array(id) => {
                let index = match index_value {
                    Value::Number(n) => {
                        if n < 0.0 || n.fract() != 0.0 {
                            return Err(DryadError::new(
                                3080,
                                "Índice deve ser um número inteiro não negativo",
                            ));
                        }
                        n as usize
                    }
                    _ => return Err(DryadError::new(3081, "Índice deve ser um número")),
                };

                let heap_obj = self.heap.get_mut(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Array reference not found")
                })?;

                if let ManagedObject::Array(elements) = heap_obj {
                    if index >= elements.len() {
                        return Err(DryadError::new(
                            3082,
                            &format!(
                                "Índice {} fora dos limites do array (tamanho: {})",
                                index,
                                elements.len()
                            ),
                        ));
                    }
                    elements[index] = value.clone();
                    Ok(value)
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Array"))
                }
            }
            Value::Object(id) => {
                let key = match index_value {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    _ => {
                        return Err(DryadError::new(
                            3084,
                            "Chave do objeto deve ser string ou número",
                        ))
                    }
                };

                let heap_obj = self.heap.get_mut(id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Object reference not found")
                })?;

                if let ManagedObject::Object { properties, .. } = heap_obj {
                    properties.insert(key, value.clone());
                    Ok(value)
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Object"))
                }
            }
            _ => Err(DryadError::new(
                3085,
                "Tentativa de atribuir índice a valor que não é array nem objeto",
            )),
        }
    }

    fn call_function_value(
        &mut self,
        func: &Value,
        args: Vec<Value>,
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        match func {
            Value::Function { name, params, body } => self.call_user_function_values(
                name.clone(),
                params.clone(),
                body.clone(),
                args,
                location,
            ),
            Value::Lambda(id) => {
                let heap_obj = self.heap.get(*id).ok_or_else(|| {
                    DryadError::new(3100, "Heap error: Lambda reference not found")
                })?;

                if let ManagedObject::Lambda {
                    params,
                    body,
                    closure,
                } = heap_obj
                {
                    self.call_lambda_values(
                        params.clone(),
                        body.clone(),
                        closure.clone(),
                        args,
                        location,
                    )
                } else {
                    Err(DryadError::new(3101, "Heap error: Expected Lambda"))
                }
            }
            _ => Err(DryadError::new(
                3033,
                "Tentativa de chamar um valor que não é uma função",
            )),
        }
    }

    fn eval_array_method(
        &mut self,
        object_expr: &Expr,
        method_name: &str,
        args: &[Expr],
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        // Avalia os argumentos
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate(arg)?);
        }

        let object = self.evaluate(object_expr)?;

        if let Value::Array(id) = object {
            // "Take" os elementos do heap temporariamente para satisfazer o borrow checker
            let mut elements = match self.heap.get_mut(id) {
                Some(ManagedObject::Array(e)) => std::mem::take(e),
                _ => {
                    return Err(DryadError::new(
                        3100,
                        "Heap error: Array not found or not an array",
                    ))
                }
            };

            let result =
                self.apply_array_method(id, &mut elements, method_name, arg_values, location);

            // "Replace" os elementos de volta no heap
            if let Some(ManagedObject::Array(e)) = self.heap.get_mut(id) {
                *e = elements;
            }

            result
        } else {
            Err(DryadError::new(
                3102,
                "Tentativa de chamar método de array em valor que não é array",
            ))
        }
    }

    fn apply_array_method(
        &mut self,
        array_id: HeapId,
        elements: &mut Vec<Value>,
        method_name: &str,
        arg_values: Vec<Value>,
        location: &SourceLocation,
    ) -> Result<Value, DryadError> {
        match method_name {
            // Basic Methods
            "push" => {
                elements.extend(arg_values);
                Ok(Value::Number(elements.len() as f64))
            }
            "pop" => {
                if let Some(v) = elements.pop() {
                    Ok(v)
                } else {
                    Ok(Value::Null)
                }
            }
            "shift" => {
                if !elements.is_empty() {
                    Ok(elements.remove(0))
                } else {
                    Ok(Value::Null)
                }
            }
            "unshift" => {
                for arg in arg_values.into_iter().rev() {
                    elements.insert(0, arg);
                }
                Ok(Value::Number(elements.len() as f64))
            }
            "length" => Ok(Value::Number(elements.len() as f64)),

            // Mapping & Filtering
            "forEach" => {
                if arg_values.is_empty() {
                    return Ok(Value::Null);
                }
                let callback = &arg_values[0];
                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    self.call_function_value(callback, args, location)?;
                }
                Ok(Value::Null)
            }
            "map" => {
                if arg_values.is_empty() {
                    let new_id = self.heap.allocate(ManagedObject::Array(Vec::new()));
                    return Ok(Value::Array(new_id));
                }
                let callback = &arg_values[0];
                let mut results = Vec::new();
                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    results.push(res);
                }
                let new_id = self.heap.allocate(ManagedObject::Array(results));
                Ok(Value::Array(new_id))
            }
            "filter" => {
                if arg_values.is_empty() {
                    let new_id = self.heap.allocate(ManagedObject::Array(Vec::new()));
                    return Ok(Value::Array(new_id));
                }
                let callback = &arg_values[0];
                let mut results = Vec::new();
                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    if self.is_truthy(&res) {
                        results.push(element.clone());
                    }
                }
                let new_id = self.heap.allocate(ManagedObject::Array(results));
                Ok(Value::Array(new_id))
            }
            "reduce" => {
                if arg_values.is_empty() {
                    return Err(DryadError::new(3025, "reduce requer callback"));
                }
                let callback = &arg_values[0];
                let mut iter = elements.iter().enumerate();
                let mut accumulator;

                if arg_values.len() > 1 {
                    accumulator = arg_values[1].clone();
                } else {
                    if let Some((_, head)) = iter.next() {
                        accumulator = head.clone();
                    } else {
                        return Err(DryadError::new(
                            3028,
                            "reduce em array vazio sem valor inicial",
                        ));
                    }
                }

                for (index, element) in iter {
                    let args = vec![
                        accumulator.clone(),
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    accumulator = self.call_function_value(callback, args, location)?;
                }
                Ok(accumulator)
            }
            "reduceRight" => {
                if arg_values.is_empty() {
                    return Err(DryadError::new(3025, "reduceRight requer callback"));
                }
                let callback = &arg_values[0];
                let mut iter = elements.iter().enumerate().rev();
                let mut accumulator;

                if arg_values.len() > 1 {
                    accumulator = arg_values[1].clone();
                } else {
                    if let Some((_, tail)) = iter.next() {
                        accumulator = tail.clone();
                    } else {
                        return Err(DryadError::new(
                            3028,
                            "reduceRight em array vazio sem valor inicial",
                        ));
                    }
                }

                for (index, element) in iter {
                    let args = vec![
                        accumulator.clone(),
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    accumulator = self.call_function_value(callback, args, location)?;
                }
                Ok(accumulator)
            }

            // Search & Inspection
            "includes" => {
                let target = if !arg_values.is_empty() {
                    &arg_values[0]
                } else {
                    &Value::Null
                };
                let start_index = if arg_values.len() > 1 {
                    match &arg_values[1] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };

                let len = elements.len() as isize;
                let mut idx = if start_index >= 0 {
                    start_index
                } else {
                    len + start_index
                };
                if idx < 0 {
                    idx = 0;
                }

                let mut found = false;
                for i in (idx as usize)..elements.len() {
                    if &elements[i] == target {
                        found = true;
                        break;
                    }
                }
                Ok(Value::Bool(found))
            }
            "indexOf" => {
                let target = if !arg_values.is_empty() {
                    &arg_values[0]
                } else {
                    &Value::Null
                };
                let start_index = if arg_values.len() > 1 {
                    match &arg_values[1] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };

                let len = elements.len() as isize;
                let mut idx = if start_index >= 0 {
                    start_index
                } else {
                    len + start_index
                };
                if idx < 0 {
                    idx = 0;
                }

                let mut found_idx = -1.0;
                for i in (idx as usize)..elements.len() {
                    if &elements[i] == target {
                        found_idx = i as f64;
                        break;
                    }
                }
                Ok(Value::Number(found_idx))
            }
            "lastIndexOf" => {
                let target = if !arg_values.is_empty() {
                    &arg_values[0]
                } else {
                    &Value::Null
                };
                let len = elements.len();
                let start_index = if arg_values.len() > 1 {
                    match &arg_values[1] {
                        Value::Number(n) => *n as isize,
                        _ => (len as isize) - 1,
                    }
                } else {
                    (len as isize) - 1
                };

                let mut idx = if start_index >= 0 {
                    if start_index >= len as isize {
                        len as isize - 1
                    } else {
                        start_index
                    }
                } else {
                    len as isize + start_index
                };

                let mut found_idx = -1.0;
                if idx >= 0 {
                    for i in (0..=(idx as usize)).rev() {
                        if &elements[i] == target {
                            found_idx = i as f64;
                            break;
                        }
                    }
                }
                Ok(Value::Number(found_idx))
            }
            "find" => {
                if arg_values.is_empty() {
                    return Ok(Value::Null);
                }
                let callback = &arg_values[0];
                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    if self.is_truthy(&res) {
                        return Ok(element.clone());
                    }
                }
                Ok(Value::Null) // undefined in JS
            }
            "findIndex" => {
                if arg_values.is_empty() {
                    return Ok(Value::Number(-1.0));
                }
                let callback = &arg_values[0];
                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    if self.is_truthy(&res) {
                        return Ok(Value::Number(index as f64));
                    }
                }
                Ok(Value::Number(-1.0))
            }
            "every" => {
                if arg_values.is_empty() {
                    return Ok(Value::Bool(true));
                }
                let callback = &arg_values[0];
                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    if !self.is_truthy(&res) {
                        return Ok(Value::Bool(false));
                    }
                }
                Ok(Value::Bool(true))
            }
            "some" => {
                if arg_values.is_empty() {
                    return Ok(Value::Bool(false));
                }
                let callback = &arg_values[0];
                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    if self.is_truthy(&res) {
                        return Ok(Value::Bool(true));
                    }
                }
                Ok(Value::Bool(false))
            }

            // Transformation & Ordering
            "sort" => {
                if !arg_values.is_empty() {
                    let callback = &arg_values[0];
                    let mut error = None;

                    elements.sort_by(|a, b| {
                        if error.is_some() {
                            return std::cmp::Ordering::Equal;
                        }

                        let args = vec![a.clone(), b.clone()];
                        match self.call_function_value(callback, args, location) {
                            Ok(res) => match res {
                                Value::Number(n) => {
                                    if n < 0.0 {
                                        std::cmp::Ordering::Less
                                    } else if n > 0.0 {
                                        std::cmp::Ordering::Greater
                                    } else {
                                        std::cmp::Ordering::Equal
                                    }
                                }
                                _ => std::cmp::Ordering::Equal,
                            },
                            Err(e) => {
                                error = Some(e);
                                std::cmp::Ordering::Equal
                            }
                        }
                    });

                    if let Some(e) = error {
                        return Err(e);
                    }
                } else {
                    elements.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                }
                Ok(Value::Array(array_id))
            }
            "reverse" => {
                elements.reverse();
                Ok(Value::Array(array_id))
            }
            "slice" => {
                let start = if !arg_values.is_empty() {
                    match &arg_values[0] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };

                let end = if arg_values.len() > 1 {
                    match &arg_values[1] {
                        Value::Number(n) => *n as isize,
                        _ => elements.len() as isize,
                    }
                } else {
                    elements.len() as isize
                };

                let len = elements.len() as isize;
                let mut idx_start = if start >= 0 { start } else { len + start };
                if idx_start < 0 {
                    idx_start = 0;
                }
                if idx_start > len {
                    idx_start = len;
                }

                let mut idx_end = if end >= 0 { end } else { len + end };
                if idx_end < 0 {
                    idx_end = 0;
                }
                if idx_end > len {
                    idx_end = len;
                }

                let mut result = Vec::new();
                if idx_start < idx_end {
                    for i in idx_start..idx_end {
                        result.push(elements[i as usize].clone());
                    }
                }

                let new_id = self.heap.allocate(ManagedObject::Array(result));
                Ok(Value::Array(new_id))
            }
            "splice" => {
                let start = if !arg_values.is_empty() {
                    match &arg_values[0] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };

                let len = elements.len() as isize;
                let mut idx_start = if start >= 0 { start } else { len + start };
                if idx_start < 0 {
                    idx_start = 0;
                }
                if idx_start > len {
                    idx_start = len;
                }

                let delete_count = if arg_values.len() > 1 {
                    match &arg_values[1] {
                        Value::Number(n) => {
                            let n = *n as isize;
                            if n < 0 {
                                0
                            } else {
                                n
                            }
                        }
                        _ => 0,
                    }
                } else {
                    len - idx_start
                };

                // Add items
                let items_to_add = if arg_values.len() > 2 {
                    arg_values[2..].to_vec()
                } else {
                    Vec::new()
                };

                // Perform splice
                // Vec::splice returns an iterator, we need to collect removed items
                let range_start = idx_start as usize;
                let range_end = (idx_start + delete_count).min(len) as usize;

                let removed: Vec<Value> = elements
                    .splice(range_start..range_end, items_to_add)
                    .collect();

                let new_id = self.heap.allocate(ManagedObject::Array(removed));
                Ok(Value::Array(new_id))
            }
            "concat" => {
                let mut result = elements.clone();
                for arg in arg_values {
                    if let Value::Array(other_id) = arg {
                        if let Some(ManagedObject::Array(other_elements)) = self.heap.get(other_id)
                        {
                            result.extend(other_elements.clone());
                        }
                    } else {
                        result.push(arg);
                    }
                }
                let new_id = self.heap.allocate(ManagedObject::Array(result));
                Ok(Value::Array(new_id))
            }
            "join" => {
                let separator = if !arg_values.is_empty() {
                    match &arg_values[0] {
                        Value::String(s) => s.clone(),
                        _ => ",".to_string(),
                    }
                } else {
                    ",".to_string()
                };

                let strings: Vec<String> = elements.iter().map(|v| v.to_string()).collect();
                Ok(Value::String(strings.join(&separator)))
            }
            "fill" => {
                if arg_values.is_empty() {
                    return Ok(Value::Array(array_id));
                }
                let value = &arg_values[0];

                let start = if arg_values.len() > 1 {
                    match &arg_values[1] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };

                let end = if arg_values.len() > 2 {
                    match &arg_values[2] {
                        Value::Number(n) => *n as isize,
                        _ => elements.len() as isize,
                    }
                } else {
                    elements.len() as isize
                };

                let len = elements.len() as isize;
                let mut idx_start = if start >= 0 { start } else { len + start };
                if idx_start < 0 {
                    idx_start = 0;
                }
                if idx_start > len {
                    idx_start = len;
                }

                let mut idx_end = if end >= 0 { end } else { len + end };
                if idx_end < 0 {
                    idx_end = 0;
                }
                if idx_end > len {
                    idx_end = len;
                }

                if idx_start < idx_end {
                    for i in idx_start..idx_end {
                        elements[i as usize] = value.clone();
                    }
                }
                Ok(Value::Array(array_id))
            }
            "copyWithin" => {
                // copyWithin(target, start, end)
                let len = elements.len() as isize;

                let target = if !arg_values.is_empty() {
                    match &arg_values[0] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };
                let mut to = if target >= 0 { target } else { len + target };
                if to < 0 {
                    to = 0;
                }
                if to >= len {
                    to = len;
                }

                let start = if arg_values.len() > 1 {
                    match &arg_values[1] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };
                let mut from = if start >= 0 { start } else { len + start };
                if from < 0 {
                    from = 0;
                }
                if from >= len {
                    from = len;
                }

                let end = if arg_values.len() > 2 {
                    match &arg_values[2] {
                        Value::Number(n) => *n as isize,
                        _ => len,
                    }
                } else {
                    len
                };
                let mut final_end = if end >= 0 { end } else { len + end };
                if final_end < 0 {
                    final_end = 0;
                }
                if final_end > len {
                    final_end = len;
                }

                let count = (final_end - from).min(len - to);

                if count > 0 {
                    // We need to copy carefully handling overlap
                    let from_idx = from as usize;
                    let to_idx = to as usize;
                    let count_idx = count as usize;

                    // Manual copy since Value doesn't implement Copy trait
                    let mut temp = Vec::new();
                    for i in 0..count_idx {
                        if from_idx + i < elements.len() {
                            temp.push(elements[from_idx + i].clone());
                        }
                    }
                    for (i, val) in temp.into_iter().enumerate() {
                        if to_idx + i < elements.len() {
                            elements[to_idx + i] = val;
                        }
                    }
                }
                Ok(Value::Array(array_id))
            }

            // Advanced / Utility
            "unique" => {
                let mut unique = Vec::new();
                for item in elements.iter() {
                    if !unique.contains(item) {
                        unique.push(item.clone());
                    }
                }
                let new_id = self.heap.allocate(ManagedObject::Array(unique));
                Ok(Value::Array(new_id))
            }
            "at" => {
                let idx = if !arg_values.is_empty() {
                    match &arg_values[0] {
                        Value::Number(n) => *n as isize,
                        _ => 0,
                    }
                } else {
                    0
                };
                let len = elements.len() as isize;
                let final_idx = if idx < 0 { len + idx } else { idx };
                if final_idx >= 0 && final_idx < len {
                    Ok(elements[final_idx as usize].clone())
                } else {
                    Ok(Value::Null)
                }
            }
            "flat" => {
                let depth = if !arg_values.is_empty() {
                    match &arg_values[0] {
                        Value::Number(n) => *n as i32,
                        _ => 1,
                    }
                } else {
                    1
                };

                let flattened = self.flatten(array_id, depth);
                let new_id = self.heap.allocate(ManagedObject::Array(flattened));
                Ok(Value::Array(new_id))
            }
            "flatMap" => {
                if arg_values.is_empty() {
                    let new_id = self.heap.allocate(ManagedObject::Array(Vec::new()));
                    return Ok(Value::Array(new_id));
                }
                let callback = &arg_values[0];
                let mut mapped_results = Vec::new();

                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    mapped_results.push(res);
                }

                let temp_id = self.heap.allocate(ManagedObject::Array(mapped_results));
                let flattened = self.flatten(temp_id, 1);
                let new_id = self.heap.allocate(ManagedObject::Array(flattened));
                Ok(Value::Array(new_id))
            }
            "chunk" => {
                let size = if !arg_values.is_empty() {
                    match &arg_values[0] {
                        Value::Number(n) => *n as usize,
                        _ => 1,
                    }
                } else {
                    1
                };
                if size == 0 {
                    let new_id = self.heap.allocate(ManagedObject::Array(Vec::new()));
                    return Ok(Value::Array(new_id));
                }

                let mut chunks = Vec::new();
                for chunk in elements.chunks(size) {
                    let chunk_id = self.heap.allocate(ManagedObject::Array(chunk.to_vec()));
                    chunks.push(Value::Array(chunk_id));
                }
                let new_id = self.heap.allocate(ManagedObject::Array(chunks));
                Ok(Value::Array(new_id))
            }
            "groupBy" => {
                if arg_values.is_empty() {
                    let obj_id = self.heap.allocate(ManagedObject::Object {
                        properties: HashMap::new(),
                        methods: HashMap::new(),
                    });
                    return Ok(Value::Object(obj_id));
                }
                let callback = &arg_values[0];
                let mut groups: HashMap<String, Vec<Value>> = HashMap::new();

                for (index, element) in elements.iter().enumerate() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let key_val = self.call_function_value(callback, args, location)?;
                    let key = match key_val {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => "null".to_string(),
                    };
                    groups
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .push(element.clone());
                }

                let mut properties = HashMap::new();
                for (key, values) in groups {
                    let val_id = self.heap.allocate(ManagedObject::Array(values));
                    properties.insert(key, Value::Array(val_id));
                }
                let obj_id = self.heap.allocate(ManagedObject::Object {
                    properties,
                    methods: HashMap::new(),
                });
                Ok(Value::Object(obj_id))
            }
            "zip" => {
                let mut iterators_elements: Vec<Vec<Value>> = Vec::new();
                iterators_elements.push(elements.clone());

                for arg in arg_values {
                    if let Value::Array(other_id) = arg {
                        if let Some(ManagedObject::Array(other_elements)) = self.heap.get(other_id)
                        {
                            iterators_elements.push(other_elements.clone());
                        }
                    }
                }

                if iterators_elements.is_empty() {
                    let new_id = self.heap.allocate(ManagedObject::Array(Vec::new()));
                    return Ok(Value::Array(new_id));
                }

                let min_len = iterators_elements
                    .iter()
                    .map(|v| v.len())
                    .min()
                    .unwrap_or(0);
                let mut result = Vec::new();

                for i in 0..min_len {
                    let mut tuple_vec = Vec::new();
                    for iter in &iterators_elements {
                        tuple_vec.push(iter[i].clone());
                    }
                    let tuple_id = self.heap.allocate(ManagedObject::Array(tuple_vec));
                    result.push(Value::Array(tuple_id));
                }
                let new_id = self.heap.allocate(ManagedObject::Array(result));
                Ok(Value::Array(new_id))
            }
            "reverseMap" => {
                if arg_values.is_empty() {
                    let new_id = self.heap.allocate(ManagedObject::Array(Vec::new()));
                    return Ok(Value::Array(new_id));
                }
                let callback = &arg_values[0];
                let mut results = Vec::new();

                for (index, element) in elements.iter().enumerate().rev() {
                    let args = vec![
                        element.clone(),
                        Value::Number(index as f64),
                        Value::Array(array_id),
                    ];
                    let res = self.call_function_value(callback, args, location)?;
                    results.push(res);
                }
                let new_id = self.heap.allocate(ManagedObject::Array(results));
                Ok(Value::Array(new_id))
            }

            _ => Err(DryadError::new(
                3100,
                &format!(
                    "Método '{}' não encontrado ou não implementado em Array",
                    method_name
                ),
            )),
        }
    }

    fn flatten(&self, id: HeapId, depth: i32) -> Vec<Value> {
        let mut result = Vec::new();
        self.flatten_recursive(Value::Array(id), depth, &mut result);
        result
    }

    fn flatten_recursive(&self, val: Value, depth: i32, result: &mut Vec<Value>) {
        if let Value::Array(id) = val {
            if let Some(ManagedObject::Array(items)) = self.heap.get(id) {
                for item in items {
                    if let Value::Array(inner_id) = item {
                        if depth > 0 {
                            self.flatten_recursive(Value::Array(*inner_id), depth - 1, result);
                        } else {
                            result.push(item.clone());
                        }
                    } else {
                        result.push(item.clone());
                    }
                }
            }
        } else {
            result.push(val);
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (Value::Exception(a), Value::Exception(b)) => a == b,
            (
                Value::Function {
                    name: n1,
                    params: p1,
                    ..
                },
                Value::Function {
                    name: n2,
                    params: p2,
                    ..
                },
            ) => n1 == n2 && p1 == p2,
            (Value::Lambda(a), Value::Lambda(b)) => a == b,
            (Value::Class(a), Value::Class(b)) => a == b,
            (Value::Instance(a), Value::Instance(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            _ => false,
        }
    }
}

/// Alias para compatibilidade com módulos nativos
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            // Strings
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            // Mixed Types priority: Number < String < Bool < Null < Array < Object < Function
            (Value::Number(_), _) => Some(std::cmp::Ordering::Less),
            (_, Value::Number(_)) => Some(std::cmp::Ordering::Greater),
            (Value::String(_), _) => Some(std::cmp::Ordering::Less),
            (_, Value::String(_)) => Some(std::cmp::Ordering::Greater),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::Bool(_), _) => Some(std::cmp::Ordering::Less),
            (_, Value::Bool(_)) => Some(std::cmp::Ordering::Greater),
            _ => Some(std::cmp::Ordering::Equal),
        }
    }
}
