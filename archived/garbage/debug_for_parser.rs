use dryad_parser::Parser;
use dryad_lexer::Lexer;

fn main() {
    let input = r#"
    for i = 0; i < 5; i = i + 1 {
        print(i);
    }
    "#;
    
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        println!("Token: {:?}", token);
        match token {
            dryad_lexer::Token::Eof => break,
            token => tokens.push(token),
        }
    }
    
    println!("\nParsing with {} tokens...", tokens.len());
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => println!("Success! Parsed {} statements", program.statements.len()),
        Err(e) => println!("Error: {:?}", e),
    }
}
