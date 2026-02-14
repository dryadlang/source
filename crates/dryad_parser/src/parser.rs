// crates/dryad_parser/src/parser.rs
use crate::ast::{
    ClassMember, Expr, ImportKind, Literal, MatchArm, Pattern, Program, Stmt, Visibility,
};
use dryad_errors::{DryadError, SourceLocation};
use dryad_lexer::{
    token::{Token, TokenWithLocation},
    Lexer,
};

pub struct Parser {
    tokens: Vec<TokenWithLocation>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithLocation>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn new_from_lexer(lexer: &mut Lexer) -> Result<Self, DryadError> {
        let mut tokens = Vec::new();
        loop {
            let token_with_loc = lexer.next_token()?;
            if matches!(token_with_loc.token, Token::Eof) {
                tokens.push(token_with_loc);
                break;
            }
            tokens.push(token_with_loc);
        }
        Ok(Parser {
            tokens,
            position: 0,
        })
    }

    fn current_location(&self) -> SourceLocation {
        if self.position < self.tokens.len() {
            self.tokens[self.position].location.clone()
        } else {
            SourceLocation {
                file: None,
                line: 0,
                column: 0,
                position: 0,
                source_line: None,
            }
        }
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, DryadError> {
        match self.statement()? {
            Some(stmt) => Ok(stmt),
            None => Err(DryadError::new(2033, "Esperado statement")),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, DryadError> {
        self.expression()
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
            Token::Symbol('{') => Ok(Some(self.block_statement()?)),
            Token::NativeDirective(module_name) => {
                let location = self.current_location();
                let module = module_name.clone();
                self.advance(); // consome a diretiva
                Ok(Some(Stmt::NativeDirective(module, location)))
            }
            Token::Keyword(keyword) if keyword == "let" => Ok(Some(self.var_declaration()?)),
            Token::Keyword(keyword) if keyword == "const" => Ok(Some(self.const_declaration()?)),
            Token::Keyword(keyword) if keyword == "if" => Ok(Some(self.if_statement()?)),
            Token::Keyword(keyword) if keyword == "while" => Ok(Some(self.while_statement()?)),
            Token::Keyword(keyword) if keyword == "do" => Ok(Some(self.do_while_statement()?)),
            Token::Keyword(keyword) if keyword == "for" => Ok(Some(self.for_statement()?)),
            Token::Keyword(keyword) if keyword == "break" => Ok(Some(self.break_statement()?)),
            Token::Keyword(keyword) if keyword == "continue" => {
                Ok(Some(self.continue_statement()?))
            }
            Token::Keyword(keyword) if keyword == "try" => Ok(Some(self.try_statement()?)),
            Token::Keyword(keyword) if keyword == "throw" => Ok(Some(self.throw_statement()?)),
            Token::Keyword(keyword) if keyword == "function" => {
                Ok(Some(self.function_declaration()?))
            }
            Token::Keyword(keyword) if keyword == "async" => {
                Ok(Some(self.async_function_declaration()?))
            }
            Token::Keyword(keyword) if keyword == "thread" => {
                // Verifica se é "thread function" ou "thread("
                if matches!(self.peek_next(), Token::Keyword(kw) if kw == "function") {
                    Ok(Some(self.thread_function_declaration()?))
                } else {
                    // É uma expressão thread(), trata como expression statement
                    let expr = self.expression()?;
                    self.consume_semicolon()?;
                    Ok(Some(Stmt::Expression(expr, self.current_location())))
                }
            }
            Token::Keyword(keyword) if keyword == "class" => Ok(Some(self.class_declaration()?)),
            Token::Keyword(keyword) if keyword == "export" => Ok(Some(self.export_statement()?)),
            Token::Keyword(keyword) if keyword == "import" => Ok(Some(self.import_statement()?)),
            Token::Keyword(keyword) if keyword == "use" => Ok(Some(self.use_statement()?)),
            Token::Keyword(keyword) if keyword == "return" => Ok(Some(self.return_statement()?)),
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
                        Token::Operator(op)
                            if op == "+=" || op == "-=" || op == "*=" || op == "/=" =>
                        {
                            // É um assignment composto, volta e processa
                            self.position = checkpoint;
                            let stmt = self.assignment_statement()?;
                            self.consume_semicolon()?;
                            Ok(Some(stmt))
                        }
                        Token::Symbol('[') => {
                            // Pode ser array/object index assignment: identifier[index] = value
                            self.advance(); // consume '['

                            // Parse index expression
                            let index_expr = self.expression()?;

                            if matches!(self.peek(), Token::Symbol(']')) {
                                self.advance(); // consume ']'
                                if matches!(self.peek(), Token::Symbol('=')) {
                                    // É index assignment, volta ao início e processa
                                    self.position = checkpoint;
                                    let stmt = self.index_assignment_statement()?;
                                    self.consume_semicolon()?;
                                    return Ok(Some(stmt));
                                }
                            }

                            // Não é index assignment, volta e trata como expressão
                            self.position = checkpoint;
                            let expr = self.expression()?;
                            self.consume_semicolon()?;
                            Ok(Some(Stmt::Expression(expr, self.current_location())))
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
                            Ok(Some(Stmt::Expression(expr, self.current_location())))
                        }
                        _ => {
                            // Não é assignment, volta e trata como expressão
                            self.position = checkpoint;
                            let expr = self.expression()?;
                            self.consume_semicolon()?;
                            Ok(Some(Stmt::Expression(expr, self.current_location())))
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
                        Ok(Some(Stmt::Expression(expr, self.current_location())))
                    } else {
                        // Não é identificador ou this, trata como expressão
                        let expr = self.expression()?;
                        self.consume_semicolon()?;
                        Ok(Some(Stmt::Expression(expr, self.current_location())))
                    }
                } else {
                    // Não é identificador ou this, trata como expressão
                    let expr = self.expression()?;
                    self.consume_semicolon()?;
                    Ok(Some(Stmt::Expression(expr, self.current_location())))
                }
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'let'

        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(DryadError::new(
                    2011,
                    "Esperado nome da variável após 'let'",
                ))
            }
        };

        // Parse optional type: let x: number
        let var_type = if matches!(self.peek(), Token::Symbol(':')) {
            self.advance(); // consume ':'
            Some(self.parse_type()?)
        } else {
            None
        };

        let initializer = if matches!(self.peek(), Token::Symbol('=')) {
            self.advance(); // consume '='
            Some(self.expression()?)
        } else {
            None
        };

        self.consume_semicolon()?;
        Ok(Stmt::VarDeclaration(
            Pattern::Identifier(name),
            var_type,
            initializer,
            location,
        ))
    }

