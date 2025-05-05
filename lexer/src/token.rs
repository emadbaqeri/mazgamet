#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Illegal,
    EOF,

    Integer,
    Indetifier,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,
    Let,
}

impl TokenType {
    pub fn lookup_ident(ident: &str) -> Self {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Indetifier,
        }
    }
}

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
