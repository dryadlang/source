use crate::{ErrorCategory, ErrorDef};

// =============================================================================
// LEXER ERRORS (1000-1999)
// =============================================================================

pub const fn e1001() -> ErrorDef {
    ErrorDef {
        code: 1001,
        category: ErrorCategory::Lexer,
        message: "Unexpected character",
        suggestion: Some("Use only letters, numbers, underscores, and valid operators"),
    }
}

pub const fn e1002() -> ErrorDef {
    ErrorDef {
        code: 1002,
        category: ErrorCategory::Lexer,
        message: "Unclosed string literal",
        suggestion: Some("Add a closing quote to terminate the string"),
    }
}

pub const fn e1003() -> ErrorDef {
    ErrorDef {
        code: 1003,
        category: ErrorCategory::Lexer,
        message: "Unclosed block comment",
        suggestion: Some("Add */ to close the block comment"),
    }
}

pub const fn e1004() -> ErrorDef {
    ErrorDef {
        code: 1004,
        category: ErrorCategory::Lexer,
        message: "Invalid number format",
        suggestion: Some("Check number format: 0b for binary, 0o for octal, 0x for hex"),
    }
}

pub const fn e1005() -> ErrorDef {
    ErrorDef {
        code: 1005,
        category: ErrorCategory::Lexer,
        message: "Invalid escape sequence",
        suggestion: Some("Valid sequences: \\n, \\t, \\r, \\\\, \\\", \\', \\u{XXXX}"),
    }
}

pub const fn e1006() -> ErrorDef {
    ErrorDef {
        code: 1006,
        category: ErrorCategory::Lexer,
        message: "Invalid native directive",
        suggestion: Some("Use format: #<module_name> with alphanumeric characters and underscores"),
    }
}

// =============================================================================
// PARSER ERRORS (2000-2999)
// =============================================================================

pub const fn e2001() -> ErrorDef {
    ErrorDef {
        code: 2001,
        category: ErrorCategory::Parser,
        message: "Unexpected token",
        suggestion: Some("Check your syntax near this token"),
    }
}

pub const fn e2003() -> ErrorDef {
    ErrorDef {
        code: 2003,
        category: ErrorCategory::Parser,
        message: "Expected ';' after statement",
        suggestion: Some("Add a semicolon to terminate the statement"),
    }
}

pub const fn e2005() -> ErrorDef {
    ErrorDef {
        code: 2005,
        category: ErrorCategory::Parser,
        message: "Expected ')' after expression",
        suggestion: Some("Check that all parentheses are balanced"),
    }
}

pub const fn e2008() -> ErrorDef {
    ErrorDef {
        code: 2008,
        category: ErrorCategory::Parser,
        message: "Invalid assignment target",
        suggestion: Some("Left side of assignment must be a variable or property"),
    }
}

pub const fn e2012() -> ErrorDef {
    ErrorDef {
        code: 2012,
        category: ErrorCategory::Parser,
        message: "Expected function name",
        suggestion: Some("Provide a name after 'function' keyword"),
    }
}

pub const fn e2013() -> ErrorDef {
    ErrorDef {
        code: 2013,
        category: ErrorCategory::Parser,
        message: "Constant must have an initial value",
        suggestion: Some("Use: const name = value;"),
    }
}

pub const fn e2014() -> ErrorDef {
    ErrorDef {
        code: 2014,
        category: ErrorCategory::Parser,
        message: "Expected parameter name",
        suggestion: Some("Parameter names must be valid identifiers"),
    }
}

pub const fn e2016() -> ErrorDef {
    ErrorDef {
        code: 2016,
        category: ErrorCategory::Parser,
        message: "Expected ')' after parameters",
        suggestion: Some("Close the parameter list with ')'"),
    }
}

pub const fn e2017() -> ErrorDef {
    ErrorDef {
        code: 2017,
        category: ErrorCategory::Parser,
        message: "Expected 'function' after 'async'",
        suggestion: Some("Use: async function name() { }"),
    }
}

pub const fn e2018() -> ErrorDef {
    ErrorDef {
        code: 2018,
        category: ErrorCategory::Parser,
        message: "Expected ')' after function arguments",
        suggestion: Some("Close the argument list with ')'"),
    }
}

pub const fn e2023() -> ErrorDef {
    ErrorDef {
        code: 2023,
        category: ErrorCategory::Parser,
        message: "Expected 'function' after 'thread'",
        suggestion: Some("Use: thread function name() { }"),
    }
}

pub const fn e2024() -> ErrorDef {
    ErrorDef {
        code: 2024,
        category: ErrorCategory::Parser,
        message: "Expected thread function name",
        suggestion: Some("Provide a name after 'thread function'"),
    }
}

pub const fn e2028() -> ErrorDef {
    ErrorDef {
        code: 2028,
        category: ErrorCategory::Parser,
        message: "Unexpected token in template string",
        suggestion: Some("Check template string interpolation syntax: ${expr}"),
    }
}

pub const fn e2029() -> ErrorDef {
    ErrorDef {
        code: 2029,
        category: ErrorCategory::Parser,
        message: "Expected ')' after 'mutex('",
        suggestion: Some("Close the mutex expression with ')'"),
    }
}

