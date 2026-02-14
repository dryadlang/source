// crates/dryad_bytecode/src/compiler.rs
//! Compilador AST -> Bytecode
//!
//! Este módulo implementa o compilador que traduz a AST (Abstract Syntax Tree)
//! do Dryad para bytecode executável pela VM.

use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::value::Value;
use dryad_errors::SourceLocation;
use dryad_parser::ast::{ClassMember, Expr, ImportKind, InterfaceMember, Literal, MatchArm, ObjectProperty, Pattern, Program, Stmt, Type, Visibility};

/// Informações sobre um loop atual (para break/continue)
#[derive(Debug, Clone)]
struct LoopInfo {
    /// Posição do início do loop (para continue)
    start_pos: usize,
    /// Lista de jumps de break para resolver
    breaks: Vec<usize>,
    /// Profundidade do escopo quando o loop começou
    scope_depth: usize,
}

/// Compilador que converte AST em bytecode
pub struct Compiler {
    /// Chunk atual sendo compilado
    current_chunk: Chunk,
    /// Locais (variáveis locais) no escopo atual
    locals: Vec<Local>,
    /// Profundidade do escopo atual
    scope_depth: usize,
    /// Chunk sendo compilado (principal)
    chunks: Vec<Chunk>,
    /// Pilha de loops ativos (para break/continue)
    loop_stack: Vec<LoopInfo>,
}

/// Uma variável local
#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
    is_captured: bool,
}

/// Representa uma função em compilação
#[derive(Debug)]
struct FunctionCompiler {
    /// Chunk da função
    chunk: Chunk,
    /// Tipo de função (função, método, etc.)
    function_type: FunctionType,
    /// Locais da função
    locals: Vec<Local>,
    /// Profundidade do escopo
    scope_depth: usize,
    /// Número de upvalues
    upvalue_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    Function,
    Method,
    Initializer,
    Script,
}

impl Compiler {
    /// Cria um novo compilador
    pub fn new() -> Self {
        Self {
            current_chunk: Chunk::new("script"),
            locals: Vec::new(),
            scope_depth: 0,
            chunks: Vec::new(),
            loop_stack: Vec::new(),
        }
    }

    /// Compila um programa completo
    pub fn compile(&mut self, program: Program) -> Result<Chunk, String> {
        self.current_chunk = Chunk::new("script");
        self.locals.clear();
        self.scope_depth = 0;
        self.chunks.clear();
        self.loop_stack.clear();

        // Compila cada statement
        for stmt in program.statements {
            self.compile_statement(stmt)?;
        }

        // Adiciona retorno implícito
        self.emit_op(OpCode::Nil, 0);
        self.emit_op(OpCode::Return, 0);

        Ok(self.current_chunk.clone())
    }

    // ============================================
    // Compilação de Statements
    // ============================================

    fn compile_statement(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr, loc) => {
                self.compile_expression(expr)?;
                self.emit_op(OpCode::Pop, loc.line);
                Ok(())
            }

            Stmt::VarDeclaration(pattern, _type, initializer, loc) => {
                self.compile_var_declaration(pattern, initializer, loc.line)
            }

            Stmt::ConstDeclaration(pattern, _type, value, loc) => {
                // const é tratado igual a var, mas com valor obrigatório
                self.compile_var_declaration(pattern, Some(value), loc.line)
            }

            Stmt::Assignment(pattern, value, loc) => {
                self.compile_assignment(pattern, value, loc.line)
            }

            Stmt::PropertyAssignment(object, property, value, loc) => {
                self.compile_property_assignment(object, property, value, loc.line)
            }

            Stmt::IndexAssignment(array, index, value, loc) => {
                self.compile_index_assignment(array, index, value, loc.line)
            }

            Stmt::Block(statements, loc) => {
                self.begin_scope();
                for stmt in statements {
                    self.compile_statement(stmt)?;
                }
                self.end_scope(loc.line);
                Ok(())
            }

            Stmt::If(condition, then_branch, loc) => {
                self.compile_if(condition, *then_branch, None, loc.line)
            }

            Stmt::IfElse(condition, then_branch, else_branch, loc) => {
                self.compile_if(condition, *then_branch, Some(*else_branch), loc.line)
            }

            Stmt::While(condition, body, loc) => {
                self.compile_while(condition, *body, loc.line)
            }

            Stmt::DoWhile(body, condition, loc) => {
                self.compile_do_while(*body, condition, loc.line)
            }

            Stmt::For(init, condition, update, body, loc) => {
                self.compile_for(init, condition, update, *body, loc.line)
            }

            Stmt::ForEach(pattern, iterable, body, loc) => {
                self.compile_foreach(pattern, iterable, *body, loc.line)
            }

