use dryad_lexer::{token::Token, Lexer};

fn tokenize_all(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.next_token().unwrap() {
            tok if tok.token == Token::Eof => break,
            tok => tokens.push(tok.token),
        }
    }

    tokens
}

// Test 1: Verify 'interface' keyword is recognized
#[test]
fn test_tokenize_interface_keyword() {
    let input = "interface";
    let tokens = tokenize_all(input);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("interface".to_string()));
}

// Test 2: Verify 'implements' keyword is recognized
#[test]
fn test_tokenize_implements_keyword() {
    let input = "implements";
    let tokens = tokenize_all(input);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("implements".to_string()));
}

// Test 3: Verify 'get' keyword is recognized
#[test]
fn test_tokenize_get_keyword() {
    let input = "get";
    let tokens = tokenize_all(input);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("get".to_string()));
}

// Test 4: Verify 'set' keyword is recognized
#[test]
fn test_tokenize_set_keyword() {
    let input = "set";
    let tokens = tokenize_all(input);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("set".to_string()));
}

// Test 5: Verify 'namespace' keyword is recognized
#[test]
fn test_tokenize_namespace_keyword() {
    let input = "namespace";
    let tokens = tokenize_all(input);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Keyword("namespace".to_string()));
}

// Test 6: Verify all 5 OOP keywords in a single statement
#[test]
fn test_all_oop_keywords_together() {
    let input = "interface implements get set namespace";
    let tokens = tokenize_all(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Keyword("interface".to_string()));
    assert_eq!(tokens[1], Token::Keyword("implements".to_string()));
    assert_eq!(tokens[2], Token::Keyword("get".to_string()));
    assert_eq!(tokens[3], Token::Keyword("set".to_string()));
    assert_eq!(tokens[4], Token::Keyword("namespace".to_string()));
}

// Test 7: Verify interface declaration is tokenized correctly
#[test]
fn test_interface_declaration_structure() {
    let input = r#"
    interface Animal {
        get name;
        set value;
    }
    "#;

    let tokens = tokenize_all(input);

    // Check that interface, get, and set are recognized as keywords
    assert!(tokens.contains(&Token::Keyword("interface".to_string())));
    assert!(tokens.contains(&Token::Keyword("get".to_string())));
    assert!(tokens.contains(&Token::Keyword("set".to_string())));
    assert!(tokens.contains(&Token::Identifier("Animal".to_string())));
    assert!(tokens.contains(&Token::Identifier("name".to_string())));
    assert!(tokens.contains(&Token::Identifier("value".to_string())));
    assert!(tokens.contains(&Token::Symbol('{')));
    assert!(tokens.contains(&Token::Symbol('}')));
}

// Test 8: Verify implements keyword in class declaration
#[test]
fn test_implements_in_class_declaration() {
    let input = r#"
    class Dog implements Animal {
        function bark() {
            print("woof");
        }
    }
    "#;

    let tokens = tokenize_all(input);

    assert!(tokens.contains(&Token::Keyword("class".to_string())));
    assert!(tokens.contains(&Token::Keyword("implements".to_string())));
    assert!(tokens.contains(&Token::Identifier("Dog".to_string())));
    assert!(tokens.contains(&Token::Identifier("Animal".to_string())));
}

// Test 9: Verify namespace keyword usage
#[test]
fn test_namespace_declaration() {
    let input = r#"
    namespace MyNamespace {
        interface IService {
            get status;
        }
    }
    "#;

    let tokens = tokenize_all(input);

    assert!(tokens.contains(&Token::Keyword("namespace".to_string())));
    assert!(tokens.contains(&Token::Keyword("interface".to_string())));
    assert!(tokens.contains(&Token::Keyword("get".to_string())));
    assert!(tokens.contains(&Token::Identifier("MyNamespace".to_string())));
}

// Test 10: Verify identifiers containing OOP keywords are NOT matched as keywords
#[test]
fn test_identifiers_with_oop_keywords_not_matched_as_keywords() {
    let input = r#"
    let interfaceImpl = "not keyword";
    let implementsHelper = 42;
    let getName = function() {};
    let setter = "helper";
    let namespaceURI = "http";
    "#;

    let tokens = tokenize_all(input);

    // These should be identifiers, not keywords
    assert!(tokens.contains(&Token::Identifier("interfaceImpl".to_string())));
    assert!(tokens.contains(&Token::Identifier("implementsHelper".to_string())));
    assert!(tokens.contains(&Token::Identifier("getName".to_string())));
    assert!(tokens.contains(&Token::Identifier("setter".to_string())));
    assert!(tokens.contains(&Token::Identifier("namespaceURI".to_string())));

    // Verify exact count of OOP keywords (should be 0 or very few if any legitimate usage)
    let oop_keywords = tokens.iter()
        .filter(|token| matches!(*token, 
            Token::Keyword(s) if s == "interface" || s == "implements" || s == "get" || s == "set" || s == "namespace"
        ))
        .count();

    // None of these identifiers should produce OOP keywords
    assert_eq!(oop_keywords, 0);
}