pub const fn e2030() -> ErrorDef {
    ErrorDef {
        code: 2030,
        category: ErrorCategory::Parser,
        message: "Expected '(' after 'mutex'",
        suggestion: Some("Use: mutex(expression)"),
    }
}

pub const fn e2032() -> ErrorDef {
    ErrorDef {
        code: 2032,
        category: ErrorCategory::Parser,
        message: "Expected '(' after 'thread'",
        suggestion: Some("Use: thread function name() { }"),
    }
}

pub const fn e2033() -> ErrorDef {
    ErrorDef {
        code: 2033,
        category: ErrorCategory::Parser,
        message: "Expected statement",
        suggestion: Some("Provide a valid statement or expression"),
    }
}

pub const fn e2035() -> ErrorDef {
    ErrorDef {
        code: 2035,
        category: ErrorCategory::Parser,
        message: "Expected '=>' after match pattern",
        suggestion: Some("Use: pattern => expression"),
    }
}

pub const fn e2036() -> ErrorDef {
    ErrorDef {
        code: 2036,
        category: ErrorCategory::Parser,
        message: "Expected '}' to close match block",
        suggestion: Some("Add '}' to close the match expression"),
    }
}

pub const fn e2037() -> ErrorDef {
    ErrorDef {
        code: 2037,
        category: ErrorCategory::Parser,
        message: "Expected ']' after array patterns",
        suggestion: Some("Close the array pattern with ']'"),
    }
}

pub const fn e2038() -> ErrorDef {
    ErrorDef {
        code: 2038,
        category: ErrorCategory::Parser,
        message: "Expected ')' after tuple patterns",
        suggestion: Some("Close the tuple pattern with ')'"),
    }
}

pub const fn e2041() -> ErrorDef {
    ErrorDef {
        code: 2041,
        category: ErrorCategory::Parser,
        message: "Expected '}' after object patterns",
        suggestion: Some("Close the object pattern with '}'"),
    }
}

pub const fn e2042() -> ErrorDef {
    ErrorDef {
        code: 2042,
        category: ErrorCategory::Parser,
        message: "Invalid assignment operator",
        suggestion: Some("Valid operators: =, +=, -=, *=, /=, %=, **=, &&=, ||="),
    }
}

pub const fn e2043() -> ErrorDef {
    ErrorDef {
        code: 2043,
        category: ErrorCategory::Parser,
        message: "Expected '(' after function name",
        suggestion: Some("Function declaration requires parameter list: function name() { }"),
    }
}

pub const fn e2044() -> ErrorDef {
    ErrorDef {
        code: 2044,
        category: ErrorCategory::Parser,
        message: "Expected async function name",
        suggestion: Some("Provide a name: async function name() { }"),
    }
}

pub const fn e2045() -> ErrorDef {
    ErrorDef {
        code: 2045,
        category: ErrorCategory::Parser,
        message: "Expected '{' after 'else'",
        suggestion: Some("Use: else { ... } or else if (condition) { ... }"),
    }
}

pub const fn e2046() -> ErrorDef {
    ErrorDef {
        code: 2046,
        category: ErrorCategory::Parser,
        message: "Expected '}' to close class body",
        suggestion: Some("Add '}' to close the class declaration"),
    }
}

pub const fn e2047() -> ErrorDef {
    ErrorDef {
        code: 2047,
        category: ErrorCategory::Parser,
        message: "Expected method name",
        suggestion: Some("Provide a method name identifier"),
    }
}

pub const fn e2048() -> ErrorDef {
    ErrorDef {
        code: 2048,
        category: ErrorCategory::Parser,
        message: "Expected '[' after identifier",
        suggestion: Some("Use bracket notation: identifier[index]"),
    }
}

pub const fn e2049() -> ErrorDef {
    ErrorDef {
        code: 2049,
        category: ErrorCategory::Parser,
        message: "Expected ']' after index",
        suggestion: Some("Close the index access with ']'"),
    }
}

pub const fn e2051() -> ErrorDef {
    ErrorDef {
        code: 2051,
        category: ErrorCategory::Parser,
        message: "Expected ')' after while condition",
        suggestion: Some("Close the while condition: while (condition) { }"),
    }
}

pub const fn e2053() -> ErrorDef {
    ErrorDef {
        code: 2053,
        category: ErrorCategory::Parser,
        message: "Expected '{' after 'do'",
        suggestion: Some("Use: do { ... } while (condition);"),
    }
}

pub const fn e2063() -> ErrorDef {
    ErrorDef {
        code: 2063,
        category: ErrorCategory::Parser,
        message: "Expected '{' after for parentheses",
        suggestion: Some("Add '{' to start the for loop body"),
    }
}

pub const fn e2071() -> ErrorDef {
    ErrorDef {
        code: 2071,
        category: ErrorCategory::Parser,
        message: "Expected ']' after array index",
        suggestion: Some("Close the array access with ']'"),
    }
}

