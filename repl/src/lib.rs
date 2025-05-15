use lexer::Lexer;
use lexer::TokenType;
use std::io::{BufRead, Write};

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) {
    loop {
        // Print prompt
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        // Read a line from input
        let mut line = String::new();
        let bytes_read = input.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            // EOF
            break;
        }

        // Create a lexer for the input line
        let mut lexer = Lexer::new(line.as_bytes());

        // Print tokens until EOF
        loop {
            let tok = lexer.next_token();
            if tok.token_type == TokenType::EOF {
                break;
            }
            writeln!(output, "{:?}", tok).unwrap();
        }
    }
}
