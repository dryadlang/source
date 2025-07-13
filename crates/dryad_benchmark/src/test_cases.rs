use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestCategory {
    Lexer,
    Parser,
    Runtime,
    EndToEnd,
}

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
    
    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "complete_program".to_string(),
            code: "function add(a, b) { return a + b; }; let result = add(5, 3);".to_string(),
            description: "Complete program execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "while_loop".to_string(),
            code: "let i = 0; while (i < 100) { i++; }".to_string(),
            description: "While loop execution".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);
    
    cases
}

pub fn get_test_cases_by_category(category: TestCategory) -> Vec<TestCase> {
    vec![]
}