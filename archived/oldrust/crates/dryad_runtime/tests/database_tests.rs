// crates/dryad_runtime/tests/database_tests.rs

use dryad_lexer::Lexer;
use dryad_parser::Parser;
use dryad_runtime::interpreter::Interpreter;
use dryad_runtime::interpreter::Value;
use dryad_runtime::heap::ManagedObject;

fn execute_dryad_code(interpreter: &mut Interpreter, code: &str) -> Result<Value, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    interpreter.execute_and_return_value(&program)
}

#[test]
fn test_sqlite_basic_flow() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("database").unwrap();
    
    let code = r#"
        #<database>
        let db = sqlite_open(":memory:");
        
        // Create table
        sqlite_execute(db.id, "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)");
        
        // Insert data
        sqlite_execute(db.id, "INSERT INTO users (name) VALUES ('Alice')");
        sqlite_execute(db.id, "INSERT INTO users (name) VALUES ('Bob')");
        
        // Query data
        let rows = sqlite_query(db.id, "SELECT id, name FROM users ORDER BY id");
        
        sqlite_close(db.id);
        rows
    "#;
    
    let result = execute_dryad_code(&mut interpreter, code).expect("Execução falhou");
    
    match result {
        Value::Array(id) => {
            let arr_obj = interpreter.heap.get(id).unwrap();
            if let ManagedObject::Array(rows) = arr_obj {
                assert_eq!(rows.len(), 2);
                
                // Check first row (Alice)
                if let Value::Object(row1_id) = rows[0] {
                    let row1 = interpreter.heap.get(row1_id).unwrap();
                    if let ManagedObject::Object { properties, .. } = row1 {
                        assert_eq!(properties.get("name").unwrap(), &Value::String("Alice".to_string()));
                        assert_eq!(properties.get("id").unwrap(), &Value::Number(1.0));
                    }
                } else { panic!("Esperado objeto na linha 1"); }

                // Check second row (Bob)
                if let Value::Object(row2_id) = rows[1] {
                    let row2 = interpreter.heap.get(row2_id).unwrap();
                    if let ManagedObject::Object { properties, .. } = row2 {
                        assert_eq!(properties.get("name").unwrap(), &Value::String("Bob".to_string()));
                        assert_eq!(properties.get("id").unwrap(), &Value::Number(2.0));
                    }
                } else { panic!("Esperado objeto na linha 2"); }
            }
        }
        _ => panic!("Esperado Array, recebido: {:?}", result),
    }
}

#[test]
fn test_sqlite_execute_results() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("database").unwrap();
    
    let code = r#"
        #<database>
        let db = sqlite_open(":memory:");
        sqlite_execute(db.id, "CREATE TABLE logs (id INTEGER PRIMARY KEY AUTOINCREMENT, msg TEXT)");
        let res = sqlite_execute(db.id, "INSERT INTO logs (msg) VALUES ('Hello')");
        sqlite_close(db.id);
        res
    "#;
    
    let result = execute_dryad_code(&mut interpreter, code).expect("Execução falhou");
    
    match result {
        Value::Object(id) => {
            let obj = interpreter.heap.get(id).unwrap();
            if let ManagedObject::Object { properties, .. } = obj {
                assert_eq!(properties.get("rows_affected").unwrap(), &Value::Number(1.0));
                assert_eq!(properties.get("last_insert_id").unwrap(), &Value::Number(1.0));
            }
        }
        _ => panic!("Esperado Object, recebido: {:?}", result),
    }
}

#[test]
fn test_sqlite_error_handling() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("database").unwrap();
    
    let code = r#"
        #<database>
        let db = sqlite_open(":memory:");
        // SQL Inválido
        sqlite_query(db.id, "SELECT * FROM non_existent_table")
    "#;
    
    let result = execute_dryad_code(&mut interpreter, code);
    assert!(result.is_err(), "Deveria falhar ao consultar tabela inexistente");
}

#[test]
fn test_sqlite_file_based() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("database").unwrap();
    
    let db_path = "test_persistent.db";
    
    // Ensure clean state
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).unwrap();
    }
    
    // 1. Create and Insert
    let code1 = format!(r#"
        #<database>
        let db = sqlite_open("{}");
        sqlite_execute(db.id, "CREATE TABLE notes (id INTEGER PRIMARY KEY, content TEXT)");
        sqlite_execute(db.id, "INSERT INTO notes (content) VALUES ('Persistence test')");
        sqlite_close(db.id);
    "#, db_path);
    
    execute_dryad_code(&mut interpreter, &code1).expect("Primeira execução falhou");
    
    // 2. Re-open and Verify
    let code2 = format!(r#"
        #<database>
        let db = sqlite_open("{}");
        let rows = sqlite_query(db.id, "SELECT content FROM notes");
        sqlite_close(db.id);
        rows
    "#, db_path);
    
    let result = execute_dryad_code(&mut interpreter, &code2).expect("Segunda execução falhou");
    
    // Verify results
    match result {
        Value::Array(id) => {
            let arr_obj = interpreter.heap.get(id).unwrap();
            if let ManagedObject::Array(rows) = arr_obj {
                assert_eq!(rows.len(), 1);
                if let Value::Object(row_id) = rows[0] {
                    let row = interpreter.heap.get(row_id).unwrap();
                    if let ManagedObject::Object { properties, .. } = row {
                        assert_eq!(properties.get("content").unwrap(), &Value::String("Persistence test".to_string()));
                    }
                } else { panic!("Esperado objeto na linha"); }
            }
        }
        _ => panic!("Esperado Array, recebido: {:?}", result),
    }
    
    // Cleanup
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).unwrap();
    }
}

