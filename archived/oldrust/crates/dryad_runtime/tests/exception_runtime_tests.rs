#[cfg(test)]
mod exception_runtime_tests {
    use dryad_lexer::lexer::Lexer;
    use dryad_parser::Parser;
    use dryad_runtime::interpreter::{Interpreter, Value};

    fn interpret_code(input: &str) -> Result<Value, dryad_errors::DryadError> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        
        loop {
            match lexer.next_token() {
                Ok(token) if token.token == dryad_lexer::token::Token::Eof => {
                    tokens.push(token);
                    break;
                }
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            }
        }
        
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        
        let mut interpreter = Interpreter::new();
        interpreter.execute_and_return_value(&program)
    }

    #[test]
    fn test_try_catch_basic() {
        let input = r#"
            let result = 0;
            try {
                result = 1;
                throw "error";
                result = 2;
            } catch (e) {
                result = 3;
            }
            result
        "#;
        
        let result = interpret_code(input).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_try_catch_no_exception() {
        let input = r#"
            let result = 0;
            try {
                result = 1;
            } catch (e) {
                result = 2;
            }
            result
        "#;
        
        let result = interpret_code(input).unwrap();
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn test_try_finally() {
        let input = r#"
            let result = 0;
            try {
                result = 1;
                throw "error";
            } finally {
                result = result + 10;
            }
            result
        "#;
        
        // Should throw error but finally should execute
        let result = interpret_code(input);
        assert!(result.is_err());
        // Note: In real implementation, we might want to test that finally executed
        // by having some side effect, but for now we just check that error is propagated
    }

    #[test]
    fn test_try_catch_finally_no_exception() {
        let input = r#"
            let result = 0;
            try {
                result = 1;
            } catch (e) {
                result = 2;
            } finally {
                result = result + 10;
            }
            result
        "#;
        
        let result = interpret_code(input).unwrap();
        assert_eq!(result, Value::Number(11.0)); // 1 + 10
    }

    #[test]
    fn test_try_catch_finally_with_exception() {
        let input = r#"
            let result = 0;
            try {
                result = 1;
                throw "error";
            } catch (e) {
                result = 5;
            } finally {
                result = result + 10;
            }
            result
        "#;
        
        let result = interpret_code(input).unwrap();
        assert_eq!(result, Value::Number(15.0)); // 5 + 10
    }

    #[test]
    fn test_throw_string() {
        let input = r#"
            throw "This is an error";
        "#;
        
        let result = interpret_code(input);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), 3020);
        assert!(error.message().contains("This is an error"));
    }

    #[test]
    fn test_throw_variable() {
        let input = r#"
            let error_msg = "Variable error";
            throw error_msg;
        "#;
        
        let result = interpret_code(input);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), 3020);
        assert!(error.message().contains("Variable error"));
    }

    #[test]
    fn test_catch_variable_access() {
        let input = r#"
            let caught_message = "";
            try {
                throw "test error";
            } catch (e) {
                caught_message = "caught";
            }
            caught_message
        "#;
        
        let result = interpret_code(input).unwrap();
        assert_eq!(result, Value::String("caught".to_string()));
    }

    #[test]
    fn test_nested_try_catch() {
        let input = r#"
            let result = 0;
            try {
                try {
                    result = 1;
                    throw "inner error";
                } catch (e) {
                    result = 2;
                    throw "outer error";
                }
            } catch (e) {
                result = 3;
            }
            result
        "#;
        
        let result = interpret_code(input).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_exception_in_finally() {
        let input = r#"
            try {
                throw "first error";
            } catch (e) {
                let x = 1; // Caught successfully
            } finally {
                throw "finally error";
            }
        "#;
        
        let result = interpret_code(input);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("finally error"));
    }
}
