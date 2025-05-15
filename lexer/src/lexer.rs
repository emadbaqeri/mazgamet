use crate::token::{Token, TokenType};
use log::{debug, trace};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        debug!("Created lexer: {:?}", lexer);
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            trace!("read_char: EOF at position {}", self.read_position);
            self.ch = None; // EOF
        } else {
            // Direct byte access - much more efficient than chars() iterator
            self.ch = Some(self.input[self.read_position]);
            trace!(
                "read_char: '{}' at position {}",
                self.ch.unwrap() as char,
                self.read_position
            );
        }
        self.position = self.read_position;
        self.read_position += 1; // Always advance by 1 byte
        trace!(
            "read_char: new position={}, read_position={}",
            self.position, self.read_position
        );
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        debug!(
            "Current character: {:?}, position: {}, read_position: {}",
            self.ch.map(|c| c as char),
            self.position,
            self.read_position
        );
        let token = match self.ch {
            Some(b'=') => {
                debug!("Found ASSIGN or EQ token");
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    self.read_char();
                    Token::new(TokenType::EQ, "==".to_string())
                } else {
                    self.read_char();
                    Token::new_char(TokenType::Assign, '=')
                }
            }
            Some(b'!') => {
                debug!("Found BANG or NotEQ token");
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    self.read_char();
                    Token::new(TokenType::NotEQ, "!=".to_string())
                } else {
                    self.read_char();
                    Token::new_char(TokenType::Bang, '!')
                }
            }
            Some(b';') => {
                debug!("Found SEMICOLON token");
                self.read_char();
                Token::new_char(TokenType::Semicolon, ';')
            }
            Some(b'(') => {
                debug!("Found LEFT_PAREN token");
                self.read_char();
                Token::new_char(TokenType::LeftParen, '(')
            }
            Some(b')') => {
                debug!("Found RIGHT_PAREN token");
                self.read_char();
                Token::new_char(TokenType::RightParen, ')')
            }
            Some(b',') => {
                debug!("Found COMMA token");
                self.read_char();
                Token::new_char(TokenType::Comma, ',')
            }
            Some(b'+') => {
                debug!("Found PLUS token");
                self.read_char();
                Token::new_char(TokenType::Plus, '+')
            }
            Some(b'-') => {
                debug!("Found MINUS token");
                self.read_char();
                Token::new_char(TokenType::Minus, '-')
            }
            Some(b'/') => {
                debug!("Found SLASH token");
                self.read_char();
                Token::new_char(TokenType::Slash, '/')
            }
            Some(b'*') => {
                debug!("Found ASTERISK token");
                self.read_char();
                Token::new_char(TokenType::Asterisk, '*')
            }
            Some(b'<') => {
                debug!("Found LT token");
                self.read_char();
                Token::new_char(TokenType::LT, '<')
            }
            Some(b'>') => {
                debug!("Found GT token");
                self.read_char();
                Token::new_char(TokenType::GT, '>')
            }
            Some(b'{') => {
                debug!("Found LEFT_BRACE token");
                self.read_char();
                Token::new_char(TokenType::LeftBrace, '{')
            }
            Some(b'}') => {
                debug!("Found RIGHT_BRACE token");
                self.read_char();
                Token::new_char(TokenType::RightBrace, '}')
            }
            Some(ch) => {
                if Self::is_letter(ch) {
                    debug!("Found letter character: {}", ch as char);
                    let literal = self.read_identifier();
                    let token_type = TokenType::lookup_ident(&literal);
                    debug!("Read identifier: {}, token type: {:?}", literal, token_type);
                    Token::new(token_type, literal)
                } else if Self::is_digit(ch) {
                    debug!("Found digit character: {}", ch as char);
                    let literal = self.read_number();
                    debug!("Read number: {}", literal);
                    Token::new(TokenType::Integer, literal)
                } else {
                    debug!("Found ILLEGAL character: {}", ch as char);
                    self.read_char();
                    // Convert byte to char for the token
                    let ch_as_char = ch as char;
                    Token::new_char(TokenType::Illegal, ch_as_char)
                }
            }
            None => {
                debug!("Found EOF token");
                Token::new(TokenType::EOF, "".to_string())
            }
        };
        debug!(
            "Returning token: {:?} with literal: {}",
            token.token_type, token.literal
        );
        token
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        trace!("Starting read_identifier at position: {}", position);
        while let Some(ch) = self.ch {
            if Self::is_letter(ch) {
                trace!("Reading letter: {}", ch as char);
                self.read_char();
            } else {
                trace!("Stopping at non-letter: {:?}", self.ch.map(|c| c as char));
                break;
            }
        }
        let result = String::from_utf8(self.input[position..self.position].to_vec())
            .unwrap_or_else(|_| String::new());
        trace!(
            "Identifier read: {}, new position: {}",
            result, self.position
        );
        result
    }

    pub fn is_letter(ch: u8) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == b'_'
    }

    fn is_digit(ch: u8) -> bool {
        ch.is_ascii_digit()
    }

    pub fn skip_whitespace(&mut self) {
        trace!("Starting to skip whitespace");
        let initial_pos = self.position;
        while let Some(ch) = self.ch {
            if ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r' {
                trace!("Skipping whitespace character: '{}'", ch as char);
                self.read_char();
            } else {
                break;
            }
        }
        if initial_pos != self.position {
            trace!(
                "Skipped whitespace from position {} to {}",
                initial_pos, self.position
            );
        } else {
            trace!("No whitespace to skip");
        }
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        trace!("Starting read_number at position: {}", position);
        while let Some(ch) = self.ch {
            if Self::is_digit(ch) {
                trace!("Reading digit: {}", ch as char);
                self.read_char();
            } else {
                trace!("Stopping at non-digit: {:?}", self.ch.map(|c| c as char));
                break;
            }
        }

        let result = String::from_utf8(self.input[position..self.position].to_vec())
            .unwrap_or_else(|_| String::new());
        trace!("Number read: {}, new position: {}", result, self.position);
        result
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            // Try to decode the next char from the byte slice
            let slice = &self.input[self.read_position..];
            match std::str::from_utf8(slice) {
                Ok(s) => s.chars().next(),
                Err(_) => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logger::init_test_logger;

    #[test]
    fn test_next_token() {
        // Initialize the logger for testing
        let _ = init_test_logger();

        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;
"#;

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Identifier, "five"),
            (TokenType::Assign, "="),
            (TokenType::Integer, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Integer, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LeftParen, "("),
            (TokenType::Identifier, "x"),
            (TokenType::Comma, ","),
            (TokenType::Identifier, "y"),
            (TokenType::RightParen, ")"),
            (TokenType::LeftBrace, "{"),
            (TokenType::Identifier, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Identifier, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RightBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Identifier, "result"),
            (TokenType::Assign, "="),
            (TokenType::Identifier, "add"),
            (TokenType::LeftParen, "("),
            (TokenType::Identifier, "five"),
            (TokenType::Comma, ","),
            (TokenType::Identifier, "ten"),
            (TokenType::RightParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Integer, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Integer, "5"),
            (TokenType::LT, "<"),
            (TokenType::Integer, "10"),
            (TokenType::GT, ">"),
            (TokenType::Integer, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::IF, "if"),
            (TokenType::LeftParen, "("),
            (TokenType::Integer, "5"),
            (TokenType::LT, "<"),
            (TokenType::Integer, "10"),
            (TokenType::RightParen, ")"),
            (TokenType::LeftBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RightBrace, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LeftBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RightBrace, "}"),
            (TokenType::Integer, "10"),
            (TokenType::EQ, "=="),
            (TokenType::Integer, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Integer, "10"),
            (TokenType::NotEQ, "!="),
            (TokenType::Integer, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        // Convert the string to bytes for the byte-based lexer
        let mut lexer = Lexer::new(input.as_bytes());

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                &tok.token_type, expected_type,
                "tests[{}] - token type wrong. expected={:?}, got={:?}",
                i, expected_type, tok.token_type
            );

            assert_eq!(
                &tok.literal, expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, tok.literal
            );
        }
    }
}