pub const fn e2073() -> ErrorDef {
    ErrorDef {
        code: 2073,
        category: ErrorCategory::Parser,
        message: "Expected ',' or ')' in method argument list",
        suggestion: Some("Separate arguments with ',' or close with ')'"),
    }
}

pub const fn e2075() -> ErrorDef {
    ErrorDef {
        code: 2075,
        category: ErrorCategory::Parser,
        message: "Expected ')' after parameters",
        suggestion: Some("Close the parameter list with ')'"),
    }
}

pub const fn e2080() -> ErrorDef {
    ErrorDef {
        code: 2080,
        category: ErrorCategory::Parser,
        message: "Expected '{' after 'try'",
        suggestion: Some("Use: try { ... } catch (e) { ... }"),
    }
}

pub const fn e2081() -> ErrorDef {
    ErrorDef {
        code: 2081,
        category: ErrorCategory::Parser,
        message: "Expected '(' after 'catch'",
        suggestion: Some("Use: catch (error) { ... }"),
    }
}

pub const fn e2082() -> ErrorDef {
    ErrorDef {
        code: 2082,
        category: ErrorCategory::Parser,
        message: "Expected ')' after arguments",
        suggestion: Some("Close the argument list with ')'"),
    }
}

pub const fn e2083() -> ErrorDef {
    ErrorDef {
        code: 2083,
        category: ErrorCategory::Parser,
        message: "Expected ')' after catch variable",
        suggestion: Some("Close catch: catch (error) { ... }"),
    }
}

pub const fn e2085() -> ErrorDef {
    ErrorDef {
        code: 2085,
        category: ErrorCategory::Parser,
        message: "Expected '{' after 'finally'",
        suggestion: Some("Use: finally { ... }"),
    }
}

pub const fn e2090() -> ErrorDef {
    ErrorDef {
        code: 2090,
        category: ErrorCategory::Parser,
        message: "Expected class name after 'new'",
        suggestion: Some("Use: new ClassName()"),
    }
}

pub const fn e2091() -> ErrorDef {
    ErrorDef {
        code: 2091,
        category: ErrorCategory::Parser,
        message: "Expected async method name",
        suggestion: Some("Provide a name for the async method"),
    }
}

pub const fn e2092() -> ErrorDef {
    ErrorDef {
        code: 2092,
        category: ErrorCategory::Parser,
        message: "Expected '(' after method name",
        suggestion: Some("Method declaration requires parameter list"),
    }
}

pub const fn e2094() -> ErrorDef {
    ErrorDef {
        code: 2094,
        category: ErrorCategory::Parser,
        message: "Expected parameter name",
        suggestion: Some("Parameters must be valid identifiers"),
    }
}

pub const fn e2095() -> ErrorDef {
    ErrorDef {
        code: 2095,
        category: ErrorCategory::Parser,
        message: "Expected property name",
        suggestion: Some("Property names must be valid identifiers"),
    }
}

pub const fn e2098() -> ErrorDef {
    ErrorDef {
        code: 2098,
        category: ErrorCategory::Parser,
        message: "Expected getter name",
        suggestion: Some("Use: get propertyName() { ... }"),
    }
}

pub const fn e2099() -> ErrorDef {
    ErrorDef {
        code: 2099,
        category: ErrorCategory::Parser,
        message: "Expected ')' in getter",
        suggestion: Some("Getter takes no parameters: get name() { }"),
    }
}

pub const fn e2100() -> ErrorDef {
    ErrorDef {
        code: 2100,
        category: ErrorCategory::Parser,
        message: "Expected setter name",
        suggestion: Some("Use: set propertyName(value) { ... }"),
    }
}

pub const fn e2101() -> ErrorDef {
    ErrorDef {
        code: 2101,
        category: ErrorCategory::Parser,
        message: "Expected '(' after setter name",
        suggestion: Some("Setter requires a parameter: set name(value) { }"),
    }
}

pub const fn e2102() -> ErrorDef {
    ErrorDef {
        code: 2102,
        category: ErrorCategory::Parser,
        message: "Expected parameter in setter",
        suggestion: Some("Setter must have exactly one parameter"),
    }
}

pub const fn e2103() -> ErrorDef {
    ErrorDef {
        code: 2103,
        category: ErrorCategory::Parser,
        message: "Expected ')' in setter",
        suggestion: Some("Close setter parameter list with ')'"),
    }
}

pub const fn e2104() -> ErrorDef {
    ErrorDef {
        code: 2104,
        category: ErrorCategory::Parser,
        message: "Expected '=' for index assignment",
        suggestion: Some("Use: array[index] = value"),
    }
}

pub const fn e2107() -> ErrorDef {
    ErrorDef {
        code: 2107,
        category: ErrorCategory::Parser,
        message: "Expected '}' to close interface",
        suggestion: Some("Add '}' to close the interface declaration"),
    }
}

pub const fn e2108() -> ErrorDef {
    ErrorDef {
        code: 2108,
        category: ErrorCategory::Parser,
        message: "Expected 'function' in interface",
        suggestion: Some("Interface methods: function name(params);"),
    }
}