    fn const_declaration(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'const'

        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(DryadError::new(
                    2012,
                    "Esperado nome da constante após 'const'",
                ))
            }
        };

        // Parse optional type: const X: number
        let const_type = if matches!(self.peek(), Token::Symbol(':')) {
            self.advance(); // consume ':'
            Some(self.parse_type()?)
        } else {
            None
        };

        // const deve ter um valor obrigatório
        if !matches!(self.peek(), Token::Symbol('=')) {
            return Err(DryadError::new(2013, "Constante deve ter um valor inicial"));
        }

        self.advance(); // consume '='
        let value = self.expression()?;

        self.consume_semicolon()?;
        Ok(Stmt::ConstDeclaration(
            Pattern::Identifier(name),
            const_type,
            value,
            location,
        ))
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
                Err(DryadError::parser(
                    2003,
                    "Esperado ';' após declaração",
                    self.current_location(),
                    vec![";".to_string()],
                    format!("{:?}", self.peek()),
                ))
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
                Expr::Variable(_name, _) => {
                    // Assignments de variáveis simples ainda não suportados como expressão
                    return Err(DryadError::new(
                        2008,
                        "Atribuição de variável ainda não suportada em expressões",
                    ));
                }
                Expr::PropertyAccess(_, _, _) => {
                    // Property assignments também não suportados como expressão
                    return Err(DryadError::new(
                        2008,
                        "Atribuição de propriedade ainda não suportada em expressões",
                    ));
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
            let location = self.current_location();
            let operator = "||".to_string();
            let right = self.logical_and()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.bitwise_or()?;

        while self.match_operator("&&") {
            let location = self.current_location();
            let operator = "&&".to_string();
            let right = self.bitwise_or()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn bitwise_or(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.bitwise_xor()?;

        while self.match_operator("|") {
            let location = self.current_location();
            let operator = "|".to_string();
            let right = self.bitwise_xor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn bitwise_xor(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.bitwise_and()?;

        while self.match_operator("^") {
            let location = self.current_location();
            let operator = "^".to_string();
            let right = self.bitwise_and()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn bitwise_and(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.equality()?;

        while self.match_operator("&") {
            let location = self.current_location();
            let operator = "&".to_string();
            let right = self.equality()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.comparison()?;

        while self.match_any_operator(&["==", "!="]) {
            let location = self.current_location();
            let operator = self.previous_operator().unwrap();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.shift()?;

        while self.match_any_operator(&[">", ">=", "<", "<="]) {
            let location = self.current_location();
            let operator = self.previous_operator().unwrap();
            let right = self.shift()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn shift(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.term()?;

        while self.match_any_operator(&["<<", ">>", "<<<", ">>>"]) {
            let location = self.current_location();
            let operator = self.previous_operator().unwrap();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.factor()?;

        while self.match_any_operator(&["-", "+"]) {
            let location = self.current_location();
            let operator = self.previous_operator().unwrap();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.power()?;

        while self.match_any_operator(&["/", "*", "%", "%%"]) {
            let location = self.current_location();
            let operator = self.previous_operator().unwrap();
            let right = self.power()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.unary()?;

        // Operadores de potência têm associatividade à direita
        if self.match_any_operator(&["**", "^^", "##", "^"]) {
            let location = self.current_location();
            let operator = self.previous_operator().unwrap();
            let right = self.power()?; // Recursão à direita para associatividade
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right), location);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, DryadError> {
        let location = self.current_location();
        if self.match_any_operator(&["!", "-"]) {
            let operator = self.previous_operator().unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right), location));
        }

        // Pré-incremento e pré-decremento
        if self.match_any_operator(&["++"]) {
            let expr = self.unary()?;
            return Ok(Expr::PreIncrement(Box::new(expr), location));
        }

        if self.match_any_operator(&["--"]) {
            let expr = self.unary()?;
            return Ok(Expr::PreDecrement(Box::new(expr), location));
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, DryadError> {
        let mut expr = self.primary()?;

        loop {
            let location = self.current_location();
            match self.peek() {
                // Pós-incremento e pós-decremento
                Token::Operator(op) if op == "++" || op == "--" => {
                    let operator = op.clone();
                    self.advance();
                    match operator.as_str() {
                        "++" => expr = Expr::PostIncrement(Box::new(expr), location),
                        "--" => expr = Expr::PostDecrement(Box::new(expr), location),
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
                    expr = Expr::Index(Box::new(expr), Box::new(index), location);
                }
                // Acesso a tupla, propriedade ou método: expr.index/property/method
                Token::Symbol('.') => {
                    self.advance(); // consome '.'
                    match self.peek() {
                        Token::Number(index_num) => {
                            // Acesso a tupla: expr.index
                            let index = *index_num as usize;
                            self.advance();
                            expr = Expr::TupleAccess(Box::new(expr), index, location);
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
                                    return Err(DryadError::new(
                                        2074,
                                        "Esperado ')' após argumentos do método",
                                    ));
                                }

                                expr = Expr::MethodCall(Box::new(expr), name, args, location);
                            } else {
                                // Property access
                                expr = Expr::PropertyAccess(Box::new(expr), name, location);
                            }
                        }
                        _ => {
                            return Err(DryadError::new(
                                2072,
                                "Esperado número ou identificador após '.' para acesso",
                            ));
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
                                _ => {
                                    return Err(DryadError::new(
                                        2075,
                                        "Esperado ',' ou ')' na lista de argumentos da chamada",
                                    ))
                                }
                            }
                        }
                    }

                    // Expect closing parenthesis
                    if !matches!(self.advance(), Token::Symbol(')')) {
                        return Err(DryadError::new(
                            2076,
                            "Esperado ')' após argumentos da chamada",
                        ));
                    }

                    expr = Expr::Call(Box::new(expr), args, location);
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, DryadError> {
        let location = self.current_location();
        match &self.peek() {
            Token::Boolean(value) => {
                let val = *value;
                self.advance();
                Ok(Expr::Literal(Literal::Bool(val), location))
            }
            Token::Number(value) => {
                let val = *value;
                self.advance();
                Ok(Expr::Literal(Literal::Number(val), location))
            }
            Token::String(value) => {
                let val = value.clone();
                self.advance();
                Ok(Expr::Literal(Literal::String(val), location))
            }
            Token::Literal(value) if value == "null" => {
                self.advance();
                Ok(Expr::Literal(Literal::Null, location))
            }
            Token::Keyword(k) if k == "this" => {
                self.advance();
                Ok(Expr::This(location))
            }
            Token::Keyword(k) if k == "super" => {
                self.advance(); // consume 'super'

                let mut expr = Expr::Super(location.clone());

                // Check for property access or method call: super.property or super.method()
                while matches!(self.peek(), Token::Symbol('.')) {
                    self.advance(); // consume '.'
                    let current_location = self.current_location();

                    let name = match self.peek() {
                        Token::Identifier(name) => {
                            let name = name.clone();
                            self.advance();
                            name
                        }
                        _ => {
                            return Err(DryadError::new(
                                2080,
                                "Esperado identificador após '.' em 'super'",
                            ));
                        }
                    };

                    // Check if this is a method call (super.method(args...))
                    if matches!(self.peek(), Token::Symbol('(')) {
                        self.advance(); // consume '('

                        let mut args = Vec::new();

                        if !matches!(self.peek(), Token::Symbol(')')) {
                            loop {
                                let arg = self.expression()?;
                                args.push(arg);

                                match self.peek() {
                                    Token::Symbol(',') => {
                                        self.advance();
                                        continue;
                                    }
                                    Token::Symbol(')') => break,
                                    _ => {
                                        return Err(DryadError::new(
                                            2081,
                                            "Esperado ',' ou ')' após argumentos",
                                        ));
                                    }
                                }
                            }
                        }

                        if !matches!(self.advance(), Token::Symbol(')')) {
                            return Err(DryadError::new(2082, "Esperado ')' após argumentos"));
                        }

                        expr = Expr::MethodCall(Box::new(expr), name, args, current_location);
                    } else {
                        // Property access: super.property
                        expr = Expr::PropertyAccess(Box::new(expr), name, current_location);
                    }
                }

                Ok(expr)
            }
            Token::Keyword(k) if k == "await" => {
                self.advance(); // consume 'await'
                let expr = Box::new(self.unary()?);
                Ok(Expr::Await(expr, location))
            }
            Token::Keyword(k) if k == "mutex" => {
                self.advance(); // consume 'mutex'
                                // Expect '(' and ')' for mutex()
                if matches!(self.peek(), Token::Symbol('(')) {
                    self.advance(); // consume '('
                    if matches!(self.peek(), Token::Symbol(')')) {
                        self.advance(); // consume ')'
                        Ok(Expr::MutexCreation(location))
                    } else {
                        Err(DryadError::new(2029, "Esperado ')' após 'mutex('"))
                    }
                } else {
                    Err(DryadError::new(2030, "Esperado '(' após 'mutex'"))
                }
            }
            Token::Keyword(k) if k == "match" => self.parse_match_expression(),
            Token::Keyword(k) if k == "new" => {
                self.advance(); // consume 'new'

                // Parse class name (identifier)
                let class_name = match self.peek() {
                    Token::Identifier(name) => {
                        let name = name.clone();
                        self.advance();
                        name
                    }
                    _ => {
                        return Err(DryadError::new(2090, "Esperado nome da classe após 'new'"));
                    }
                };

                // Parse constructor arguments if present
                let args = if matches!(self.peek(), Token::Symbol('(')) {
                    self.advance(); // consume '('

                    let mut args = Vec::new();

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
                                _ => {
                                    return Err(DryadError::new(
                                        2091,
                                        "Esperado ',' ou ')' na lista de argumentos do construtor",
                                    ));
                                }
                            }
                        }
                    }

                    if !matches!(self.advance(), Token::Symbol(')')) {
                        return Err(DryadError::new(
                            2092,
                            "Esperado ')' após argumentos do construtor",
                        ));
                    }

                    args
                } else {
                    Vec::new()
                };

                Ok(Expr::ClassInstantiation(class_name, args, location))
            }
            Token::Keyword(k) if k == "thread" => {
                self.advance(); // consume 'thread'
                                // Expect '(' for thread(function, args...)
                if matches!(self.peek(), Token::Symbol('(')) {
                    self.advance(); // consume '('

                    // Parse function expression
                    let func = Box::new(self.expression()?);

                    let mut args = Vec::new();

                    // Parse optional arguments
                    while matches!(self.peek(), Token::Symbol(',')) {
                        self.advance(); // consume ','
                        let arg = self.expression()?;
                        args.push(arg);
                    }

                    if matches!(self.peek(), Token::Symbol(')')) {
                        self.advance(); // consume ')'
                        Ok(Expr::ThreadCall(func, args, location))
                    } else {
                        Err(DryadError::new(
                            2031,
                            "Esperado ')' após argumentos do thread",
                        ))
                    }
                } else {
                    Err(DryadError::new(2032, "Esperado '(' após 'thread'"))
                }
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance();

                // Check if this is a lambda with single parameter (x => expr)
                if matches!(self.peek(), Token::Arrow) {
                    self.advance(); // consume '=>'
                    let body = self.expression()?;
                    return Ok(Expr::Lambda {
                        params: vec![(var_name, None)],
                        body: Box::new(body),
                        return_type: None,
                        location,
                    });
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
                                _ => {
                                    return Err(DryadError::new(
                                        2017,
                                        "Esperado ',' ou ')' na lista de argumentos",
                                    ))
                                }
                            }
                        }
                    }

                    // Expect closing parenthesis
                    if !matches!(self.advance(), Token::Symbol(')')) {
                        return Err(DryadError::new(2018, "Esperado ')' após argumentos"));
                    }

                    Ok(Expr::Call(
                        Box::new(Expr::Variable(var_name, location.clone())),
                        args,
                        location,
                    ))
                } else {
                    // Just a variable reference
                    Ok(Expr::Variable(var_name, location))
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
                        return Ok(Expr::Lambda {
                            params: Vec::new(),
                            body: Box::new(body),
                            return_type: None,
                            location,
                        });
                    }
                    return Ok(Expr::Tuple(Vec::new(), location));
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
                                params.push((param_name, None));
                            } else {
                                // Reset position e parse como expressão normal
                                self.position = start_pos;
                            }
                        }
                        Token::Symbol(':') | Token::Symbol(',') => {
                            // Se tem ':' ou ',', é uma lista de parâmetros (lambda)
                            is_lambda = true;

                            // O primeiro parâmetro (param_name) pode ter tipo
                            let param_type = if matches!(self.peek(), Token::Symbol(':')) {
                                self.advance(); // consume ':'
                                Some(self.parse_type()?)
                            } else {
                                None
                            };
                            params.push((param_name, param_type));

                            if matches!(self.peek(), Token::Symbol(',')) {
                                self.advance(); // consome ','
                            }

                            while !matches!(self.peek(), Token::Symbol(')')) {
                                if let Token::Identifier(param) = self.advance() {
                                    let p_name = param.clone();
                                    let p_type = if matches!(self.peek(), Token::Symbol(':')) {
                                        self.advance(); // consume ':'
                                        Some(self.parse_type()?)
                                    } else {
                                        None
                                    };
                                    params.push((p_name, p_type));
                                } else {
                                    return Err(DryadError::new(
                                        2019,
                                        "Esperado identificador de parâmetro",
                                    ));
                                }

                                if matches!(self.peek(), Token::Symbol(',')) {
                                    self.advance(); // consome ','
                                } else {
                                    break;
                                }
                            }

                            if !matches!(self.advance(), Token::Symbol(')')) {
                                return Err(DryadError::new(
                                    2020,
                                    "Esperado ')' após parâmetros da lambda",
                                ));
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
                    // Parse optional return type: (a, b): number => ...
                    let return_type = if matches!(self.peek(), Token::Symbol(':')) {
                        self.advance(); // consume ':'
                        Some(self.parse_type()?)
                    } else {
                        None
                    };

                    if !matches!(self.advance(), Token::Arrow) {
                        return Err(DryadError::new(
                            2021,
                            "Esperado '=>' após parâmetros da lambda",
                        ));
                    }
                    let body = self.expression()?;
                    return Ok(Expr::Lambda {
                        params,
                        body: Box::new(body),
                        return_type,
                        location,
                    });
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
                    Ok(Expr::Tuple(elements, location))
                } else {
                    // Expressão agrupada simples
                    if !matches!(self.peek(), Token::Symbol(')')) {
                        return Err(DryadError::new(2005, "Esperado ')' após expressão"));
                    }
                    self.advance(); // consome ')'
                    Ok(first_expr)
                }
            }
            Token::TemplateStart => self.parse_template_string(),
            _ => Err(DryadError::new(
                2001,
                &format!("Token inesperado: {:?}", self.peek()),
            )),
        }
    }

    fn assignment_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        let name = match self.advance() {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(DryadError::new(
                    2012,
                    "Esperado identificador para assignment",
                ))
            }
        };

        match self.advance() {
            Token::Symbol('=') => {
                // Assignment simples: x = value
                let value = self.expression()?;
                Ok(Stmt::Assignment(Pattern::Identifier(name), value, location))
            }
            Token::Operator(op) if op == "+=" => {
                // x += value  =>  x = x + value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone(), self.current_location())),
                    "+".to_string(),
                    Box::new(value),
                    self.current_location(),
                );
                Ok(Stmt::Assignment(
                    Pattern::Identifier(name),
                    assignment_value,
                    location,
                ))
            }
            Token::Operator(op) if op == "-=" => {
                // x -= value  =>  x = x - value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone(), self.current_location())),
                    "-".to_string(),
                    Box::new(value),
                    self.current_location(),
                );
                Ok(Stmt::Assignment(
                    Pattern::Identifier(name),
                    assignment_value,
                    location,
                ))
            }
            Token::Operator(op) if op == "*=" => {
                // x *= value  =>  x = x * value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone(), self.current_location())),
                    "*".to_string(),
                    Box::new(value),
                    self.current_location(),
                );
                Ok(Stmt::Assignment(
                    Pattern::Identifier(name),
                    assignment_value,
                    location,
                ))
            }
            Token::Operator(op) if op == "/=" => {
                // x /= value  =>  x = x / value
                let value = self.expression()?;
                let assignment_value = Expr::Binary(
                    Box::new(Expr::Variable(name.clone(), self.current_location())),
                    "/".to_string(),
                    Box::new(value),
                    self.current_location(),
                );
                Ok(Stmt::Assignment(
                    Pattern::Identifier(name),
                    assignment_value,
                    location,
                ))
            }
            _ => Err(DryadError::new(2013, "Operador de assignment inválido")),
        }
    }

    fn export_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        // Consome 'export'
        self.advance();

        // O próximo token deve ser function, class, let ou variable
        match self.peek() {
            Token::Keyword(keyword) if keyword == "function" => {
                let func_stmt = self.function_declaration()?;
                Ok(Stmt::Export(Box::new(func_stmt), location))
            }
            Token::Keyword(keyword) if keyword == "class" => {
                let class_stmt = self.class_declaration()?;
                Ok(Stmt::Export(Box::new(class_stmt), location))
            }
            Token::Keyword(keyword) if keyword == "let" => {
                let var_stmt = self.var_declaration()?;
                Ok(Stmt::Export(Box::new(var_stmt), location))
            }
            _ => Err(DryadError::new(
                4001,
                "Export deve ser seguido por 'function', 'class' ou 'let'",
            )),
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
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

        Ok(Stmt::Block(statements, location))
    }

    fn while_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'while'

        // Expect opening parenthesis: while (
        if !matches!(self.peek(), Token::Symbol('(')) {
            return Err(DryadError::new(
                2050,
                "Esperado '(' após 'while' - sintaxe: while (condição)",
            ));
        }
        self.advance(); // consume '('

        // Parse condition expression
        let condition = self.expression()?;

        // Expect closing parenthesis: )
        if !matches!(self.peek(), Token::Symbol(')')) {
            return Err(DryadError::new(2051, "Esperado ')' após condição do while"));
        }
        self.advance(); // consume ')'

        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2052,
                "Esperado '{' após parênteses do while",
            ));
        }

        // Parse loop body block
        let body = Box::new(self.block_statement()?);

        Ok(Stmt::While(condition, body, location))
    }

    fn do_while_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'do'

        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(2053, "Esperado '{' após 'do'"));
        }

        // Parse loop body block first
        let body = Box::new(self.block_statement()?);

        // Expect 'while' keyword
        if !matches!(self.peek(), Token::Keyword(ref k) if k == "while") {
            return Err(DryadError::new(
                2054,
                "Esperado 'while' após corpo do do-while",
            ));
        }
        self.advance(); // consume 'while'

        // Expect opening parenthesis: while (
        if !matches!(self.peek(), Token::Symbol('(')) {
            return Err(DryadError::new(
                2065,
                "Esperado '(' após 'while' no do-while - sintaxe: do { ... } while (condição);",
            ));
        }
        self.advance(); // consume '('

        // Parse condition expression
        let condition = self.expression()?;

        // Expect closing parenthesis: )
        if !matches!(self.peek(), Token::Symbol(')')) {
            return Err(DryadError::new(
                2066,
                "Esperado ')' após condição do do-while",
            ));
        }
        self.advance(); // consume ')'

        // Expect semicolon
        if !matches!(self.peek(), Token::Symbol(';')) {
            return Err(DryadError::new(
                2067,
                "Esperado ';' após parênteses do do-while",
            ));
        }
        self.advance(); // consume ';'

        Ok(Stmt::DoWhile(body, condition, location))
    }

    fn break_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'break'

        // Expect semicolon
        if matches!(self.peek(), Token::Symbol(';')) {
            self.advance(); // consume ';'
        }

        Ok(Stmt::Break(location))
    }

    fn continue_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'continue'

        // Expect semicolon
        if matches!(self.peek(), Token::Symbol(';')) {
            self.advance(); // consume ';'
        }

        Ok(Stmt::Continue(location))
    }

    fn for_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'for'

        // Expect opening parenthesis: for (
        if !matches!(self.peek(), Token::Symbol('(')) {
            return Err(DryadError::new(
                2055,
                "Esperado '(' após 'for' - sintaxe: for (init; condition; update)",
            ));
        }
        self.advance(); // consume '('

        // Check if it's a foreach (for-in) by looking ahead
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
                        2056,
                        "Esperado '=' na inicialização do for - sintaxe: for (i = 0; ...)",
                    ));
                }
                self.advance(); // consume '='

                let expr = self.expression()?;
                Some(Box::new(Stmt::Assignment(
                    Pattern::Identifier(var),
                    expr,
                    location.clone(),
                )))
            } else {
                return Err(DryadError::new(
                    2057,
                    "Esperado identificador na inicialização do for - sintaxe: for (i = 0; ...)",
                ));
            }
        };

        // Consume primeiro ';'
        if !matches!(self.peek(), Token::Symbol(';')) {
            return Err(DryadError::new(
                2058,
                "Esperado ';' após inicialização do for - sintaxe: for (i = 0; condition; ...)",
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
                2059,
                "Esperado ';' após condição do for - sintaxe: for (...; i < 10; update)",
            ));
        }
        self.advance(); // consume ';'

        // Parse update (opcional)
        let update = if matches!(self.peek(), Token::Symbol(')')) {
            None
        } else {
            // Parse assignment or increment/decrement
            if let Token::Identifier(var_name) = self.peek() {
                let var = var_name.clone();
                self.advance(); // consume identifier

                // Check for increment/decrement operators
                if matches!(self.peek(), Token::Operator(ref op) if op == "++" || op == "--") {
                    let op = match self.peek() {
                        Token::Operator(o) => o.clone(),
                        _ => unreachable!(),
                    };
                    self.advance(); // consume operator

                    // Convert increment/decrement to assignment
                    let expr = if op == "++" {
                        Expr::Binary(
                            Box::new(Expr::Variable(var.clone(), location.clone())),
                            "+".to_string(),
                            Box::new(Expr::Literal(Literal::Number(1.0), location.clone())),
                            location.clone(),
                        )
                    } else {
                        Expr::Binary(
                            Box::new(Expr::Variable(var.clone(), location.clone())),
                            "-".to_string(),
                            Box::new(Expr::Literal(Literal::Number(1.0), location.clone())),
                            location.clone(),
                        )
                    };
                    Some(Box::new(Stmt::Assignment(
                        Pattern::Identifier(var),
                        expr,
                        location.clone(),
                    )))
                } else if matches!(self.peek(), Token::Symbol('=')) {
                    self.advance(); // consume '='
                    let expr = self.expression()?;
                    Some(Box::new(Stmt::Assignment(
                        Pattern::Identifier(var),
                        expr,
                        location.clone(),
                    )))
                } else {
                    return Err(DryadError::new(
                        2060, "Esperado '=', '++' ou '--' no update do for - sintaxe: for (...; ...; i++)"
                    ));
                }
            } else {
                return Err(DryadError::new(
                    2061,
                    "Esperado identificador no update do for - sintaxe: for (...; ...; i++)",
                ));
            }
        };

        // Expect closing parenthesis: )
        if !matches!(self.peek(), Token::Symbol(')')) {
            return Err(DryadError::new(
                2062,
                "Esperado ')' após declaração do for - sintaxe: for (init; condition; update)",
            ));
        }
        self.advance(); // consume ')'

        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(2063, "Esperado '{' após parênteses do for"));
        }

        // Parse loop body block
        let body = Box::new(self.block_statement()?);

        Ok(Stmt::For(init, condition, update, body, location))
    }

    fn foreach_statement(&mut self, var_name: String) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        // Já temos o var_name, agora consume 'in'
        if !matches!(self.peek(), Token::Keyword(ref k) if k == "in") {
            return Err(DryadError::new(
                2068,
                "Esperado 'in' em foreach loop - sintaxe: for (item in array)",
            ));
        }
        self.advance(); // consume 'in'

        // Parse the iterable expression
        let iterable = self.expression()?;

        // Expect closing parenthesis: )
        if !matches!(self.peek(), Token::Symbol(')')) {
            return Err(DryadError::new(
                2069,
                "Esperado ')' após expressão do foreach - sintaxe: for (item in array)",
            ));
        }
        self.advance(); // consume ')'

        // Expect opening brace for loop body
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2070,
                "Esperado '{' após parênteses do foreach",
            ));
        }

        // Parse loop body block
        let body = Box::new(self.block_statement()?);

        Ok(Stmt::ForEach(
            Pattern::Identifier(var_name),
            iterable,
            body,
            location,
        ))
    }

    fn function_declaration(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'function'

        // Parse function name
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err(DryadError::new(2012, "Esperado nome da função")),
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
                        let name = param_name.clone();
                        let param_type = if matches!(self.peek(), Token::Symbol(':')) {
                            self.advance(); // consume ':'
                            Some(self.parse_type()?)
                        } else {
                            None
                        };
                        params.push((name, param_type));
                    }
                    _ => return Err(DryadError::new(2014, "Esperado nome do parâmetro")),
                }

                match self.peek() {
                    Token::Symbol(',') => {
                        self.advance(); // consume comma
                        continue;
                    }
                    Token::Symbol(')') => break,
                    _ => {
                        return Err(DryadError::new(
                            2015,
                            "Esperado ',' ou ')' na lista de parâmetros",
                        ))
                    }
                }
            }
        }

        // Expect closing parenthesis
        if !matches!(self.advance(), Token::Symbol(')')) {
            return Err(DryadError::new(2016, "Esperado ')' após parâmetros"));
        }

        // Parse return type
        let return_type = if matches!(self.peek(), Token::Symbol(':')) {
            self.advance(); // consume ':'
            Some(self.parse_type()?)
        } else {
            None
        };

        // Parse function body (block)
        let body = Box::new(self.block_statement()?);

        Ok(Stmt::FunctionDeclaration {
            name,
            params,
            return_type,
            body,
            location,
            is_async: false,
        })
    }

    fn async_function_declaration(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'async'

        // Expect 'function'
        match self.advance() {
            Token::Keyword(k) if k == "function" => {}
            _ => return Err(DryadError::new(2017, "Esperado 'function' após 'async'")),
        }

        // Parse function name
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err(DryadError::new(2018, "Esperado nome da função async")),
        };

        // Expect opening parenthesis
        if !matches!(self.advance(), Token::Symbol('(')) {
            return Err(DryadError::new(
                2019,
                "Esperado '(' após nome da função async",
            ));
        }

        // Parse parameters
        let mut params = Vec::new();
        if !matches!(self.peek(), Token::Symbol(')')) {
            loop {
                match self.advance() {
                    Token::Identifier(param_name) => {
                        let name = param_name.clone();
                        let param_type = if matches!(self.peek(), Token::Symbol(':')) {
                            self.advance(); // consume ':'
                            Some(self.parse_type()?)
                        } else {
                            None
                        };
                        params.push((name, param_type));
                    }
                    _ => {
                        return Err(DryadError::new(
                            2020,
                            "Esperado nome do parâmetro na função async",
                        ))
                    }
                }

                match self.peek() {
                    Token::Symbol(',') => {
                        self.advance(); // consume comma
                        continue;
                    }
                    Token::Symbol(')') => break,
                    _ => {
                        return Err(DryadError::new(
                            2021,
                            "Esperado ',' ou ')' na lista de parâmetros da função async",
                        ))
                    }
                }
            }
        }

        // Expect closing parenthesis
        if !matches!(self.advance(), Token::Symbol(')')) {
            return Err(DryadError::new(
                2022,
                "Esperado ')' após parâmetros da função async",
            ));
        }

        // Parse return type
        let return_type = if matches!(self.peek(), Token::Symbol(':')) {
            self.advance(); // consume ':'
            Some(self.parse_type()?)
        } else {
            None
        };

        // Parse function body (block)
        let body = Box::new(self.block_statement()?);

        Ok(Stmt::FunctionDeclaration {
            name,
            params,
            return_type,
            body,
            location,
            is_async: true,
        })
    }

    fn thread_function_declaration(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'thread'

        // Expect 'function'
        match self.advance() {
            Token::Keyword(k) if k == "function" => {}
            _ => return Err(DryadError::new(2023, "Esperado 'function' após 'thread'")),
        }

        // Parse function name
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err(DryadError::new(2024, "Esperado nome da função thread")),
        };

        // Expect opening parenthesis
        if !matches!(self.advance(), Token::Symbol('(')) {
            return Err(DryadError::new(
                2025,
                "Esperado '(' após nome da função thread",
            ));
        }

        // Parse parameters
        let mut params = Vec::new();
        if !matches!(self.peek(), Token::Symbol(')')) {
            loop {
                match self.advance() {
                    Token::Identifier(param_name) => {
                        let name = param_name.clone();
                        let param_type = if matches!(self.peek(), Token::Symbol(':')) {
                            self.advance(); // consume ':'
                            Some(self.parse_type()?)
                        } else {
                            None
                        };
                        params.push((name, param_type));
                    }
                    _ => {
                        return Err(DryadError::new(
                            2026,
                            "Esperado nome do parâmetro na função thread",
                        ))
                    }
                }

                match self.peek() {
                    Token::Symbol(',') => {
                        self.advance(); // consume comma
                        continue;
                    }
                    Token::Symbol(')') => break,
                    _ => {
                        return Err(DryadError::new(
                            2027,
                            "Esperado ',' ou ')' na lista de parâmetros da função thread",
                        ))
                    }
                }
            }
        }

        // Expect closing parenthesis
        if !matches!(self.advance(), Token::Symbol(')')) {
            return Err(DryadError::new(
                2028,
                "Esperado ')' após parâmetros da função thread",
            ));
        }

        // Parse function body (block)
        let body = Box::new(self.block_statement()?);

        Ok(Stmt::ThreadFunctionDeclaration {
            name,
            params,
            body,
            location,
        })
    }

    fn return_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'return'

        // Check if there's an expression to return
        let value = match self.peek() {
            Token::Symbol(';') => None,    // return;
            _ => Some(self.expression()?), // return expression;
        };

        self.consume_semicolon()?;

        Ok(Stmt::Return(value, location))
    }

    fn class_declaration(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'class'

        // Parse class name
        let name = match self.peek() {
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                name
            }
            _ => {
                return Err(DryadError::new(
                    2087,
                    "Esperado nome da classe após 'class'",
                ))
            }
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
                _ => {
                    return Err(DryadError::new(
                        2088,
                        "Esperado nome da classe pai após 'extends'",
                    ))
                }
            }
        } else {
            None
        };

        // Expect opening brace
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2089,
                "Esperado '{' após declaração da classe",
            ));
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

        Ok(Stmt::ClassDeclaration(name, parent, members, location))
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

        // Parse member type (function, async function, or property)
        match self.peek() {
            Token::Keyword(k) if k == "async" => {
                // Check if it's "async function"
                let next_token = self.peek_next();
                if matches!(next_token, Token::Keyword(kw) if kw == "function") {
                    self.advance(); // consume 'async'
                    self.advance(); // consume 'function'

                    // Parse method name
                    let name = match self.peek() {
                        Token::Identifier(id) => {
                            let name = id.clone();
                            self.advance();
                            name
                        }
                        _ => return Err(DryadError::new(2091, "Esperado nome do método async")),
                    };

                    // Parse parameters
                    if !matches!(self.peek(), Token::Symbol('(')) {
                        return Err(DryadError::new(
                            2092,
                            "Esperado '(' após nome do método async",
                        ));
                    }
                    self.advance(); // consume '('

                    let mut params = Vec::new();
                    if !matches!(self.peek(), Token::Symbol(')')) {
                        loop {
                            if let Token::Identifier(param_name) = self.advance() {
                                let name = param_name.clone();
                                let param_type = if matches!(self.peek(), Token::Symbol(':')) {
                                    self.advance(); // consume ':'
                                    Some(self.parse_type()?)
                                } else {
                                    None
                                };
                                params.push((name, param_type));
                            } else {
                                return Err(DryadError::new(2094, "Esperado nome do parâmetro"));
                            }

                            if matches!(self.peek(), Token::Symbol(',')) {
                                self.advance(); // consome ','
                            } else {
                                break;
                            }
                        }
                    }
                    self.advance(); // consume ')'

                    // Parse return type
                    let return_type = if matches!(self.peek(), Token::Symbol(':')) {
                        self.advance(); // consume ':'
                        Some(self.parse_type()?)
                    } else {
                        None
                    };

                    // Parse method body
                    let body = Box::new(self.block_statement()?);

                    Ok(ClassMember::Method {
                        visibility,
                        is_static,
                        is_async: true,
                        name,
                        params,
                        return_type,
                        body,
                    })
                } else {
                    return Err(DryadError::new(
                        2096,
                        "Esperado 'function' após 'async' em classe",
                    ));
                }
            }
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
                    match self.advance() {
                        Token::Identifier(param_name) => {
                            let name = param_name.clone();
                            let param_type = if matches!(self.peek(), Token::Symbol(':')) {
                                self.advance(); // consume ':'
                                Some(self.parse_type()?)
                            } else {
                                None
                            };
                            params.push((name, param_type));

                            if matches!(self.peek(), Token::Symbol(',')) {
                                self.advance(); // consome ','
                            } else if !matches!(self.peek(), Token::Symbol(')')) {
                                return Err(DryadError::new(
                                    2093,
                                    "Esperado ',' ou ')' na lista de parâmetros",
                                ));
                            }
                        }
                        _ => return Err(DryadError::new(2094, "Esperado nome do parâmetro")),
                    }
                }
                self.advance(); // consume ')'

                // Parse return type
                let return_type = if matches!(self.peek(), Token::Symbol(':')) {
                    self.advance(); // consume ':'
                    Some(self.parse_type()?)
                } else {
                    None
                };

                // Parse method body
                let body = Box::new(self.block_statement()?);

                Ok(ClassMember::Method {
                    visibility,
                    is_static,
                    is_async: false,
                    name,
                    params,
                    return_type,
                    body,
                })
            }
            Token::Keyword(k) if k == "get" => {
                self.advance(); // consume 'get'

                // Parse property name
                let name = match self.peek() {
                    Token::Identifier(id) => {
                        let name = id.clone();
                        self.advance();
                        name
                    }
                    _ => {
                        return Err(DryadError::new(
                            2097,
                            "Esperado nome da propriedade após 'get'",
                        ))
                    }
                };

                // Expect '(' and ')'
                if !matches!(self.peek(), Token::Symbol('(')) {
                    return Err(DryadError::new(2098, "Esperado '(' após nome do getter"));
                }
                self.advance(); // consume '('
                if !matches!(self.peek(), Token::Symbol(')')) {
                    return Err(DryadError::new(2099, "Esperado ')' no getter"));
                }
                self.advance(); // consume ')'

                // Parse getter body
                let body = Box::new(self.block_statement()?);

                Ok(ClassMember::Getter {
                    visibility,
                    name,
                    body,
                })
            }
            Token::Keyword(k) if k == "set" => {
                self.advance(); // consume 'set'

                // Parse property name
                let name = match self.peek() {
                    Token::Identifier(id) => {
                        let name = id.clone();
                        self.advance();
                        name
                    }
                    _ => {
                        return Err(DryadError::new(
                            2100,
                            "Esperado nome da propriedade após 'set'",
                        ))
                    }
                };

                // Expect '(' and parameter
                if !matches!(self.peek(), Token::Symbol('(')) {
                    return Err(DryadError::new(2101, "Esperado '(' após nome do setter"));
                }
                self.advance(); // consume '('

                // Parse parameter name
                let param = match self.peek() {
                    Token::Identifier(id) => {
                        let param = id.clone();
                        self.advance();
                        param
                    }
                    _ => return Err(DryadError::new(2102, "Esperado parâmetro no setter")),
                };

                if !matches!(self.peek(), Token::Symbol(')')) {
                    return Err(DryadError::new(2103, "Esperado ')' no setter"));
                }
                self.advance(); // consume ')'

                // Parse setter body
                let body = Box::new(self.block_statement()?);

                Ok(ClassMember::Setter {
                    visibility,
                    name,
                    param,
                    body,
                })
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

                // Parse optional type
                let prop_type = if matches!(self.peek(), Token::Symbol(':')) {
                    self.advance(); // consume ':'
                    Some(self.parse_type()?)
                } else {
                    None
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

                Ok(ClassMember::Property(
                    visibility,
                    is_static,
                    name,
                    prop_type,
                    default_value,
                ))
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

                // Parse optional type
                let prop_type = if matches!(self.peek(), Token::Symbol(':')) {
                    self.advance(); // consume ':'
                    Some(self.parse_type()?)
                } else {
                    None
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

                Ok(ClassMember::Property(
                    visibility,
                    is_static,
                    name,
                    prop_type,
                    default_value,
                ))
            }
            _ => Err(DryadError::new(
                2096,
                "Esperado declaração de método ou propriedade",
            )),
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

    fn parse_type(&mut self) -> Result<crate::ast::Type, DryadError> {
        let token = self.advance();
        match token {
            Token::Identifier(name) => {
                let name_str = name.clone();
                match name_str.as_str() {
                    "number" => Ok(crate::ast::Type::Number),
                    "string" => Ok(crate::ast::Type::String),
                    "bool" => Ok(crate::ast::Type::Bool),
                    "null" => Ok(crate::ast::Type::Null),
                    "any" => Ok(crate::ast::Type::Any),
                    _ => {
                        // Check for array suffix: type[]
                        if matches!(self.peek(), Token::Symbol('['))
                            && matches!(self.peek_next(), Token::Symbol(']'))
                        {
                            self.advance(); // [
                            self.advance(); // ]
                            Ok(crate::ast::Type::Array(Box::new(crate::ast::Type::Class(
                                name_str,
                            ))))
                        } else {
                            Ok(crate::ast::Type::Class(name_str))
                        }
                    }
                }
            }
            Token::Symbol('(') => {
                // Parse tuple type: (type1, type2)
                let mut types = Vec::new();
                if !matches!(self.peek(), Token::Symbol(')')) {
                    loop {
                        types.push(self.parse_type()?);
                        if matches!(self.peek(), Token::Symbol(',')) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                if !matches!(self.advance(), Token::Symbol(')')) {
                    return Err(DryadError::new(
                        2100,
                        "Esperado ')' após lista de tipos de tupla",
                    ));
                }
                Ok(crate::ast::Type::Tuple(types))
            }
            _ => Err(DryadError::new(
                2101,
                &format!("Tipo inválido: {:?}", token),
            )),
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
            if let Token::Operator(op) = &self.tokens[self.position - 1].token {
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
            &self.tokens[self.position].token
        }
    }

    fn peek_next(&self) -> &Token {
        if self.position + 1 >= self.tokens.len() {
            &Token::Eof
        } else {
            &self.tokens[self.position + 1].token
        }
    }

    fn advance(&mut self) -> &Token {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        self.previous()
    }

    fn if_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'if'

        // Parse condition expression
        let condition = self.expression()?;

        // Expect opening brace for then block
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(2050, "Esperado '{' após condição do if"));
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
                Ok(Stmt::IfElse(condition, then_block, else_block, location))
            } else {
                // It's just "else" - expect a block
                if !matches!(self.peek(), Token::Symbol('{')) {
                    return Err(DryadError::new(2051, "Esperado '{' após 'else'"));
                }
                let else_block = Box::new(self.block_statement()?);
                Ok(Stmt::IfElse(condition, then_block, else_block, location))
            }
        } else {
            // No else clause
            Ok(Stmt::If(condition, then_block, location))
        }
    }

    fn previous(&self) -> &Token {
        if self.position > 0 {
            &self.tokens[self.position - 1].token
        } else {
            &Token::Eof
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn parse_array(&mut self) -> Result<Expr, DryadError> {
        let location = self.current_location();
        self.advance(); // consome '['

        let mut elements = Vec::new();

        // Array vazio
        if matches!(self.peek(), Token::Symbol(']')) {
            self.advance(); // consome ']'
            return Ok(Expr::Array(elements, location));
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
            return Err(DryadError::new(
                2070,
                "Esperado ']' após elementos do array",
            ));
        }
        self.advance(); // consome ']'

        Ok(Expr::Array(elements, location))
    }

    fn parse_object_literal(&mut self) -> Result<Expr, DryadError> {
        let location = self.current_location();
        self.advance(); // consome '{'

        let mut properties = Vec::new();

        // Objeto vazio
        if matches!(self.peek(), Token::Symbol('}')) {
            self.advance(); // consome '}'
            return Ok(Expr::ObjectLiteral(properties, location));
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
            return Err(DryadError::new(
                2071,
                "Esperado '}' após propriedades do objeto",
            ));
        }
        self.advance(); // consome '}'

        Ok(Expr::ObjectLiteral(properties, location))
    }

    fn parse_object_property(&mut self) -> Result<crate::ast::ObjectProperty, DryadError> {
        // Esperamos uma chave (identificador ou string)
        let key = match self.advance() {
            Token::Identifier(name) => name.clone(),
            Token::String(name) => name.clone(),
            _ => {
                return Err(DryadError::new(
                    2072,
                    "Esperado identificador ou string como chave da propriedade",
                ))
            }
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
                        if let Token::Identifier(param_name) = self.advance() {
                            let name = param_name.clone();
                            let param_type = if matches!(self.peek(), Token::Symbol(':')) {
                                self.advance(); // consume ':'
                                Some(self.parse_type()?)
                            } else {
                                None
                            };
                            params.push((name, param_type));
                        } else {
                            return Err(DryadError::new(
                                2073,
                                "Esperado identificador de parâmetro",
                            ));
                        }

                        match self.peek() {
                            Token::Symbol(',') => {
                                self.advance(); // consome ','
                                continue;
                            }
                            Token::Symbol(')') => break,
                            _ => {
                                return Err(DryadError::new(
                                    2074,
                                    "Esperado ',' ou ')' na lista de parâmetros",
                                ))
                            }
                        }
                    }
                }

                if !matches!(self.advance(), Token::Symbol(')')) {
                    return Err(DryadError::new(2075, "Esperado ')' após parâmetros"));
                }

                let return_type = if matches!(self.peek(), Token::Symbol(':')) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };

                // Parse body
                if !matches!(self.peek(), Token::Symbol('{')) {
                    return Err(DryadError::new(
                        2076,
                        "Esperado '{' após parâmetros do método",
                    ));
                }
                let body = Box::new(self.block_statement()?);

                Ok(crate::ast::ObjectProperty::Method {
                    name: key,
                    params,
                    return_type,
                    body,
                })
            }
            _ => {
                return Err(DryadError::new(
                    2077,
                    "Esperado ':' ou '(' após chave da propriedade",
                ))
            }
        }
    }

    fn try_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'try'

        // Parse try block
        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(2080, "Esperado '{' após 'try'"));
        }
        let try_block = Box::new(self.block_statement()?);

        // Parse optional catch clause
        let mut catch_clause = None;
        if matches!(self.peek(), Token::Keyword(keyword) if keyword == "catch") {
            self.advance(); // consume 'catch'

            // Expect (variable)
            if !matches!(self.peek(), Token::Symbol('(')) {
                return Err(DryadError::new(2081, "Esperado '(' após 'catch'"));
            }
            self.advance(); // consume '('

            let catch_var = match self.advance() {
                Token::Identifier(name) => name.clone(),
                _ => {
                    return Err(DryadError::new(
                        2082,
                        "Esperado nome da variável de exceção",
                    ))
                }
            };

            if !matches!(self.peek(), Token::Symbol(')')) {
                return Err(DryadError::new(2083, "Esperado ')' após variável de catch"));
            }
            self.advance(); // consume ')'

            // Parse catch block
            if !matches!(self.peek(), Token::Symbol('{')) {
                return Err(DryadError::new(
                    2084,
                    "Esperado '{' após parâmetro de catch",
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
                return Err(DryadError::new(2085, "Esperado '{' após 'finally'"));
            }
            finally_clause = Some(Box::new(self.block_statement()?));
        }

        // Validate that we have at least catch or finally
        if catch_clause.is_none() && finally_clause.is_none() {
            return Err(DryadError::new(
                2086,
                "Bloco try deve ter pelo menos um catch ou finally",
            ));
        }

        Ok(Stmt::Try(try_block, catch_clause, finally_clause, location))
    }

    fn throw_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'throw'

        // Parse expression to throw
        let expr = self.expression()?;

        // Expect semicolon
        if matches!(self.peek(), Token::Symbol(';')) {
            self.advance(); // consume ';'
        }

        Ok(Stmt::Throw(expr, location))
    }

    fn property_assignment_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        // Parse object expression (could be 'this' or identifier)
        let object_expr = match self.peek() {
            Token::Keyword(k) if k == "this" => {
                self.advance(); // consume 'this'
                Expr::This(self.current_location())
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance(); // consume identifier
                Expr::Variable(var_name, self.current_location())
            }
            _ => {
                return Err(DryadError::new(
                    2097,
                    "Esperado 'this' ou identificador para property assignment",
                ))
            }
        };

        // Expect '.'
        if !matches!(self.peek(), Token::Symbol('.')) {
            return Err(DryadError::new(
                2098,
                "Esperado '.' após objeto para property assignment",
            ));
        }
        self.advance(); // consume '.'

        // Parse property name
        let property_name = match self.peek() {
            Token::Identifier(name) => {
                let prop_name = name.clone();
                self.advance(); // consume property name
                prop_name
            }
            _ => {
                return Err(DryadError::new(
                    2099,
                    "Esperado nome da propriedade após '.'",
                ))
            }
        };

        // Expect '='
        if !matches!(self.peek(), Token::Symbol('=')) {
            return Err(DryadError::new(
                2100,
                "Esperado '=' para property assignment",
            ));
        }
        self.advance(); // consume '='

        // Parse value expression
        let value_expr = self.expression()?;

        Ok(Stmt::PropertyAssignment(
            object_expr,
            property_name,
            value_expr,
            location,
        ))
    }

    fn index_assignment_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        // Parse array/object expression (should be identifier)
        let array_expr = match self.peek() {
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance(); // consume identifier
                Expr::Variable(var_name, self.current_location())
            }
            _ => {
                return Err(DryadError::new(
                    2101,
                    "Esperado identificador para index assignment",
                ))
            }
        };

        // Expect '['
        if !matches!(self.peek(), Token::Symbol('[')) {
            return Err(DryadError::new(2102, "Esperado '[' após identificador"));
        }
        self.advance(); // consume '['

        // Parse index expression
        let index_expr = self.expression()?;

        // Expect ']'
        if !matches!(self.peek(), Token::Symbol(']')) {
            return Err(DryadError::new(2103, "Esperado ']' após índice"));
        }
        self.advance(); // consume ']'

        // Expect '='
        if !matches!(self.peek(), Token::Symbol('=')) {
            return Err(DryadError::new(2104, "Esperado '=' para index assignment"));
        }
        self.advance(); // consume '='

        // Parse value expression
        let value_expr = self.expression()?;

        Ok(Stmt::IndexAssignment(
            array_expr, index_expr, value_expr, location,
        ))
    }

    fn use_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        // Consome 'use'
        self.advance();

        // Deve ser seguido por uma string com o caminho do módulo
        match self.peek() {
            Token::String(path) => {
                let module_path = path.clone();
                self.advance(); // consome a string
                self.consume_semicolon()?; // consome o ponto e vírgula opcional
                Ok(Stmt::Use(module_path, location))
            }
            _ => Err(DryadError::new(
                4002,
                "Use deve ser seguido por uma string com o caminho do módulo",
            )),
        }
    }

    fn import_statement(&mut self) -> Result<Stmt, DryadError> {
        let location = self.current_location();
        // Consome 'import'
        self.advance();

        // Três casos:
        // 1. import "module";                    // side effect
        // 2. import * as name from "module";     // namespace
        // 3. import { x, y } from "module";      // named

        match self.peek() {
            Token::String(path) => {
                // Caso 1: import "module";
                let module_path = path.clone();
                self.advance();
                self.consume_semicolon()?;
                Ok(Stmt::Import(ImportKind::SideEffect, module_path, location))
            }
            Token::Operator(op) if op == "*" => {
                // Caso 2: import * as name from "module";
                self.advance(); // consome '*'

                // Espera 'as'
                if let Token::Keyword(kw) = self.peek() {
                    if kw != "as" {
                        return Err(DryadError::new(4002, "Esperado 'as' após '*' no import"));
                    }
                    self.advance(); // consome 'as'
                } else {
                    return Err(DryadError::new(4002, "Esperado 'as' após '*' no import"));
                }

                // Espera identificador do namespace
                let namespace = if let Token::Identifier(name) = self.peek() {
                    let ns = name.clone();
                    self.advance();
                    ns
                } else {
                    return Err(DryadError::new(
                        4002,
                        "Esperado identificador após 'as' no import",
                    ));
                };

                // Espera 'from'
                if let Token::Keyword(kw) = self.peek() {
                    if kw != "from" {
                        return Err(DryadError::new(4002, "Esperado 'from' no import"));
                    }
                    self.advance(); // consome 'from'
                } else {
                    return Err(DryadError::new(4002, "Esperado 'from' no import"));
                }

                // Espera string do módulo
                let module_path = if let Token::String(path) = self.peek() {
                    let p = path.clone();
                    self.advance();
                    p
                } else {
                    return Err(DryadError::new(
                        4002,
                        "Esperado string com caminho do módulo após 'from'",
                    ));
                };

                self.consume_semicolon()?;
                Ok(Stmt::Import(
                    ImportKind::Namespace(namespace),
                    module_path,
                    location,
                ))
            }
            Token::Symbol('{') => {
                // Caso 3: import { x, y } from "module";
                self.advance(); // consome '{'

                let mut imports = Vec::new();

                loop {
                    // Espera identificador
                    if let Token::Identifier(name) = self.peek() {
                        imports.push(name.clone());
                        self.advance();
                    } else {
                        return Err(DryadError::new(
                            4002,
                            "Esperado identificador na lista de imports",
                        ));
                    }

                    // Verifica vírgula ou fecha chaves
                    match self.peek() {
                        Token::Symbol(',') => {
                            self.advance();
                            // Permite vírgula final opcional
                            if matches!(self.peek(), Token::Symbol('}')) {
                                break;
                            }
                        }
                        Token::Symbol('}') => break,
                        _ => {
                            return Err(DryadError::new(
                                4002,
                                "Esperado ',' ou '}' na lista de imports",
                            ));
                        }
                    }
                }

                self.advance(); // consome '}'

                // Espera 'from'
                if let Token::Keyword(kw) = self.peek() {
                    if kw != "from" {
                        return Err(DryadError::new(4002, "Esperado 'from' no import"));
                    }
                    self.advance(); // consome 'from'
                } else {
                    return Err(DryadError::new(4002, "Esperado 'from' no import"));
                }

                // Espera string do módulo
                let module_path = if let Token::String(path) = self.peek() {
                    let p = path.clone();
                    self.advance();
                    p
                } else {
                    return Err(DryadError::new(
                        4002,
                        "Esperado string com caminho do módulo após 'from'",
                    ));
                };

                self.consume_semicolon()?;
                Ok(Stmt::Import(
                    ImportKind::Named(imports),
                    module_path,
                    location,
                ))
            }
            _ => Err(DryadError::new(
                4002,
                "Import deve ser seguido por '{', '*' ou string",
            )),
        }
    }

    fn parse_template_string(&mut self) -> Result<Expr, DryadError> {
        let location = self.current_location();
        self.advance(); // consume TemplateStart

        let mut expr: Option<Expr> = None;

        while !matches!(self.peek(), Token::TemplateEnd) && !self.is_at_end() {
            let part = match self.peek() {
                Token::TemplateContent(content) => {
                    let loc = self.current_location();
                    let val = content.clone();
                    self.advance();
                    Expr::Literal(Literal::String(val), loc)
                }
                Token::InterpolationStart => {
                    self.advance();
                    let inner_expr = self.expression()?;
                    if !matches!(self.peek(), Token::InterpolationEnd) {
                        return Err(DryadError::new(
                            2027,
                            "Esperado '}' após interpolação em template string",
                        ));
                    }
                    self.advance(); // consume InterpolationEnd
                    inner_expr
                }
                _ => return Err(DryadError::new(2028, "Token inesperado em template string")),
            };

            expr = match expr {
                Some(prev) => Some(Expr::Binary(
                    Box::new(prev),
                    "+".to_string(),
                    Box::new(part),
                    location.clone(),
                )),
                None => Some(part),
            };
        }

        if !matches!(self.peek(), Token::TemplateEnd) {
            return Err(DryadError::new(1002, "Template string não fechada"));
        }
        self.advance(); // consume TemplateEnd

        Ok(expr.unwrap_or_else(|| Expr::Literal(Literal::String("".to_string()), location)))
    }

    fn parse_match_expression(&mut self) -> Result<Expr, DryadError> {
        let location = self.current_location();
        self.advance(); // consume 'match'

        let target = Box::new(self.expression()?);

        if !matches!(self.peek(), Token::Symbol('{')) {
            return Err(DryadError::new(
                2034,
                "Esperado '{' após expressão do match",
            ));
        }
        self.advance(); // consume '{'

        let mut arms = Vec::new();

        while !matches!(self.peek(), Token::Symbol('}')) && !self.is_at_end() {
            let arm_location = self.current_location();
            let pattern = self.parse_pattern()?;

            let mut guard = None;
            if let Token::Keyword(k) = self.peek() {
                if k == "if" {
                    self.advance(); // consume 'if'
                    guard = Some(self.expression()?);
                }
            }

            if !matches!(self.peek(), Token::Arrow) {
                return Err(DryadError::new(2035, "Esperado '=>' após padrão do match"));
            }
            self.advance(); // consume '=>'

            let body = match self.peek() {
                Token::Symbol('{') => self.block_statement()?,
                _ => {
                    let expr = self.expression()?;
                    // Consume comma if present
                    if matches!(self.peek(), Token::Symbol(',')) {
                        self.advance();
                    }
                    Stmt::Expression(expr, arm_location.clone())
                }
            };

            // Optional comma after arm
            if matches!(self.peek(), Token::Symbol(',')) {
                self.advance();
            }

            arms.push(MatchArm {
                pattern,
                guard,
                body,
                location: arm_location,
            });
        }

        if !matches!(self.peek(), Token::Symbol('}')) {
            return Err(DryadError::new(2036, "Esperado '}' para fechar o match"));
        }
        self.advance(); // consume '}'

        Ok(Expr::Match(target, arms, location))
    }

    fn parse_pattern(&mut self) -> Result<Pattern, DryadError> {
        match self.peek() {
            Token::Boolean(v) => {
                let val = *v;
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(val)))
            }
            Token::Number(v) => {
                let val = *v;
                self.advance();
                Ok(Pattern::Literal(Literal::Number(val)))
            }
            Token::String(v) => {
                let val = v.clone();
                self.advance();
                Ok(Pattern::Literal(Literal::String(val)))
            }
            Token::Literal(v) if v == "null" => {
                self.advance();
                Ok(Pattern::Literal(Literal::Null))
            }
            Token::Identifier(name) => {
                let text = name.clone();
                self.advance();
                if text == "_" {
                    Ok(Pattern::Wildcard)
                } else {
                    Ok(Pattern::Identifier(text))
                }
            }
            Token::Symbol('[') => {
                self.advance(); // consume '['
                let mut patterns = Vec::new();
                if !matches!(self.peek(), Token::Symbol(']')) {
                    loop {
                        patterns.push(self.parse_pattern()?);
                        if matches!(self.peek(), Token::Symbol(',')) {
                            self.advance();
                            continue;
                        }
                        break;
                    }
                }
                if !matches!(self.peek(), Token::Symbol(']')) {
                    return Err(DryadError::new(2037, "Esperado ']' após padrões do array"));
                }
                self.advance();
                Ok(Pattern::Array(patterns))
            }
            Token::Symbol('(') => {
                self.advance(); // consume '('
                let mut patterns = Vec::new();
                if !matches!(self.peek(), Token::Symbol(')')) {
                    loop {
                        patterns.push(self.parse_pattern()?);
                        if matches!(self.peek(), Token::Symbol(',')) {
                            self.advance();
                            continue;
                        }
                        break;
                    }
                }
                if !matches!(self.peek(), Token::Symbol(')')) {
                    return Err(DryadError::new(2038, "Esperado ')' após padrões da tupla"));
                }
                self.advance();
                Ok(Pattern::Tuple(patterns))
            }
            Token::Symbol('{') => {
                self.advance(); // consume '{'
                let mut patterns = Vec::new();
                if !matches!(self.peek(), Token::Symbol('}')) {
                    loop {
                        let key = if let Token::Identifier(k) = self.peek() {
                            let key_text = k.clone();
                            self.advance();
                            key_text
                        } else if let Token::String(s) = self.peek() {
                            let key_text = s.clone();
                            self.advance();
                            key_text
                        } else {
                            return Err(DryadError::new(
                                2039,
                                "Esperado chave do objeto no padrão",
                            ));
                        };

                        if !matches!(self.peek(), Token::Symbol(':')) {
                            return Err(DryadError::new(
                                2040,
                                "Esperado ':' após chave do objeto no padrão",
                            ));
                        }
                        self.advance(); // consume ':'

                        let pattern = self.parse_pattern()?;
                        patterns.push((key, pattern));

                        if matches!(self.peek(), Token::Symbol(',')) {
                            self.advance();
                            continue;
                        }
                        break;
                    }
                }
                if !matches!(self.peek(), Token::Symbol('}')) {
                    return Err(DryadError::new(2041, "Esperado '}' após padrões do objeto"));
                }
                self.advance();
                Ok(Pattern::Object(patterns))
            }
            _ => Err(DryadError::new(
                2042,
                &format!("Padrão inválido: {:?}", self.peek()),
            )),
        }
    }
}
