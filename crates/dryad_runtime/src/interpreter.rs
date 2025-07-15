// crates/dryad_runtime/src/interpreter.rs
use dryad_parser::ast::{Expr, Literal, Stmt, Program, ClassMember, Visibility, ObjectProperty};
use dryad_errors::DryadError;
use crate::native_modules::NativeModuleManager;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde_json::{self, Value as JsonValue};

#[derive(Debug, Clone)]
pub enum FlowControl {
    Return(Value),
    Break,
    Continue,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    native_modules: NativeModuleManager, // Gerenciador de módulos nativos
    classes: HashMap<String, Value>, // Para armazenar definições de classe
    current_instance: Option<Value>, // Para contexto de 'this'
    imported_modules: HashMap<String, HashMap<String, Value>>, // Módulos importados com seus namespaces
    current_file_path: Option<PathBuf>, // Caminho do arquivo atual para resolver imports relativos
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Exception(String), // Para representar exceções
    Function {
        name: String,
        params: Vec<String>,
        body: Stmt,
    },
    Lambda {
        params: Vec<String>,
        body: Expr,
        closure: HashMap<String, Value>, // Captura o escopo onde a lambda foi criada
    },
    Class {
        name: String,
        parent: Option<String>,
        methods: HashMap<String, ClassMethod>,
        properties: HashMap<String, ClassProperty>,
    },
    Instance {
        class_name: String,
        properties: HashMap<String, Value>,
    },
    Object {
        properties: HashMap<String, Value>,
        methods: HashMap<String, ObjectMethod>,
    },
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
            Value::Array(elements) => {
                let str_elements: Vec<String> = elements.iter().map(|v| v.to_string()).collect();
                format!("[{}]", str_elements.join(", "))
            }
            Value::Tuple(elements) => {
                let str_elements: Vec<String> = elements.iter().map(|v| v.to_string()).collect();
                format!("({})", str_elements.join(", "))
            }
            Value::Exception(msg) => format!("Exception: {}", msg),
            Value::Function { name, .. } => format!("function {}", name),
            Value::Lambda { params, .. } => {
                format!("({}) => lambda", params.join(", "))
            }
            Value::Class { name, .. } => {
                format!("class {}", name)
            }
            Value::Instance { class_name, .. } => {
                format!("instance of {}", class_name)
            }
            Value::Object { properties, .. } => {
                let prop_strings: Vec<String> = properties.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{ {} }}", prop_strings.join(", "))
            }
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(elements) => !elements.is_empty(),
            Value::Tuple(elements) => !elements.is_empty(),
            Value::Exception(_) => false, // Exceções são sempre falsy
            Value::Function { .. } => true, // Funções são sempre truthy
            Value::Lambda { .. } => true, // Lambdas são sempre truthy
            Value::Class { .. } => true, // Classes são sempre truthy
            Value::Instance { .. } => true, // Instâncias são sempre truthy
            Value::Object { .. } => true, // Objetos são sempre truthy
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            native_modules: NativeModuleManager::new(),
            classes: HashMap::new(),
            current_instance: None,
            imported_modules: HashMap::new(),
            current_file_path: None,
        }
    }

    pub fn set_current_file(&mut self, file_path: PathBuf) {
        self.current_file_path = Some(file_path);
    }

    pub fn execute(&mut self, program: &Program) -> Result<String, DryadError> {
        let mut last_value = Value::Null;
        
        for statement in &program.statements {
            last_value = self.execute_statement(statement)?;
        }
        
        Ok(last_value.to_string())
    }

    pub fn execute_and_return_value(&mut self, program: &Program) -> Result<Value, DryadError> {
        let mut last_value = Value::Null;
        
        for statement in &program.statements {
            last_value = self.execute_statement(statement)?;
        }
        
        Ok(last_value)
    }

    // Método antigo mantido para compatibilidade com testes existentes
    pub fn eval(&mut self, expr: &Expr) -> Result<String, DryadError> {
        let value = self.evaluate(expr)?;
        Ok(value.to_string())
    }

    pub fn execute_statement(&mut self, stmt: &Stmt) -> Result<Value, DryadError> {
        match stmt {
            Stmt::NativeDirective(module_name) => {
                // Usar exclusivamente o novo sistema modular
                match self.activate_native_category(module_name) {
                    Ok(_) => {
                        // println!("📦 Categoria nativa carregada: {}", module_name);
                        Ok(Value::Null)
                    }
                    Err(err) => {
                        Err(DryadError::new(6001, &format!("Categoria nativa desconhecida: {} ({})", module_name, err)))
                    }
                }
            }
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::VarDeclaration(name, initializer) => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Null,
                };
                
                self.variables.insert(name.clone(), value);
                Ok(Value::Null) // Declarações de variáveis sempre retornam null
            }
            Stmt::Assignment(name, expr) => {
                let value = self.evaluate(expr)?;
                
                if !self.variables.contains_key(name) {
                    return Err(DryadError::new(3001, &format!("Variável '{}' não foi declarada", name)));
                }
                
                self.variables.insert(name.clone(), value.clone());
                Ok(value)
            }
            Stmt::PropertyAssignment(object_expr, property_name, value_expr) => {
                let value = self.evaluate(value_expr)?;
                let object = self.evaluate(object_expr)?;
                
                match object {
                    Value::Instance { class_name, mut properties } => {
                        properties.insert(property_name.clone(), value.clone());
                        
                        // Update the instance in current_instance if it matches
                        if let Some(Value::Instance { class_name: current_class, properties: current_props }) = &mut self.current_instance {
                            if current_class == &class_name {
                                current_props.insert(property_name.clone(), value.clone());
                            }
                        }
                        
                        Ok(value)
                    }
                    _ => Err(DryadError::new(3034, "Tentativa de atribuir propriedade a valor que não é uma instância"))
                }
            }
            Stmt::Block(statements) => {
                self.execute_block(statements)
            }
            Stmt::If(condition, then_stmt) => {
                let condition_value = self.evaluate(condition)?;
                if self.is_truthy(&condition_value) {
                    self.execute_statement(then_stmt)
                } else {
                    Ok(Value::Null)
                }
            }
            Stmt::IfElse(condition, then_stmt, else_stmt) => {
                let condition_value = self.evaluate(condition)?;
                if self.is_truthy(&condition_value) {
                    self.execute_statement(then_stmt)
                } else {
                    self.execute_statement(else_stmt)
                }
            }
            Stmt::While(condition, body) => {
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
                            if err.code() == 3010 { // Break
                                break;
                            } else if err.code() == 3011 { // Continue
                                continue;
                            } else {
                                return Err(err);
                            }
                        }
                    }
                }
                
                Ok(last_value)
            }
            Stmt::DoWhile(body, condition) => {
                let mut last_value = Value::Null;
                
                // Do-while executa o corpo pelo menos uma vez
                loop {
                    // Execute o corpo do loop primeiro
                    match self.execute_statement(body) {
                        Ok(value) => last_value = value,
                        Err(err) => {
                            // Verifica se é break ou continue
                            if err.code() == 3010 { // Break
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
            Stmt::Break => {
                Err(DryadError::new(3010, "break"))
            }
            Stmt::Continue => {
                Err(DryadError::new(3011, "continue"))
            }
            Stmt::For(init, condition, update, body) => {
                self.execute_for_loop(init, condition, update, body)
            }
            Stmt::ForEach(var_name, iterable, body) => {
                self.execute_foreach_loop(var_name, iterable, body)
            }
            Stmt::Try(try_block, catch_clause, finally_block) => {
                self.execute_try_catch_finally(try_block, catch_clause, finally_block)
            }
            Stmt::Throw(expr) => {
                let value = self.evaluate(expr)?;
                let exception_msg = match value {
                    Value::String(s) => s,
                    _ => value.to_string(),
                };
                Err(DryadError::new(3020, &exception_msg)) // Código específico para exceções lançadas
            }
            Stmt::FunctionDeclaration(name, params, body) => {
                let function = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: (**body).clone(),
                };
                self.variables.insert(name.clone(), function);
                Ok(Value::Null)
            }
            Stmt::ClassDeclaration(name, parent, members) => {
                let mut methods = HashMap::new();
                let mut properties = HashMap::new();
                
                // Process class members
                for member in members {
                    match member {
                        ClassMember::Method(visibility, is_static, method_name, params, body) => {
                            let method = ClassMethod {
                                visibility: visibility.clone(),
                                is_static: *is_static,
                                params: params.clone(),
                                body: (**body).clone(),
                            };
                            methods.insert(method_name.clone(), method);
                        }
                        ClassMember::Property(visibility, is_static, prop_name, default_value) => {
                            let default_val = match default_value {
                                Some(expr) => Some(self.evaluate(expr)?),
                                None => None,
                            };
                            let property = ClassProperty {
                                visibility: visibility.clone(),
                                is_static: *is_static,
                                default_value: default_val,
                            };
                            properties.insert(prop_name.clone(), property);
                        }
                    }
                }
                
                let class = Value::Class {
                    name: name.clone(),
                    parent: parent.clone(),
                    methods,
                    properties,
                };
                
                self.classes.insert(name.clone(), class.clone());
                self.variables.insert(name.clone(), class); // Também disponível como variável
                Ok(Value::Null)
            }
            Stmt::Return(expr) => {
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
                    Value::Array(_) => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                    Value::Tuple(_) => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                    Value::Exception(_) => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                    Value::Function { .. } => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                    Value::Lambda { .. } => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                    Value::Class { .. } => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                    Value::Instance { .. } => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                    Value::Object { .. } => Err(DryadError::new(3021, &format!("RETURN_OTHER:{}", value.to_string()))),
                }
            }
            Stmt::Export(stmt) => {
                // Por enquanto, simplesmente executa o statement interno
                // Em uma implementação completa, isto seria registrado como exportação
                self.execute_statement(stmt)
            }
            Stmt::Use(module_path) => {
                // Importa o módulo especificado
                self.import_module(module_path)
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        match expr {
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::Variable(name) => self.eval_variable(name),
            Expr::Binary(left, operator, right) => {
                self.eval_binary(left, operator, right)
            }
            Expr::Unary(operator, operand) => {
                self.eval_unary(operator, operand)
            }
            Expr::Call(func_expr, args) => self.eval_call(func_expr, args),
            Expr::PostIncrement(expr) => self.eval_post_increment(expr),
            Expr::PostDecrement(expr) => self.eval_post_decrement(expr),
            Expr::PreIncrement(expr) => self.eval_pre_increment(expr),
            Expr::PreDecrement(expr) => self.eval_pre_decrement(expr),
            Expr::Array(elements) => self.eval_array(elements),
            Expr::Tuple(elements) => self.eval_tuple(elements),
            Expr::Index(array_expr, index_expr) => self.eval_index(array_expr, index_expr),
            Expr::TupleAccess(tuple_expr, index) => self.eval_tuple_access(tuple_expr, *index),
            Expr::Lambda(params, body) => {
                Ok(Value::Lambda {
                    params: params.clone(),
                    body: *body.clone(),
                    closure: self.variables.clone(), // Captura o escopo atual
                })
            }
            Expr::This => {
                if let Some(instance) = &self.current_instance {
                    Ok(instance.clone())
                } else {
                    Err(DryadError::new(3022, "'this' usado fora do contexto de uma instância"))
                }
            }
            Expr::Super => {
                // Para implementar super, precisaríamos do contexto da classe pai
                // Por agora, retorna erro
                Err(DryadError::new(3023, "'super' ainda não implementado"))
            }
            Expr::MethodCall(object_expr, method_name, args) => {
                self.eval_method_call(object_expr, method_name, args)
            }
            Expr::PropertyAccess(object_expr, property_name) => {
                self.eval_property_access(object_expr, property_name)
            }
            Expr::ClassInstantiation(class_name, args) => {
                self.eval_class_instantiation(class_name, args)
            }
            Expr::ObjectLiteral(properties) => {
                self.eval_object_literal(properties)
            }
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
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| DryadError::new(3001, &format!("Variável '{}' não definida", name)))
    }

    fn eval_binary(&mut self, left: &Expr, operator: &str, right: &Expr) -> Result<Value, DryadError> {
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
            _ => Err(DryadError::new(3002, &format!("Operador desconhecido: {}", operator))),
        }
    }

    fn eval_call(&mut self, func_expr: &Expr, args: &[Expr]) -> Result<Value, DryadError> {
        // Se a expressão da função é uma variável simples, usar o caminho otimizado
        if let Expr::Variable(name) = func_expr {
            return self.eval_call_by_name(name, args);
        }
        
        // Para expressões complexas (como lambdas imediatas), avaliar a expressão primeiro
        let function_value = self.evaluate(func_expr)?;
        
        match function_value {
            Value::Function { name: _, params, body } => {
                self.call_user_function(params, body, args)
            }
            Value::Lambda { params, body, closure } => {
                self.call_lambda(params, body, closure, args)
            }
            _ => {
                Err(DryadError::new(3003, "Expressão não é uma função"))
            }
        }
    }

    fn eval_call_by_name(&mut self, name: &str, args: &[Expr]) -> Result<Value, DryadError> {
        // Primeiro verificar se é uma classe (para instanciação)
        if self.classes.contains_key(name) {
            return self.eval_class_instantiation(name, args);
        }
        
        // Segundo verificar se é uma função nativa do novo sistema modular
        if let Some(_) = self.native_modules.get_function(name) {
            // Avaliar argumentos primeiro
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(self.evaluate(arg)?);
            }
            // Agora obter a função e chamar
            if let Some(native_func) = self.native_modules.get_function(name) {
                // Converter RuntimeError para DryadError
                return native_func(&arg_values).map_err(|e| {
                    DryadError::new(3005, &format!("Erro na função nativa '{}': {}", name, e))
                });
            }
        }

        match name {
            "print" => {
                // Implementação simples do print (fallback)
                if !args.is_empty() {
                    let arg_value = self.evaluate(&args[0])?;
                    Ok(Value::String(arg_value.to_string()))
                } else {
                    Ok(Value::String("".to_string()))
                }
            }
            _ => {
                // Verificar se é uma função definida pelo usuário
                if let Some(function_value) = self.variables.get(name).cloned() {
                    match function_value {
                        Value::Function { name: _, params, body } => {
                            self.call_user_function(params, body, args)
                        }
                        Value::Lambda { params, body, closure } => {
                            self.call_lambda(params, body, closure, args)
                        }
                        _ => {
                            Err(DryadError::new(3003, &format!("'{}' não é uma função", name)))
                        }
                    }
                } else {
                    Err(DryadError::new(3003, &format!("Função '{}' não definida", name)))
                }
            }
        }
    }

    fn call_user_function(&mut self, params: Vec<String>, body: Stmt, args: &[Expr]) -> Result<Value, DryadError> {
        // Verificar número de argumentos
        if args.len() != params.len() {
            return Err(DryadError::new(3004, &format!(
                "Número incorreto de argumentos: esperado {}, encontrado {}",
                params.len(),
                args.len()
            )));
        }
        
        // Salvar estado atual das variáveis (para escopo)
        let saved_variables = self.variables.clone();
        
        // Avaliar argumentos e criar parâmetros
        for (i, param) in params.iter().enumerate() {
            let arg_value = self.evaluate(&args[i])?;
            self.variables.insert(param.clone(), arg_value);
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
        
        // Restaurar estado das variáveis
        self.variables = saved_variables;
        
        result.or(Ok(Value::Null)) // Se não teve return explícito, retorna null
    }

    fn call_lambda(&mut self, params: Vec<String>, body: Expr, closure: HashMap<String, Value>, args: &[Expr]) -> Result<Value, DryadError> {
        // Verificar número de argumentos
        if args.len() != params.len() {
            return Err(DryadError::new(3004, &format!(
                "Número incorreto de argumentos: esperado {}, encontrado {}",
                params.len(),
                args.len()
            )));
        }
        
        // Salvar estado atual das variáveis
        let saved_variables = self.variables.clone();
        
        // Restaurar o closure (escopo onde a lambda foi criada)
        self.variables = closure;
        
        // Avaliar argumentos e criar parâmetros (sobrescreve no escopo da lambda)
        for (i, param) in params.iter().enumerate() {
            let arg_value = self.evaluate(&args[i])?;
            self.variables.insert(param.clone(), arg_value);
        }
        
        // Executar corpo da lambda (é uma expressão)
        let result = self.evaluate(&body);
        
        // Restaurar estado das variáveis original
        self.variables = saved_variables;
        
        result
    }

    fn add_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_string()))),
            (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_string(), b))),
            _ => Err(DryadError::new(3004, "Operação '+' inválida para estes tipos")),
        }
    }

    fn subtract_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(DryadError::new(3005, "Operação '-' só é válida para números")),
        }
    }

    fn multiply_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(DryadError::new(3006, "Operação '*' só é válida para números")),
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
            _ => Err(DryadError::new(3008, "Operação '/' só é válida para números")),
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
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
                _ => Err(DryadError::new(3005, "Operação '-' só é válida para números")),
            }
            "!" => Ok(Value::Bool(!self.is_truthy(&value))),
            _ => Err(DryadError::new(3006, &format!("Operador unário '{}' desconhecido", operator))),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(elements) => !elements.is_empty(),
            Value::Tuple(elements) => !elements.is_empty(),
            Value::Exception(_) => false, // Exceções são sempre falsy
            Value::Function { .. } => true, // Funções são sempre truthy
            Value::Lambda { .. } => true, // Lambdas são sempre truthy
            Value::Class { .. } => true, // Classes são sempre truthy
            Value::Instance { .. } => true, // Instâncias são sempre truthy
            Value::Object { .. } => true, // Objetos são sempre truthy
        }
    }

    fn eval_post_increment(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para x++: retorna o valor atual e depois incrementa
        if let Expr::Variable(name) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    // Incrementa a variável
                    self.variables.insert(name.clone(), Value::Number(n + 1.0));
                    // Retorna o valor original
                    Ok(Value::Number(n))
                }
                _ => Err(DryadError::new(3007, "Operador ++ só é válido para números")),
            }
        } else {
            Err(DryadError::new(3008, "Operador ++ só pode ser aplicado a variáveis"))
        }
    }

    fn eval_post_decrement(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para x--: retorna o valor atual e depois decrementa
        if let Expr::Variable(name) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    // Decrementa a variável
                    self.variables.insert(name.clone(), Value::Number(n - 1.0));
                    // Retorna o valor original
                    Ok(Value::Number(n))
                }
                _ => Err(DryadError::new(3009, "Operador -- só é válido para números")),
            }
        } else {
            Err(DryadError::new(3010, "Operador -- só pode ser aplicado a variáveis"))
        }
    }

    fn eval_pre_increment(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para ++x: incrementa primeiro e retorna o novo valor
        if let Expr::Variable(name) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    let new_value = n + 1.0;
                    // Incrementa a variável
                    self.variables.insert(name.clone(), Value::Number(new_value));
                    // Retorna o novo valor
                    Ok(Value::Number(new_value))
                }
                _ => Err(DryadError::new(3011, "Operador ++ só é válido para números")),
            }
        } else {
            Err(DryadError::new(3012, "Operador ++ só pode ser aplicado a variáveis"))
        }
    }

    fn eval_pre_decrement(&mut self, expr: &Expr) -> Result<Value, DryadError> {
        // Para --x: decrementa primeiro e retorna o novo valor
        if let Expr::Variable(name) = expr {
            let current_value = self.eval_variable(name)?;
            match current_value {
                Value::Number(n) => {
                    let new_value = n - 1.0;
                    // Decrementa a variável
                    self.variables.insert(name.clone(), Value::Number(new_value));
                    // Retorna o novo valor
                    Ok(Value::Number(new_value))
                }
                _ => Err(DryadError::new(3013, "Operador -- só é válido para números")),
            }
        } else {
            Err(DryadError::new(3014, "Operador -- só pode ser aplicado a variáveis"))
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
            _ => Err(DryadError::new(3016, "Operação '%' só é válida para números")),
        }
    }

    fn power_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                Ok(Value::Number(a.powf(b)))
            }
            _ => Err(DryadError::new(3017, "Operação '**' só é válida para números")),
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
            _ => Err(DryadError::new(3021, "Operação '^^' só é válida para números")),
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
            _ => Err(DryadError::new(3023, "Operação '%%' só é válida para números")),
        }
    }

    fn power_of_ten_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                // a ## b = a * 10^b
                Ok(Value::Number(a * 10.0_f64.powf(b)))
            }
            _ => Err(DryadError::new(3024, "Operação '##' só é válida para números")),
        }
    }

    fn bitwise_and_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                let a_int = a as i64;
                let b_int = b as i64;
                Ok(Value::Number((a_int & b_int) as f64))
            }
            _ => Err(DryadError::new(3026, "Operação '&' só é válida para números")),
        }
    }

    fn bitwise_or_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                let a_int = a as i64;
                let b_int = b as i64;
                Ok(Value::Number((a_int | b_int) as f64))
            }
            _ => Err(DryadError::new(3027, "Operação '|' só é válida para números")),
        }
    }

    fn bitwise_xor_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                let a_int = a as i64;
                let b_int = b as i64;
                Ok(Value::Number((a_int ^ b_int) as f64))
            }
            _ => Err(DryadError::new(3028, "Operação '^' só é válida para números")),
        }
    }

    fn left_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(3029, "Não é possível fazer shift com número negativo"))
                } else {
                    // Left shift: a << b = a * 2^b
                    let result = a * 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(3030, "Operação '<<' só é válida para números")),
        }
    }

    fn right_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(3031, "Não é possível fazer shift com número negativo"))
                } else {
                    // Right shift: a >> b = a / 2^b
                    let result = a / 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(3032, "Operação '>>' só é válida para números")),
        }
    }

    fn symmetric_left_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(3033, "Não é possível fazer shift com número negativo"))
                } else {
                    // Symmetric left shift: a <<< b = a * 2^b (igual ao left shift padrão)
                    let result = a * 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(3034, "Operação '<<<' só é válida para números")),
        }
    }

    fn symmetric_right_shift_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b < 0.0 {
                    Err(DryadError::new(3035, "Não é possível fazer shift com número negativo"))
                } else {
                    // Symmetric right shift: a >>> b = a / 2^b (igual ao right shift padrão)
                    let result = a / 2.0_f64.powf(b);
                    Ok(Value::Number(result))
                }
            }
            _ => Err(DryadError::new(3036, "Operação '>>>' só é válida para números")),
        }
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<Value, DryadError> {
        // Backup das variáveis atuais para implementar escopo de bloco
        let backup_variables = self.variables.clone();
        
        // Track das variáveis declaradas no bloco (para shadow)
        let mut declared_in_block = std::collections::HashSet::new();
        
        let mut last_value = Value::Null;
        
        // Execute todas as declarações no bloco
        for stmt in statements {
            // Se é uma VarDeclaration, marca como declarada no bloco
            if let Stmt::VarDeclaration(name, _) = stmt {
                declared_in_block.insert(name.clone());
            }
            last_value = self.execute_statement(stmt)?;
        }
        
        // Implementa escopo correto:
        // 1. Remove variáveis declaradas no bloco (shadow)
        // 2. Restaura variáveis que existiam antes e foram shadowed
        // 3. Mantém modificações de variáveis que já existiam (assignments)
        for var_name in declared_in_block {
            // Remove a variável declarada no bloco
            self.variables.remove(&var_name);
            
            // Se existia uma variável com o mesmo nome antes, restaura
            if let Some(original_value) = backup_variables.get(&var_name) {
                self.variables.insert(var_name, original_value.clone());
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
        var_name: &str,
        iterable: &Expr,
        body: &Box<Stmt>,
    ) -> Result<Value, DryadError> {
        // Avalia a expressão iterável
        let iterable_value = self.evaluate(iterable)?;
        
        // Salva o valor anterior da variável de iteração (se existir)
        let previous_value = self.variables.get(var_name).cloned();
        
        let mut last_value = Value::Null;
        
        // Itera sobre os elementos dependendo do tipo
        match iterable_value {
            Value::Array(elements) => {
                for element in elements {
                    // Define a variável de iteração
                    self.variables.insert(var_name.to_string(), element);
                    
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
                                self.variables.insert(var_name.to_string(), prev_val);
                            } else {
                                self.variables.remove(var_name);
                            }
                            return Err(e);
                        }
                    }
                }
            }
            Value::Tuple(elements) => {
                for element in elements {
                    // Define a variável de iteração
                    self.variables.insert(var_name.to_string(), element);
                    
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
                                self.variables.insert(var_name.to_string(), prev_val);
                            } else {
                                self.variables.remove(var_name);
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
                    self.variables.insert(var_name.to_string(), char_value);
                    
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
                                self.variables.insert(var_name.to_string(), prev_val);
                            } else {
                                self.variables.remove(var_name);
                            }
                            return Err(e);
                        }
                    }
                }
            }
            Value::Number(_) | Value::Bool(_) | Value::Null | Value::Exception(_) | 
            Value::Function { .. } | Value::Lambda { .. } | Value::Class { .. } | Value::Instance { .. } | Value::Object { .. } => {
                return Err(DryadError::new(
                    3030, 
                    &format!("Valor não é iterável: {}", iterable_value.to_string())
                ));
            }
        }
        
        // Restaura o valor anterior da variável (se existia)
        if let Some(prev_val) = previous_value {
            self.variables.insert(var_name.to_string(), prev_val);
        } else {
            self.variables.remove(var_name);
        }
        
        Ok(last_value)
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    fn eval_array(&mut self, elements: &[Expr]) -> Result<Value, DryadError> {
        let mut values = Vec::new();
        
        for element in elements {
            let value = self.evaluate(element)?;
            values.push(value);
        }
        
        Ok(Value::Array(values))
    }
    
    fn eval_tuple(&mut self, elements: &[Expr]) -> Result<Value, DryadError> {
        let mut values = Vec::new();
        
        for element in elements {
            let value = self.evaluate(element)?;
            values.push(value);
        }
        
        Ok(Value::Tuple(values))
    }
    
    fn eval_index(&mut self, array_expr: &Expr, index_expr: &Expr) -> Result<Value, DryadError> {
        let array_value = self.evaluate(array_expr)?;
        let index_value = self.evaluate(index_expr)?;
        
        // Índice deve ser um número
        let index = match index_value {
            Value::Number(n) => {
                if n < 0.0 || n.fract() != 0.0 {
                    return Err(DryadError::new(3080, "Índice deve ser um número inteiro não negativo"));
                }
                n as usize
            },
            _ => return Err(DryadError::new(3081, "Índice deve ser um número")),
        };
        
        match array_value {
            Value::Array(elements) => {
                if index >= elements.len() {
                    return Err(DryadError::new(3082, &format!("Índice {} fora dos limites do array (tamanho: {})", index, elements.len())));
                }
                Ok(elements[index].clone())
            },
            Value::Number(_) | Value::Bool(_) | Value::String(_) | Value::Null | Value::Tuple(_) | 
            Value::Exception(_) | Value::Function { .. } | Value::Lambda { .. } | 
            Value::Class { .. } | Value::Instance { .. } | Value::Object { .. } => {
                Err(DryadError::new(3083, "Operador [] só pode ser usado em arrays"))
            },
        }
    }
    
    fn eval_tuple_access(&mut self, tuple_expr: &Expr, index: usize) -> Result<Value, DryadError> {
        let tuple_value = self.evaluate(tuple_expr)?;
        
        match tuple_value {
            Value::Tuple(elements) => {
                if index >= elements.len() {
                    return Err(DryadError::new(3084, &format!("Índice {} fora dos limites da tupla (tamanho: {})", index, elements.len())));
                }
                Ok(elements[index].clone())
            },
            Value::Number(_) | Value::Bool(_) | Value::String(_) | Value::Null | Value::Array(_) | 
            Value::Exception(_) | Value::Function { .. } | Value::Lambda { .. } | 
            Value::Class { .. } | Value::Instance { .. } | Value::Object { .. } => {
                Err(DryadError::new(3085, "Operador . só pode ser usado em tuplas"))
            },
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
            let old_value = self.variables.get(catch_var).cloned();
            self.variables.insert(catch_var.clone(), exception_value);
            
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
                    self.variables.insert(catch_var.clone(), old_val);
                }
                None => {
                    self.variables.remove(catch_var);
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

    fn eval_method_call(&mut self, object_expr: &Expr, method_name: &str, args: &[Expr]) -> Result<Value, DryadError> {
        let object = self.evaluate(object_expr)?;
        
        match object {
            Value::Class { name: class_name, methods, .. } => {
                // Static method call on a class
                if let Some(method) = methods.get(method_name) {
                    // Check if method is static
                    if !method.is_static {
                        return Err(DryadError::new(3024, &format!("Método '{}' não é estático", method_name)));
                    }
                    
                    // Check visibility (simplified - public only for now)
                    match method.visibility {
                        Visibility::Private => {
                            return Err(DryadError::new(3024, &format!("Método '{}' é privado", method_name)));
                        }
                        _ => {} // Public and Protected allowed for now
                    }
                    
                    // Evaluate arguments
                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(self.evaluate(arg)?);
                    }
                    
                    // Check parameter count
                    if arg_values.len() != method.params.len() {
                        return Err(DryadError::new(3025, &format!(
                            "Método '{}' espera {} argumentos, mas recebeu {}",
                            method_name, method.params.len(), arg_values.len()
                        )));
                    }
                    
                    // Save current state
                    let saved_vars = self.variables.clone();
                    let saved_instance = self.current_instance.clone();
                    
                    // Static methods don't have 'this' context
                    self.current_instance = None;
                    
                    // Bind parameters
                    for (param, value) in method.params.iter().zip(arg_values.iter()) {
                        self.variables.insert(param.clone(), value.clone());
                    }
                    
                    // Execute method body
                    let result = match self.execute_statement(&method.body) {
                        Ok(value) => Ok(value),
                        Err(e) => {
                            // Check if it's a return value
                            if e.code() == 3021 {
                                self.parse_return_value(e.message())
                            } else {
                                Err(e)
                            }
                        }
                    };
                    
                    // Restore state
                    self.variables = saved_vars;
                    self.current_instance = saved_instance;
                    
                    result
                } else {
                    Err(DryadError::new(3026, &format!("Método estático '{}' não encontrado na classe '{}'", method_name, class_name)))
                }
            }
            Value::Instance { class_name, properties } => {
                // Get the class definition
                if let Some(Value::Class { methods, .. }) = self.classes.get(&class_name).cloned() {
                    if let Some(method) = methods.get(method_name) {
                        // Check visibility (simplified - public only for now)
                        match method.visibility {
                            Visibility::Private => {
                                return Err(DryadError::new(3024, &format!("Método '{}' é privado", method_name)));
                            }
                            _ => {} // Public and Protected allowed for now
                        }
                        
                        // Evaluate arguments
                        let mut arg_values = Vec::new();
                        for arg in args {
                            arg_values.push(self.evaluate(arg)?);
                        }
                        
                        // Check parameter count
                        if arg_values.len() != method.params.len() {
                            return Err(DryadError::new(3025, &format!(
                                "Método '{}' espera {} argumentos, mas recebeu {}",
                                method_name, method.params.len(), arg_values.len()
                            )));
                        }
                        
                        // Save current state
                        let saved_vars = self.variables.clone();
                        let saved_instance = self.current_instance.clone();
                        
                        // Set up method context
                        self.current_instance = Some(Value::Instance { class_name, properties });
                        
                        // Bind parameters
                        for (param, value) in method.params.iter().zip(arg_values.iter()) {
                            self.variables.insert(param.clone(), value.clone());
                        }
                        
                        // Execute method body
                        let result = match self.execute_statement(&method.body) {
                            Ok(value) => Ok(value),
                            Err(e) => {
                                // Check if it's a return value
                                if e.code() == 3021 {
                                    self.parse_return_value(e.message())
                                } else {
                                    Err(e)
                                }
                            }
                        };
                        
                        // Restore state
                        self.variables = saved_vars;
                        self.current_instance = saved_instance;
                        
                        result
                    } else {
                        Err(DryadError::new(3026, &format!("Método '{}' não encontrado na classe '{}'", method_name, class_name)))
                    }
                } else {
                    Err(DryadError::new(3027, &format!("Definição da classe '{}' não encontrada", class_name)))
                }
            }
            Value::Object { properties, methods } => {
                // Check if method exists in object literal
                if let Some(method) = methods.get(method_name) {
                    // Clone the method to avoid borrow issues
                    let method = method.clone();
                    
                    // Evaluate arguments
                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(self.evaluate(arg)?);
                    }
                    
                    // Check parameter count
                    if arg_values.len() != method.params.len() {
                        return Err(DryadError::new(3025, &format!(
                            "Método '{}' espera {} argumentos, mas recebeu {}",
                            method_name, method.params.len(), arg_values.len()
                        )));
                    }
                    
                    // Save current state
                    let saved_vars = self.variables.clone();
                    let saved_instance = self.current_instance.clone();
                    
                    // Set up method context - for object literals, 'this' refers to the object itself
                    self.current_instance = Some(Value::Object { properties, methods });
                    
                    // Bind parameters
                    for (param, value) in method.params.iter().zip(arg_values.iter()) {
                        self.variables.insert(param.clone(), value.clone());
                    }
                    
                    // Execute method body
                    let result = match self.execute_statement(&method.body) {
                        Ok(value) => Ok(value),
                        Err(e) => {
                            // Check if it's a return value
                            if e.code() == 3021 {
                                self.parse_return_value(e.message())
                            } else {
                                Err(e)
                            }
                        }
                    };
                    
                    // Restore state
                    self.variables = saved_vars;
                    self.current_instance = saved_instance;
                    
                    result
                } else {
                    Err(DryadError::new(3026, &format!("Método '{}' não encontrado no objeto", method_name)))
                }
            }
            _ => Err(DryadError::new(3028, "Tentativa de chamar método em valor que não é uma instância ou objeto"))
        }
    }
    
    fn eval_property_access(&mut self, object_expr: &Expr, property_name: &str) -> Result<Value, DryadError> {
        let object = self.evaluate(object_expr)?;
        
        match object {
            Value::Class { name: class_name, properties: class_props, .. } => {
                // Static property access on a class
                if let Some(class_prop) = class_props.get(property_name) {
                    // Check if property is static
                    if !class_prop.is_static {
                        return Err(DryadError::new(3029, &format!("Propriedade '{}' não é estática", property_name)));
                    }
                    
                    // Check visibility (simplified - public only for now)
                    match class_prop.visibility {
                        Visibility::Private => {
                            return Err(DryadError::new(3029, &format!("Propriedade '{}' é privada", property_name)));
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
                    Err(DryadError::new(3030, &format!("Propriedade estática '{}' não encontrada na classe '{}'", property_name, class_name)))
                }
            }
            Value::Instance { class_name, properties } => {
                // First check instance properties
                if let Some(value) = properties.get(property_name) {
                    return Ok(value.clone());
                }
                
                // Then check class properties
                if let Some(Value::Class { properties: class_props, .. }) = self.classes.get(&class_name) {
                    if let Some(class_prop) = class_props.get(property_name) {
                        match class_prop.visibility {
                            Visibility::Private => {
                                return Err(DryadError::new(3029, &format!("Propriedade '{}' é privada", property_name)));
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
                
                Err(DryadError::new(3030, &format!("Propriedade '{}' não encontrada", property_name)))
            }
            Value::Object { properties, .. } => {
                // Check object literal properties
                if let Some(value) = properties.get(property_name) {
                    Ok(value.clone())
                } else {
                    Err(DryadError::new(3030, &format!("Propriedade '{}' não encontrada", property_name)))
                }
            }
            _ => Err(DryadError::new(3031, "Tentativa de acessar propriedade em valor que não é uma instância ou objeto"))
        }
    }
    
    fn eval_class_instantiation(&mut self, class_name: &str, args: &[Expr]) -> Result<Value, DryadError> {
        // Check if it's a class call or regular function call
        if let Some(Value::Class { methods, properties, .. }) = self.classes.get(class_name).cloned() {
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
            
            let mut instance = Value::Instance {
                class_name: class_name.to_string(),
                properties: instance_properties,
            };
            
            // Call init method if it exists
            if let Some(init_method) = methods.get("init") {
                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate(arg)?);
                }
                
                // Check parameter count
                if arg_values.len() != init_method.params.len() {
                    return Err(DryadError::new(3032, &format!(
                        "Construtor da classe '{}' espera {} argumentos, mas recebeu {}",
                        class_name, init_method.params.len(), arg_values.len()
                    )));
                }
                
                // Save current state
                let saved_vars = self.variables.clone();
                let saved_instance = self.current_instance.clone();
                
                // Set up constructor context
                self.current_instance = Some(instance.clone());
                
                // Bind parameters
                for (param, value) in init_method.params.iter().zip(arg_values.iter()) {
                    self.variables.insert(param.clone(), value.clone());
                }
                
                // Execute constructor
                let _ = match self.execute_statement(&init_method.body) {
                    Ok(_) => {},
                    Err(e) => {
                        // Check if it's a return (constructors shouldn't return values, but handle it gracefully)
                        if e.code() != 3021 {
                            // Restore state before returning error
                            self.variables = saved_vars;
                            self.current_instance = saved_instance;
                            return Err(e);
                        }
                    }
                };
                
                // Update instance with any property changes made in constructor
                if let Some(Value::Instance { properties: updated_props, .. }) = &self.current_instance {
                    instance = Value::Instance {
                        class_name: class_name.to_string(),
                        properties: updated_props.clone(),
                    };
                }
                
                // Restore state
                self.variables = saved_vars;
                self.current_instance = saved_instance;
            } else if !args.is_empty() {
                return Err(DryadError::new(3033, &format!(
                    "Classe '{}' não tem construtor 'init', mas argumentos foram fornecidos",
                    class_name
                )));
            }
            
            Ok(instance)
        } else {
            // Not a class, treat as regular function call
            self.eval_call_by_name(class_name, args)
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
        Err(DryadError::new(3035, &format!("Erro ao processar return: {}", error_message)))
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
                ObjectProperty::Method(key, params, body) => {
                    let method = ObjectMethod {
                        params: params.clone(),
                        body: *body.clone(),
                    };
                    object_methods.insert(key.clone(), method);
                }
            }
        }

        Ok(Value::Object {
            properties: object_properties,
            methods: object_methods,
        })
    }

    fn import_module(&mut self, module_path: &str) -> Result<Value, DryadError> {
        println!("Importando módulo: {}", module_path);
        
        // 1. Resolver o caminho do módulo
        let resolved_path = self.resolve_module_path(module_path)?;
        println!("Caminho resolvido: {:?}", resolved_path);
        
        // 2. Verificar se o módulo já foi importado
        if self.imported_modules.contains_key(&resolved_path.to_string_lossy().to_string()) {
            println!("Módulo já importado: {}", resolved_path.display());
            return self.apply_imported_module(&resolved_path.to_string_lossy().to_string());
        }
        
        // 3. Ler o arquivo do módulo
        let source_code = fs::read_to_string(&resolved_path)
            .map_err(|e| DryadError::Runtime(3001, format!("Erro ao ler módulo '{}': {}", resolved_path.display(), e)))?;
        
        // 4. Fazer lexing e parsing do módulo
        let mut lexer = dryad_lexer::lexer::Lexer::new(&source_code);
        let mut tokens = Vec::new();
        
        // Coletar todos os tokens
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    let is_eof = matches!(token, dryad_lexer::token::Token::Eof);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                },
                Err(e) => return Err(DryadError::Runtime(3002, format!("Erro de lexing no módulo '{}': {:?}", resolved_path.display(), e)))
            }
        }
        
        let mut parser = dryad_parser::parser::Parser::new(tokens);
        let program = parser.parse()
            .map_err(|e| DryadError::Runtime(3003, format!("Erro de parsing no módulo '{}': {:?}", resolved_path.display(), e)))?;
        
        // 5. Executar o módulo em um contexto separado e capturar exports
        let exported_symbols = self.execute_module_and_capture_exports(&program, &resolved_path)?;
        
        // 6. Armazenar os símbolos exportados
        let module_key = resolved_path.to_string_lossy().to_string();
        self.imported_modules.insert(module_key.clone(), exported_symbols);
        
        // 7. Aplicar as importações ao escopo atual
        self.apply_imported_module(&module_key)
    }
    
    fn resolve_module_path(&self, module_path: &str) -> Result<PathBuf, DryadError> {
        if module_path.starts_with("./") || module_path.starts_with("../") {
            // Caminho relativo
            if let Some(current_file) = &self.current_file_path {
                let base_dir = current_file.parent()
                    .ok_or_else(|| DryadError::Runtime(3004, "Não é possível determinar diretório base".to_string()))?;
                Ok(base_dir.join(module_path))
            } else {
                // Se não há arquivo atual, usar diretório de trabalho
                Ok(PathBuf::from(module_path))
            }
        } else if module_path.starts_with("@/") {
            // Caminho absoluto do projeto
            let relative_path = &module_path[2..]; // Remove "@/"
            Ok(PathBuf::from(relative_path))
        } else {
            // Tentativa de usar Oak (oaklock.json)
            self.resolve_oak_module(module_path)
        }
    }
    
    fn resolve_oak_module(&self, module_alias: &str) -> Result<PathBuf, DryadError> {
        // Tentar carregar oaklock.json
        let oaklock_path = PathBuf::from("oaklock.json");
        
        if !oaklock_path.exists() {
            return Err(DryadError::Runtime(3005, format!(
                "oaklock.json não encontrado. Não é possível resolver módulo '{}'", 
                module_alias
            )));
        }
        
        let oaklock_content = fs::read_to_string(&oaklock_path)
            .map_err(|e| DryadError::Runtime(3006, format!("Erro ao ler oaklock.json: {}", e)))?;
        
        let oaklock: JsonValue = serde_json::from_str(&oaklock_content)
            .map_err(|e| DryadError::Runtime(3007, format!("Erro ao parsear oaklock.json: {}", e)))?;
        
        // Parsear alias do tipo "matematica-utils/matematica"
        let parts: Vec<&str> = module_alias.split('/').collect();
        if parts.len() != 2 {
            return Err(DryadError::Runtime(3008, format!(
                "Alias de módulo inválido: '{}'. Esperado formato 'pacote/módulo'", 
                module_alias
            )));
        }
        
        let package_name = parts[0];
        let module_name = parts[1];
        
        // Procurar no oaklock.json
        let modules = oaklock.get("modules")
            .ok_or_else(|| DryadError::Runtime(3009, "Seção 'modules' não encontrada no oaklock.json".to_string()))?;
        
        let package = modules.get(package_name)
            .ok_or_else(|| DryadError::Runtime(3010, format!("Pacote '{}' não encontrado no oaklock.json", package_name)))?;
        
        let paths = package.get("paths")
            .ok_or_else(|| DryadError::Runtime(3011, format!("Seção 'paths' não encontrada para pacote '{}'", package_name)))?;
        
        let module_path = paths.get(module_name)
            .ok_or_else(|| DryadError::Runtime(3012, format!("Módulo '{}' não encontrado no pacote '{}'", module_name, package_name)))?
            .as_str()
            .ok_or_else(|| DryadError::Runtime(3013, format!("Caminho inválido para módulo '{}/{}'", package_name, module_name)))?;
        
        Ok(PathBuf::from(module_path))
    }
    
    fn execute_module_and_capture_exports(&mut self, program: &Program, module_path: &PathBuf) -> Result<HashMap<String, Value>, DryadError> {
        // Salvar estado atual
        let original_file_path = self.current_file_path.clone();
        let original_variables = self.variables.clone();
        let original_classes = self.classes.clone();
        
        // Definir contexto do módulo
        self.current_file_path = Some(module_path.clone());
        
        // Executar todas as declarações do módulo
        let mut exported_symbols = HashMap::new();
        
        for stmt in &program.statements {
            match stmt {
                Stmt::Export(exported_stmt) => {
                    // Executar a declaração exportada
                    self.execute_statement(exported_stmt)?;
                    
                    // Capturar o símbolo exportado
                    match exported_stmt.as_ref() {
                        Stmt::VarDeclaration(name, _) => {
                            if let Some(value) = self.variables.get(name) {
                                exported_symbols.insert(name.clone(), value.clone());
                            }
                        },
                        Stmt::FunctionDeclaration(name, _, _) => {
                            if let Some(value) = self.variables.get(name) {
                                exported_symbols.insert(name.clone(), value.clone());
                            }
                        },
                        Stmt::ClassDeclaration(name, _, _) => {
                            if let Some(value) = self.classes.get(name) {
                                exported_symbols.insert(name.clone(), value.clone());
                            }
                        },
                        _ => {} // Outros tipos de export
                    }
                },
                _ => {
                    // Executar declarações normais (não exportadas)
                    self.execute_statement(stmt)?;
                }
            }
        }
        
        // Restaurar estado original
        self.current_file_path = original_file_path;
        self.variables = original_variables;
        self.classes = original_classes;
        
        Ok(exported_symbols)
    }
    
    fn apply_imported_module(&mut self, module_key: &str) -> Result<Value, DryadError> {
        if let Some(exported_symbols) = self.imported_modules.get(module_key) {
            // Aplicar todos os símbolos exportados ao escopo atual
            for (name, value) in exported_symbols {
                match value {
                    Value::Class { .. } => {
                        // Classes vão para ambos os namespaces
                        self.classes.insert(name.clone(), value.clone());
                        self.variables.insert(name.clone(), value.clone()); // Também como variável para acesso estático
                    },
                    _ => {
                        // Variáveis e funções vão para o namespace de variáveis
                        self.variables.insert(name.clone(), value.clone());
                    }
                }
            }
            
            println!("Módulo '{}' importado com sucesso! {} símbolos importados.", 
                     module_key, exported_symbols.len());
            Ok(Value::Null)
        } else {
            Err(DryadError::Runtime(3014, format!("Módulo '{}' não encontrado nos módulos importados", module_key)))
        }
    }

    // === MÉTODOS PARA NOVO SISTEMA DE MÓDULOS NATIVOS ===
    
    /// Ativa uma categoria de funções nativas através de diretiva #<categoria>
    pub fn activate_native_category(&mut self, category: &str) -> Result<(), String> {
        self.native_modules.activate_category(category)
    }
    
    /// Desativa uma categoria de funções nativas
    pub fn deactivate_native_category(&mut self, category: &str) {
        self.native_modules.deactivate_category(category);
    }
    
    /// Verifica se uma categoria está ativa
    pub fn is_native_category_active(&self, category: &str) -> bool {
        self.native_modules.is_category_active(category)
    }
    
    /// Lista todas as categorias ativas
    pub fn list_active_native_categories(&self) -> Vec<String> {
        self.native_modules.list_active_categories()
    }
    
    /// Lista todas as funções nativas ativas
    pub fn list_active_native_functions(&self) -> Vec<String> {
        self.native_modules.list_active_functions()
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
            (Value::Function { name: n1, params: p1, .. }, Value::Function { name: n2, params: p2, .. }) => {
                // Comparar apenas nome e parâmetros para funções
                n1 == n2 && p1 == p2
            },
            (Value::Object { properties: p1, .. }, Value::Object { properties: p2, .. }) => {
                // Comparar propriedades dos objetos (métodos não são comparáveis facilmente)
                p1 == p2
            },
            _ => false,
        }
    }
}

/// Alias para compatibilidade com módulos nativos
pub type RuntimeValue = Value;
