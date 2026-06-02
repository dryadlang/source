use dryad_errors::DryadError;
use dryad_parser::ast::{ClassMember, Expr, ObjectProperty, Program, Stmt, Type};
use std::collections::HashMap;

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
    errors: Vec<DryadError>,
    classes: HashMap<String, ClassType>,
    interfaces: HashMap<String, InterfaceType>,
}

struct ClassType {
    parent: Option<String>,
    interfaces: Vec<String>,
    members: HashMap<String, Type>, // For simplicity, map member names to types
}

struct InterfaceType {
    methods: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            errors: Vec::new(),
            classes: HashMap::new(),
            interfaces: HashMap::new(),
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), Vec<DryadError>> {
        for stmt in &program.statements {
            self.check_stmt(stmt);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDeclaration(name, var_type, initializer, _location) => {
                let init_type = initializer.as_ref().map(|expr| self.check_expr(expr));

                if let Some(var_name) = name.identifier_name() {
                    if let Some(expected_type) = var_type {
                        if let Some(actual_type) = init_type {
                            if !self.is_assignable(expected_type, &actual_type) {
                                self.errors.push(DryadError::new(
                                    3001,
                                    &format!("Tipo incompatível na variável '{}'. Esperado {:?}, encontrado {:?}", var_name, expected_type, actual_type)
                                ));
                            }
                        }
                        self.define(var_name.clone(), expected_type.clone());
                    } else if let Some(actual_type) = init_type {
                        self.define(var_name.clone(), actual_type);
                    } else {
                        self.define(var_name.clone(), Type::Any);
                    }
                }
            }
            Stmt::ConstDeclaration(name, const_type, initializer, _location) => {
                let init_type = self.check_expr(initializer);

                if let Some(const_name) = name.identifier_name() {
                    if let Some(expected_type) = const_type {
                        if !self.is_assignable(expected_type, &init_type) {
                            self.errors.push(DryadError::new(
                                3002,
                                &format!("Tipo incompatível na constante '{}'. Esperado {:?}, encontrado {:?}", const_name, expected_type, init_type)
                            ));
                        }
                        self.define(const_name.clone(), expected_type.clone());
                    } else {
                        self.define(const_name.clone(), init_type);
                    }
                }
            }
            Stmt::Block(statements, _location) => {
                self.begin_scope();
                for s in statements {
                    self.check_stmt(s);
                }
                self.end_scope();
            }
            Stmt::Expression(expr, _location) => {
                self.check_expr(expr);
            }
            Stmt::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                location: _,
                is_async: _,
                rest_param: _,
            } => {
                // Pre-define function for recursion
                // For now, simplify function type representation
                // For now, simplify function representation in checker
                self.define(
                    name.clone(),
                    Type::Function(Vec::new(), Box::new(Type::Any)),
                );

                self.begin_scope();
                for (param_name, param_type, _) in params {
                    self.define(param_name.clone(), param_type.clone().unwrap_or(Type::Any));
                }
                self.check_stmt(body);
                // TODO: Check if all return paths match return_type
                self.end_scope();
            }
            Stmt::ClassDeclaration(name, parent, interfaces, members, _location) => {
                let mut member_types = HashMap::new();
                for member in members {
                    match member {
                        ClassMember::Method { name, params, return_type, .. } => {
                            let param_types: Vec<Type> = params.iter().map(|(_, t, _)| t.clone().unwrap_or(Type::Any)).collect();
                            let ret_type = return_type.clone().unwrap_or(Type::Any);
                            member_types.insert(name.clone(), Type::Function(param_types, Box::new(ret_type)));
                        }
                        ClassMember::Property(_, _, name, prop_type, _) => {
                            member_types.insert(name.clone(), prop_type.clone().unwrap_or(Type::Any));
                        }
                        _ => {}
                    }
                }
                self.classes.insert(name.clone(), ClassType {
                    parent: parent.clone(),
                    interfaces: interfaces.clone(),
                    members: member_types,
                });
                self.define(name.clone(), Type::Class(name.clone()));
            }
            Stmt::InterfaceDeclaration(name, members, _location) => {
                let mut methods = HashMap::new();
                for member in members {
                    if let dryad_parser::ast::InterfaceMember::Method(m) = member {
                        let param_types: Vec<Type> = m.params.iter().map(|(_, t, _)| t.clone().unwrap_or(Type::Any)).collect();
                        let ret_type = m.return_type.clone().unwrap_or(Type::Any);
                        methods.insert(m.name.clone(), Type::Function(param_types, Box::new(ret_type)));
                    }
                }
                self.interfaces.insert(name.clone(), InterfaceType { methods });
                self.define(name.clone(), Type::Class(name.clone())); // Interfaces also act as types
            }
            _ => {
                // Implement other statements as needed
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(lit, _location) => match lit {
                dryad_parser::ast::Literal::Number(_) => Type::Number,
                dryad_parser::ast::Literal::String(_) => Type::String,
                dryad_parser::ast::Literal::Bool(_) => Type::Bool,
                dryad_parser::ast::Literal::Null => Type::Null,
            },
            Expr::Variable(name, _location) => self.resolve(name).cloned().unwrap_or(Type::Any),
            Expr::Binary(left, op, right, _location) => {
                let lt = self.check_expr(left);
                let rt = self.check_expr(right);

                match op.as_str() {
                    "+" | "-" | "*" | "/" | "%" | "**" => {
                        if lt != Type::Number || rt != Type::Number {
                            // If it's '+', it could be string concatenation
                            if op == "+" && (lt == Type::String || rt == Type::String) {
                                return Type::String;
                            }

                            if lt != Type::Any && rt != Type::Any {
                                self.errors.push(DryadError::new(
                                    3003,
                                    &format!(
                                        "Operação '{}' não pode ser aplicada a {:?} e {:?}",
                                        op, lt, rt
                                    ),
                                ));
                            }
                        }
                        Type::Number
                    }
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => Type::Bool,
                    "&&" | "||" => Type::Bool,
                    _ => Type::Any,
                }
            }
            Expr::Call(callee, _args, _location) => {
                self.check_expr(callee);
                // TODO: Check argument types against function signature
                Type::Any
            }
            Expr::PropertyAccess(object, property, _location) => {
                let obj_type = self.check_expr(object);
                if let Type::Class(class_name) = obj_type {
                    if let Some(cls) = self.classes.get(&class_name) {
                        if let Some(t) = cls.members.get(property) {
                            return t.clone();
                        }
                    }
                }
                Type::Any
            }
            Expr::MethodCall(object, method, args, _location) => {
                let obj_type = self.check_expr(object);
                for arg in args { self.check_expr(arg); }
                if let Type::Class(class_name) = obj_type {
                    if let Some(cls) = self.classes.get(&class_name) {
                        if let Some(t) = cls.members.get(method) {
                            if let Type::Function(_, ret) = t {
                                return (**ret).clone();
                            }
                        }
                    }
                }
                Type::Any
            }
            Expr::ClassInstantiation(name, args, _location) => {
                for arg in args { self.check_expr(arg); }
                Type::Class(name.clone())
            }
            _ => Type::Any,
        }
    }

    fn is_assignable(&self, target: &Type, source: &Type) -> bool {
        if target == &Type::Any || source == &Type::Any {
            return true;
        }
        if target == source {
            return true;
        }

        // Check inheritance and interfaces
        if let (Type::Class(target_name), Type::Class(source_name)) = (target, source) {
            if self.is_subtype(source_name, target_name) {
                return true;
            }
        }

        false
    }

    fn is_subtype(&self, source: &str, target: &str) -> bool {
        if source == target {
            return true;
        }

        if let Some(cls) = self.classes.get(source) {
            if let Some(parent) = &cls.parent {
                if self.is_subtype(parent, target) {
                    return true;
                }
            }
            for interface in &cls.interfaces {
                if self.is_subtype_interface(interface, target) {
                    return true;
                }
            }
        }

        false
    }

    fn is_subtype_interface(&self, interface_name: &str, target: &str) -> bool {
        if interface_name == target {
            return true;
        }
        // Future: interface inheritance
        false
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: String, t: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, t);
        }
    }

    fn resolve(&self, name: &str) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(t) = scope.get(name) {
                return Some(t);
            }
        }
        None
    }
}
