// crates/dryad_parser/tests/async_threading_parser_tests.rs
use dryad_lexer::Lexer;
use dryad_parser::{ast::*, Parser};

#[test]
fn test_async_function_declaration() {
    let input = "async function getData() { return await http_get('url'); }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser deveria ser criado");

    match parser.parse_statement() {
        Ok(Stmt::FunctionDeclaration {
            name,
            params,
            is_async,
            ..
        }) => {
            assert_eq!(name, "getData");
            assert_eq!(params.len(), 0);
            assert!(is_async, "Expected async function");
        }
        Ok(stmt) => panic!(
            "Esperado FunctionDeclaration com is_async=true, encontrado: {:?}",
            stmt
        ),
        Err(e) => panic!("Erro no parser: {:?}", e),
    }
}

#[test]
fn test_thread_function_declaration() {
    let input = "thread function backgroundTask(data) { native_println(data); }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser deveria ser criado");

    match parser.parse_statement() {
        Ok(Stmt::ThreadFunctionDeclaration { name, params, .. }) => {
            assert_eq!(name, "backgroundTask");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].0, "data");
        }
        Ok(stmt) => panic!("Esperado ThreadFunctionDeclaration, encontrado: {:?}", stmt),
        Err(e) => panic!("Erro no parser: {:?}", e),
    }
}

#[test]
fn test_await_expression() {
    let input = "await getData()";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser deveria ser criado");

    match parser.parse_expression() {
        Ok(Expr::Await(expr, _)) => match *expr {
            Expr::Call(func, args, _) => {
                match *func {
                    Expr::Variable(ref name, _) => assert_eq!(name, "getData"),
                    _ => panic!("Esperado Variable, encontrado: {:?}", func),
                }
                assert_eq!(args.len(), 0);
            }
            _ => panic!("Esperado Call, encontrado: {:?}", expr),
        },
        Ok(expr) => panic!("Esperado Await, encontrado: {:?}", expr),
        Err(e) => panic!("Erro no parser: {:?}", e),
    }
}

#[test]
fn test_mutex_creation() {
    let input = "mutex()";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser deveria ser criado");

    match parser.parse_expression() {
        Ok(Expr::MutexCreation(_)) => {
            // Mutex criado
        }
        Ok(expr) => panic!("Esperado MutexCreation, encontrado: {:?}", expr),
        Err(e) => panic!("Erro no parser: {:?}", e),
    }
}

#[test]
fn test_thread_instantiation() {
    let input = "thread(myFunction, arg1, arg2)";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Parser deveria ser criado");

    match parser.parse_expression() {
        Ok(Expr::ThreadCall(func, args, _)) => {
            match *func {
                Expr::Variable(name, _) => assert_eq!(name, "myFunction"),
                _ => panic!("Esperado Variable, encontrado: {:?}", func),
            }
            assert_eq!(args.len(), 2);
        }
        Ok(expr) => panic!("Esperado ThreadCall, encontrado: {:?}", expr),
        Err(e) => panic!("Erro no parser: {:?}", e),
    }
}