            Stmt::Break(loc) => {
                // Sai do escopo do loop
                if let Some(loop_info) = self.loop_stack.last() {
                    // Remove variáveis locais até a profundidade do loop
                    let pop_count = self.locals.len() - loop_info.scope_depth;
                    if pop_count > 0 {
                        if pop_count == 1 {
                            self.emit_op(OpCode::Pop, loc.line);
                        } else {
                            self.emit_op(OpCode::PopN(pop_count as u8), loc.line);
                        }
                    }
                    
                    // Emite jump que será resolvido no final do loop
                    let break_jump = self.emit_jump(OpCode::Jump(0), loc.line);
                    
                    // Registra o break no loop atual
                    if let Some(loop_info) = self.loop_stack.last_mut() {
                        loop_info.breaks.push(break_jump);
                    }
                    
                    Ok(())
                } else {
                    Err("'break' fora de um loop".to_string())
                }
            }

            Stmt::Continue(loc) => {
                // Sai do escopo do loop para ir para a atualização/condição
                if let Some(loop_info) = self.loop_stack.last() {
                    // Remove variáveis locais até a profundidade do loop
                    let pop_count = self.locals.len() - loop_info.scope_depth;
                    if pop_count > 0 {
                        if pop_count == 1 {
                            self.emit_op(OpCode::Pop, loc.line);
                        } else {
                            self.emit_op(OpCode::PopN(pop_count as u8), loc.line);
                        }
                    }
                    
                    // Jump de volta ao início do loop
                    let offset = self.current_chunk.len() - loop_info.start_pos + 1;
                    self.emit_op(OpCode::Loop(offset as u16), loc.line);
                    
                    Ok(())
                } else {
                    Err("'continue' fora de um loop".to_string())
                }
            }

            Stmt::Return(expr, loc) => {
                if let Some(expr) = expr {
                    self.compile_expression(expr)?;
                } else {
                    self.emit_op(OpCode::Nil, loc.line);
                }
                self.emit_op(OpCode::Return, loc.line);
                Ok(())
            }

            Stmt::FunctionDeclaration { name, params, body, location, .. } => {
                self.compile_function_declaration(name, params, *body, location.line)
            }

            Stmt::Print(expr, loc) => {
                self.compile_expression(expr)?;
                self.emit_op(OpCode::PrintLn, loc.line);
                Ok(())
            }

            Stmt::ClassDeclaration(name, superclass, _interfaces, members, loc) => {
                self.compile_class_declaration(name, superclass, members, loc.line)
            }

            Stmt::Try(try_block, catch_clause, finally_clause, loc) => {
                self.compile_try_catch(*try_block, catch_clause, finally_clause, loc.line)
            }

            Stmt::Throw(expr, loc) => {
                self.compile_expression(expr)?;
                self.emit_op(OpCode::Throw, loc.line);
                Ok(())
            }

