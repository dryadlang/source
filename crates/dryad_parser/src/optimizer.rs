use crate::ast::*;
use dryad_errors::SourceLocation;

pub struct AstOptimizer {
    optimizations_applied: usize,
}

impl AstOptimizer {
    pub fn new() -> Self {
        AstOptimizer {
            optimizations_applied: 0,
        }
    }

    pub fn optimize(&mut self, program: &mut Program) {
        self.optimizations_applied = 0;

        for stmt in &mut program.statements {
            self.optimize_statement(stmt);
        }
    }

    fn new_location() -> SourceLocation {
        SourceLocation::new(None, 0, 0, 0)
    }

    fn optimize_statement(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Block(statements, _) => {
                let mut i = 0;
                while i < statements.len() {
                    self.optimize_statement(&mut statements[i]);
                    i += 1;
                }
            }
            Stmt::If(condition, then_branch, _) => {
                self.optimize_expression(condition);
                self.optimize_statement(then_branch);
            }
            Stmt::IfElse(condition, then_branch, else_branch, _) => {
                self.optimize_expression(condition);
                self.optimize_statement(then_branch);
                self.optimize_statement(else_branch);
            }
            Stmt::While(condition, body, _) => {
                self.optimize_expression(condition);
                self.optimize_statement(body);
            }
            Stmt::For(init, condition, update, body, _) => {
                if let Some(init) = init {
                    self.optimize_statement(init);
                }
                if let Some(condition) = condition {
                    self.optimize_expression(condition);
                }
                if let Some(update) = update {
                    self.optimize_statement(update);
                }
                self.optimize_statement(body);
            }
            Stmt::ForEach(_, iterable, body, _) => {
                self.optimize_expression(iterable);
                self.optimize_statement(body);
            }
            Stmt::FunctionDeclaration { body, .. } => {
                self.optimize_statement(body);
            }
            Stmt::ClassDeclaration(_, _, members, _) => {
                for member in members {
                    self.optimize_class_member(member);
                }
            }
            Stmt::Expression(expr, _) => {
                self.optimize_expression(expr);
            }
            Stmt::Return(Some(expr), _) => {
                self.optimize_expression(expr);
            }
            Stmt::Try(body, catch, finally, _) => {
                self.optimize_statement(body);
                if let Some((_, catch_body)) = catch {
                    self.optimize_statement(catch_body);
                }
                if let Some(finally_body) = finally {
                    self.optimize_statement(finally_body);
                }
            }
            _ => {}
        }
    }

    fn optimize_class_member(&mut self, member: &mut ClassMember) {
        match member {
            ClassMember::Method { body, .. } => {
                self.optimize_statement(body);
            }
            ClassMember::Property(_, _, _, _, value) => {
                if let Some(expr) = value {
                    self.optimize_expression(expr);
                }
            }
        }
    }

    fn optimize_expression(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Binary(left, op, right, _) => {
                self.optimize_expression(left);
                self.optimize_expression(right);
                self.constant_folding_binary(expr);
            }
            Expr::Unary(op, operand, _) => {
                self.optimize_expression(operand);
                self.constant_folding_unary(expr);
            }
            Expr::Call(func, args, _) => {
                self.optimize_expression(func);
                for arg in args {
                    self.optimize_expression(arg);
                }
            }
            Expr::MethodCall(obj, _, args, _) => {
                self.optimize_expression(obj);
                for arg in args {
                    self.optimize_expression(arg);
                }
            }
            Expr::PropertyAccess(obj, _, _) => {
                self.optimize_expression(obj);
            }
            Expr::Index(array, index, _) => {
                self.optimize_expression(array);
                self.optimize_expression(index);
            }
            Expr::Lambda { body, .. } => {
                self.optimize_expression(body);
            }
            Expr::Array(elements, _) => {
                for elem in elements {
                    self.optimize_expression(elem);
                }
            }
            Expr::ObjectLiteral(properties, _) => {
                for prop in properties {
                    if let ObjectProperty::Property(_, value) = prop {
                        self.optimize_expression(value);
                    }
                }
            }
            Expr::Match(expr, arms, _) => {
                self.optimize_expression(expr);
                for arm in arms {
                    if let Some(guard) = &mut arm.guard {
                        self.optimize_expression(guard);
                    }
                    self.optimize_statement(&mut arm.body);
                }
            }
            _ => {}
        }
    }

    fn constant_folding_binary(&mut self, expr: &mut Expr) {
        let Expr::Binary(left, op, right, _) = expr else {
            return;
        };

        let left_value = self.evaluate_constant(left);
        let right_value = self.evaluate_constant(right);

        if let (Some(l), Some(r)) = (left_value, right_value) {
            if let Some(result) = self.compute_binary_op(&l, op, &r) {
                self.optimizations_applied += 1;
                *expr = result;
                return;
            }
        }

        if let Some(result) = self.short_circuit_eval(op, left, right) {
            self.optimizations_applied += 1;
            *expr = result;
        }
    }

    fn constant_folding_unary(&mut self, expr: &mut Expr) {
        let Expr::Unary(op, operand, _) = expr else {
            return;
        };

        if let Some(value) = self.evaluate_constant(operand) {
            if let Some(result) = self.compute_unary_op(op, &value) {
                self.optimizations_applied += 1;
                *expr = result;
            }
        }
    }

    fn evaluate_constant(&self, expr: &Expr) -> Option<ConstantValue> {
        match expr {
            Expr::Literal(Literal::Number(n), _) => Some(ConstantValue::Number(*n)),
            Expr::Literal(Literal::String(s), _) => Some(ConstantValue::String(s.clone())),
            Expr::Literal(Literal::Bool(b), _) => Some(ConstantValue::Boolean(*b)),
            Expr::Literal(Literal::Null, _) => Some(ConstantValue::Null),
            Expr::Array(elements, _) => {
                let mut values = Vec::new();
                for elem in elements {
                    if let Some(c) = self.evaluate_constant(elem) {
                        values.push(c);
                    } else {
                        return None;
                    }
                }
                Some(ConstantValue::Array(values))
            }
            _ => None,
        }
    }

    fn compute_binary_op(
        &self,
        left: &ConstantValue,
        op: &str,
        right: &ConstantValue,
    ) -> Option<Expr> {
        let loc = Self::new_location();

        match (left, op, right) {
            (ConstantValue::Number(l), "+", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Number(l + r), loc))
            }
            (ConstantValue::Number(l), "-", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Number(l - r), loc))
            }
            (ConstantValue::Number(l), "*", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Number(l * r), loc))
            }
            (ConstantValue::Number(l), "/", ConstantValue::Number(r)) => {
                if *r != 0.0 {
                    Some(Expr::Literal(Literal::Number(l / r), loc))
                } else {
                    None
                }
            }
            (ConstantValue::Number(l), "%", ConstantValue::Number(r)) => {
                if *r != 0.0 {
                    Some(Expr::Literal(Literal::Number(l % r), loc))
                } else {
                    None
                }
            }
            (ConstantValue::Boolean(l), "&&", ConstantValue::Boolean(r)) => {
                Some(Expr::Literal(Literal::Bool(*l && *r), loc))
            }
            (ConstantValue::Boolean(l), "||", ConstantValue::Boolean(r)) => {
                Some(Expr::Literal(Literal::Bool(*l || *r), loc))
            }
            (ConstantValue::String(l), "+", ConstantValue::String(r)) => {
                Some(Expr::Literal(Literal::String(format!("{}{}", l, r)), loc))
            }
            (ConstantValue::Number(l), "==", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Bool(l == r), loc))
            }
            (ConstantValue::Number(l), "!=", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Bool(l != r), loc))
            }
            (ConstantValue::Number(l), "<", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Bool(l < r), loc))
            }
            (ConstantValue::Number(l), "<=", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Bool(l <= r), loc))
            }
            (ConstantValue::Number(l), ">", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Bool(l > r), loc))
            }
            (ConstantValue::Number(l), ">=", ConstantValue::Number(r)) => {
                Some(Expr::Literal(Literal::Bool(l >= r), loc))
            }
            (ConstantValue::Boolean(l), "==", ConstantValue::Boolean(r)) => {
                Some(Expr::Literal(Literal::Bool(l == r), loc))
            }
            (ConstantValue::Boolean(l), "!=", ConstantValue::Boolean(r)) => {
                Some(Expr::Literal(Literal::Bool(l != r), loc))
            }
            (ConstantValue::String(l), "==", ConstantValue::String(r)) => {
                Some(Expr::Literal(Literal::Bool(l == r), loc))
            }
            (ConstantValue::String(l), "!=", ConstantValue::String(r)) => {
                Some(Expr::Literal(Literal::Bool(l != r), loc))
            }
            _ => None,
        }
    }

    fn compute_unary_op(&self, op: &str, value: &ConstantValue) -> Option<Expr> {
        let loc = Self::new_location();

        match (op, value) {
            ("-", ConstantValue::Number(n)) => Some(Expr::Literal(Literal::Number(-n), loc)),
            ("!", ConstantValue::Boolean(b)) => Some(Expr::Literal(Literal::Bool(!b), loc)),
            _ => None,
        }
    }

    fn short_circuit_eval(&self, op: &str, left: &Expr, right: &Expr) -> Option<Expr> {
        let loc = Self::new_location();

        match (op, left, right) {
            ("&&", Expr::Literal(Literal::Bool(false), _), _) => {
                Some(Expr::Literal(Literal::Bool(false), loc))
            }
            ("&&", Expr::Literal(Literal::Bool(true), _), _) => Some(right.clone()),
            ("||", Expr::Literal(Literal::Bool(true), _), _) => {
                Some(Expr::Literal(Literal::Bool(true), loc))
            }
            ("||", Expr::Literal(Literal::Bool(false), _), _) => Some(right.clone()),
            _ => None,
        }
    }

    pub fn optimizations_count(&self) -> usize {
        self.optimizations_applied
    }
}

#[derive(Clone, Debug)]
enum ConstantValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<ConstantValue>),
}

impl Default for AstOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
