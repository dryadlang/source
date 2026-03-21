// crates/dryad_errors/src/error_urls.rs

/// Generates documentation URLs based on the error code
pub fn get_error_documentation_url(error_code: u16) -> String {
    let base_url = "https://dryadlang.org/errors";

    match error_code {
        // ✅ Implemented Lexer Errors (1000-1999)
        1001 => format!("{}#e1001-unexpected-character", base_url),
        1002 => format!("{}#e1002-unterminated-string-literal", base_url),
        1003 => format!("{}#e1003-unterminated-comment-block", base_url),
        1004 => format!("{}#e1004-invalid-number-format", base_url),
        1005 => format!("{}#e1005-invalid-escape-sequence", base_url),
        1006 => format!("{}#e1006-invalid-native-directive", base_url),

        // ✅ Implemented Parser Errors (2000-2999)
        2001 => format!("{}#e2001-unexpected-token", base_url),
        2003 => format!("{}#e2003-missing-semicolon", base_url),
        2005 => format!("{}#e2005-missing-closing-parenthesis", base_url),
        2011 => format!("{}#e2011-invalid-variable-declaration", base_url),

        // 🟡 Expected Parser Errors (2000-2999)
        2017 => format!("{}#e2017-missing-function-parameters", base_url),
        2018 => format!("{}#e2018-missing-while-condition", base_url),
        2019 => format!("{}#e2019-missing-for-components", base_url),

        // ✅ Deduplicated Parser Errors (2042-2049, 2116-2117)
        2042 => format!("{}#e2042-invalid-assignment-target", base_url),
        2043 => format!("{}#e2043-invalid-function-parameter", base_url),
        2044 => format!("{}#e2044-missing-async-function-name", base_url),
        2045 => format!("{}#e2045-missing-else-brace", base_url),
        2046 => format!("{}#e2046-missing-class-closing-brace", base_url),
        2047 => format!("{}#e2047-invalid-regular-method", base_url),
        2048 => format!("{}#e2048-missing-index-bracket", base_url),
        2049 => format!("{}#e2049-missing-index-closing-bracket", base_url),
        2116 => format!("{}#e2116-import-error", base_url),
        2117 => format!("{}#e2117-template-string-parse-error", base_url),

        // ✅ Implemented Runtime Errors (3000-3999)
        3000 => format!("{}#e3000-runtime-error", base_url),
        3001 => format!("{}#e3001-undefined-variable", base_url),
        3002 => format!("{}#e3002-assignment-error", base_url),
        3005 => format!("{}#e3005-invalid-arithmetic-operation", base_url),
        3006 => format!("{}#e3006-invalid-multiplication", base_url),
        3007 => format!("{}#e3007-division-by-zero", base_url),
        3009 => format!("{}#e3009-invalid-comparison", base_url),
        3010 => format!("{}#e3010-break-outside-loop", base_url),
        3011 => format!("{}#e3011-continue-outside-loop", base_url),
        3020 => format!("{}#e3020-exception-thrown", base_url),
        3021 => format!("{}#e3021-function-return", base_url),
        3022 => format!("{}#e3022-invalid-this-context", base_url),
        3023 => format!("{}#e3023-super-not-implemented", base_url),
        3034 => format!("{}#e3034-invalid-property-assignment", base_url),
        3040 => format!("{}#e3040-stack-overflow", base_url),
        3104 => format!("{}#e3104-native-function-error", base_url),
        3105 => format!("{}#e3105-promise-error", base_url),
        3106 => format!("{}#e3106-runtime-type-error", base_url),

        // 🟡 Planned Type Errors (4000-4999)
        4001 => format!("{}#e4001-incompatible-types", base_url),
        4002 => format!("{}#e4002-invalid-conversion", base_url),

        // 🟡 Planned I/O Errors (5000-5999)
        5001 => format!("{}#e5001-file-not-found", base_url),
        5002 => format!("{}#e5002-permission-denied", base_url),

        // ✅ Implemented Module Errors (6000-6999)
        6001 => format!("{}#e6001-unknown-native-module", base_url),
        6002 => format!("{}#e6002-import-circular", base_url),

        // 🟡 Planned Syntax Errors (7000-7999)
        7001 => format!("{}#e7001-invalid-syntax-declaration", base_url),

        // 🟡 Planned Warnings (8000-8999)
        8001 => format!("{}#w8001-unused-variable", base_url),
        8002 => format!("{}#w8002-deprecated-function", base_url),
        8003 => format!("{}#w8003-potential-memory-leak", base_url),

        // 🟡 Planned System Errors (9000-9999)
        9001 => format!("{}#e9001-insufficient-memory", base_url),
        9002 => format!("{}#e9002-stack-overflow", base_url),

        // Generic URL for unmapped codes
        _ => {
            let category = error_code / 1000;
            match category {
                1 => format!("{}#lexer-errors-1000-1999", base_url),
                2 => format!("{}#parser-errors-2000-2999", base_url),
                3 => format!("{}#runtime-errors-3000-3999", base_url),
                4 => format!("{}#type-errors-4000-4999", base_url),
                5 => format!("{}#io-errors-5000-5999", base_url),
                6 => format!("{}#module-errors-6000-6999", base_url),
                7 => format!("{}#syntax-errors-7000-7999", base_url),
                8 => format!("{}#warnings-8000-8999", base_url),
                9 => format!("{}#system-errors-9000-9999", base_url),
                _ => base_url.to_string(),
            }
        }
    }
}