            // Statements não implementados ainda
            _ => {
                // Para statements não suportados
                Err(format!("Statement ainda não suportado pelo bytecode: {:?}", stmt))
            }
        }
    }

    fn compile_var_declaration(
        &mut self,
        pattern: Pattern,
        initializer: Option<Expr>,
        line: usize,
    ) -> Result<(), String> {
        // Compila o valor inicial
        if let Some(expr) = initializer {
            self.compile_expression(expr)?;
        } else {
            self.emit_op(OpCode::Nil, line);
        }

        match pattern {
            Pattern::Identifier(name) => {
                if self.scope_depth > 0 {
                    // Variável local
                    self.add_local(name);
                } else {
                    // Variável global
                    let idx = self.make_constant(Value::String(name), line)?;
                    self.emit_op(OpCode::DefineGlobal(idx), line);
                }
            }
            _ => {
                return Err(format!("Padrões complexos ainda não suportados"));
            }
        }

        Ok(())
    }

    fn compile_assignment(
        &mut self,
        pattern: Pattern,
        value: Expr,
        line: usize,
    ) -> Result<(), String> {
        match pattern {
            Pattern::Identifier(name) => {
                self.compile_expression(value)?;

                if let Some(local_idx) = self.resolve_local(&name) {
                    // Atribuição a variável local
                    self.emit_op(OpCode::SetLocal(local_idx), line);
                } else {
                    // Atribuição a variável global
                    let idx = self.make_constant(Value::String(name), line)?;
                    self.emit_op(OpCode::SetGlobal(idx), line);
                }
            }
            _ => {
                return Err(format!("Padrões em atribuição ainda não suportados"));
            }
        }

        Ok(())
    }

    fn compile_property_assignment(
        &mut self,
        object: Expr,
        property: String,
        value: Expr,
        line: usize,
    ) -> Result<(), String> {
        // Compila o objeto, valor e índice
        self.compile_expression(object)?;
        self.compile_expression(value)?;
        
        // TODO: Implementar set property
        // Por enquanto, apenas deixamos os valores na pilha
        self.emit_op(OpCode::Pop, line);
        self.emit_op(OpCode::Pop, line);
        
        Err(format!("Atribuição de propriedade ainda não implementada"))
    }

    fn compile_index_assignment(
        &mut self,
        array: Expr,
        index: Expr,
        value: Expr,
        line: usize,
    ) -> Result<(), String> {
        // Compila array, índice e valor
        self.compile_expression(array)?;
        self.compile_expression(index)?;
        self.compile_expression(value)?;
        
        // TODO: Implementar SetIndex
        self.emit_op(OpCode::SetIndex, line);
        
        Ok(())
    }

    fn compile_if(
        &mut self,
        condition: Expr,
        then_branch: Stmt,
        else_branch: Option<Stmt>,
        line: usize,
    ) -> Result<(), String> {
        // Compila a condição
        self.compile_expression(condition)?;

        // Jump para else (ou fim do if)
        let then_jump = self.emit_jump(OpCode::JumpIfFalse(0), line);
        
        // Remove a condição da pilha
        self.emit_op(OpCode::Pop, line);

        // Compila o bloco then
        self.compile_statement(then_branch)?;

        // Jump para o fim
        let else_jump = self.emit_jump(OpCode::Jump(0), line);

        // Patch do jump then
        self.patch_jump(then_jump);
        
        // Remove a condição (caso o jump não tenha sido tomado)
        self.emit_op(OpCode::Pop, line);

        // Compila o else (se existir)
        if let Some(else_stmt) = else_branch {
            self.compile_statement(else_stmt)?;
        }

        // Patch do jump else
        self.patch_jump(else_jump);

        Ok(())
    }

    fn compile_while(
        &mut self,
        condition: Expr,
        body: Stmt,
        line: usize,
    ) -> Result<(), String> {
        // Marca o início do loop
        let loop_start = self.current_chunk.len();

        // Adiciona informação do loop na pilha
        self.loop_stack.push(LoopInfo {
            start_pos: loop_start,
            breaks: Vec::new(),
            scope_depth: self.scope_depth,
        });

        // Compila a condição
        self.compile_expression(condition)?;

        // Jump para fora do loop se falso
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0), line);

        // Remove a condição
        self.emit_op(OpCode::Pop, line);

        // Compila o corpo
        self.compile_statement(body)?;

        // Loop de volta
        let offset = self.current_chunk.len() - loop_start + 1;
        self.emit_op(OpCode::Loop(offset as u16), line);

        // Patch do exit jump
        self.patch_jump(exit_jump);
        
        // Remove a condição final
        self.emit_op(OpCode::Pop, line);

        // Resolve os breaks
        if let Some(loop_info) = self.loop_stack.pop() {
            for break_jump in loop_info.breaks {
                self.patch_jump(break_jump);
            }
        }

        Ok(())
    }

    fn compile_do_while(
        &mut self,
        body: Stmt,
        condition: Expr,
        line: usize,
    ) -> Result<(), String> {
        // Marca o início
        let loop_start = self.current_chunk.len();

        // Compila o corpo
        self.compile_statement(body)?;

        // Compila a condição
        self.compile_expression(condition)?;

        // Continua se verdadeiro
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0), line);

        // Remove condição
        self.emit_op(OpCode::Pop, line);

        // Loop
        let offset = self.current_chunk.len() - loop_start + 1;
        self.emit_op(OpCode::Loop(offset as u16), line);

        // Patch exit
        self.patch_jump(exit_jump);
        self.emit_op(OpCode::Pop, line);

        Ok(())
    }

    fn compile_for(
        &mut self,
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Box<Stmt>>,
        body: Stmt,
        line: usize,
    ) -> Result<(), String> {
        self.begin_scope();

        // Inicialização
        if let Some(init_stmt) = init {
            self.compile_statement(*init_stmt)?;
        }

        // Marca início do loop
        let loop_start = self.current_chunk.len();

        // Adiciona informação do loop na pilha
        self.loop_stack.push(LoopInfo {
            start_pos: loop_start,
            breaks: Vec::new(),
            scope_depth: self.scope_depth,
        });

        // Condição
        let exit_jump = if let Some(cond) = condition {
            self.compile_expression(cond)?;
            let jump = self.emit_jump(OpCode::JumpIfFalse(0), line);
            self.emit_op(OpCode::Pop, line);
            Some(jump)
        } else {
            None
        };

        // Corpo
        self.compile_statement(body)?;

        // Atualização
        if let Some(update_stmt) = update {
            self.compile_statement(*update_stmt)?;
        }

        // Loop
        let offset = self.current_chunk.len() - loop_start + 1;
        self.emit_op(OpCode::Loop(offset as u16), line);

        // Patch exit
        if let Some(exit) = exit_jump {
            self.patch_jump(exit);
            self.emit_op(OpCode::Pop, line);
        }

        // Resolve os breaks
        if let Some(loop_info) = self.loop_stack.pop() {
            for break_jump in loop_info.breaks {
                self.patch_jump(break_jump);
            }
        }

        self.end_scope(line);
        Ok(())
    }

    fn compile_foreach(
        &mut self,
        pattern: Pattern,
        iterable: Expr,
        body: Stmt,
        line: usize,
    ) -> Result<(), String> {
        self.begin_scope();

        // Compila o iterable e armazena em variável local temporária
        self.compile_expression(iterable)?;
        self.add_local("__iterable".to_string());

        // Inicializa índice em 0
        let zero_idx = self.make_constant(Value::Number(0.0), line)?;
        self.emit_op(OpCode::Constant(zero_idx), line);
        self.add_local("__index".to_string());

        // Marca início do loop
        let loop_start = self.current_chunk.len();

        // Verifica se índice < tamanho do array
        // Compila: __index < len(__iterable)
        self.emit_op(OpCode::GetLocal(self.resolve_local("__index").unwrap()), line);
        
        // Para obter o tamanho, precisamos de um opcode de len
        // Por enquanto, vamos usar uma abordagem diferente:
        // Tentamos acessar o índice e se falhar (nil), saímos do loop
        self.emit_op(OpCode::GetLocal(self.resolve_local("__iterable").unwrap()), line);
        self.emit_op(OpCode::GetLocal(self.resolve_local("__index").unwrap()), line);
        self.emit_op(OpCode::Index, line);
        
        // Se o resultado for nil, sai do loop
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse(0), line);
        self.emit_op(OpCode::Pop, line); // Remove o valor nil/falso

        // Atribui à variável do padrão
        match pattern {
            Pattern::Identifier(name) => {
                // Duplica o valor do topo (elemento atual)
                self.emit_op(OpCode::Dup, line);
                // Adiciona como variável local
                self.add_local(name);
            }
            _ => {
                return Err("Padrões complexos em foreach ainda não suportados".to_string());
            }
        }

        // Compila o corpo
        self.compile_statement(body)?;

        // Remove a variável do padrão
        self.end_scope(line);
        // Recria o escopo temporário
        self.begin_scope();
        self.add_local("__iterable".to_string());
        self.add_local("__index".to_string());

        // Incrementa o índice: __index = __index + 1
        self.emit_op(OpCode::GetLocal(self.resolve_local("__index").unwrap()), line);
        let one_idx = self.make_constant(Value::Number(1.0), line)?;
        self.emit_op(OpCode::Constant(one_idx), line);
        self.emit_op(OpCode::Add, line);
        self.emit_op(OpCode::SetLocal(self.resolve_local("__index").unwrap()), line);
        self.emit_op(OpCode::Pop, line);

        // Loop de volta
        let offset = self.current_chunk.len() - loop_start + 1;
        self.emit_op(OpCode::Loop(offset as u16), line);

        // Patch do exit jump
        self.patch_jump(exit_jump);
        self.emit_op(OpCode::Pop, line); // Remove o valor que fez o jump

        // Limpa as variáveis temporárias
        self.end_scope(line);

        Ok(())
    }

    fn compile_try_catch(
        &mut self,
        try_block: Stmt,
        catch_clause: Option<(String, Box<Stmt>)>,
        finally_clause: Option<Box<Stmt>>,
        line: usize,
    ) -> Result<(), String> {
        // Emite início do try
        // TryBegin tem dois offsets: para catch e para finally
        let try_begin_pos = self.current_chunk.len();
        self.emit_op(OpCode::TryBegin(0, 0), line);

        // Compila o bloco try
        self.compile_statement(try_block)?;

        // Fim do try
        self.emit_op(OpCode::TryEnd, line);

        // Jump para finally (ou fim se não houver finally)
        let finally_jump = self.emit_jump(OpCode::Jump(0), line);

        // Posição do catch
        let catch_pos = self.current_chunk.len();

        // Compila o catch se existir
        if let Some((var_name, catch_body)) = catch_clause {
            self.begin_scope();
            
            // Captura a exceção na variável
            let var_idx = self.make_constant(Value::String(var_name), line)?;
            self.emit_op(OpCode::Catch(var_idx), line);
            self.add_local(var_name);
            
            // Compila o corpo do catch
            self.compile_statement(*catch_body)?;
            
            self.end_scope(line);
        }

        // Posição do finally
        let finally_pos = self.current_chunk.len();

        // Compila o finally se existir
        if let Some(finally_body) = finally_clause {
            self.compile_statement(*finally_body)?;
        }

        // Patch do jump do try para o finally
        self.patch_jump(finally_jump);

        // Patch do TryBegin com os offsets corretos
        if let Some(OpCode::TryBegin(ref mut catch_offset, ref mut finally_offset)) = 
            self.current_chunk.code.get_mut(try_begin_pos) {
            *catch_offset = (catch_pos - try_begin_pos) as u16;
            *finally_offset = (finally_pos - try_begin_pos) as u16;
        }

        Ok(())
    }

    fn compile_function_declaration(
        &mut self,
        name: String,
        params: Vec<(String, Option<Type>)>,
        body: Stmt,
        line: usize,
    ) -> Result<(), String> {
        // Salva o chunk atual
        let enclosing_chunk = std::mem::replace(&mut self.current_chunk, Chunk::new(&name));
        let enclosing_locals = std::mem::take(&mut self.locals);
        let enclosing_scope = self.scope_depth;
        
        // Reseta para novo escopo de função
        self.scope_depth = 1;  // Começa em 1 para permitir variáveis locais
        self.locals.clear();
        
        // Adiciona parâmetros como variáveis locais
        for (param_name, _) in &params {
            self.add_local(param_name.clone());
        }
        
        // Compila o corpo da função
        self.compile_statement(body)?;
        
        // Garante que há um retorno no final
        self.emit_op(OpCode::Nil, line);
        self.emit_op(OpCode::Return, line);
        
        // Cria a função
        let function_chunk = std::mem::replace(&mut self.current_chunk, enclosing_chunk);
        let function = crate::value::Function {
            name: name.clone(),
            arity: params.len(),
            chunk: function_chunk,
            upvalue_count: 0,
        };
        
        // Restaura estado anterior
        self.locals = enclosing_locals;
        self.scope_depth = enclosing_scope;
        
        // Emite instrução para criar a função
        let idx = self.make_constant(crate::value::Value::Function(std::rc::Rc::new(function)), line)?;
        self.emit_op(OpCode::Constant(idx), line);
        
        // Define a função como variável global ou local
        if self.scope_depth > 0 {
            self.add_local(name);
        } else {
            let name_idx = self.make_constant(crate::value::Value::String(name), line)?;
            self.emit_op(OpCode::DefineGlobal(name_idx), line);
        }
        
        Ok(())
    }

    fn compile_class_declaration(
        &mut self,
        name: String,
        superclass: Option<String>,
        members: Vec<ClassMember>,
        line: usize,
    ) -> Result<(), String> {
        use dryad_parser::ast::ClassMember;

        // Cria a classe no heap
        let name_idx = self.make_constant(crate::value::Value::String(name.clone()), line)?;
        self.emit_op(OpCode::Class(name_idx), line);

        // Se tem superclasse, carrega ela
        if let Some(super_name) = superclass {
            let super_idx = self.make_constant(crate::value::Value::String(super_name), line)?;
            self.emit_op(OpCode::GetGlobal(super_idx), line);
            // TODO: Implementar herança completa (super)
        }

        // Compila métodos
        for member in members {
            match member {
                ClassMember::Method {
                    name: method_name,
                    params,
                    body,
                    visibility,
                    is_static,
                    is_async,
                    return_type,
                } => {
                    // Compila o método como uma função
                    let mut method_compiler = Compiler::new();
                    method_compiler.scope_depth = 1;
                    
                    // Adiciona 'this' como primeira variável local
                    method_compiler.add_local("this".to_string());
                    
                    // Adiciona parâmetros
                    for (param_name, _) in &params {
                        method_compiler.add_local(param_name.clone());
                    }
                    
                    // Compila corpo
                    method_compiler.compile_statement(*body)?;
                    
                    // Garante retorno
                    method_compiler.emit_op(OpCode::Nil, line);
                    method_compiler.emit_op(OpCode::Return, line);
                    
                    // Cria a função do método
                    let method_function = crate::value::Function {
                        name: method_name.clone(),
                        arity: params.len(),
                        chunk: method_compiler.current_chunk,
                        upvalue_count: 0,
                    };
                    
                    // Emite o método
                    let method_idx = self.make_constant(
                        crate::value::Value::Function(std::rc::Rc::new(method_function)),
                        line
                    )?;
                    self.emit_op(OpCode::Constant(method_idx), line);
                    
                    let method_name_idx = self.make_constant(
                        crate::value::Value::String(method_name),
                        line
                    )?;
                    self.emit_op(OpCode::Method(method_name_idx), line);
                }
                ClassMember::Property(_, _, prop_name, _, default) => {
                    // Propriedade - se tiver valor default, compila
                    if let Some(default_expr) = default {
                        self.compile_expression(default_expr)?;
                        let prop_name_idx = self.make_constant(
                            crate::value::Value::String(prop_name),
                            line
                        )?;
                        self.emit_op(OpCode::SetProperty(prop_name_idx), line);
                    }
                }
                _ => {
                    // Getters e setters - não implementados ainda
                }
            }
        }

        // Define a classe como variável global
        let class_name_idx = self.make_constant(crate::value::Value::String(name), line)?;
        self.emit_op(OpCode::DefineGlobal(class_name_idx), line);

        Ok(())
    }

    // ============================================
    // Compilação de Expressões
    // ============================================

    fn compile_expression(&mut self, expr: Expr) -> Result<(), String> {
        match expr {
            Expr::Literal(lit, loc) => {
                self.compile_literal(lit, loc.line)
            }

            Expr::Variable(name, loc) => {
                self.compile_variable(name, loc.line)
            }

            Expr::Binary(left, op, right, loc) => {
                self.compile_binary(*left, op, *right, loc.line)
            }

            Expr::Unary(op, expr, loc) => {
                self.compile_unary(op, *expr, loc.line)
            }

            Expr::PostIncrement(expr, loc) => {
                self.compile_post_increment(*expr, true, loc.line)
            }

            Expr::PostDecrement(expr, loc) => {
                self.compile_post_increment(*expr, false, loc.line)
            }

            Expr::PreIncrement(expr, loc) => {
                self.compile_pre_increment(*expr, true, loc.line)
            }

            Expr::PreDecrement(expr, loc) => {
                self.compile_pre_increment(*expr, false, loc.line)
            }

            Expr::Array(elements, loc) => {
                self.compile_array(elements, loc.line)
            }

            Expr::Tuple(elements, loc) => {
                self.compile_tuple(elements, loc.line)
            }

            Expr::Index(array, index, loc) => {
                self.compile_index(*array, *index, loc.line)
            }

            Expr::TupleAccess(tuple, idx, loc) => {
                self.compile_tuple_access(*tuple, idx, loc.line)
            }

            Expr::PropertyAccess(object, property, loc) => {
                self.compile_property_access(*object, property, loc.line)
            }

            Expr::MethodCall(object, method, args, loc) => {
                self.compile_method_call(*object, method, args, loc.line)
            }

            Expr::Call(callee, args, loc) => {
                self.compile_call(*callee, args, loc.line)
            }

            Expr::ClassInstantiation(class_name, args, loc) => {
                self.compile_class_instantiation(class_name, args, loc.line)
            }

            // Expressões não implementadas
            _ => {
                Err(format!("Expressão ainda não suportada pelo bytecode: {:?}", expr))
            }
        }
    }

    fn compile_literal(&mut self, lit: Literal, line: usize) -> Result<(), String> {
        match lit {
            Literal::Null => self.emit_op(OpCode::Nil, line),
            Literal::Bool(b) => {
                if b {
                    self.emit_op(OpCode::True, line)
                } else {
                    self.emit_op(OpCode::False, line)
                }
            }
            Literal::Number(n) => {
                let idx = self.make_constant(Value::Number(n), line)?;
                self.emit_op(OpCode::Constant(idx), line);
            }
            Literal::String(s) => {
                let idx = self.make_constant(Value::String(s), line)?;
                self.emit_op(OpCode::Constant(idx), line);
            }
        }
        Ok(())
    }

    fn compile_variable(&mut self, name: String, line: usize) -> Result<(), String> {
        if let Some(local_idx) = self.resolve_local(&name) {
            // Variável local
            self.emit_op(OpCode::GetLocal(local_idx), line);
        } else {
            // Variável global
            let idx = self.make_constant(Value::String(name), line)?;
            self.emit_op(OpCode::GetGlobal(idx), line);
        }
        Ok(())
    }

    fn compile_binary(
        &mut self,
        left: Expr,
        op: String,
        right: Expr,
        line: usize,
    ) -> Result<(), String> {
        // Compila operandos
        self.compile_expression(left)?;
        self.compile_expression(right)?;

        // Emite operação
        match op.as_str() {
            "+" => self.emit_op(OpCode::Add, line),
            "-" => self.emit_op(OpCode::Subtract, line),
            "*" => self.emit_op(OpCode::Multiply, line),
            "/" => self.emit_op(OpCode::Divide, line),
            "%" => self.emit_op(OpCode::Modulo, line),
            "==" => self.emit_op(OpCode::Equal, line),
            "!=" => {
                self.emit_op(OpCode::Equal, line);
                self.emit_op(OpCode::Not, line);
            }
            ">" => self.emit_op(OpCode::Greater, line),
            "<" => self.emit_op(OpCode::Less, line),
            ">=" => self.emit_op(OpCode::GreaterEqual, line),
            "<=" => self.emit_op(OpCode::LessEqual, line),
            "&&" => self.emit_op(OpCode::And, line),
            "||" => self.emit_op(OpCode::Or, line),
            "&" => self.emit_op(OpCode::BitAnd, line),
            "|" => self.emit_op(OpCode::BitOr, line),
            "^" => self.emit_op(OpCode::BitXor, line),
            "<<" => self.emit_op(OpCode::ShiftLeft, line),
            ">>" => self.emit_op(OpCode::ShiftRight, line),
            _ => return Err(format!("Operador binário não suportado: {}", op)),
        }

        Ok(())
    }

    fn compile_unary(&mut self, op: String, expr: Expr, line: usize) -> Result<(), String> {
        self.compile_expression(expr)?;

        match op.as_str() {
            "-" => self.emit_op(OpCode::Negate, line),
            "!" => self.emit_op(OpCode::Not, line),
            "~" => self.emit_op(OpCode::BitNot, line),
            _ => return Err(format!("Operador unário não suportado: {}", op)),
        }

        Ok(())
    }

    fn compile_post_increment(
        &mut self,
        expr: Expr,
        is_increment: bool,
        line: usize,
    ) -> Result<(), String> {
        // x++ ou x--: retorna valor atual, depois incrementa/decrementa
        match expr {
            Expr::Variable(name, _) => {
                // Carrega o valor atual
                self.compile_variable(name.clone(), line)?;
                
                // Duplica para manter o valor original no topo (retorno)
                self.emit_op(OpCode::Dup, line);
                
                // Empilha 1
                let one_idx = self.make_constant(Value::Number(1.0), line)?;
                self.emit_op(OpCode::Constant(one_idx), line);
                
                // Adiciona ou subtrai
                if is_increment {
                    self.emit_op(OpCode::Add, line);
                } else {
                    self.emit_op(OpCode::Subtract, line);
                }
                
                // Armazena de volta
                if let Some(local_idx) = self.resolve_local(&name) {
                    self.emit_op(OpCode::SetLocal(local_idx), line);
                } else {
                    let name_idx = self.make_constant(Value::String(name), line)?;
                    self.emit_op(OpCode::SetGlobal(name_idx), line);
                }
                self.emit_op(OpCode::Pop, line); // Remove o resultado do Set
                
                // O valor original ainda está no topo (por causa do Dup)
                Ok(())
            }
            _ => Err("Incremento/decremento só funciona com variáveis".to_string()),
        }
    }

    fn compile_pre_increment(
        &mut self,
        expr: Expr,
        is_increment: bool,
        line: usize,
    ) -> Result<(), String> {
        // ++x ou --x: incrementa/decrementa primeiro, depois retorna
        match expr {
            Expr::Variable(name, _) => {
                // Carrega o valor atual
                self.compile_variable(name.clone(), line)?;
                
                // Empilha 1
                let one_idx = self.make_constant(Value::Number(1.0), line)?;
                self.emit_op(OpCode::Constant(one_idx), line);
                
                // Adiciona ou subtrai
                if is_increment {
                    self.emit_op(OpCode::Add, line);
                } else {
                    self.emit_op(OpCode::Subtract, line);
                }
                
                // Duplica o novo valor (para retornar e armazenar)
                self.emit_op(OpCode::Dup, line);
                
                // Armazena de volta
                if let Some(local_idx) = self.resolve_local(&name) {
                    self.emit_op(OpCode::SetLocal(local_idx), line);
                } else {
                    let name_idx = self.make_constant(Value::String(name), line)?;
                    self.emit_op(OpCode::SetGlobal(name_idx), line);
                }
                self.emit_op(OpCode::Pop, line); // Remove o resultado do Set
                
                // O novo valor está no topo (por causa do Dup)
                Ok(())
            }
            _ => Err("Incremento/decremento só funciona com variáveis".to_string()),
        }
    }

    fn compile_array(&mut self, elements: Vec<Expr>, line: usize) -> Result<(), String> {
        // Compila os elementos
        for elem in elements {
            self.compile_expression(elem)?;
        }

        // Cria o array
        let count = elements.len();
        if count > u16::MAX as usize {
            return Err("Array muito grande".to_string());
        }
        self.emit_op(OpCode::Array(count as u16), line);

        Ok(())
    }

    fn compile_tuple(&mut self, elements: Vec<Expr>, line: usize) -> Result<(), String> {
        // Compila os elementos
        for elem in elements {
            self.compile_expression(elem)?;
        }

        // Cria o tuple
        let count = elements.len();
        if count > u8::MAX as usize {
            return Err("Tuple muito grande".to_string());
        }
        self.emit_op(OpCode::Tuple(count as u8), line);

        Ok(())
    }

    fn compile_index(
        &mut self,
        array: Expr,
        index: Expr,
        line: usize,
    ) -> Result<(), String> {
        // Compila array e índice
        self.compile_expression(array)?;
        self.compile_expression(index)?;

        // Indexa
        self.emit_op(OpCode::Index, line);

        Ok(())
    }

    fn compile_tuple_access(
        &mut self,
        tuple: Expr,
        idx: usize,
        line: usize,
    ) -> Result<(), String> {
        // Compila o tuple
        self.compile_expression(tuple)?;

        // Acessa o elemento
        if idx > u8::MAX as usize {
            return Err("Índice de tuple muito grande".to_string());
        }
        self.emit_op(OpCode::TupleAccess(idx as u8), line);

        Ok(())
    }

    fn compile_property_access(
        &mut self,
        object: Expr,
        property: String,
        line: usize,
    ) -> Result<(), String> {
        // Compila o objeto
        self.compile_expression(object)?;

        // Obtém a propriedade
        let idx = self.make_constant(Value::String(property), line)?;
        self.emit_op(OpCode::GetProperty(idx), line);

        Ok(())
    }

    fn compile_method_call(
        &mut self,
        object: Expr,
        method: String,
        args: Vec<Expr>,
        line: usize,
    ) -> Result<(), String> {
        // Compila o objeto
        self.compile_expression(object)?;

        // Compila os argumentos
        for arg in args.iter() {
            self.compile_expression(arg.clone())?;
        }

        // Invoca o método
        let idx = self.make_constant(Value::String(method), line)?;
        self.emit_op(OpCode::Invoke(args.len() as u8), line);

        Ok(())
    }

    fn compile_call(
        &mut self,
        callee: Expr,
        args: Vec<Expr>,
        line: usize,
    ) -> Result<(), String> {
        // Compila a função
        self.compile_expression(callee)?;

        // Compila os argumentos
        for arg in args.iter() {
            self.compile_expression(arg.clone())?;
        }

        // Chama a função
        self.emit_op(OpCode::Call(args.len() as u8), line);

        Ok(())
    }

    fn compile_class_instantiation(
        &mut self,
        class_name: String,
        args: Vec<Expr>,
        line: usize,
    ) -> Result<(), String> {
        // Carrega a classe
        let class_idx = self.make_constant(Value::String(class_name), line)?;
        self.emit_op(OpCode::GetGlobal(class_idx), line);

        // Compila argumentos
        for arg in args.iter() {
            self.compile_expression(arg.clone())?;
        }

        // Cria instância e chama construtor
        // TODO: Implementar construtor adequadamente
        // Por enquanto, apenas cria a instância
        self.emit_op(OpCode::Call(args.len() as u8), line);

        Ok(())
    }

    // ============================================
    // Gerenciamento de Escopo
    // ============================================

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self, line: usize) {
        self.scope_depth -= 1;

        // Remove variáveis locais deste escopo
        let mut pop_count = 0;
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                if local.is_captured {
                    // TODO: Implementar CloseUpvalue
                }
                self.locals.pop();
                pop_count += 1;
            } else {
                break;
            }
        }

        if pop_count == 1 {
            self.emit_op(OpCode::Pop, line);
        } else if pop_count > 1 {
            self.emit_op(OpCode::PopN(pop_count as u8), line);
        }
    }

    fn add_local(&mut self, name: String) {
        self.locals.push(Local {
            name,
            depth: self.scope_depth,
            is_captured: false,
        });
    }

    fn resolve_local(&self, name: &str) -> Option<u8> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i as u8);
            }
        }
        None
    }

    // ============================================
    // Emissão de Código
    // ============================================

    fn emit_op(&mut self, op: OpCode, line: usize) {
        self.current_chunk.push_op(op, line);
    }

    fn make_constant(&mut self, value: Value, line: usize) -> Result<u8, String> {
        match self.current_chunk.add_constant(value) {
            Ok(idx) => Ok(idx),
            Err(_) => {
                // Tabela cheia, tenta com ConstantLong
                let idx = self.current_chunk.add_constant_long(value)?;
                // Emite ConstantLong e retorna índice especial
                self.emit_op(OpCode::ConstantLong(idx), line);
                Ok(u8::MAX) // Índice especial indica ConstantLong
            }
        }
    }

    fn emit_jump(&mut self, op: OpCode, line: usize) -> usize {
        self.emit_op(op, line);
        self.current_chunk.len() - 1
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.current_chunk.len() - offset - 1;

        // Atualiza o opcode com o offset correto
        if let Some(op) = self.current_chunk.code.get_mut(offset) {
            match op {
                OpCode::Jump(ref mut off) => *off = jump as u16,
                OpCode::JumpIfFalse(ref mut off) => *off = jump as u16,
                OpCode::JumpIfTrue(ref mut off) => *off = jump as u16,
                _ => {}
            }
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_loc() -> SourceLocation {
        SourceLocation {
            line: 1,
            column: 1,
            file: None,
        }
    }

    #[test]
    fn test_compile_simple_expression() {
        let mut compiler = Compiler::new();
        // Programa simples: 1 + 2
        let program = Program {
            statements: vec![
                Stmt::Expression(
                    Expr::Binary(
                        Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
                        "+".to_string(),
                        Box::new(Expr::Literal(Literal::Number(2.0), dummy_loc())),
                        dummy_loc(),
                    ),
                    dummy_loc(),
                )
            ]
        };

        let result = compiler.compile(program);
        assert!(result.is_ok());
    }
}
