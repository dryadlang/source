// crates/dryad_lexer/src/lexer.rs
use dryad_errors::DryadError;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, DryadError> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(Token::Eof);
        }

        let ch = self.advance();
        
        match ch {
            // Números
            '0'..='9' => self.number(),
            
            // Strings
            '"' => self.string('"'),
            '\'' => self.string('\''),
            
            // Identificadores e palavras-chave
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            
            // Operadores e símbolos
            '+' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator("+=".to_string()))
                } else if self.peek() == '+' {
                    self.advance();
                    Ok(Token::Operator("++".to_string()))
                } else {
                    Ok(Token::Operator("+".to_string()))
                }
            },
            '-' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator("-=".to_string()))
                } else if self.peek() == '-' {
                    self.advance();
                    Ok(Token::Operator("--".to_string()))
                } else {
                    Ok(Token::Operator("-".to_string()))
                }
            },
            '*' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator("*=".to_string()))
                } else if self.peek() == '*' {
                    self.advance();
                    Ok(Token::Operator("**".to_string()))
                } else {
                    Ok(Token::Operator("*".to_string()))
                }
            },
            '/' => {
                if self.peek() == '/' {
                    self.line_comment()
                } else if self.peek() == '*' {
                    self.block_comment()
                } else if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator("/=".to_string()))
                } else {
                    Ok(Token::Operator("/".to_string()))
                }
            },
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator("==".to_string()))
                } else if self.peek() == '>' {
                    self.advance();
                    Ok(Token::Arrow)
                } else {
                    Ok(Token::Symbol('='))
                }
            },
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator("!=".to_string()))
                } else {
                    Ok(Token::Operator("!".to_string()))
                }
            },
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator("<=".to_string()))
                } else if self.peek() == '<' {
                    self.advance();
                    // Verifica se é <<< (symmetric left shift)
                    if self.peek() == '<' {
                        self.advance();
                        Ok(Token::Operator("<<<".to_string()))
                    } else {
                        Ok(Token::Operator("<<".to_string()))
                    }
                } else {
                    Ok(Token::Operator("<".to_string()))
                }
            },
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operator(">=".to_string()))
                } else if self.peek() == '>' {
                    self.advance();
                    // Verifica se é >>> (symmetric right shift)
                    if self.peek() == '>' {
                        self.advance();
                        Ok(Token::Operator(">>>".to_string()))
                    } else {
                        Ok(Token::Operator(">>".to_string()))
                    }
                } else {
                    Ok(Token::Operator(">".to_string()))
                }
            },
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    Ok(Token::Operator("&&".to_string()))
                } else {
                    Ok(Token::Operator("&".to_string()))
                }
            },
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    Ok(Token::Operator("||".to_string()))
                } else {
                    Ok(Token::Operator("|".to_string()))
                }
            },
            '%' => {
                if self.peek() == '%' {
                    self.advance();
                    Ok(Token::Operator("%%".to_string()))
                } else {
                    Ok(Token::Operator("%".to_string()))
                }
            },
            '^' => {
                if self.peek() == '^' {
                    self.advance();
                    Ok(Token::Operator("^^".to_string()))
                } else {
                    Ok(Token::Operator("^".to_string()))
                }
            },
            '#' => {
                if self.peek() == '#' {
                    self.advance();
                    Ok(Token::Operator("##".to_string()))
                } else if self.peek() == '<' {
                    // Diretiva nativa: #<module_name>
                    self.native_directive()
                } else {
                    Err(DryadError::new(1001, &format!("Caracter inesperado '#' na linha {}, coluna {}", self.line, self.column - 1)))
                }
            },
            
            // Ponto - só pode ser símbolo quando encontrado isoladamente
            // Números com ponto decimal são processados na função number()
            '.' => Ok(Token::Symbol('.')),
            
            // Símbolos simples
            '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | ':' => Ok(Token::Symbol(ch)),
            
            _ => Err(DryadError::new(1001, &format!("Caracter inesperado '{}' na linha {}, coluna {}", ch, self.line, self.column - 1))),
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            let ch = self.source[self.position];
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            ch
        } else {
            '\0'
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' | '\n' => {
                    self.advance();
                },
                _ => break,
            }
        }
    }

    fn number(&mut self) -> Result<Token, DryadError> {
        let start = self.position - 1;
        
        // Verifica se é um literal de base especial (0b, 0o, 0x)
        if self.source[start] == '0' && !self.is_at_end() {
            let next_char = self.peek().to_ascii_lowercase();
            match next_char {
                'b' => return self.binary_number(),
                'o' => return self.octal_number(),
                'x' => return self.hexadecimal_number(),
                _ => {} // Continua com número decimal normal
            }
        }
        
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Verifica se há parte decimal
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consome o '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let text: String = self.source[start..self.position].iter().collect();
        match text.parse::<f64>() {
            Ok(value) => Ok(Token::Number(value)),
            Err(_) => Err(DryadError::new(1004, &format!("Formato de número inválido: '{}'", text))),
        }
    }

    fn string(&mut self, delimiter: char) -> Result<Token, DryadError> {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != delimiter {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            
            if self.peek() == '\\' {
                self.advance(); // consome '\'
                match self.peek() {
                    'n' => {
                        value.push('\n');
                        self.advance();
                    },
                    't' => {
                        value.push('\t');
                        self.advance();
                    },
                    'r' => {
                        value.push('\r');
                        self.advance();
                    },
                    '\\' => {
                        value.push('\\');
                        self.advance();
                    },
                    '"' => {
                        value.push('"');
                        self.advance();
                    },
                    '\'' => {
                        value.push('\'');
                        self.advance();
                    },
                    'u' => {
                        // Unicode escape sequence \uXXXX
                        self.advance(); // consome 'u'
                        let mut unicode_digits = String::new();
                        for _ in 0..4 {
                            if !self.is_at_end() && self.peek().is_ascii_hexdigit() {
                                unicode_digits.push(self.peek());
                                self.advance();
                            } else {
                                return Err(DryadError::new(1005, "Sequência de escape Unicode inválida: esperado 4 dígitos hexadecimais"));
                            }
                        }
                        if let Ok(code_point) = u32::from_str_radix(&unicode_digits, 16) {
                            if let Some(unicode_char) = char::from_u32(code_point) {
                                value.push(unicode_char);
                            } else {
                                return Err(DryadError::new(1005, &format!("Código Unicode inválido: \\u{}", unicode_digits)));
                            }
                        } else {
                            return Err(DryadError::new(1005, &format!("Sequência de escape Unicode inválida: \\u{}", unicode_digits)));
                        }
                    },
                    c => {
                        return Err(DryadError::new(1005, &format!("Sequência de escape inválida: '\\{}'", c)));
                    }
                }
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(DryadError::new(1002, &format!("String literal não fechada (delimitador '{}')", delimiter)));
        }

        // Consome a aspas de fechamento
        self.advance();
        Ok(Token::String(value))
    }

    fn identifier(&mut self) -> Result<Token, DryadError> {
        let start = self.position - 1;
        
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.advance();
        }

        let text: String = self.source[start..self.position].iter().collect();
        
        // Verifica se é uma palavra-chave
        let token = match text.as_str() {
            "let" | "if" | "else" | "function" | "class" | "return" | "for" | "while" | "do" | "break" | "continue" | "import" | "export" | "use" | "try" | "catch" | "finally" | "throw" | "in" | "this" | "super" | "static" | "public" | "private" | "protected" | "extends" => {
                Token::Keyword(text)
            },
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "null" => Token::Literal("null".to_string()),
            _ => Token::Identifier(text),
        };

        Ok(token)
    }

    fn line_comment(&mut self) -> Result<Token, DryadError> {
        // Consome o segundo '/'
        self.advance();
        
        // Pula até o final da linha
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        
        // Recursivamente obtém o próximo token
        self.next_token()
    }

    fn block_comment(&mut self) -> Result<Token, DryadError> {
        // Consome o '*'
        self.advance();
        
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // consome '*'
                self.advance(); // consome '/'
                return self.next_token();
            }
            self.advance();
        }
        
        Err(DryadError::new(1003, "Comentário de bloco não fechado"))
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + 1]
        }
    }

    fn binary_number(&mut self) -> Result<Token, DryadError> {
        // Consome o 'b'
        self.advance();
        
        let start = self.position;
        let mut has_digits = false;
        
        // Consome dígitos binários (0-1)
        while !self.is_at_end() && (self.peek() == '0' || self.peek() == '1') {
            self.advance();
            has_digits = true;
        }
        
        if !has_digits {
            return Err(DryadError::new(1004, "Número binário vazio após '0b'"));
        }
        
        // Verifica se há dígitos inválidos
        if !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek().is_ascii_alphabetic()) {
            let invalid_char = self.peek();
            return Err(DryadError::new(1004, &format!("Dígito inválido '{}' em número binário", invalid_char)));
        }
        
        let binary_text: String = self.source[start..self.position].iter().collect();
        
        // Converte binário para decimal
        match u64::from_str_radix(&binary_text, 2) {
            Ok(value) => Ok(Token::Number(value as f64)),
            Err(_) => Err(DryadError::new(1004, &format!("Número binário inválido: '0b{}'", binary_text))),
        }
    }

    fn octal_number(&mut self) -> Result<Token, DryadError> {
        // Consome o 'o'
        self.advance();
        
        let start = self.position;
        let mut has_digits = false;
        
        // Consome dígitos octais (0-7)
        while !self.is_at_end() && self.peek().is_ascii_digit() && self.peek() <= '7' {
            self.advance();
            has_digits = true;
        }
        
        if !has_digits {
            return Err(DryadError::new(1004, "Número octal vazio após '0o'"));
        }
        
        // Verifica se há dígitos inválidos
        if !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek().is_ascii_alphabetic()) {
            let invalid_char = self.peek();
            return Err(DryadError::new(1004, &format!("Dígito inválido '{}' em número octal", invalid_char)));
        }
        
        let octal_text: String = self.source[start..self.position].iter().collect();
        
        // Converte octal para decimal
        match u64::from_str_radix(&octal_text, 8) {
            Ok(value) => Ok(Token::Number(value as f64)),
            Err(_) => Err(DryadError::new(1004, &format!("Número octal inválido: '0o{}'", octal_text))),
        }
    }

    fn hexadecimal_number(&mut self) -> Result<Token, DryadError> {
        // Consome o 'x'
        self.advance();
        
        let start = self.position;
        let mut has_digits = false;
        
        // Consome dígitos hexadecimais (0-9, A-F, a-f)
        while !self.is_at_end() {
            let ch = self.peek().to_ascii_lowercase();
            if ch.is_ascii_digit() || (ch >= 'a' && ch <= 'f') {
                self.advance();
                has_digits = true;
            } else {
                break;
            }
        }
        
        if !has_digits {
            return Err(DryadError::new(1004, "Número hexadecimal vazio após '0x'"));
        }
        
        // Verifica se há dígitos inválidos
        if !self.is_at_end() && self.peek().is_ascii_alphabetic() {
            let invalid_char = self.peek();
            return Err(DryadError::new(1004, &format!("Dígito inválido '{}' em número hexadecimal", invalid_char)));
        }
        
        let hex_text: String = self.source[start..self.position].iter().collect();
        
        // Converte hexadecimal para decimal
        match u64::from_str_radix(&hex_text, 16) {
            Ok(value) => Ok(Token::Number(value as f64)),
            Err(_) => Err(DryadError::new(1004, &format!("Número hexadecimal inválido: '0x{}'", hex_text))),
        }
    }

    fn native_directive(&mut self) -> Result<Token, DryadError> {
        // Já verificamos que o próximo char é '<'
        self.advance(); // consome '<'
        
        let mut module_name = String::new();
        
        while !self.is_at_end() && self.peek() != '>' {
            let ch = self.advance();
            if ch.is_ascii_alphanumeric() || ch == '_' {
                module_name.push(ch);
            } else {
                return Err(DryadError::new(1006, &format!("Caracter inválido '{}' em diretiva nativa", ch)));
            }
        }
        
        if self.is_at_end() {
            return Err(DryadError::new(1006, "Diretiva nativa não fechada - esperado '>'"));
        }
        
        self.advance(); // consome '>'
        
        if module_name.is_empty() {
            return Err(DryadError::new(1006, "Nome do módulo nativo não pode estar vazio"));
        }
        
        Ok(Token::NativeDirective(module_name))
    }
}
