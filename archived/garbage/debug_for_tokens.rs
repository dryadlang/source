use dryad_lexer::Lexer;

fn main() {
    let input = r#"
    for i = 0; i < 5; i = i + 1 {
        print(i);
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    loop {
        let token = lexer.next_token().unwrap();
        println!("{:?}", token);
        if matches!(token, dryad_lexer::Token::Eof) {
            break;
        }
    }
}
