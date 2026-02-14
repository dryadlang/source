// crates/dryad_parser/src/ast.rs
use dryad_errors::SourceLocation;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Bool,
    Null,
    Any,
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>), // (params) -> return
    Class(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "number"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Null => write!(f, "null"),
            Type::Any => write!(f, "any"),
            Type::Array(inner) => write!(f, "{}[]", inner),
            Type::Tuple(elements) => {
                write!(f, "(")?;
                for (i, el) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", el)?;
                }
                write!(f, ")")
            }
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Class(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr, SourceLocation),
    VarDeclaration(Pattern, Option<Type>, Option<Expr>, SourceLocation), // padrão, tipo opcional, valor opcional
    ConstDeclaration(Pattern, Option<Type>, Expr, SourceLocation), // padrão, tipo opcional, valor obrigatório
    Assignment(Pattern, Expr, SourceLocation), // padrão (pode ser simples id), valor
    PropertyAssignment(Expr, String, Expr, SourceLocation), // object, property, value
    IndexAssignment(Expr, Expr, Expr, SourceLocation), // array/object, index, value
    Block(Vec<Stmt>, SourceLocation),          // { stmt1; stmt2; ... }
    If(Expr, Box<Stmt>, SourceLocation),       // if (condição) { bloco }
    IfElse(Expr, Box<Stmt>, Box<Stmt>, SourceLocation), // if (condição) { bloco } else { bloco }
    While(Expr, Box<Stmt>, SourceLocation),    // while (condição) { bloco }
    DoWhile(Box<Stmt>, Expr, SourceLocation),  // do { bloco } while (condição);
    For(
        Option<Box<Stmt>>,
        Option<Expr>,
        Option<Box<Stmt>>,
        Box<Stmt>,
        SourceLocation,
    ), // for (init; condition; update) { body }
    ForEach(Pattern, Expr, Box<Stmt>, SourceLocation), // for pattern in iterable { body }
    Break(SourceLocation),                     // break;
    Continue(SourceLocation),                  // continue;
    Try(
        Box<Stmt>,
        Option<(String, Box<Stmt>)>,
        Option<Box<Stmt>>,
        SourceLocation,
    ), // try { } catch (var) { } finally { }
    Throw(Expr, SourceLocation),               // throw expression;
    Return(Option<Expr>, SourceLocation),      // return [expression];
    NativeDirective(String, SourceLocation),   // #<module_name>
    FunctionDeclaration {
        name: String,
        params: Vec<(String, Option<Type>)>,
        return_type: Option<Type>,
        body: Box<Stmt>,
        location: SourceLocation,
        is_async: bool,
    },
    ThreadFunctionDeclaration {
        name: String,
        params: Vec<(String, Option<Type>)>,
        body: Box<Stmt>,
        location: SourceLocation,
    },
    ClassDeclaration(String, Option<String>, Vec<ClassMember>, SourceLocation), // class Name [extends Parent] { members... }
    Export(Box<Stmt>, SourceLocation),                                          // export statement
    Use(String, SourceLocation),                                                // use "module/path"
    Import(ImportKind, String, SourceLocation),                                 // import statement
}

#[derive(Debug, Clone)]
pub enum ImportKind {
    Named(Vec<String>), // import { func1, func2 } from "module"
    Namespace(String),  // import * as name from "module"
    SideEffect,         // import "module"
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal, SourceLocation),
    Binary(Box<Expr>, String, Box<Expr>, SourceLocation),
    Unary(String, Box<Expr>, SourceLocation),
    Variable(String, SourceLocation),
    Call(Box<Expr>, Vec<Expr>, SourceLocation),
    PostIncrement(Box<Expr>, SourceLocation),      // x++
    PostDecrement(Box<Expr>, SourceLocation),      // x--
    PreIncrement(Box<Expr>, SourceLocation),       // ++x
    PreDecrement(Box<Expr>, SourceLocation),       // --x
    Array(Vec<Expr>, SourceLocation),              // [expr1, expr2, ...]
    Tuple(Vec<Expr>, SourceLocation),              // (expr1, expr2, ...)
    Index(Box<Expr>, Box<Expr>, SourceLocation),   // array[index]
    TupleAccess(Box<Expr>, usize, SourceLocation), // tuple.index
    Lambda {
        params: Vec<(String, Option<Type>)>,
        body: Box<Expr>,
        return_type: Option<Type>,
        location: SourceLocation,
    },
    This(SourceLocation),                                     // this
    Super(SourceLocation),                                    // super
    MethodCall(Box<Expr>, String, Vec<Expr>, SourceLocation), // object.method(args...)
    PropertyAccess(Box<Expr>, String, SourceLocation),        // object.property
    ClassInstantiation(String, Vec<Expr>, SourceLocation),    // ClassName(args...)
    ObjectLiteral(Vec<ObjectProperty>, SourceLocation),       // { key: value, method() { ... } }
    Await(Box<Expr>, SourceLocation),                         // await expr
    ThreadCall(Box<Expr>, Vec<Expr>, SourceLocation),         // thread(func, args...)
    MutexCreation(SourceLocation),                            // mutex()
    Match(Box<Expr>, Vec<MatchArm>, SourceLocation),          // match expr { pat => body, ... }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String), // Binding
    Wildcard,           // _
    Array(Vec<Pattern>),
    Tuple(Vec<Pattern>),
    Object(Vec<(String, Pattern)>),
}

impl Pattern {
    /// Retorna o nome do identificador se for um Pattern::Identifier
    pub fn identifier_name(&self) -> Option<&String> {
        match self {
            Pattern::Identifier(name) => Some(name),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Stmt,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum ObjectProperty {
    Property(String, Expr), // key: value
    Method {
        name: String,
        params: Vec<(String, Option<Type>)>,
        return_type: Option<Type>,
        body: Box<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum ClassMember {
    Method {
        visibility: Visibility,
        is_static: bool,
        is_async: bool,
        name: String,
        params: Vec<(String, Option<Type>)>,
        return_type: Option<Type>,
        body: Box<Stmt>,
    },
    Property(Visibility, bool, String, Option<Type>, Option<Expr>), // visibility, is_static, name, type, default_value
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}