// Test 11: Verify OOP keywords within strings are not tokenized as keywords
#[test]
fn test_oop_keywords_in_strings() {
    let input = r#"
    let text = "interface implements get set namespace";
    interface IService {
        get value;
    }
    "#;

    let tokens = tokenize_all(input);

    // Verify the string is captured as a single token
    assert!(tokens.contains(&Token::String(
        "interface implements get set namespace".to_string()
    )));

    // Count keyword occurrences - should be 3 (interface, get, set outside the string)
    let interface_keywords = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "interface"))
        .count();
    assert_eq!(interface_keywords, 1);

    let get_keywords = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "get"))
        .count();
    assert_eq!(get_keywords, 1);
}

// Test 12: Verify complex OOP structure with multiple keywords
#[test]
fn test_complex_oop_structure() {
    let input = r#"
    namespace MyApp {
        interface IRepository {
            get items;
            set items;
        }
        
        class UserRepository implements IRepository {
            get items() {
                return this.data;
            }
            
            set items(value) {
                this.data = value;
            }
        }
    }
    "#;

    let tokens = tokenize_all(input);

    // Verify all keywords appear the expected number of times
    let namespace_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "namespace"))
        .count();
    assert_eq!(namespace_count, 1);

    let interface_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "interface"))
        .count();
    assert_eq!(interface_count, 1);

    let implements_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "implements"))
        .count();
    assert_eq!(implements_count, 1);

    let get_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "get"))
        .count();
    assert_eq!(get_count, 2);

    let set_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "set"))
        .count();
    assert_eq!(set_count, 2);
}

// Test 13: Verify get and set with method bodies
#[test]
fn test_get_set_with_method_bodies() {
    let input = r#"
    class Person {
        get fullName() {
            return this.first + " " + this.last;
        }
        
        set fullName(name) {
            let parts = name.split(" ");
            this.first = parts[0];
            this.last = parts[1];
        }
    }
    "#;

    let tokens = tokenize_all(input);

    // Verify get and set are keywords, not identifiers
    let get_keywords = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "get"))
        .count();
    assert_eq!(get_keywords, 1);

    let set_keywords = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "set"))
        .count();
    assert_eq!(set_keywords, 1);

    // Verify other identifiers are correctly recognized
    assert!(tokens.contains(&Token::Identifier("fullName".to_string())));
    assert!(tokens.contains(&Token::Identifier("first".to_string())));
    assert!(tokens.contains(&Token::Identifier("last".to_string())));
}

// Test 14: Verify partial word matches don't trigger keywords
#[test]
fn test_partial_word_matching() {
    let input = r#"
    let interfaceName = "test";
    let implementsCheck = true;
    let getter = 10;
    let setSomething = 20;
    let namespacePath = "/path";
    
    interface ITest {
        get status;
    }
    "#;

    let tokens = tokenize_all(input);

    // Verify only legitimate keywords are captured
    let interface_keywords = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "interface"))
        .count();
    assert_eq!(interface_keywords, 1);

    // Verify partial matches are identifiers
    assert!(tokens.contains(&Token::Identifier("interfaceName".to_string())));
    assert!(tokens.contains(&Token::Identifier("implementsCheck".to_string())));
    assert!(tokens.contains(&Token::Identifier("getter".to_string())));
    assert!(tokens.contains(&Token::Identifier("setSomething".to_string())));
    assert!(tokens.contains(&Token::Identifier("namespacePath".to_string())));
}

// Test 15: Verify multiple interfaces and implementations
#[test]
fn test_multiple_interfaces_and_implementations() {
    let input = r#"
    interface IReader {
        get content;
    }
    
    interface IWriter {
        set content;
    }
    
    class Document implements IReader implements IWriter {
        get content() {
            return this.data;
        }
        
        set content(value) {
            this.data = value;
        }
    }
    "#;

    let tokens = tokenize_all(input);

    let interface_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "interface"))
        .count();
    assert_eq!(interface_count, 2);

    let implements_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "implements"))
        .count();
    assert_eq!(implements_count, 2);

    let get_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "get"))
        .count();
    assert_eq!(get_count, 2);

    let set_count = tokens
        .iter()
        .filter(|token| matches!(*token, Token::Keyword(s) if s == "set"))
        .count();
    assert_eq!(set_count, 2);
}
