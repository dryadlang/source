// crates/dryad_lexer/src/token.rs
use dryad_errors::SourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithLocation {
    pub token: Token,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literais
    Identifier(String),
    // Template Strings
    TemplateStart,       // `
    TemplateEnd,         // `
    TemplateContent(String),
    InterpolationStart,  // ${
    InterpolationEnd,    // }

    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Literal(String), // Para null e outros literais especiais

    // Palavras-chave
    Keyword(String),
    
    // Operadores
    Operator(String),
    
    // Arrow para lambdas
    Arrow, // =>
    
    // Símbolos
    Symbol(char),
    
    // Diretivas nativas
    NativeDirective(String), // Para #<module_name>
    
    // Fim do arquivo
    Eof,
}

impl Token {
    /// Retorna true se o token é uma palavra-chave específica
    pub fn is_keyword(&self, keyword: &str) -> bool {
        matches!(self, Token::Keyword(k) if k == keyword)
    }
    
    /// Retorna true se o token é um operador específico
    pub fn is_operator(&self, operator: &str) -> bool {
        matches!(self, Token::Operator(op) if op == operator)
    }
    
    /// Retorna true se o token é um símbolo específico
    pub fn is_symbol(&self, symbol: char) -> bool {
        matches!(self, Token::Symbol(s) if *s == symbol)
    }
}
