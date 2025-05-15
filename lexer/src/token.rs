#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Illegal,
    EOF,

    Integer,
    Identifier,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,

    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,
    Let,
    True,
    False,
    IF,
    ELSE,
    Return,

    EQ,
    NotEQ,
}

impl TokenType {
    pub fn lookup_ident(ident: &str) -> Self {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::Return,
            "==" => TokenType::EQ,
            "!=" => TokenType::NotEQ,
            _ => TokenType::Identifier,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }

    pub fn new_char(token_type: TokenType, ch: char) -> Self {
        Self {
            token_type,
            literal: ch.to_string(),
        }
    }
}
