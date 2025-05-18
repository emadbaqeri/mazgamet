use ast::{Identifier, LetStatement, Program, ReturnStatement, Statement};
use lexer::{Lexer, Token, TokenType};

type Errors = Vec<String>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Errors,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::new(TokenType::EOF, String::new()),
            peek_token: Token::new(TokenType::EOF, String::new()),
            errors: Vec::new(),
        };

        // Read two tokens to initialize cur_token and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = std::mem::replace(
            &mut self.peek_token,
            Token::new(TokenType::EOF, String::new()),
        );
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };
        while !self.current_token_is(TokenType::EOF) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        program
    }

    pub fn errors(&self) -> &Errors {
        &self.errors
    }

    fn peek_error(&mut self, expected_token: &TokenType) {
        let message = format!(
            "expected next token to be {}, got {} instead",
            expected_token, self.peek_token.token_type
        );
        self.errors.push(message);
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();
        self.next_token();
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Box::new(ReturnStatement {
            token,
            return_value: None,
        }))
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();

        let statement = LetStatement {
            token,
            name: None,
            value: None,
        };

        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let ident = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        let mut statement = Box::new(statement);
        statement.name = Some(Box::new(ident));

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(statement)
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.token_type == *token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(&token_type);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Node;

    #[test]
    fn test_let_statements() {
        let input = r#"
            true -
            let error_prune_statement :=
            let x = 5;
            let y = 10;
            let foobar = 121212;
            "#;
        let lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert!(
            program.statements.len() == 3,
            "Expected 3 statements, got {}",
            program.statements.len()
        );

        let tests = ["x", "y", "foobar"];

        for (i, &expected_ident) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            test_let_statement(expected_ident, statement.as_ref());
        }
    }

    fn test_let_statement(expected_ident: &str, statement: &dyn Statement) {
        assert_eq!(
            statement.token_literal(),
            "let",
            "Statement token literal should be 'let'"
        );

        let let_stmt = match statement.as_any().downcast_ref::<LetStatement>() {
            Some(s) => s,
            #[allow(non_snake_case)]
            None => panic!("Statement is not a LetStatement"),
        };

        match let_stmt.name {
            Some(ref ident) => {
                assert_eq!(
                    ident.value, expected_ident,
                    "Identifier value should be '{}', got '{}'",
                    expected_ident, ident.value
                );

                assert_eq!(
                    ident.token_literal(),
                    expected_ident,
                    "Identifier token literal should be '{}', got '{}'",
                    expected_ident,
                    ident.token_literal()
                );
            }
            #[allow(non_snake_case)]
            None => panic!("Let statement's name is None"),
        }

        if let Some(ref ident) = let_stmt.name {
            assert_eq!(ident.value, expected_ident);
            assert_eq!(ident.token_literal(), expected_ident);
        } else {
            panic!("Let statement's name is None");
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
            return 5;
            return 10;
            return 888888;
            "#;

        let lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            3,
            "Expected 3 statements, got {}",
            program.statements.len()
        );

        for statement in &program.statements {
            let return_statement = match statement.as_any().downcast_ref::<ReturnStatement>() {
                Some(statement) => statement,
                #[allow(non_snake_case)]
                None => panic!("Statement is not a ReturnStatement"),
            };

            assert_eq!(
                return_statement.token_literal(),
                "return",
                "Return statement token literal should be 'return'"
            );
        }
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        assert!(
            errors.is_empty(),
            "Parser had {} errors: {:?}",
            errors.len(),
            errors
        );
    }
}