/// Generates contextual suggestions based on the error code
pub fn get_error_suggestions(error_code: u16) -> Vec<String> {
    match error_code {
        // ✅ Implemented Lexer Errors
        1001 => vec![
            "Use only letters, numbers, underscores, and valid operators".to_string(),
            "Remove unsupported special characters (@, $, etc.)".to_string(),
            "Check for invisible control characters".to_string(),
        ],
        1002 => vec![
            "Add \" to close the string".to_string(),
            "Check for unintentional line breaks".to_string(),
            "Use \\\" to include quotes inside strings".to_string(),
        ],
        1003 => vec![
            "Add */ to close the block comment".to_string(),
            "Consider using // for single-line comments".to_string(),
            "Check for incorrectly nested comments".to_string(),
        ],
        1004 => vec![
            "For decimal numbers use only one dot: 3.14".to_string(),
            "For binary use only 0 and 1: 0b1010".to_string(),
            "For octal use only 0-7: 0o755".to_string(),
            "For hexadecimal use only 0-9, A-F: 0xFF".to_string(),
        ],
        1005 => vec![
            "Use valid sequences: \\n, \\t, \\r, \\\\, \\\", \\'".to_string(),
            "For Unicode use \\u{XXXX} with 4 hex digits".to_string(),
            "Escape special characters in strings".to_string(),
        ],
        1006 => vec![
            "Use correct format: #<module_name>".to_string(),
            "Valid modules: console_io, file_io, http, crypto, etc.".to_string(),
            "Use only alphanumeric characters and underscores".to_string(),
        ],

        // ✅ Implemented Parser Errors
        2001 => vec![
            "Check if parentheses and braces are balanced".to_string(),
            "Add missing tokens (commas, operators, etc.)".to_string(),
            "Remove unnecessary or duplicate tokens".to_string(),
        ],
        2003 => vec![
            "Add ; at the end of the statement".to_string(),
            "Separate multiple statements with ;".to_string(),
            "Check if the statement syntax is correct".to_string(),
        ],
        2005 => vec![
            "Add ) to close the expression".to_string(),
            "Check if all parentheses are balanced".to_string(),
            "Use parentheses only where necessary".to_string(),
        ],
        2011 => vec![
            "Use: let variable_name = value;".to_string(),
            "Variable name must start with a letter or _".to_string(),
            "Do not use numbers at the start of the variable name".to_string(),
        ],

        // 🟡 Expected Parser Errors
        2017 => vec![
            "Add () after the function name".to_string(),
            "Example: function test() { ... }".to_string(),
        ],
        2018 => vec![
            "Add condition between parentheses".to_string(),
            "Example: while (condition) { ... }".to_string(),
        ],
        2019 => vec![
            "Use: for (init; condition; increment) { ... }".to_string(),
            "All components are optional but ; are required".to_string(),
        ],

        // ✅ Deduplicated Parser Error Suggestions
        2042 => vec![
            "The left side of an assignment must be a variable or property".to_string(),
            "Example: let x = 5; x = 10;".to_string(),
        ],
        2043 => vec![
            "Function parameters must be valid identifiers".to_string(),
            "Example: function foo(a, b, c) { ... }".to_string(),
        ],
        2044 => vec![
            "Async functions must have a name".to_string(),
            "Example: async function fetchData() { ... }".to_string(),
        ],
        2045 => vec![
            "Add { after else keyword".to_string(),
            "Example: else { ... }".to_string(),
        ],
        2046 => vec![
            "Add } to close the class body".to_string(),
            "Check if all braces are balanced inside the class".to_string(),
        ],
        2047 => vec![
            "Class methods must be valid function declarations".to_string(),
            "Example: methodName() { ... }".to_string(),
        ],
        2048 => vec![
            "Use [ for index access".to_string(),
            "Example: array[0] or object[\"key\"]".to_string(),
        ],
        2049 => vec![
            "Add ] to close the index expression".to_string(),
            "Check if all brackets are balanced".to_string(),
        ],
        2116 => vec![
            "Check import syntax: import { name } from \"module\"".to_string(),
            "Verify the module path is correct".to_string(),
        ],
        2117 => vec![
            "Check template string syntax: `text ${expression}`".to_string(),
            "Ensure expressions inside ${} are valid".to_string(),
        ],

        // ✅ Implemented Runtime Errors
        3000 => vec![
            "A general runtime error occurred".to_string(),
            "Check the error message for specific details".to_string(),
        ],
        3001 => vec![
            "Declare the variable: let variable_name = value;".to_string(),
            "Check the variable name spelling".to_string(),
            "Check if the variable is in the correct scope".to_string(),
        ],
        3002 => vec![
            "Check the assignment target is a valid variable".to_string(),
            "Ensure the variable has been declared before assignment".to_string(),
        ],
        3005 => vec![
            "Use compatible operations: numbers with numbers".to_string(),
            "For strings use only + (concatenation)".to_string(),
            "Convert types when necessary".to_string(),
        ],
        3006 => vec![
            "Use * only between numbers".to_string(),
            "For strings use repetition: String(value) * number".to_string(),
        ],
        3007 => vec![
            "Check if divisor != 0 before the operation".to_string(),
            "Use: if (divisor != 0) { result = a / divisor; }".to_string(),
            "Implement error handling for division by zero".to_string(),
        ],
        3009 => vec![
            "Compare compatible types: numbers with numbers".to_string(),
            "Strings are compared lexicographically".to_string(),
            "Use === for strict comparison".to_string(),
        ],
        3010 => vec![
            "Use break only inside while, for, or do-while".to_string(),
            "To exit functions use return".to_string(),
        ],
        3011 => vec![
            "Use continue only inside while, for, or do-while".to_string(),
            "Continue skips to the next loop iteration".to_string(),
        ],
        3020 => vec![
            "Use try/catch to handle exceptions".to_string(),
            "Example: try { ... } catch (e) { ... }".to_string(),
        ],
        3022 => vec![
            "Use this only in class methods".to_string(),
            "this refers to the current class instance".to_string(),
        ],
        3023 => vec![
            "super will be implemented in future versions".to_string(),
            "Use direct calls for now".to_string(),
        ],
        3034 => vec![
            "Use properties only on class instances".to_string(),
            "Example: instance.property = value;".to_string(),
        ],
        3040 => vec![
            "Check for infinite recursion in your code".to_string(),
            "Increase the recursion limit if needed (runtime configuration)".to_string(),
            "Try converting recursion to iteration (loops)".to_string(),
        ],
        3104 => vec![
            "Check the native function arguments".to_string(),
            "Ensure the function is called with the correct number of arguments".to_string(),
        ],
        3105 => vec![
            "Check the promise chain for errors".to_string(),
            "Ensure async functions are properly awaited".to_string(),
        ],
        3106 => vec![
            "Check if the value has the expected type".to_string(),
            "Use type conversion when necessary".to_string(),
        ],

        // ✅ Implemented Module Errors
        6001 => vec![
            "Check if the module exists".to_string(),
            "Available modules: console_io, file_io, http, crypto, etc.".to_string(),
            "Use #<module_name> at the start of the file".to_string(),
        ],

        // 🟡 Planned Warnings
        8001 => vec![
            "Remove the variable if it's not needed".to_string(),
            "Use the variable in the code".to_string(),
            "Prefix with _ if intentional: let _variable = value;".to_string(),
        ],

        // Generic suggestions
        _ => vec![
            "Consult the error guide for more information".to_string(),
            "Check the error context and stack trace".to_string(),
            "Review the official syntax in SYNTAX.md".to_string(),
        ],
    }
}