pub const fn e2109() -> ErrorDef {
    ErrorDef {
        code: 2109,
        category: ErrorCategory::Parser,
        message: "Expected method name in interface",
        suggestion: Some("Provide a name for the interface method"),
    }
}

pub const fn e2110() -> ErrorDef {
    ErrorDef {
        code: 2110,
        category: ErrorCategory::Parser,
        message: "Expected '(' after method name in interface",
        suggestion: Some("Interface methods require parameter list"),
    }
}

pub const fn e2111() -> ErrorDef {
    ErrorDef {
        code: 2111,
        category: ErrorCategory::Parser,
        message: "Expected parameter name in interface method",
        suggestion: Some("Parameters must be valid identifiers"),
    }
}

pub const fn e2112() -> ErrorDef {
    ErrorDef {
        code: 2112,
        category: ErrorCategory::Parser,
        message: "Expected ')' after interface method parameters",
        suggestion: Some("Close parameter list with ')'"),
    }
}

pub const fn e2115() -> ErrorDef {
    ErrorDef {
        code: 2115,
        category: ErrorCategory::Parser,
        message: "Expected '}' to close namespace",
        suggestion: Some("Add '}' to close the namespace block"),
    }
}

pub const fn e2116() -> ErrorDef {
    ErrorDef {
        code: 2116,
        category: ErrorCategory::Parser,
        message: "Invalid import syntax",
        suggestion: Some("Use: import { name } from 'module' or import * as name from 'module'"),
    }
}

pub const fn e2117() -> ErrorDef {
    ErrorDef {
        code: 2117,
        category: ErrorCategory::Parser,
        message: "Unclosed template string",
        suggestion: Some("Add closing backtick to terminate template string"),
    }
}

pub const fn e2011() -> ErrorDef {
    ErrorDef {
        code: 2011,
        category: ErrorCategory::Parser,
        message: "Expected variable name after 'let'",
        suggestion: Some("Use: let name = value;"),
    }
}

pub const fn e2015() -> ErrorDef {
    ErrorDef {
        code: 2015,
        category: ErrorCategory::Parser,
        message: "Expected ',' or ')' in parameter list",
        suggestion: Some("Separate parameters with ',' or close with ')'"),
    }
}

pub const fn e2019() -> ErrorDef {
    ErrorDef {
        code: 2019,
        category: ErrorCategory::Parser,
        message: "Expected parameter identifier",
        suggestion: Some("Parameters must be valid identifiers"),
    }
}

pub const fn e2020() -> ErrorDef {
    ErrorDef {
        code: 2020,
        category: ErrorCategory::Parser,
        message: "Expected ')' after lambda parameters",
        suggestion: Some("Close lambda parameter list with ')'"),
    }
}

pub const fn e2021() -> ErrorDef {
    ErrorDef {
        code: 2021,
        category: ErrorCategory::Parser,
        message: "Expected '=>' after lambda parameters",
        suggestion: Some("Use arrow syntax: (params) => expression"),
    }
}

pub const fn e2022() -> ErrorDef {
    ErrorDef {
        code: 2022,
        category: ErrorCategory::Parser,
        message: "Expected ')' after async function parameters",
        suggestion: Some("Close parameter list: async function name(params) { }"),
    }
}

pub const fn e2025() -> ErrorDef {
    ErrorDef {
        code: 2025,
        category: ErrorCategory::Parser,
        message: "Expected '(' after thread function name",
        suggestion: Some("Use: thread function name() { }"),
    }
}

pub const fn e2026() -> ErrorDef {
    ErrorDef {
        code: 2026,
        category: ErrorCategory::Parser,
        message: "Expected parameter name in thread function",
        suggestion: Some("Parameters must be valid identifiers"),
    }
}

pub const fn e2027() -> ErrorDef {
    ErrorDef {
        code: 2027,
        category: ErrorCategory::Parser,
        message: "Expected ',' or ')' in thread function parameter list",
        suggestion: Some("Separate parameters with ',' or close with ')'"),
    }
}

pub const fn e2031() -> ErrorDef {
    ErrorDef {
        code: 2031,
        category: ErrorCategory::Parser,
        message: "Expected ')' after thread arguments",
        suggestion: Some("Close the thread argument list with ')'"),
    }
}

pub const fn e2034() -> ErrorDef {
    ErrorDef {
        code: 2034,
        category: ErrorCategory::Parser,
        message: "Expected '{' after match expression",
        suggestion: Some("Use: match (expr) { pattern => result }"),
    }
}

pub const fn e2039() -> ErrorDef {
    ErrorDef {
        code: 2039,
        category: ErrorCategory::Parser,
        message: "Expected object key in pattern",
        suggestion: Some("Object patterns use: { key: pattern }"),
    }
}

pub const fn e2040() -> ErrorDef {
    ErrorDef {
        code: 2040,
        category: ErrorCategory::Parser,
        message: "Expected ':' after object key in pattern",
        suggestion: Some("Use: { key: pattern }"),
    }
}

pub const fn e2052() -> ErrorDef {
    ErrorDef {
        code: 2052,
        category: ErrorCategory::Parser,
        message: "Expected '{' after while parentheses",
        suggestion: Some("Add '{' to start the while loop body"),
    }
}

