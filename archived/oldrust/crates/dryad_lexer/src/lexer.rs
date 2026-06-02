use crate::token::{Token, TokenWithLocation};
use dryad_errors::{error_catalog, DryadError, SourceLocation};
use std::path::PathBuf;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    source_str: &'a str,
    chars: Chars<'a>,
    source_lines: Vec<String>,
    position: usize,
    line: usize,
    column: usize,
    file_path: Option<PathBuf>,
    template_nesting: Vec<usize>,
    brace_level: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let source_lines = source.lines().map(|s| s.to_string()).collect();
        Lexer {
            source_str: source,
            chars: source.chars(),
            source_lines,
            position: 0,
            line: 1,
            column: 1,
            file_path: None,
            template_nesting: Vec::new(),
            brace_level: 0,
        }
    }

    pub fn new_with_file(source: &'a str, file_path: PathBuf) -> Self {
        let mut lexer = Self::new(source);
        lexer.file_path = Some(file_path);
        lexer
    }

    fn current_location(&self) -> SourceLocation {
        let source_line = if self.line > 0 && self.line <= self.source_lines.len() {
            Some(self.source_lines[self.line - 1].clone())
        } else {
            None
        };

        SourceLocation::new(
            self.file_path.clone(),
            self.line,
            self.column,
            self.position,
        )
        .with_source_line(source_line.unwrap_or_default())
    }

    pub fn next_token(&mut self) -> Result<TokenWithLocation, DryadError> {
        if let Some(&level) = self.template_nesting.last() {
            if self.brace_level == level {
                return self.template_content();
            }
        }

        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(TokenWithLocation {
                token: Token::Eof,
                location: self.current_location(),
            });
        }

        let start_location = self.current_location();
        let ch = self.advance();

        match ch {
            '0'..='9' => self.number(),
            '"' => self.string('"'),
            '\'' => self.string('\''),
            '`' => {
                self.template_nesting.push(self.brace_level);
                Ok(TokenWithLocation {
                    token: Token::TemplateStart,
                    location: start_location,
                })
            }
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '+' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("+=".to_string()),
                        location: start_location,
                    })
                } else if self.peek() == '+' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("++".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("+".to_string()),
                        location: start_location,
                    })
                }
            }
            '-' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("-=".to_string()),
                        location: start_location,
                    })
                } else if self.peek() == '-' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("--".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("-".to_string()),
                        location: start_location,
                    })
                }
            }
            '*' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("*=".to_string()),
                        location: start_location,
                    })
                } else if self.peek() == '*' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("**".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("*".to_string()),
                        location: start_location,
                    })
                }
            }
            '/' => {
                if self.peek() == '/' {
                    self.line_comment()
                } else if self.peek() == '*' {
                    self.block_comment()
                } else if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("/=".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("/".to_string()),
                        location: start_location,
                    })
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("==".to_string()),
                        location: start_location,
                    })
                } else if self.peek() == '>' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Arrow,
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Symbol('='),
                        location: start_location,
                    })
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("!=".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("!".to_string()),
                        location: start_location,
                    })
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("<=".to_string()),
                        location: start_location,
                    })
                } else if self.peek() == '<' {
                    self.advance();
                    if self.peek() == '<' {
                        self.advance();
                        Ok(TokenWithLocation {
                            token: Token::Operator("<<<".to_string()),
                            location: start_location,
                        })
                    } else {
                        Ok(TokenWithLocation {
                            token: Token::Operator("<<".to_string()),
                            location: start_location,
                        })
                    }
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("<".to_string()),
                        location: start_location,
                    })
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator(">=".to_string()),
                        location: start_location,
                    })
                } else if self.peek() == '>' {
                    self.advance();
                    if self.peek() == '>' {
                        self.advance();
                        Ok(TokenWithLocation {
                            token: Token::Operator(">>>".to_string()),
                            location: start_location,
                        })
                    } else {
                        Ok(TokenWithLocation {
                            token: Token::Operator(">>".to_string()),
                            location: start_location,
                        })
                    }
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator(">".to_string()),
                        location: start_location,
                    })
                }
            }
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("&&".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("&".to_string()),
                        location: start_location,
                    })
                }
            }
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("||".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("|".to_string()),
                        location: start_location,
                    })
                }
            }
            '%' => {
                if self.peek() == '%' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("%%".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("%".to_string()),
                        location: start_location,
                    })
                }
            }
            '^' => {
                if self.peek() == '^' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("^^".to_string()),
                        location: start_location,
                    })
                } else {
                    Ok(TokenWithLocation {
                        token: Token::Operator("^".to_string()),
                        location: start_location,
                    })
                }
            }
            '#' => {
                if self.peek() == '#' {
                    self.advance();
                    Ok(TokenWithLocation {
                        token: Token::Operator("##".to_string()),
                        location: start_location,
                    })
                } else if self.peek() == '<' {
                    self.native_directive()
                } else {
                    Err(DryadError::from_catalog_fmt(
                        error_catalog::e1001(),
                        &format!(
                            "Unexpected character '#' at line {}, column {}",
                            self.line,
                            self.column - 1
                        ),
                        self.current_location(),
                    ))
                }
            }
            '.' => Ok(TokenWithLocation {
                token: Token::Symbol('.'),
                location: start_location,
            }),
            '{' => {
                self.brace_level += 1;
                Ok(TokenWithLocation {
                    token: Token::Symbol('{'),
                    location: start_location,
                })
            }
            '}' => {
                if self.brace_level > 0 {
                    self.brace_level -= 1;
                    if let Some(&template_level) = self.template_nesting.last() {
                        if self.brace_level == template_level {
                            return Ok(TokenWithLocation {
                                token: Token::InterpolationEnd,
                                location: start_location,
                            });
                        }
                    }
                }
                Ok(TokenWithLocation {
                    token: Token::Symbol('}'),
                    location: start_location,
                })
            }
            '(' | ')' | '[' | ']' | ';' | ',' | ':' => Ok(TokenWithLocation {
                token: Token::Symbol(ch),
                location: start_location,
            }),
            _ => {
                let location = self.current_location();
                Err(DryadError::from_catalog_fmt(
                    error_catalog::e1001(),
                    &format!("Unexpected character '{}'", ch),
                    location,
                )
                .with_auto_context())
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Safely slice the source string ensuring byte boundaries are valid
    fn safe_slice(&self, start: usize, end: usize) -> Option<&str> {
        let source_len = self.source_str.len();
        if start > source_len || end > source_len || start > end {
            return None;
        }
        // Check if start and end are at valid UTF-8 boundaries
        if self.source_str.is_char_boundary(start) && self.source_str.is_char_boundary(end) {
            Some(&self.source_str[start..end])
        } else {
            None
        }
    }

    fn advance(&mut self) -> char {
        if let Some(ch) = self.chars.next() {
            self.position += ch.len_utf8();
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
        self.chars.clone().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        let mut cloned = self.chars.clone();
        cloned.next();
        cloned.next().unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.chars.clone().next() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn number(&mut self) -> Result<TokenWithLocation, DryadError> {
        let start_pos = self.position.saturating_sub(1); // Assuming it started at digit

        // Handling special bases (0b, 0o, 0x)
        if self
            .safe_slice(start_pos, start_pos + 1)
            .map_or(false, |s| s.starts_with('0'))
        {
            let next = self.peek().to_ascii_lowercase();
            match next {
                'b' => return self.binary_number(),
                'o' => return self.octal_number(),
                'x' => return self.hexadecimal_number(),
                _ => {}
            }
        }

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' {
            let mut cloned = self.chars.clone();
            cloned.next();
            if cloned.next().map_or(false, |c| c.is_ascii_digit()) {
                self.advance(); // .
                while self.peek().is_ascii_digit() {
                    self.advance();
                }
            }
        }

        let text = self.safe_slice(start_pos, self.position).ok_or_else(|| {
            DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Index error while processing number",
                SourceLocation::unknown(),
            )
        })?;
        let location = self.current_location();
        match text.parse::<f64>() {
            Ok(value) => Ok(TokenWithLocation {
                token: Token::Number(value),
                location,
            }),
            Err(_) => Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                &format!("Invalid number format: '{}'", text),
                SourceLocation::unknown(),
            )),
        }
    }

    fn string(&mut self, delimiter: char) -> Result<TokenWithLocation, DryadError> {
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != delimiter {
            let ch = self.peek();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if ch == '\\' {
                self.advance(); // \
                match self.advance() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    'u' => {
                        let mut unicode_digits = String::new();
                        for _ in 0..4 {
                            let next = self.advance();
                            if next.is_ascii_hexdigit() {
                                unicode_digits.push(next);
                            } else {
                                return Err(DryadError::from_catalog_fmt(
                                    error_catalog::e1005(),
                                    "Invalid Unicode escape sequence",
                                    SourceLocation::unknown(),
                                ));
                            }
                        }
                        if let Ok(code_point) = u32::from_str_radix(&unicode_digits, 16) {
                            if let Some(unicode_char) = char::from_u32(code_point) {
                                value.push(unicode_char);
                            } else {
                                return Err(DryadError::from_catalog_fmt(
                                    error_catalog::e1005(),
                                    "Invalid Unicode code",
                                    SourceLocation::unknown(),
                                ));
                            }
                        } else {
                            return Err(DryadError::from_catalog_fmt(
                                error_catalog::e1005(),
                                "Invalid Unicode sequence",
                                SourceLocation::unknown(),
                            ));
                        }
                    }
                    c => {
                        return Err(DryadError::from_catalog_fmt(
                            error_catalog::e1005(),
                            &format!("Invalid escape sequence: '\\{}'", c),
                            SourceLocation::unknown(),
                        ))
                    }
                }
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1002(),
                &format!("Unclosed string ({})", delimiter),
                self.current_location(),
            )
            .with_auto_context());
        }

        self.advance(); // delimiter
        Ok(TokenWithLocation {
            token: Token::String(value),
            location: self.current_location(),
        })
    }

    fn identifier(&mut self) -> Result<TokenWithLocation, DryadError> {
        let start_pos = self.position.saturating_sub(1);

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.advance();
        }

        let text = self.safe_slice(start_pos, self.position).ok_or_else(|| {
            DryadError::from_catalog_fmt(
                error_catalog::e1001(),
                "Index error while processing identifier",
                SourceLocation::unknown(),
            )
        })?;
        let location = self.current_location();

        let token = match text {
            "let" | "const" | "if" | "else" | "function" | "fn" | "class" | "return" | "for"
            | "while" | "do" | "break" | "continue" | "import" | "export" | "use" | "try"
            | "catch" | "finally" | "throw" | "in" | "this" | "super" | "static" | "public"
            | "private" | "protected" | "extends" | "async" | "await" | "thread" | "mutex"
            | "as" | "from" | "match" | "new" | "interface" | "implements" | "get" | "set"
            | "namespace" => Token::Keyword(text.to_string()),
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "null" => Token::Literal("null".to_string()),
            _ => Token::Identifier(text.to_string()),
        };

        Ok(TokenWithLocation { token, location })
    }

    fn line_comment(&mut self) -> Result<TokenWithLocation, DryadError> {
        self.advance(); // /
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        self.next_token()
    }

    fn block_comment(&mut self) -> Result<TokenWithLocation, DryadError> {
        self.advance(); // *
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // *
                self.advance(); // /
                return self.next_token();
            }
            self.advance();
        }
        Err(DryadError::from_catalog(
            error_catalog::e1003(),
            SourceLocation::unknown(),
        ))
    }

    fn binary_number(&mut self) -> Result<TokenWithLocation, DryadError> {
        self.advance(); // b
        let start_pos = self.position;
        let mut has_digits = false;

        while !self.is_at_end() && (self.peek() == '0' || self.peek() == '1') {
            self.advance();
            has_digits = true;
        }

        if !has_digits {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Empty binary number",
                SourceLocation::unknown(),
            ));
        }

        if !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek().is_ascii_alphabetic())
        {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Invalid digit in binary number",
                SourceLocation::unknown(),
            ));
        }

        let text = self.safe_slice(start_pos, self.position).ok_or_else(|| {
            DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Index error in binary number",
                SourceLocation::unknown(),
            )
        })?;
        let location = self.current_location();
        match u64::from_str_radix(text, 2) {
            Ok(value) => Ok(TokenWithLocation {
                token: Token::Number(value as f64),
                location,
            }),
            Err(_) => Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Invalid binary number",
                SourceLocation::unknown(),
            )),
        }
    }

    fn octal_number(&mut self) -> Result<TokenWithLocation, DryadError> {
        self.advance(); // o
        let start_pos = self.position;
        let mut has_digits = false;

        while !self.is_at_end() && self.peek().is_ascii_digit() && self.peek() <= '7' {
            self.advance();
            has_digits = true;
        }

        if !has_digits {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Empty octal number",
                SourceLocation::unknown(),
            ));
        }

        if !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek().is_ascii_alphabetic())
        {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Invalid digit in octal number",
                SourceLocation::unknown(),
            ));
        }

        let text = self.safe_slice(start_pos, self.position).ok_or_else(|| {
            DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Index error in octal number",
                SourceLocation::unknown(),
            )
        })?;
        let location = self.current_location();
        match u64::from_str_radix(text, 8) {
            Ok(value) => Ok(TokenWithLocation {
                token: Token::Number(value as f64),
                location,
            }),
            Err(_) => Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Invalid octal number",
                SourceLocation::unknown(),
            )),
        }
    }

    fn hexadecimal_number(&mut self) -> Result<TokenWithLocation, DryadError> {
        self.advance(); // x
        let start_pos = self.position;
        let mut has_digits = false;

        while !self.is_at_end() && self.peek().is_ascii_hexdigit() {
            self.advance();
            has_digits = true;
        }

        if !has_digits {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Empty hexadecimal number",
                SourceLocation::unknown(),
            ));
        }

        if !self.is_at_end() && self.peek().is_ascii_alphabetic() {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Invalid digit in hexadecimal number",
                SourceLocation::unknown(),
            ));
        }

        let text = self.safe_slice(start_pos, self.position).ok_or_else(|| {
            DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Index error in hexadecimal number",
                SourceLocation::unknown(),
            )
        })?;
        let location = self.current_location();
        match u64::from_str_radix(text, 16) {
            Ok(value) => Ok(TokenWithLocation {
                token: Token::Number(value as f64),
                location,
            }),
            Err(_) => Err(DryadError::from_catalog_fmt(
                error_catalog::e1004(),
                "Invalid hexadecimal number",
                SourceLocation::unknown(),
            )),
        }
    }

    fn native_directive(&mut self) -> Result<TokenWithLocation, DryadError> {
        let start_location = self.current_location();
        self.advance(); // <

        let mut module_name = String::new();
        while !self.is_at_end() && self.peek() != '>' {
            let ch = self.advance();
            if ch.is_ascii_alphanumeric() || ch == '_' {
                module_name.push(ch);
            } else {
                return Err(DryadError::from_catalog_fmt(
                    error_catalog::e1006(),
                    "Invalid character in native directive",
                    SourceLocation::unknown(),
                ));
            }
        }

        if self.is_at_end() {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1006(),
                "Unclosed native directive",
                SourceLocation::unknown(),
            ));
        }

        self.advance(); // >
        if module_name.is_empty() {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1006(),
                "Native directive name cannot be empty",
                SourceLocation::unknown(),
            ));
        }

        Ok(TokenWithLocation {
            token: Token::NativeDirective(module_name),
            location: start_location,
        })
    }

    fn template_content(&mut self) -> Result<TokenWithLocation, DryadError> {
        let start_location = self.current_location();

        if self.is_at_end() {
            return Err(DryadError::from_catalog_fmt(
                error_catalog::e1002(),
                "Unclosed template string",
                start_location,
            ));
        }

        if self.peek() == '`' {
            self.advance();
            self.template_nesting.pop();
            return Ok(TokenWithLocation {
                token: Token::TemplateEnd,
                location: start_location,
            });
        }

        if self.peek() == '$' && self.peek_next() == '{' {
            self.advance(); // $
            self.advance(); // {
            self.brace_level += 1;
            return Ok(TokenWithLocation {
                token: Token::InterpolationStart,
                location: start_location,
            });
        }

        let mut value = String::new();
        while !self.is_at_end() {
            let ch = self.peek();
            if ch == '`' || (ch == '$' && self.peek_next() == '{') {
                break;
            }

            if ch == '\\' {
                self.advance(); // \
                match self.advance() {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '\\' => value.push('\\'),
                    '`' => value.push('`'),
                    '$' => value.push('$'),
                    '{' => value.push('{'),
                    c => value.push(c),
                }
            } else {
                value.push(self.advance());
            }
        }

        Ok(TokenWithLocation {
            token: Token::TemplateContent(value),
            location: start_location,
        })
    }
}
