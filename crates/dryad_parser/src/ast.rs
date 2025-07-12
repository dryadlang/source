// crates/dryad_parser/src/ast.rs
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    VarDeclaration(String, Option<Expr>), // nome, valor opcional
    Assignment(String, Expr),             // nome, valor
    PropertyAssignment(Expr, String, Expr), // object, property, value
    Block(Vec<Stmt>),                    // { stmt1; stmt2; ... }
    If(Expr, Box<Stmt>),                 // if (condição) { bloco }
    IfElse(Expr, Box<Stmt>, Box<Stmt>),  // if (condição) { bloco } else { bloco }
    While(Expr, Box<Stmt>),              // while (condição) { bloco }
    DoWhile(Box<Stmt>, Expr),            // do { bloco } while (condição);
    For(Option<Box<Stmt>>, Option<Expr>, Option<Box<Stmt>>, Box<Stmt>), // for (init; condition; update) { body }
    ForEach(String, Expr, Box<Stmt>),    // for var in iterable { body }
    Break,                               // break;
    Continue,                            // continue;
    Try(Box<Stmt>, Option<(String, Box<Stmt>)>, Option<Box<Stmt>>), // try { } catch (var) { } finally { }
    Throw(Expr),                         // throw expression;
    FunctionDeclaration(String, Vec<String>, Box<Stmt>), // function name(params...) { body }
    ClassDeclaration(String, Option<String>, Vec<ClassMember>), // class Name [extends Parent] { members... }
    Export(Box<Stmt>),                   // export statement
    Use(String),                         // use "module/path"
    Return(Option<Expr>),                // return [expression];
    NativeDirective(String),             // #<module_name>
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
    Variable(String),
    Call(Box<Expr>, Vec<Expr>),
    PostIncrement(Box<Expr>),             // x++
    PostDecrement(Box<Expr>),             // x--
    PreIncrement(Box<Expr>),              // ++x
    PreDecrement(Box<Expr>),              // --x
    Array(Vec<Expr>),                     // [expr1, expr2, ...]
    Tuple(Vec<Expr>),                     // (expr1, expr2, ...)
    Index(Box<Expr>, Box<Expr>),          // array[index]
    TupleAccess(Box<Expr>, usize),        // tuple.index
    Lambda(Vec<String>, Box<Expr>),       // (params...) => expr
    This,                                 // this
    Super,                                // super
    MethodCall(Box<Expr>, String, Vec<Expr>), // object.method(args...)
    PropertyAccess(Box<Expr>, String),    // object.property
    ClassInstantiation(String, Vec<Expr>), // ClassName(args...)
    ObjectLiteral(Vec<ObjectProperty>),   // { key: value, method() { ... } }
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
    Property(String, Expr),                     // key: value
    Method(String, Vec<String>, Box<Stmt>),     // key() { ... }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum ClassMember {
    Method(Visibility, bool, String, Vec<String>, Box<Stmt>), // visibility, is_static, name, params, body
    Property(Visibility, bool, String, Option<Expr>),         // visibility, is_static, name, default_value
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