pub const fn e2054() -> ErrorDef {
    ErrorDef {
        code: 2054,
        category: ErrorCategory::Parser,
        message: "Expected 'while' after do-while body",
        suggestion: Some("Use: do { ... } while (condition);"),
    }
}

pub const fn e2055() -> ErrorDef {
    ErrorDef {
        code: 2055,
        category: ErrorCategory::Parser,
        message: "Expected '(' after 'for'",
        suggestion: Some("Use: for (init; condition; update) { }"),
    }
}

pub const fn e2056() -> ErrorDef {
    ErrorDef {
        code: 2056,
        category: ErrorCategory::Parser,
        message: "Expected '=' in for loop initializer",
        suggestion: Some("Use: for (i = 0; condition; update) { }"),
    }
}

pub const fn e2057() -> ErrorDef {
    ErrorDef {
        code: 2057,
        category: ErrorCategory::Parser,
        message: "Expected identifier in for loop initializer",
        suggestion: Some("Use: for (i = 0; condition; update) { }"),
    }
}

pub const fn e2058() -> ErrorDef {
    ErrorDef {
        code: 2058,
        category: ErrorCategory::Parser,
        message: "Expected ';' after for loop initializer",
        suggestion: Some("Use: for (init; condition; update) { }"),
    }
}

pub const fn e2059() -> ErrorDef {
    ErrorDef {
        code: 2059,
        category: ErrorCategory::Parser,
        message: "Expected ';' after for loop condition",
        suggestion: Some("Use: for (init; condition; update) { }"),
    }
}

pub const fn e2060() -> ErrorDef {
    ErrorDef {
        code: 2060,
        category: ErrorCategory::Parser,
        message: "Expected '=', '++' or '--' in for loop update",
        suggestion: Some("Use: for (...; ...; i++) or for (...; ...; i = i + 1)"),
    }
}

pub const fn e2061() -> ErrorDef {
    ErrorDef {
        code: 2061,
        category: ErrorCategory::Parser,
        message: "Expected identifier in for loop update",
        suggestion: Some("Use: for (...; ...; i++) { }"),
    }
}

pub const fn e2062() -> ErrorDef {
    ErrorDef {
        code: 2062,
        category: ErrorCategory::Parser,
        message: "Expected ')' after for loop declaration",
        suggestion: Some("Close for loop: for (init; condition; update) { }"),
    }
}

pub const fn e2065() -> ErrorDef {
    ErrorDef {
        code: 2065,
        category: ErrorCategory::Parser,
        message: "Expected '(' after 'while' in do-while",
        suggestion: Some("Use: do { ... } while (condition);"),
    }
}

pub const fn e2066() -> ErrorDef {
    ErrorDef {
        code: 2066,
        category: ErrorCategory::Parser,
        message: "Expected ')' after do-while condition",
        suggestion: Some("Close condition: do { ... } while (condition);"),
    }
}

pub const fn e2067() -> ErrorDef {
    ErrorDef {
        code: 2067,
        category: ErrorCategory::Parser,
        message: "Expected ';' after do-while parentheses",
        suggestion: Some("Terminate with: do { ... } while (condition);"),
    }
}

pub const fn e2068() -> ErrorDef {
    ErrorDef {
        code: 2068,
        category: ErrorCategory::Parser,
        message: "Expected 'in' in foreach loop",
        suggestion: Some("Use: for (item in collection) { }"),
    }
}

pub const fn e2069() -> ErrorDef {
    ErrorDef {
        code: 2069,
        category: ErrorCategory::Parser,
        message: "Expected ')' after foreach expression",
        suggestion: Some("Close: for (item in collection) { }"),
    }
}

pub const fn e2070() -> ErrorDef {
    ErrorDef {
        code: 2070,
        category: ErrorCategory::Parser,
        message: "Expected '{' after foreach parentheses",
        suggestion: Some("Add '{' to start the foreach loop body"),
    }
}

pub const fn e2072() -> ErrorDef {
    ErrorDef {
        code: 2072,
        category: ErrorCategory::Parser,
        message: "Expected identifier or number after '.' for access",
        suggestion: Some("Use dot notation: object.property"),
    }
}

pub const fn e2074() -> ErrorDef {
    ErrorDef {
        code: 2074,
        category: ErrorCategory::Parser,
        message: "Expected ')' after method arguments",
        suggestion: Some("Close the method argument list with ')'"),
    }
}

pub const fn e2076() -> ErrorDef {
    ErrorDef {
        code: 2076,
        category: ErrorCategory::Parser,
        message: "Expected ')' after call arguments",
        suggestion: Some("Close the function call with ')'"),
    }
}

pub const fn e2077() -> ErrorDef {
    ErrorDef {
        code: 2077,
        category: ErrorCategory::Parser,
        message: "Expected ':' or '(' after property key",
        suggestion: Some("Object properties use: { key: value } or { method() {} }"),
    }
}

pub const fn e2084() -> ErrorDef {
    ErrorDef {
        code: 2084,
        category: ErrorCategory::Parser,
        message: "Expected '{' after catch parameter",
        suggestion: Some("Use: catch (error) { ... }"),
    }
}

