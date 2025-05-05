use lexer::{init_logger, Lexer};
use log::LevelFilter;

fn main() {
    // Initialize the logger with the desired log level
    init_logger(LevelFilter::Debug).expect("Failed to initialize logger");
    
    // Sample input code to be tokenized
    let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
    "#;
    
    // Create a new lexer instance with the input
    let mut lexer = Lexer::new(input.as_bytes());
    
    // Print the first 10 tokens
    for i in 0..10 {
        let token = lexer.next_token();
        println!("Token {}: {:?} - '{}'", i, token.token_type, token.literal);
    }
}