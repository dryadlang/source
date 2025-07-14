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

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "if_statement".to_string(),
            code: "if (true) { let x = 10; } else { let y = 20; }".to_string(),
            description: "If statement execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "for_loop".to_string(),
            code: "for (let i = 0; i < 100; i++) { console.log(i); }".to_string(),
            description: "For loop execution".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "nested_loops".to_string(),
            code: "for (let i = 0; i < 10; i++) { for (let j = 0; j < 5; j++) { console.log(i, j); } }".to_string(),
            description: "Nested loops execution".to_string(),
            expected_complexity: "O(n^2)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "switch_statement".to_string(),
            code: "let x = 2; switch (x) { case 1: console.log('one'); break; case 2: console.log('two'); break; default: console.log('other'); }".to_string(),
            description: "Switch statement execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "try_catch".to_string(),
            code: "try { throw new Error('test'); } catch (e) { console.log(e.message); }".to_string(),
            description: "Try-catch statement execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "class_definition".to_string(),
            code: "class Test { constructor() { this.x = 1; } } let obj = new Test();".to_string(),
            description: "Class definition and instantiation".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "complex_function".to_string(),
            code: "function complex(a, b) { if (a > b) { return a - b; } else { return b - a; } } let result = complex(10, 5);".to_string(),
            description: "Complex function execution".to_string(),
            expected_complexity: "O(1)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    // Stress loop test cases

    cases.insert(TestCategory::EndToEnd, vec![
        TestCase {
            name: "stress_loop_1".to_string(),
            code: "let i = 0; while (i < 100) { i++; }".to_string(),
            description: "Stress test with a large while loop".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
        TestCase {
            name: "stress_loop_2".to_string(),
            code: "#<console_io> for (let j = 0; j < 100; j++) { native_println(j); }".to_string(),
            description: "Stress test with a large for loop".to_string(),
            expected_complexity: "O(n)".to_string(),
            category: TestCategory::EndToEnd,
        },
    ]);

    
    cases
}

pub fn get_test_cases_by_category(category: TestCategory) -> Vec<TestCase> {
    vec![]
}