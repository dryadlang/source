use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestCategory {
    Lexer,
    Parser,
    Runtime,
    EndToEnd,
}

#[derive(Clone)]
pub struct TestCase {
    pub name: String,
    pub code: String,
    pub description: String,
    pub expected_complexity: String,
    pub category: TestCategory,
}

pub fn get_all_test_cases() -> HashMap<TestCategory, Vec<TestCase>> {
    let mut cases = HashMap::new();
    
    cases.insert(TestCategory::Lexer, vec![
        TestCase {
            name: "simple_arithmetic".to_string(),
            code: "let x = 10 + 20 * 30;".to_string(),
            description: "Simple arithmetic expression".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::Lexer,
        },
        TestCase {
            name: "string_literals".to_string(),
            code: r#"let text = "Hello World";"#.to_string(),
            description: "String literal parsing".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::Lexer,
        },
    ]);
    
    cases.insert(TestCategory::Parser, vec![
        TestCase {
            name: "function_definition".to_string(),
            code: "function test() { return 42; }".to_string(),
            description: "Function definition parsing".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::Parser,
        },
    ]);
    
    cases.insert(TestCategory::Runtime, vec![
        TestCase {
            name: "variable_assignment".to_string(),
            code: "let result = 5 + 3;".to_string(),
            description: "Variable assignment execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::Runtime,
        },
    ]);
    
    // ✅ CORRETO: Casos EndToEnd com sintaxe Dryad válida e variáveis declaradas
    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "complete_program".to_string(),
            code: "function add(a, b) { return a + b; } let result = add(5, 3);".to_string(),
            description: "Complete program execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "while_loop".to_string(),
            code: "let i = 0; while (i < 100) { i = i + 1; }".to_string(),
            description: "While loop execution".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "if_statement".to_string(),
            code: "if (true) { let x = 10; } else { let y = 20; }".to_string(),
            description: "If statement execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "for_loop".to_string(),
            code: "#<console_io> let i = 0; for i = 0; i < 100; i = i + 1 { native_println(i); }".to_string(),
            description: "For loop execution".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "nested_loops".to_string(),
            code: "#<console_io> let i = 0; let j = 0; for i = 0; i < 10; i = i + 1 { for j = 0; j < 5; j = j + 1 { native_println(i); native_println(j); } }".to_string(),
            description: "Nested loops execution".to_string(),
            expected_complexity: "O(n^2)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "variable_operations".to_string(),
            code: "let x = 2; let result = 0; if (x == 1) { result = 1; } else if (x == 2) { result = 2; } else { result = 0; }".to_string(),
            description: "Variable operations with conditionals".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "exception_handling".to_string(),
            code: "#<console_io> try { throw \"test error\"; } catch (e) { native_println(e); }".to_string(),
            description: "Exception handling execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "class_basic".to_string(),
            code: "class Test { function init() { this.x = 1; } function getValue() { return this.x; } } let obj = Test(); let val = obj.getValue();".to_string(),
            description: "Basic class definition and usage".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "complex_function".to_string(),
            code: "function complex(a, b) { if (a > b) { return a - b; } else { return b - a; } } let result = complex(10, 5);".to_string(),
            description: "Complex function execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "stress_loop_1".to_string(),
            code: "let i = 0; while (i < 100) { i = i + 1; }".to_string(),
            description: "Stress test with a large while loop".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "stress_loop_2".to_string(),
            code: "#<console_io> let j = 0; for j = 0; j < 100; j = j + 1 { native_println(j); }".to_string(),
            description: "Stress test with a large for loop".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases
}

pub fn get_test_cases_by_category(category: TestCategory) -> Vec<TestCase> {
    get_all_test_cases().get(&category).cloned().unwrap_or_default()
}