pub const fn e2086() -> ErrorDef {
    ErrorDef {
        code: 2086,
        category: ErrorCategory::Parser,
        message: "Try block must have at least one catch or finally",
        suggestion: Some("Add catch or finally: try { } catch (e) { } finally { }"),
    }
}

pub const fn e2087() -> ErrorDef {
    ErrorDef {
        code: 2087,
        category: ErrorCategory::Parser,
        message: "Expected class name after 'class'",
        suggestion: Some("Use: class ClassName { }"),
    }
}

pub const fn e2088() -> ErrorDef {
    ErrorDef {
        code: 2088,
        category: ErrorCategory::Parser,
        message: "Expected parent class name after 'extends'",
        suggestion: Some("Use: class Child extends Parent { }"),
    }
}

pub const fn e2089() -> ErrorDef {
    ErrorDef {
        code: 2089,
        category: ErrorCategory::Parser,
        message: "Expected '{' after class declaration",
        suggestion: Some("Use: class ClassName { ... }"),
    }
}

pub const fn e2093() -> ErrorDef {
    ErrorDef {
        code: 2093,
        category: ErrorCategory::Parser,
        message: "Expected ',' or ')' in method parameter list",
        suggestion: Some("Separate parameters with ',' or close with ')'"),
    }
}

pub const fn e2096() -> ErrorDef {
    ErrorDef {
        code: 2096,
        category: ErrorCategory::Parser,
        message: "Expected method or property declaration in class",
        suggestion: Some("Classes contain methods and properties"),
    }
}

pub const fn e2097() -> ErrorDef {
    ErrorDef {
        code: 2097,
        category: ErrorCategory::Parser,
        message: "Expected property name after 'get'",
        suggestion: Some("Use: get propertyName() { ... }"),
    }
}

pub const fn e2105() -> ErrorDef {
    ErrorDef {
        code: 2105,
        category: ErrorCategory::Parser,
        message: "Expected interface name after 'interface'",
        suggestion: Some("Use: interface InterfaceName { }"),
    }
}

pub const fn e2106() -> ErrorDef {
    ErrorDef {
        code: 2106,
        category: ErrorCategory::Parser,
        message: "Expected '{' after interface declaration",
        suggestion: Some("Use: interface InterfaceName { ... }"),
    }
}

pub const fn e4001() -> ErrorDef {
    ErrorDef {
        code: 4001,
        category: ErrorCategory::Type,
        message: "Export must be followed by 'function', 'class' or 'let'",
        suggestion: Some(
            "Use: export function name() {}, export class Name {}, or export let name = value",
        ),
    }
}

// =============================================================================
// RUNTIME ERRORS (3000-3999)
// =============================================================================

pub const fn e3000() -> ErrorDef {
    ErrorDef {
        code: 3000,
        category: ErrorCategory::Runtime,
        message: "Bytecode compilation error",
        suggestion: Some("Check the source code for syntax or type errors"),
    }
}

pub const fn e3001() -> ErrorDef {
    ErrorDef {
        code: 3001,
        category: ErrorCategory::Runtime,
        message: "Undefined variable",
        suggestion: Some("Declare the variable with 'let' or 'const' before using it"),
    }
}

pub const fn e3002() -> ErrorDef {
    ErrorDef {
        code: 3002,
        category: ErrorCategory::Runtime,
        message: "Bytecode runtime error",
        suggestion: Some("Check the bytecode execution for issues"),
    }
}

pub const fn e3003() -> ErrorDef {
    ErrorDef {
        code: 3003,
        category: ErrorCategory::Runtime,
        message: "Expression is not a function",
        suggestion: Some("Ensure the value is callable before invoking it"),
    }
}

pub const fn e3004() -> ErrorDef {
    ErrorDef {
        code: 3004,
        category: ErrorCategory::Runtime,
        message: "Cannot determine base directory",
        suggestion: Some("Check that the file path is valid"),
    }
}

pub const fn e3005() -> ErrorDef {
    ErrorDef {
        code: 3005,
        category: ErrorCategory::Runtime,
        message: "Invalid arithmetic operation",
        suggestion: Some("Use compatible types: numbers with numbers"),
    }
}

pub const fn e3006() -> ErrorDef {
    ErrorDef {
        code: 3006,
        category: ErrorCategory::Runtime,
        message: "Invalid multiplication",
        suggestion: Some("Use * only between numbers"),
    }
}

pub const fn e3007() -> ErrorDef {
    ErrorDef {
        code: 3007,
        category: ErrorCategory::Runtime,
        message: "Division by zero",
        suggestion: Some("Check that the divisor is not zero before dividing"),
    }
}

pub const fn e3008() -> ErrorDef {
    ErrorDef {
        code: 3008,
        category: ErrorCategory::Runtime,
        message: "Module resolution error",
        suggestion: Some("Check that the module path exists and is accessible"),
    }
}

pub const fn e3009() -> ErrorDef {
    ErrorDef {
        code: 3009,
        category: ErrorCategory::Runtime,
        message: "Invalid comparison",
        suggestion: Some("Comparison is only valid for numbers"),
    }
}

