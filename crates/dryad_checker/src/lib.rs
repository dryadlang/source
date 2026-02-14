use dryad_errors::DryadError;
use dryad_parser::ast::{ClassMember, Expr, ObjectProperty, Program, Stmt, Type};
use std::collections::HashMap;

pub struct TypeChecker {
    scopes: Vec<HashMap<String, Type>>,
    errors: Vec<DryadError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            errors: Vec::new(),
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
            } => {
                // Pre-define function for recursion
                // For now, simplify function type representation
                // For now, simplify function representation in checker
                self.define(
                    name.clone(),
                    Type::Function(Vec::new(), Box::new(Type::Any)),
                );

                self.begin_scope();
                for (param_name, param_type) in params {
                    self.define(param_name.clone(), param_type.clone().unwrap_or(Type::Any));
                }
                self.check_stmt(body);
                // TODO: Check if all return paths match return_type
                self.end_scope();
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
            _ => Type::Any,
        }
    }

    fn is_assignable(&self, target: &Type, source: &Type) -> bool {
        if target == &Type::Any || source == &Type::Any {
            return true;
        }
        target == source
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
