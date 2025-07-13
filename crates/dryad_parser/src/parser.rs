// crates/dryad_parser/src/parser.rs
use dryad_errors::DryadError;
use dryad_lexer::token::Token;
use crate::ast::{Expr, Literal, Stmt, Program, ClassMember, Visibility};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, DryadError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if let Some(stmt) = self.statement()? {
                statements.push(stmt);
            }
        }
        
        Ok(Program { statements })
    }

    pub fn statement(&mut self) -> Result<Option<Stmt>, DryadError> {
        match self.peek() {
            Token::Eof => Ok(None),
            Token::Symbol(';') => {
                // Semicolon vazio - consome e retorna None (não cria statement)
                self.advance();
                Ok(None)
            }
            Token::Symbol('{') => {
                Ok(Some(self.block_statement()?))
            }
            Token::NativeDirective(module_name) => {
                let module = module_name.clone();
                self.advance(); // consome a diretiva
                Ok(Some(Stmt::NativeDirective(module)))
            }
            Token::Keyword(keyword) if keyword == "let" => {
                Ok(Some(self.var_declaration()?))
            }
            Token::Keyword(keyword) if keyword == "if" => {
                Ok(Some(self.if_statement()?))
            }
            Token::Keyword(keyword) if keyword == "while" => {
                Ok(Some(self.while_statement()?))
            }
            Token::Keyword(keyword) if keyword == "do" => {
                Ok(Some(self.do_while_statement()?))
            }
            Token::Keyword(keyword) if keyword == "for" => {
                Ok(Some(self.for_statement()?))
            }
            Token::Keyword(keyword) if keyword == "break" => {
                Ok(Some(self.break_statement()?))
            }
            Token::Keyword(keyword) if keyword == "continue" => {
                Ok(Some(self.continue_statement()?))
            }
            Token::Keyword(keyword) if keyword == "try" => {
                Ok(Some(self.try_statement()?))
            }
            Token::Keyword(keyword) if keyword == "throw" => {
                Ok(Some(self.throw_statement()?))
            }
            Token::Keyword(keyword) if keyword == "function" => {
                Ok(Some(self.function_declaration()?))
            }
            Token::Keyword(keyword) if keyword == "class" => {
                Ok(Some(self.class_declaration()?))
            }
            Token::Keyword(keyword) if keyword == "export" => {
                Ok(Some(self.export_statement()?))
            }
            Token::Keyword(keyword) if keyword == "use" => {
                Ok(Some(self.use_statement()?))
            }
            Token::Keyword(keyword) if keyword == "return" => {
                Ok(Some(self.return_statement()?))
            }
            _ => {
                // Verifica se é assignment (identificador seguido de = ou +=, -=, etc.)
                if let Token::Identifier(_) = self.peek() {
                    let checkpoint = self.position;
                    self.advance(); // consume identifier
                    
                    match self.peek() {
                        Token::Symbol('=') => {
                            // É um assignment simples, volta e processa
                            self.position = checkpoint;
                            let stmt = self.assignment_statement()?;
                            self.consume_semicolon()?;
                            Ok(Some(stmt))
                        }
                        Token::Operator(op) if op == "+=" || op == "-=" || op == "*=" || op == "/=" => {
                            // É um assignment composto, volta e processa
                            self.position = checkpoint;
                            let stmt = self.assignment_statement()?;
                            self.consume_semicolon()?;
                            Ok(Some(stmt))
                        }
                        Token::Symbol('.') => {
                            // Pode ser property access seguido de assignment
                            self.advance(); // consume '.'
                            if let Token::Identifier(_) = self.peek() {
                                self.advance(); // consume property name
                                if matches!(self.peek(), Token::Symbol('=')) {
                                    // É property assignment, volta ao início e processa
                                    self.position = checkpoint;
                                    let stmt = self.property_assignment_statement()?;
                                    self.consume_semicolon()?;
                                    return Ok(Some(stmt));
                                }
                            }
                            // Não é property assignment, volta e trata como expressão
                            self.position = checkpoint;
                            let expr = self.expression()?;
                            self.consume_semicolon()?;
                            Ok(Some(Stmt::Expression(expr)))
                        }
                        _ => {
                            // Não é assignment, volta e trata como expressão
                            self.position = checkpoint;
                            let expr = self.expression()?;
                            self.consume_semicolon()?;
                            Ok(Some(Stmt::Expression(expr)))
                        }
                    }
                } else if let Token::Keyword(k) = self.peek() {
                    if k == "this" {
                        // Pode ser this.property = value
                        let checkpoint = self.position;
                        self.advance(); // consume 'this'
                        if matches!(self.peek(), Token::Symbol('.')) {
                            self.advance(); // consume '.'
                            if let Token::Identifier(_) = self.peek() {
                                self.advance(); // consume property name
                                if matches!(self.peek(), Token::Symbol('=')) {
                                    // É this property assignment
                                    self.position = checkpoint;
                                    let stmt = self.property_assignment_statement()?;
                                    self.consume_semicolon()?;
                                    return Ok(Some(stmt));
                                }
                            }
                        }
                        // Não é property assignment, volta e trata como expressão
                        self.position = checkpoint;
                        let expr = self.expression()?;
                        self.consume_semicolon()?;
                        Ok(Some(Stmt::Expression(expr)))
                    } else {
                        // Não é identificador ou this, trata como expressão
                        let expr = self.expression()?;
                        self.consume_semicolon()?;
                        Ok(Some(Stmt::Expression(expr)))
                    }
                } else {
                    // Não é identificador ou this, trata como expressão
                    let expr = self.expression()?;
                    self.consume_semicolon()?;
                    Ok(Some(Stmt::Expression(expr)))
                }
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'let'
        
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(DryadError::new(2011, "Esperado nome da variável após 'let'")),
        };

        let initializer = if matches!(self.peek(), Token::Symbol('=')) {
            self.advance(); // consume '='
            Some(self.expression()?)
        } else {
            None
        };

        self.consume_semicolon()?;
        Ok(Stmt::VarDeclaration(name, initializer))
    }

    fn consume_semicolon(&mut self) -> Result<(), DryadError> {
        if matches!(self.peek(), Token::Symbol(';')) {
            self.advance();
            Ok(())
        } else {
            // Semicolon é opcional em algumas situações (EOF, fim de bloco)
            if self.is_at_end() || matches!(self.peek(), Token::Symbol('}')) {
                Ok(())
            } else {
                Err(DryadError::new(2003, "Esperado ';' após declaração"))
            }
        }
    }

    pub fn expression(&mut self) -> Result<Expr, DryadError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, DryadError> {
        let expr = self.logical_or()?;

        if matches!(self.peek(), Token::Symbol('=')) {
            self.advance(); // consume '='
            let _value = self.assignment()?;
            
            match expr {
                Expr::Variable(_name) => {
                    // Assignments de variáveis simples ainda não suportados como expressão
                    return Err(DryadError::new(2008, "Atribuição de variável ainda não suportada em expressões"));
                }
                Expr::PropertyAccess(_, _) => {
                    // Property assignments também não suportados como expressão
                    return Err(DryadError::new(2008, "Atribuição de propriedade ainda não suportada em expressões"));
                }
                _ => {
                    return Err(DryadError::new(2008, "Target de atribuição inválido"));
                }
            }
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.logical_and()?;

        while self.match_operator("||") {
            let operator = "||".to_string();
            let right = self.logical_and()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.bitwise_or()?;

        while self.match_operator("&&") {
            let operator = "&&".to_string();
            let right = self.bitwise_or()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn bitwise_or(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.bitwise_xor()?;

        while self.match_operator("|") {
            let operator = "|".to_string();
            let right = self.bitwise_xor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn bitwise_xor(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.bitwise_and()?;

        while self.match_operator("^") {
            let operator = "^".to_string();
            let right = self.bitwise_and()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn bitwise_and(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.equality()?;

        while self.match_operator("&") {
            let operator = "&".to_string();
            let right = self.equality()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.comparison()?;

        while self.match_any_operator(&["==", "!="]) {
            let operator = self.previous_operator().unwrap();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.shift()?;

        while self.match_any_operator(&[">", ">=", "<", "<="]) {
            let operator = self.previous_operator().unwrap();
            let right = self.shift()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn shift(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.term()?;

        while self.match_any_operator(&["<<", ">>", "<<<", ">>>"]) {
            let operator = self.previous_operator().unwrap();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.factor()?;

        while self.match_any_operator(&["-", "+"]) {
            let operator = self.previous_operator().unwrap();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.power()?;

        while self.match_any_operator(&["/", "*", "%", "%%"]) {
            let operator = self.previous_operator().unwrap();
            let right = self.power()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.unary()?;

        // Operadores de potência têm associatividade à direita
        if self.match_any_operator(&["**", "^^", "##", "^"]) {
            let operator = self.previous_operator().unwrap();
            let right = self.power()?; // Recursão à direita para associatividade
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, DryadError> {
        if self.match_any_operator(&["!", "-"]) {
            let operator = self.previous_operator().unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        // Pré-incremento e pré-decremento
        if self.match_any_operator(&["++"]) {
            let expr = self.unary()?;
            return Ok(Expr::PreIncrement(Box::new(expr)));
        }
        
        if self.match_any_operator(&["--"]) {
            let expr = self.unary()?;
            return Ok(Expr::PreDecrement(Box::new(expr)));
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.primary()?;

        loop {
            match self.peek() {
                // Pós-incremento e pós-decremento
                Token::Operator(op) if op == "++" || op == "--" => {
                    let operator = op.clone();
                    self.advance();
                    match operator.as_str() {
                        "++" => expr = Expr::PostIncrement(Box::new(expr)),
                        "--" => expr = Expr::PostDecrement(Box::new(expr)),
                        _ => unreachable!(),
                    }
                }
                // Acesso a array: expr[index]
                Token::Symbol('[') => {
                    self.advance(); // consome '['
                    let index = self.expression()?;
                    if !matches!(self.peek(), Token::Symbol(']')) {
                        return Err(DryadError::new(2071, "Esperado ']' após índice do array"));
                    }
                    self.advance(); // consome ']'
                    expr = Expr::Index(Box::new(expr), Box::new(index));
                }
                // Acesso a tupla, propriedade ou método: expr.index/property/method
                Token::Symbol('.') => {
                    self.advance(); // consome '.'
                    match self.peek() {
                        Token::Number(index_num) => {
                            // Acesso a tupla: expr.index
                            let index = *index_num as usize;
                            self.advance();
                            expr = Expr::TupleAccess(Box::new(expr), index);
                        }
                        Token::Identifier(property_name) => {
                            // Acesso a propriedade ou chamada de método
                            let name = property_name.clone();
                            self.advance();
                            
                            // Check if this is a method call (followed by parentheses)
                            if matches!(self.peek(), Token::Symbol('(')) {
                                self.advance(); // consume '('
                                
                                let mut args = Vec::new();
                                
                                // Parse arguments if any
                                if !matches!(self.peek(), Token::Symbol(')')) {
                                    loop {
                                        let arg = self.expression()?;
                                        args.push(arg);
                                        
                                        match self.peek() {
                                            Token::Symbol(',') => {
                                                self.advance(); // consume comma
                                                continue;
                                            }
                                            Token::Symbol(')') => break,
                                            _ => return Err(DryadError::new(2073, "Esperado ',' ou ')' na lista de argumentos do método"))
                                        }
                                    }
                                }
                                
                                // Expect closing parenthesis
                                if !matches!(self.advance(), Token::Symbol(')')) {
                                    return Err(DryadError::new(2074, "Esperado ')' após argumentos do método"));
                                }
                                
                                expr = Expr::MethodCall(Box::new(expr), name, args);
                            } else {
                                // Property access
                                expr = Expr::PropertyAccess(Box::new(expr), name);
                            }
                        }
                        _ => {
                            return Err(DryadError::new(2072, "Esperado número ou identificador após '.' para acesso"));
                        }
                    }
                }
                // Chamada de função: expr(args...)
                Token::Symbol('(') => {
                    self.advance(); // consome '('
                    
                    let mut args = Vec::new();
                    
                    // Parse arguments if any
                    if !matches!(self.peek(), Token::Symbol(')')) {
                        loop {
                            let arg = self.expression()?;
                            args.push(arg);
                            
                            match self.peek() {
                                Token::Symbol(',') => {
                                    self.advance(); // consume comma
                                    continue;
                                }
                                Token::Symbol(')') => break,
                                _ => return Err(DryadError::new(2075, "Esperado ',' ou ')' na lista de argumentos da chamada"))
                            }
                        }
                    }
                    
                    // Expect closing parenthesis
                    if !matches!(self.advance(), Token::Symbol(')')) {
                        return Err(DryadError::new(2076, "Esperado ')' após argumentos da chamada"));
                    }
                    
                    expr = Expr::Call(Box::new(expr), args);
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, DryadError> {
        match &self.peek() {
            Token::Boolean(value) => {
                let val = *value;
                self.advance();
                Ok(Expr::Literal(Literal::Bool(val)))
            }
            Token::Number(value) => {
                let val = *value;
                self.advance();
                Ok(Expr::Literal(Literal::Number(val)))
            }
            Token::String(value) => {
                let val = value.clone();
                self.advance();
                Ok(Expr::Literal(Literal::String(val)))
            }
            Token::Literal(value) if value == "null" => {
                self.advance();
                Ok(Expr::Literal(Literal::Null))
            }
            Token::Keyword(k) if k == "this" => {
                self.advance();
                Ok(Expr::This)
            }
            Token::Keyword(k) if k == "super" => {
                self.advance();
                Ok(Expr::Super)
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                
                // Check if this is a lambda with single parameter (x => expr)
                if matches!(self.peek(), Token::Arrow) {
                    self.advance(); // consume '=>'
                    let body = self.expression()?;
                    return Ok(Expr::Lambda(vec![var_name], Box::new(body)));
                }
                
                // Check if this is a function call
                if matches!(self.peek(), Token::Symbol('(')) {
                    self.advance(); // consume '('
                    
                    let mut args = Vec::new();
                    
                    // Parse arguments if any
                    if !matches!(self.peek(), Token::Symbol(')')) {
                        loop {
                            let arg = self.expression()?;
                            args.push(arg);
                            
                            match self.peek() {
                                Token::Symbol(',') => {
                                    self.advance(); // consume comma
                                    continue;
                                }
                                Token::Symbol(')') => break,
                                _ => return Err(DryadError::new(2017, "Esperado ',' ou ')' na lista de argumentos"))
                            }
                        }
                    }
                    
                    // Expect closing parenthesis
                    if !matches!(self.advance(), Token::Symbol(')')) {
                        return Err(DryadError::new(2018, "Esperado ')' após argumentos"));
                    }
                    
                    Ok(Expr::Call(Box::new(Expr::Variable(var_name)), args))
                } else {
                    // Just a variable reference
                    Ok(Expr::Variable(var_name))
                }
            }
            Token::Symbol('[') => {
                // Array literal [expr1, expr2, ...]
                self.parse_array()
            }
            Token::Symbol('{') => {
                // Object literal { key: value, method() { ... } }
                self.parse_object_literal()
            }
            Token::Symbol('(') => {
                self.advance(); // consome '('
                
                // Verifica se é tupla vazia
                if matches!(self.peek(), Token::Symbol(')')) {
                    self.advance(); // consome ')'
                    // Verifica se tem => depois dos parênteses vazios para lambda
                    if matches!(self.peek(), Token::Arrow) {
                        self.advance(); // consome '=>'
                        let body = self.expression()?;
                        return Ok(Expr::Lambda(Vec::new(), Box::new(body)));
                    }
                    return Ok(Expr::Tuple(Vec::new()));
                }
                
                // Verifica se o primeiro token é um identificador (possível parâmetro de lambda)
                let start_pos = self.position;
                let mut is_lambda = false;
                let mut params = Vec::new();
                
                // Tenta fazer parsing como lista de parâmetros
                if let Token::Identifier(name) = self.peek() {
                    let param_name = name.clone();
                    self.advance();
                    
                    // Se tem vírgula ou parêntese fechando seguido de =>, é lambda
                    match self.peek() {
                        Token::Symbol(')') => {
                            self.advance(); // consome ')'
                            if matches!(self.peek(), Token::Arrow) {
                                is_lambda = true;
                                params.push(param_name);
                            } else {
                                // Reset position e parse como expressão normal
                                self.position = start_pos;
                            }
                        }
                        Token::Symbol(',') => {
                            // Múltiplos parâmetros - definitivamente é lambda
                            is_lambda = true;
                            params.push(param_name);
                            self.advance(); // consome ','
                            
                            while !matches!(self.peek(), Token::Symbol(')')) {
                                if let Token::Identifier(param) = self.advance() {
                                    params.push(param.clone());
                                } else {
                                    return Err(DryadError::new(2019, "Esperado identificador de parâmetro"));
                                }
                                
                                if matches!(self.peek(), Token::Symbol(',')) {
                                    self.advance(); // consome ','
                                } else {
                                    break;
                                }
                            }
                            
                            if !matches!(self.advance(), Token::Symbol(')')) {
                                return Err(DryadError::new(2020, "Esperado ')' após parâmetros da lambda"));
                            }
                        }
                        _ => {
                            // Reset position e parse como expressão normal
                            self.position = start_pos;
                        }
                    }
                }
                
                // Se detectamos lambda, processa
                if is_lambda {
                    if !matches!(self.advance(), Token::Arrow) {
                        return Err(DryadError::new(2021, "Esperado '=>' após parâmetros da lambda"));
                    }
                    let body = self.expression()?;
                    return Ok(Expr::Lambda(params, Box::new(body)));
                }
                
                // Se não é lambda, parse como expressão/tupla normal
                let first_expr = self.expression()?;
                
                // Se tem vírgula, é tupla; senão é expressão agrupada
                if matches!(self.peek(), Token::Symbol(',')) {
                    self.advance(); // consome ','
                    
                    let mut elements = vec![first_expr];
                    
                    // Continua lendo elementos da tupla
                    while !matches!(self.peek(), Token::Symbol(')')) {
                        let expr = self.expression()?;
                        elements.push(expr);
                        
                        if matches!(self.peek(), Token::Symbol(',')) {
                            self.advance(); // consome ','
                        } else {
                            break;
                        }
                    }
                    
                    if !matches!(self.peek(), Token::Symbol(')')) {
                        return Err(DryadError::new(2005, "Esperado ')' após tupla"));
                    }
                    self.advance(); // consome ')'
                    Ok(Expr::Tuple(elements))
                } else {
                    // Expressão agrupada simples
                    if !matches!(self.peek(), Token::Symbol(')')) {
                        return Err(DryadError::new(2005, "Esperado ')' após expressão"));
                    }
                    self.advance(); // consome ')'
                    Ok(first_expr)
                }
            }
            _ => {
                Err(DryadError::new(2001, &format!("Token inesperado: {:?}", self.peek())))
            }
        }
    }

    fn assignment_statement(&mut self) -> Result<Stmt, DryadError> {
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(DryadError::new(2012, "Esperado identificador para assignment")),
        };

        match self.advance() {
            Token::Symbol('=') => {
                // Assignment simples: x = value
                let value = self.expression()?;
                Ok(Stmt::Assignment(name, value))
            }
            Token::Operator(op) if op == "+=" => {
                // x += value  =>  x = x + value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone())),
                    "+".to_string(),
                    Box::new(value)
                );
                Ok(Stmt::Assignment(name, assignment_value))
            }
            Token::Operator(op) if op == "-=" => {
                // x -= value  =>  x = x - value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone())),
                    "-".to_string(),
                    Box::new(value)
                );
                Ok(Stmt::Assignment(name, assignment_value))
            }
            Token::Operator(op) if op == "*=" => {
                // x *= value  =>  x = x * value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone())),
                    "*".to_string(),
                    Box::new(value)
                );
                Ok(Stmt::Assignment(name, assignment_value))
            }
            Token::Operator(op) if op == "/=" => {
                // x /= value  =>  x = x / value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone())),
                    "/".to_string(),
                    Box::new(value)
                );
                Ok(Stmt::Assignment(name, assignment_value))
            }
            _ => Err(DryadError::new(2013, "Operador de assignment inválido")),
        }
    }

    fn export_statement(&mut self) -> Result<Stmt, DryadError> {
        // Consome 'export'
        self.advance();
        
        // O próximo token deve ser function, class, let ou variable
        match self.peek() {
            Token::Keyword(keyword) if keyword == "function" => {
                let func_stmt = self.function_declaration()?;
                Ok(Stmt::Export(Box::new(func_stmt)))
            }
            Token::Keyword(keyword) if keyword == "class" => {
                let class_stmt = self.class_declaration()?;
                Ok(Stmt::Export(Box::new(class_stmt)))
            }
            Token::Keyword(keyword) if keyword == "let" => {
                let var_stmt = self.var_declaration()?;
                Ok(Stmt::Export(Box::new(var_stmt)))
            }
            _ => {
                Err(DryadError::new(
                    4001,
                    "Export deve ser seguido por 'function', 'class' ou 'let'"
                ))
            }
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume '{'
        
        let mut statements = Vec::new();
        
        while !matches!(self.peek(), Token::Symbol('}')) && !self.is_at_end() {
            if let Some(stmt) = self.statement()? {
                statements.push(stmt);
            }
        }
        
        if !matches!(self.peek(), Token::Symbol('}')) {
            return Err(DryadError::new(2012, "Esperado '}' para fechar bloco"));
        }
        
        self.advance(); // consume '}'
        
        Ok(Stmt::Block(statements))
    }

    fn while_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'while'
        
        // Parse condition expression
        let condition = self.expression()?;
        
        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2052, "Esperado '{' após condição do while"
            ));
        }
        
        // Parse loop body block
        let body = Box::new(self.block_statement()?);
        
        Ok(Stmt::While(condition, body))
    }

    fn do_while_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'do'
        
        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2053, "Esperado '{' após 'do'"
            ));
        }
        
        // Parse loop body block first
        let body = Box::new(self.block_statement()?);
        
        // Expect 'while' keyword
        if !matches!(self.peek(), Token::Keyword(ref k) if k == "while") {
            return Err(DryadError::new(
                2054, "Esperado 'while' após corpo do do-while"
            ));
        }
        self.advance(); // consume 'while'
        
        // Parse condition expression
        let condition = self.expression()?;
        
        // Expect semicolon
        if !matches!(self.peek(), Token::Symbol(';')) {
            return Err(DryadError::new(
                2055, "Esperado ';' após condição do do-while"
            ));
        }
        self.advance(); // consume ';'
        
        Ok(Stmt::DoWhile(body, condition))
    }

    fn break_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'break'
        
        // Expect semicolon
        if matches!(self.peek(), Token::Symbol(';')) {
            self.advance(); // consume ';'
        }
        
        Ok(Stmt::Break)
    }

    fn continue_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'continue'
        
        // Expect semicolon
        if matches!(self.peek(), Token::Symbol(';')) {
            self.advance(); // consume ';'
        }
        
        Ok(Stmt::Continue)
    }

    fn for_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'for'
        
        // Primeiro vamos verificar se é um foreach (for-in) ou for tradicional
        // Olhamos adiante: se temos identifier followed by 'in', é foreach
        if let Token::Identifier(var_name) = self.peek() {
            let var = var_name.clone();
            let saved_position = self.position;
            
            self.advance(); // consume identifier
            
            // Se o próximo token é 'in', é um foreach
            if matches!(self.peek(), Token::Keyword(ref k) if k == "in") {
                return self.foreach_statement(var);
            } else {
                // Não é foreach, volta para posição anterior para fazer for tradicional
                self.position = saved_position;
            }
        }
        
        // Parse traditional for loop: for (init; condition; update) { body }
        // Parse initialization (opcional)
        let init = if matches!(self.peek(), Token::Symbol(';')) {
            None
        } else {
            // Espera uma atribuição: var = expr
            if let Token::Identifier(var_name) = self.peek() {
                let var = var_name.clone();
                self.advance(); // consume identifier
                
                if !matches!(self.peek(), Token::Symbol('=')) {
                    return Err(DryadError::new(
                        2056, "Esperado '=' na inicialização do for"
                    ));
                }
                self.advance(); // consume '='
                
                let expr = self.expression()?;
                Some(Box::new(Stmt::Assignment(var, expr)))
            } else {
                return Err(DryadError::new(
                    2057, "Esperado identificador na inicialização do for"
                ));
            }
        };

        // Consume primeiro ';'
        // Consume primeiro ';'
        if !matches!(self.peek(), Token::Symbol(';')) {
            return Err(DryadError::new(
                2058, "Esperado ';' após inicialização do for"
            ));
        }
        self.advance(); // consume ';'
        
        // Parse condition (opcional)
        let condition = if matches!(self.peek(), Token::Symbol(';')) {
            None
        } else {
            Some(self.expression()?)
        };
        
        // Consume segundo ';'
        if !matches!(self.peek(), Token::Symbol(';')) {
            return Err(DryadError::new(
                2059, "Esperado ';' após condição do for"
            ));
        }
        self.advance(); // consume ';'
        
        // Parse update (opcional)
        let update = if matches!(self.peek(), Token::Symbol('{')) {
            None
        } else {
            // Espera uma atribuição: var = expr
            if let Token::Identifier(var_name) = self.peek() {
                let var = var_name.clone();
                self.advance(); // consume identifier
                
                if !matches!(self.peek(), Token::Symbol('=')) {
                    return Err(DryadError::new(
                        2060, "Esperado '=' no update do for"
                    ));
                }
                self.advance(); // consume '='
                
                let expr = self.expression()?;
                Some(Box::new(Stmt::Assignment(var, expr)))
            } else {
                return Err(DryadError::new(
                    2061, "Esperado identificador no update do for"
                ));
            }
        };
        
        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2062, "Esperado '{' após declaração do for"
            ));
        }
        
        // Parse loop body block
        let body = Box::new(self.block_statement()?);
        
        Ok(Stmt::For(init, condition, update, body))
    }

    fn foreach_statement(&mut self, var_name: String) -> Result<Stmt, DryadError> {
        // Já temos o var_name, agora consume 'in'
        if !matches!(self.peek(), Token::Keyword(ref k) if k == "in") {
            return Err(DryadError::new(
                2063, "Esperado 'in' em foreach loop"
            ));
        }
        self.advance(); // consume 'in'
        
        // Parse the iterable expression
        let iterable = self.expression()?;
        
        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2064, "Esperado '{' após expressão do foreach"
            ));
        }
        
        // Parse loop body block
        let body = Box::new(self.block_statement()?);
        
        Ok(Stmt::ForEach(var_name, iterable, body))
    }

    fn function_declaration(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'function'
        
        // Parse function name
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err(DryadError::new(2012, "Esperado nome da função"))
        };
        
        // Expect opening parenthesis
        if !matches!(self.advance(), Token::Symbol('(')) {
            return Err(DryadError::new(2013, "Esperado '(' após nome da função"));
        }
        
        // Parse parameters
        let mut params = Vec::new();
        if !matches!(self.peek(), Token::Symbol(')')) {
            loop {
                match self.advance() {
                    Token::Identifier(param_name) => {
                        params.push(param_name.clone());
                    }
                    _ => return Err(DryadError::new(2014, "Esperado nome do parâmetro"))
                }
                
                match self.peek() {
                    Token::Symbol(',') => {
                        self.advance(); // consume comma
                        continue;
                    }
                    Token::Symbol(')') => break,
                    _ => return Err(DryadError::new(2015, "Esperado ',' ou ')' na lista de parâmetros"))
                }
            }
        }
        
        // Expect closing parenthesis
        if !matches!(self.advance(), Token::Symbol(')')) {
            return Err(DryadError::new(2016, "Esperado ')' após parâmetros"));
        }
        
        // Parse function body (block)
        let body = Box::new(self.block_statement()?);
        
        Ok(Stmt::FunctionDeclaration(name, params, body))
    }
    
    fn return_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'return'
        
        // Check if there's an expression to return
        let value = match self.peek() {
            Token::Symbol(';') => None, // return;
            _ => Some(self.expression()?) // return expression;
        };
        
        self.consume_semicolon()?;
        
        Ok(Stmt::Return(value))
    }

    fn class_declaration(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'class'
        
        // Parse class name
        let name = match self.peek() {
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                name
            }
            _ => return Err(DryadError::new(2087, "Esperado nome da classe após 'class'")),
        };
        
        // Check for inheritance (extends)
        let parent = if matches!(self.peek(), Token::Keyword(k) if k == "extends") {
            self.advance(); // consume 'extends'
            match self.peek() {
                Token::Identifier(parent_name) => {
                    let parent = parent_name.clone();
                    self.advance();
                    Some(parent)
                }
                _ => return Err(DryadError::new(2088, "Esperado nome da classe pai após 'extends'")),
            }
        } else {
            None
        };
        
        // Expect opening brace
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(2089, "Esperado '{' após declaração da classe"));
        }
        self.advance(); // consume '{'
        
        // Parse class members
        let mut members = Vec::new();
        while !matches!(self.peek(), Token::Symbol('}') | Token::Eof) {
            members.push(self.class_member()?);
        }
        
        // Expect closing brace
        if !matches!(self.peek(), Token::Symbol('}')) {
            return Err(DryadError::new(2090, "Esperado '}' para fechar classe"));
        }
        self.advance(); // consume '}'
        
        Ok(Stmt::ClassDeclaration(name, parent, members))
    }
    
    fn class_member(&mut self) -> Result<ClassMember, DryadError> {
        // Parse visibility (default is public)
        let visibility = self.parse_visibility();
        
        // Parse static keyword
        let is_static = if matches!(self.peek(), Token::Keyword(k) if k == "static") {
            self.advance(); // consume 'static'
            true
        } else {
            false
        };
        
        // Parse member type (function or property)
        match self.peek() {
            Token::Keyword(k) if k == "function" => {
                self.advance(); // consume 'function'
                
                // Parse method name
                let name = match self.peek() {
                    Token::Identifier(id) => {
                        let name = id.clone();
                        self.advance();
                        name
                    }
                    _ => return Err(DryadError::new(2091, "Esperado nome do método")),
                };
                
                // Parse parameters
                if !matches!(self.peek(), Token::Symbol('(')) {
                    return Err(DryadError::new(2092, "Esperado '(' após nome do método"));
                }
                self.advance(); // consume '('
                
                let mut params = Vec::new();
                while !matches!(self.peek(), Token::Symbol(')')) {
                    match self.peek() {
                        Token::Identifier(param_name) => {
                            params.push(param_name.clone());
                            self.advance();
                            
                            if matches!(self.peek(), Token::Symbol(',')) {
                                self.advance(); // consome ','
                            } else if !matches!(self.peek(), Token::Symbol(')')) {
                                return Err(DryadError::new(2093, "Esperado ',' ou ')' na lista de parâmetros"));
                            }
                        }
                        _ => return Err(DryadError::new(2094, "Esperado nome do parâmetro")),
                    }
                }
                self.advance(); // consume ')'
                
                // Parse method body
                let body = Box::new(self.block_statement()?);
                
                Ok(ClassMember::Method(visibility, is_static, name, params, body))
            }
            Token::Keyword(k) if k == "let" => {
                self.advance(); // consume 'let'
                
                // Property declaration with 'let'
                let name = match self.peek() {
                    Token::Identifier(id) => {
                        let name = id.clone();
                        self.advance();
                        name
                    }
                    _ => return Err(DryadError::new(2095, "Esperado nome da propriedade")),
                };
                
                // Parse optional default value
                let default_value = if matches!(self.peek(), Token::Symbol('=')) {
                    self.advance(); // consume '='
                    Some(self.expression()?)
                } else {
                    None
                };
                
                // Expect semicolon
                if matches!(self.peek(), Token::Symbol(';')) {
                    self.advance(); // consume ';'
                }
                
                Ok(ClassMember::Property(visibility, is_static, name, default_value))
            }
            Token::Identifier(_) => {
                // Property declaration without 'let'
                let name = match self.peek() {
                    Token::Identifier(id) => {
                        let name = id.clone();
                        self.advance();
                        name
                    }
                    _ => return Err(DryadError::new(2095, "Esperado nome da propriedade")),
                };
                
                // Parse optional default value
                let default_value = if matches!(self.peek(), Token::Symbol('=')) {
                    self.advance(); // consume '='
                    Some(self.expression()?)
                } else {
                    None
                };
                
                // Expect semicolon
                if matches!(self.peek(), Token::Symbol(';')) {
                    self.advance(); // consume ';'
                }
                
                Ok(ClassMember::Property(visibility, is_static, name, default_value))
            }
            _ => Err(DryadError::new(2096, "Esperado declaração de método ou propriedade")),
        }
    }
    
    fn parse_visibility(&mut self) -> Visibility {
        match self.peek() {
            Token::Keyword(k) if k == "public" => {
                self.advance();
                Visibility::Public
            }
            Token::Keyword(k) if k == "private" => {
                self.advance();
                Visibility::Private
            }
            Token::Keyword(k) if k == "protected" => {
                self.advance();
                Visibility::Protected
            }
            _ => Visibility::Public, // default
        }
    }

    // Métodos auxiliares
    fn match_operator(&mut self, op: &str) -> bool {
        if matches!(self.peek(), Token::Operator(o) if o == op) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any_operator(&mut self, ops: &[&str]) -> bool {
        for op in ops {
            if self.match_operator(op) {
                return true;
            }
        }
        false
    }

    fn previous_operator(&self) -> Option<String> {
        if self.position > 0 {
            if let Token::Operator(op) = &self.tokens[self.position - 1] {
                Some(op.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn peek(&self) -> &Token {
        if self.position >= self.tokens.len() {
            &Token::Eof
        } else {
            &self.tokens[self.position]
        }
    }

    fn advance(&mut self) -> &Token {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        self.previous()
    }

    fn if_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'if'
        
        // Parse condition expression
        let condition = self.expression()?;
        
        // Expect opening brace for then block
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2050, "Esperado '{' após condição do if"
            ));
        }
        
        // Parse then block
        let then_block = Box::new(self.block_statement()?);
        
        // Check if there's an else clause
        if matches!(self.peek(), Token::Keyword(keyword) if keyword == "else") {
            self.advance(); // consume 'else'
            
            // Check if it's "else if" or just "else"
            if matches!(self.peek(), Token::Keyword(keyword) if keyword == "if") {
                // It's "else if" - parse as nested if statement
                let else_block = Box::new(self.if_statement()?);
                Ok(Stmt::IfElse(condition, then_block, else_block))
            } else {
                // It's just "else" - expect a block
                if !matches!(self.peek(), Token::Symbol('{')) {
                    return Err(DryadError::new(
                        2051, "Esperado '{' após 'else'"
                    ));
                }
                let else_block = Box::new(self.block_statement()?);
                Ok(Stmt::IfElse(condition, then_block, else_block))
            }
        } else {
            // No else clause
            Ok(Stmt::If(condition, then_block))
        }
    }

    fn previous(&self) -> &Token {
        if self.position > 0 {
            &self.tokens[self.position - 1]
        } else {
            &Token::Eof
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn parse_array(&mut self) -> Result<Expr, DryadError> {
        self.advance(); // consome '['
        
        let mut elements = Vec::new();
        
        // Array vazio
        if matches!(self.peek(), Token::Symbol(']')) {
            self.advance(); // consome ']'
            return Ok(Expr::Array(elements));
        }
        
        // Primeiro elemento
        elements.push(self.expression()?);
        
        // Elementos restantes
        while matches!(self.peek(), Token::Symbol(',')) {
            self.advance(); // consome ','
            
            // Permite trailing comma: [1, 2, 3,]
            if matches!(self.peek(), Token::Symbol(']')) {
                break;
            }
            
            elements.push(self.expression()?);
        }
        
        if !matches!(self.peek(), Token::Symbol(']')) {
            return Err(DryadError::new(2070, "Esperado ']' após elementos do array"));
        }
        self.advance(); // consome ']'
        
        Ok(Expr::Array(elements))
    }

    fn parse_object_literal(&mut self) -> Result<Expr, DryadError> {
        self.advance(); // consome '{'
        
        let mut properties = Vec::new();
        
        // Objeto vazio
        if matches!(self.peek(), Token::Symbol('}')) {
            self.advance(); // consome '}'
            return Ok(Expr::ObjectLiteral(properties));
        }
        
        // Primeira propriedade
        properties.push(self.parse_object_property()?);
        
        // Propriedades restantes
        while matches!(self.peek(), Token::Symbol(',')) {
            self.advance(); // consome ','
            
            // Permite trailing comma: { a: 1, b: 2, }
            if matches!(self.peek(), Token::Symbol('}')) {
                break;
            }
            
            properties.push(self.parse_object_property()?);
        }
        
        if !matches!(self.peek(), Token::Symbol('}')) {
            return Err(DryadError::new(2071, "Esperado '}' após propriedades do objeto"));
        }
        self.advance(); // consome '}'
        
        Ok(Expr::ObjectLiteral(properties))
    }

    fn parse_object_property(&mut self) -> Result<crate::ast::ObjectProperty, DryadError> {
        // Esperamos uma chave (identificador)
        let key = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(DryadError::new(2072, "Esperado identificador como chave da propriedade")),
        };
        
        match self.peek() {
            Token::Symbol(':') => {
                // Propriedade: key: value
                self.advance(); // consome ':'
                let value = self.expression()?;
                Ok(crate::ast::ObjectProperty::Property(key, value))
            }
            Token::Symbol('(') => {
                // Método: key() { ... }
                self.advance(); // consome '('
                
                let mut params = Vec::new();
                
                // Parse parameters se houver
                if !matches!(self.peek(), Token::Symbol(')')) {
                    loop {
                        match self.advance() {
                            Token::Identifier(param) => params.push(param.clone()),
                            _ => return Err(DryadError::new(2073, "Esperado identificador de parâmetro")),
                        }
                        
                        match self.peek() {
                            Token::Symbol(',') => {
                                self.advance(); // consome ','
                                continue;
                            }
                            Token::Symbol(')') => break,
                            _ => return Err(DryadError::new(2074, "Esperado ',' ou ')' na lista de parâmetros")),
                        }
                    }
                }
                
                if !matches!(self.advance(), Token::Symbol(')')) {
                    return Err(DryadError::new(2075, "Esperado ')' após parâmetros"));
                }
                
                // Parse body
                if !matches!(self.peek(), Token::Symbol('{')) {
                    return Err(DryadError::new(2076, "Esperado '{' após parâmetros do método"));
                }
                let body = Box::new(self.block_statement()?);
                
                Ok(crate::ast::ObjectProperty::Method(key, params, body))
            }
            _ => return Err(DryadError::new(2077, "Esperado ':' ou '(' após chave da propriedade")),
        }
    }

    fn try_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'try'
        
        // Parse try block
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2080, "Esperado '{' após 'try'"
            ));
        }
        let try_block = Box::new(self.block_statement()?);
        
        // Parse optional catch clause
        let mut catch_clause = None;
        if matches!(self.peek(), Token::Keyword(keyword) if keyword == "catch") {
            self.advance(); // consume 'catch'
            
            // Expect (variable)
            if !matches!(self.peek(), Token::Symbol('(')) {
                return Err(DryadError::new(
                    2081, "Esperado '(' após 'catch'"
                ));
            }
            self.advance(); // consume '('
            
            let catch_var = match self.advance() {
                Token::Identifier(name) => name.clone(),
                _ => return Err(DryadError::new(2082, "Esperado nome da variável de exceção")),
            };
            
            if !matches!(self.peek(), Token::Symbol(')')) {
                return Err(DryadError::new(
                    2083, "Esperado ')' após variável de catch"
                ));
            }
            self.advance(); // consume ')'
            
            // Parse catch block
            if !matches!(self.peek(), Token::Symbol('{')) {
                return Err(DryadError::new(
                    2084, "Esperado '{' após parâmetro de catch"
                ));
            }
            let catch_block = Box::new(self.block_statement()?);
            
            catch_clause = Some((catch_var, catch_block));
        }
        
        // Parse optional finally clause
        let mut finally_clause = None;
        if matches!(self.peek(), Token::Keyword(keyword) if keyword == "finally") {
            self.advance(); // consume 'finally'
            
            // Parse finally block
            if !matches!(self.peek(), Token::Symbol('{')) {
                return Err(DryadError::new(
                    2085, "Esperado '{' após 'finally'"
                ));
            }
            finally_clause = Some(Box::new(self.block_statement()?));
        }
        
        // Validate that we have at least catch or finally
        if catch_clause.is_none() && finally_clause.is_none() {
            return Err(DryadError::new(
                2086, "Bloco try deve ter pelo menos um catch ou finally"
            ));
        }
        
        Ok(Stmt::Try(try_block, catch_clause, finally_clause))
    }

    fn throw_statement(&mut self) -> Result<Stmt, DryadError> {
        self.advance(); // consume 'throw'
        
        // Parse expression to throw
        let expr = self.expression()?;
        
        // Expect semicolon
        if matches!(self.peek(), Token::Symbol(';')) {
            self.advance(); // consume ';'
        }
        
        Ok(Stmt::Throw(expr))
    }

    fn property_assignment_statement(&mut self) -> Result<Stmt, DryadError> {
        // Parse object expression (could be 'this' or identifier)
        let object_expr = match self.peek() {
            Token::Keyword(k) if k == "this" => {
                self.advance(); // consume 'this'
                Expr::This
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance(); // consume identifier
                Expr::Variable(var_name)
            }
            _ => return Err(DryadError::new(2097, "Esperado 'this' ou identificador para property assignment")),
        };
        
        // Expect '.'
        if !matches!(self.peek(), Token::Symbol('.')) {
            return Err(DryadError::new(2098, "Esperado '.' após objeto para property assignment"));
        }
        self.advance(); // consume '.'
        
        // Parse property name
        let property_name = match self.peek() {
            Token::Identifier(name) => {
                let prop_name = name.clone();
                self.advance(); // consume property name
                prop_name
            }
            _ => return Err(DryadError::new(2099, "Esperado nome da propriedade após '.'"))
        };
        
        // Expect '='
        if !matches!(self.peek(), Token::Symbol('=')) {
            return Err(DryadError::new(2100, "Esperado '=' para property assignment"));
        }
        self.advance(); // consume '='
        
        // Parse value expression
        let value_expr = self.expression()?;
        
        Ok(Stmt::PropertyAssignment(object_expr, property_name, value_expr))
    }

    fn use_statement(&mut self) -> Result<Stmt, DryadError> {
        // Consome 'use'
        self.advance();
        
        // Deve ser seguido por uma string com o caminho do módulo
        match self.peek() {
            Token::String(path) => {
                let module_path = path.clone();
                self.advance(); // consome a string
                self.consume_semicolon()?; // consome o ponto e vírgula opcional
                Ok(Stmt::Use(module_path))
            }
            _ => {
                Err(DryadError::new(
                    4002,
                    "Use deve ser seguido por uma string com o caminho do módulo"
                ))
            }
        }
    }
}