pub const fn e3010() -> ErrorDef {
    ErrorDef {
        code: 3010,
        category: ErrorCategory::Runtime,
        message: "break",
        suggestion: None,
    }
}

pub const fn e3011() -> ErrorDef {
    ErrorDef {
        code: 3011,
        category: ErrorCategory::Runtime,
        message: "continue",
        suggestion: None,
    }
}

pub const fn e3015() -> ErrorDef {
    ErrorDef {
        code: 3015,
        category: ErrorCategory::Runtime,
        message: "Division by zero in modulo operator",
        suggestion: Some("Check that the divisor is not zero"),
    }
}

pub const fn e3020() -> ErrorDef {
    ErrorDef {
        code: 3020,
        category: ErrorCategory::Runtime,
        message: "Exception thrown",
        suggestion: Some("Use try/catch to handle exceptions"),
    }
}

pub const fn e3021() -> ErrorDef {
    ErrorDef {
        code: 3021,
        category: ErrorCategory::Runtime,
        message: "RETURN_PENDING",
        suggestion: None,
    }
}

pub const fn e3022() -> ErrorDef {
    ErrorDef {
        code: 3022,
        category: ErrorCategory::Runtime,
        message: "Invalid 'this' context",
        suggestion: Some("Use 'this' only inside class methods"),
    }
}

pub const fn e3023() -> ErrorDef {
    ErrorDef {
        code: 3023,
        category: ErrorCategory::Runtime,
        message: "'super' is not yet implemented",
        suggestion: Some("Use direct method calls as a workaround"),
    }
}

pub const fn e3025() -> ErrorDef {
    ErrorDef {
        code: 3025,
        category: ErrorCategory::Runtime,
        message: "Callback required",
        suggestion: Some("Provide a function as callback argument"),
    }
}

pub const fn e3029() -> ErrorDef {
    ErrorDef {
        code: 3029,
        category: ErrorCategory::Runtime,
        message: "Property not found",
        suggestion: Some("Check that the property exists on the object"),
    }
}

pub const fn e3030() -> ErrorDef {
    ErrorDef {
        code: 3030,
        category: ErrorCategory::Runtime,
        message: "Static property not found",
        suggestion: Some("Check that the static property is defined in the class"),
    }
}

pub const fn e3034() -> ErrorDef {
    ErrorDef {
        code: 3034,
        category: ErrorCategory::Runtime,
        message: "Invalid property assignment",
        suggestion: Some("Use properties only on class instances"),
    }
}

pub const fn e3040() -> ErrorDef {
    ErrorDef {
        code: 3040,
        category: ErrorCategory::Runtime,
        message: "Stack overflow",
        suggestion: Some("Check for infinite recursion; consider converting to iteration"),
    }
}

pub const fn e3081() -> ErrorDef {
    ErrorDef {
        code: 3081,
        category: ErrorCategory::Runtime,
        message: "Array index must be a number",
        suggestion: Some("Use numeric indices for array access"),
    }
}

pub const fn e3100() -> ErrorDef {
    ErrorDef {
        code: 3100,
        category: ErrorCategory::Runtime,
        message: "Heap reference not found",
        suggestion: Some("The referenced value may have been garbage collected or is invalid"),
    }
}

pub const fn e3101() -> ErrorDef {
    ErrorDef {
        code: 3101,
        category: ErrorCategory::Runtime,
        message: "Heap type mismatch",
        suggestion: Some("The heap value is not the expected type"),
    }
}

pub const fn e3104() -> ErrorDef {
    ErrorDef {
        code: 3104,
        category: ErrorCategory::Runtime,
        message: "Native function error",
        suggestion: Some("Check the arguments passed to the native function"),
    }
}

pub const fn e3105() -> ErrorDef {
    ErrorDef {
        code: 3105,
        category: ErrorCategory::Runtime,
        message: "Promise not resolved",
        suggestion: Some("Use 'await' to wait for the promise to resolve"),
    }
}

pub const fn e3106() -> ErrorDef {
    ErrorDef {
        code: 3106,
        category: ErrorCategory::Runtime,
        message: "Runtime type error",
        suggestion: Some("Check that the value is the expected type"),
    }
}

pub const fn e3012() -> ErrorDef {
    ErrorDef {
        code: 3012,
        category: ErrorCategory::Runtime,
        message: "Increment operator requires a variable",
        suggestion: Some("Use ++ only on variables, not expressions"),
    }
}

pub const fn e3013() -> ErrorDef {
    ErrorDef {
        code: 3013,
        category: ErrorCategory::Runtime,
        message: "Decrement operator requires a number",
        suggestion: Some("Use -- only on numeric values"),
    }
}

pub const fn e3014() -> ErrorDef {
    ErrorDef {
        code: 3014,
        category: ErrorCategory::Runtime,
        message: "Decrement operator requires a variable",
        suggestion: Some("Use -- only on variables, not expressions"),
    }
}

pub const fn e3016() -> ErrorDef {
    ErrorDef {
        code: 3016,
        category: ErrorCategory::Runtime,
        message: "Modulo operator requires numbers",
        suggestion: Some("Use '%' only between numeric values"),
    }
}

