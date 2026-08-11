// crates/dryad_errors/src/tests.rs

use super::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_location_creation() {
        let location = SourceLocation::new(Some(PathBuf::from("test.dryad")), 10, 5, 100);

        assert_eq!(location.line, 10);
        assert_eq!(location.column, 5);
        assert_eq!(location.position, 100);
        assert_eq!(location.file, Some(PathBuf::from("test.dryad")));
        assert_eq!(location.source_line, None);
    }

    #[test]
    fn test_source_location_with_source_line() {
        let location =
            SourceLocation::new(None, 1, 1, 0).with_source_line("let x = 5;".to_string());

        assert_eq!(location.source_line, Some("let x = 5;".to_string()));
    }

    #[test]
    fn test_stack_frame_creation() {
        let location = SourceLocation::new(None, 1, 1, 0);
        let frame = StackFrame::new("main".to_string(), location.clone())
            .with_context("function call".to_string());

        assert_eq!(frame.function_name, "main");
        assert_eq!(frame.location, location);
        assert_eq!(frame.context, Some("function call".to_string()));
    }

    #[test]
    fn test_debug_context_creation() {
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), "5".to_string());
        variables.insert("y".to_string(), "hello".to_string());

        let context = DebugContext::new()
            .with_variables(variables.clone())
            .with_suggestion("Check if the variable is declared".to_string())
            .with_help_url("https://raw.githubusercontent.com/Dryad-lang/source/main/DRYAD_ERROR_GUIDE.md#e3002-undefined-variable".to_string());

        assert_eq!(context.variables, Some(variables));
        assert_eq!(context.suggestions.len(), 1);
        assert_eq!(context.help_url, Some("https://raw.githubusercontent.com/Dryad-lang/source/main/DRYAD_ERROR_GUIDE.md#e3002-undefined-variable".to_string()));
    }

    #[test]
    fn test_lexer_error_creation() {
        let location = SourceLocation::new(Some(PathBuf::from("test.dryad")), 5, 10, 50)
            .with_source_line("let x = @;".to_string());

        let error = DryadError::lexer(1001, "Invalid character '@'", location.clone());

        assert_eq!(error.code(), 1001);
        assert_eq!(error.message(), "Invalid character '@'");
        assert_eq!(error.location(), &location);

        if let DryadError::Lexer { debug_context, .. } = &error {
            assert!(debug_context.is_none());
        } else {
            panic!("Error is not of type Lexer");
        }
    }

    #[test]
    fn test_parser_error_with_expected_tokens() {
        let location = SourceLocation::new(None, 1, 1, 0);
        let expected = vec!["';'".to_string(), "')'".to_string()];
        let found = "'{'".to_string();

        let error = DryadError::parser(
            2001,
            "Unexpected token",
            location,
            expected.clone(),
            found.clone(),
        );

        if let DryadError::Parser {
            expected: exp,
            found: f,
            ..
        } = &error
        {
            assert_eq!(*exp, expected);
            assert_eq!(*f, found);
        } else {
            panic!("Error is not of type Parser");
        }
    }

    #[test]
    fn test_runtime_error_with_stack_trace() {
        let location = SourceLocation::new(None, 15, 8, 200);
        let mut stack_trace = StackTrace::new();

        let frame1 = StackFrame::new("main".to_string(), SourceLocation::new(None, 1, 1, 0));
        let frame2 = StackFrame::new("foo".to_string(), SourceLocation::new(None, 10, 5, 150))
            .with_context("function call".to_string());

        stack_trace.push_frame(frame1);
        stack_trace.push_frame(frame2);

        let error = DryadError::runtime(3001, "Division by zero", location, stack_trace);

        if let DryadError::Runtime {
            stack_trace: st, ..
        } = &error
        {
            assert_eq!(st.frames.len(), 2);
            assert_eq!(st.frames[0].function_name, "main");
            assert_eq!(st.frames[1].function_name, "foo");
            assert_eq!(st.frames[1].context, Some("function call".to_string()));
        } else {
            panic!("Error is not of type Runtime");
        }
    }

    #[test]
    fn test_type_error_creation() {
        let location = SourceLocation::new(None, 8, 3, 75);
        let error = DryadError::type_error(
            4001,
            "Incompatible types",
            location,
            "Number".to_string(),
            "String".to_string(),
        );

        if let DryadError::Type {
            expected_type,
            found_type,
            ..
        } = &error
        {
            assert_eq!(*expected_type, "Number");
            assert_eq!(*found_type, "String");
        } else {
            panic!("Error is not of type Type");
        }
    }

    #[test]
    fn test_io_error_creation() {
        let location = SourceLocation::new(None, 20, 1, 300);
        let path = Some(PathBuf::from("/path/to/file.txt"));

        let error = DryadError::io_error(
            5001,
            "File not found",
            location,
            "read".to_string(),
            path.clone(),
        );

        if let DryadError::Io {
            operation, path: p, ..
        } = &error
        {
            assert_eq!(*operation, "read");
            assert_eq!(*p, path);
        } else {
            panic!("Error is not of type Io");
        }
    }

    #[test]
    fn test_error_with_debug_context() {
        let location = SourceLocation::new(None, 1, 1, 0);
        let mut variables = HashMap::new();
        variables.insert("counter".to_string(), "0".to_string());

        let debug_context = DebugContext::new()
            .with_variables(variables)
            .with_suggestion("Initialize the counter variable with a positive value".to_string());

        let error =
            DryadError::lexer(1002, "Invalid value", location).with_debug_context(debug_context);

        if let DryadError::Lexer {
            debug_context: Some(ctx),
            ..
        } = &error
        {
            assert!(ctx.variables.is_some());
            assert_eq!(ctx.suggestions.len(), 1);
        } else {
            panic!("Debug context was not added correctly");
        }
    }

    #[test]
    fn test_warning_severity() {
        let location = SourceLocation::new(None, 1, 1, 0);

        let error = DryadError::Warning {
            code: 8001,
            message: "Unused variable".to_string(),
            location,
            severity: WarningSeverity::Low,
            debug_context: None,
        };

        if let DryadError::Warning { severity, .. } = &error {
            assert_eq!(*severity, WarningSeverity::Low);
        } else {
            panic!("Error is not of type Warning");
        }
    }

    #[test]
    fn test_error_display_formatting() {
        let location = SourceLocation::new(Some(PathBuf::from("test.dryad")), 10, 5, 100)
            .with_source_line("let x = invalid_char@;".to_string());

        let error = DryadError::lexer(1001, "Invalid character '@'", location);
        let display_str = format!("{}", error);

        assert!(display_str.contains("E1001: Lexer Error"));
        assert!(display_str.contains("Invalid character '@'"));
        assert!(display_str.contains("test.dryad:10:5"));
        assert!(display_str.contains("let x = invalid_char@;"));
        assert!(display_str.contains("^")); // Visual pointer
    }

    #[test]
    fn test_backward_compatibility() {
        let error = DryadError::new(1001, "Test error");
        assert_eq!(error.code(), 1001);
        assert_eq!(error.message(), "Test error");

        // Should create a Lexer error based on the code
        assert!(matches!(error, DryadError::Lexer { .. }));
    }

    #[test]
    fn test_stack_trace_display() {
        let location = SourceLocation::new(None, 15, 8, 200);
        let mut stack_trace = StackTrace::new();

        let frame1 = StackFrame::new("main".to_string(), SourceLocation::new(None, 1, 1, 0));
        let frame2 = StackFrame::new(
            "calculate".to_string(),
            SourceLocation::new(None, 10, 5, 150),
        )
        .with_context("within for loop".to_string());

        stack_trace.push_frame(frame1);
        stack_trace.push_frame(frame2);

        let error = DryadError::runtime(3001, "Division by zero", location, stack_trace);
        let display_str = format!("{}", error);

        assert!(display_str.contains("Stack Trace:"));
        assert!(display_str.contains("main"));
        assert!(display_str.contains("calculate"));
        assert!(display_str.contains("within for loop"));
        assert!(display_str.contains("┌─")); // Visual formatting of the stack trace
    }

    #[test]
    fn test_auto_context_generation() {
        let location = SourceLocation::new(None, 1, 1, 0);
        let error = DryadError::lexer(1001, "Invalid character '@'", location).with_auto_context();

        if let DryadError::Lexer {
            debug_context: Some(ctx),
            ..
        } = &error
        {
            // Check if auto suggestions were added
            assert!(!ctx.suggestions.is_empty());
            assert!(ctx
                .suggestions
                .iter()
                .any(|s| s.contains("Use only letters, numbers, underscores")));

            // Check if the URL was generated correctly
            assert!(ctx.help_url.is_some());
            let url = ctx.help_url.as_ref().unwrap();
            assert!(url.contains("https://dryadlang.org/errors"));
            assert!(url.contains("e1001-unexpected-character"));
        } else {
            panic!("Auto context was not added correctly");
        }
    }

    #[test]
    fn test_different_error_codes_auto_context() {
        let location = SourceLocation::new(None, 1, 1, 0);

        // Test for error 1003 (unclosed comment)
        let error_1003 =
            DryadError::lexer(1003, "Unclosed comment", location.clone()).with_auto_context();

        if let DryadError::Lexer {
            debug_context: Some(ctx),
            ..
        } = &error_1003
        {
            let url = ctx.help_url.as_ref().unwrap();
            assert!(url.contains("e1003-unterminated-comment-block"));
        }

        // Test for error 3001 (undefined variable)
        let stack_trace = StackTrace::new();
        let error_3001 = DryadError::runtime(3001, "Undefined variable", location, stack_trace)
            .with_auto_context();

        if let DryadError::Runtime {
            debug_context: Some(ctx),
            ..
        } = &error_3001
        {
            let url = ctx.help_url.as_ref().unwrap();
            assert!(url.contains("e3001-undefined-variable"));
            assert!(ctx
                .suggestions
                .iter()
                .any(|s| s.contains("Declare the variable")));
        }
    }
}