pub const fn e3017() -> ErrorDef {
    ErrorDef {
        code: 3017,
        category: ErrorCategory::Runtime,
        message: "Exponent operator requires numbers",
        suggestion: Some("Use '**' only between numeric values"),
    }
}

pub const fn e3024() -> ErrorDef {
    ErrorDef {
        code: 3024,
        category: ErrorCategory::Runtime,
        message: "Integer root operator requires numbers",
        suggestion: Some("Use '##' only between numeric values"),
    }
}

pub const fn e3026() -> ErrorDef {
    ErrorDef {
        code: 3026,
        category: ErrorCategory::Runtime,
        message: "Bitwise AND requires numbers",
        suggestion: Some("Use '&' only between numeric values"),
    }
}

pub const fn e3027() -> ErrorDef {
    ErrorDef {
        code: 3027,
        category: ErrorCategory::Runtime,
        message: "Bitwise OR requires numbers",
        suggestion: Some("Use '|' only between numeric values"),
    }
}

pub const fn e3028() -> ErrorDef {
    ErrorDef {
        code: 3028,
        category: ErrorCategory::Runtime,
        message: "Bitwise XOR requires numbers",
        suggestion: Some("Use '^' only between numeric values"),
    }
}

pub const fn e3031() -> ErrorDef {
    ErrorDef {
        code: 3031,
        category: ErrorCategory::Runtime,
        message: "Cannot shift with negative number",
        suggestion: Some("Use a non-negative shift amount"),
    }
}

pub const fn e3032() -> ErrorDef {
    ErrorDef {
        code: 3032,
        category: ErrorCategory::Runtime,
        message: "Right shift requires numbers",
        suggestion: Some("Use '>>' only between numeric values"),
    }
}

pub const fn e3033() -> ErrorDef {
    ErrorDef {
        code: 3033,
        category: ErrorCategory::Runtime,
        message: "Cannot shift with negative number",
        suggestion: Some("Use a non-negative shift amount"),
    }
}

pub const fn e3035() -> ErrorDef {
    ErrorDef {
        code: 3035,
        category: ErrorCategory::Runtime,
        message: "Cannot shift with negative number",
        suggestion: Some("Use a non-negative shift amount"),
    }
}

pub const fn e3036() -> ErrorDef {
    ErrorDef {
        code: 3036,
        category: ErrorCategory::Runtime,
        message: "Unsigned right shift requires numbers",
        suggestion: Some("Use '>>>' only between numeric values"),
    }
}

pub const fn e3080() -> ErrorDef {
    ErrorDef {
        code: 3080,
        category: ErrorCategory::Runtime,
        message: "Index must be a non-negative integer",
        suggestion: Some("Use a non-negative integer as array index"),
    }
}

pub const fn e3082() -> ErrorDef {
    ErrorDef {
        code: 3082,
        category: ErrorCategory::Runtime,
        message: "Array index out of bounds",
        suggestion: Some("Check that the index is within the array length"),
    }
}

pub const fn e3083() -> ErrorDef {
    ErrorDef {
        code: 3083,
        category: ErrorCategory::Runtime,
        message: "Subscript operator requires array or object",
        suggestion: Some("Use [] only on arrays and objects"),
    }
}

pub const fn e3084() -> ErrorDef {
    ErrorDef {
        code: 3084,
        category: ErrorCategory::Runtime,
        message: "Object key must be string or number",
        suggestion: Some("Use a string or number as object key"),
    }
}

pub const fn e3085() -> ErrorDef {
    ErrorDef {
        code: 3085,
        category: ErrorCategory::Runtime,
        message: "Dot operator requires a tuple",
        suggestion: Some("Use . for tuple element access only"),
    }
}

pub const fn e3102() -> ErrorDef {
    ErrorDef {
        code: 3102,
        category: ErrorCategory::Runtime,
        message: "Interface not found",
        suggestion: Some("Check that the interface is defined before implementing it"),
    }
}

pub const fn e3103() -> ErrorDef {
    ErrorDef {
        code: 3103,
        category: ErrorCategory::Runtime,
        message: "Interface method not implemented",
        suggestion: Some("Implement all required methods from the interface"),
    }
}

pub const fn e4002() -> ErrorDef {
    ErrorDef {
        code: 4002,
        category: ErrorCategory::Runtime,
        message: "Argument count mismatch",
        suggestion: Some("Check the function signature for the expected number of arguments"),
    }
}

pub const fn e4003() -> ErrorDef {
    ErrorDef {
        code: 4003,
        category: ErrorCategory::Runtime,
        message: "Expression is not a valid thread function",
        suggestion: Some("Use a function declaration or named function for thread()"),
    }
}

// =============================================================================
// MODULE ERRORS (6000-6999)
// =============================================================================

pub const fn e6001() -> ErrorDef {
    ErrorDef {
        code: 6001,
        category: ErrorCategory::Module,
        message: "Unknown native module",
        suggestion: Some("Check available modules: console_io, file_io, http, crypto, etc."),
    }
}
