use std::collections::HashMap;

use ast::{
    Expression, ExpressionStatement, Identifier, LetStatement, Program, ReturnStatement, Statement,
}; // Added ExpressionStatement
use lexer::{Lexer, Token, TokenType};

type Errors = Vec<String>;

// Define Precedence levels for Pratt parsing
#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

// Adjusted PrefixParseFn and InfixParseFn to match your existing definition style
pub type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
pub type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Errors,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::new(TokenType::EOF, String::new()),
            peek_token: Token::new(TokenType::EOF, String::new()),
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Register prefix parsing functions
        parser.register_prefix(
            TokenType::Identifier,
            Self::parse_identifier_as_expression_prefix,
        );

        // Read two tokens to initialize cur_token and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    // Prefix parsing function for identifiers
    fn parse_identifier_as_expression_prefix(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: parser.current_token.clone(),
            value: parser.current_token.literal.clone(),
        }))
    }

    pub fn next_token(&mut self) {
        self.current_token = std::mem::replace(
            &mut self.peek_token,
            Token::new(TokenType::EOF, String::new()), // Default token
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
            // self.next_token(); // parse_statement should consume its tokens including the last one (e.g. semicolon or end of expression)
            // If parse_statement doesn't advance to the next statement's beginning, this is needed.
            // Let's assume parse_statement advances correctly.
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
            _ => self.parse_expression_statement(), // Default to parsing an expression statement
        }
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone();
        self.next_token(); // Consume 'return'

        // TODO: Parse the expression for the return value
        // For now, skip to semicolon
        while !self.current_token_is(TokenType::Semicolon) && !self.current_token_is(TokenType::EOF)
        {
            self.next_token();
        }
        if self.current_token_is(TokenType::Semicolon) {
            self.next_token(); // Consume ';'
        }

        Some(Box::new(ReturnStatement {
            token,
            return_value: None, // Placeholder
        }))
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.current_token.clone(); // The 'let' token

        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let name = Identifier {
            // This is the identifier AST node
            token: self.current_token.clone(), // The identifier token
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        self.next_token(); // Consume '='

        // TODO: Parse the expression for the value
        // For now, skip to semicolon
        while !self.current_token_is(TokenType::Semicolon) && !self.current_token_is(TokenType::EOF)
        {
            self.next_token();
        }
        if self.current_token_is(TokenType::Semicolon) {
            self.next_token(); // Consume ';'
        }

        Some(Box::new(LetStatement {
            token, // 'let' token
            name: Some(Box::new(name)),
            value: None, // Placeholder
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt_token = self.current_token.clone(); // Token that starts the expression (e.g. "foobar")

        let expression = self.parse_expression(Precedence::Lowest);

        // After parse_expression, self.current_token is the token AFTER the expression.
        // For "foobar;", current_token should now be TokenType::Semicolon.

        if let Some(expr) = expression {
            // expression is Some(Identifier("foobar"))
            let stmt = ExpressionStatement {
                token: stmt_token,
                expression: Some(expr),
            };

            // If the expression statement is followed by a semicolon, consume it.
            // This is common for expression statements in languages like JavaScript or C.
            if self.current_token_is(TokenType::Semicolon) {
                self.next_token(); // Consume the semicolon
            }
            Some(Box::new(stmt))
        } else {
            // Error already pushed by parse_expression or one of its callees
            None
        }
    }

    // Core of the Pratt parser for expressions
    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Box<dyn Expression>> {
        // '.cloned()' is used because `prefix_fn` is `fn(&mut Parser)`,
        // which means it might try to borrow `self` mutably again if not careful.
        // Cloning the function pointer itself is cheap.
        let prefix_fn_option = self
            .prefix_parse_fns
            .get(&self.current_token.token_type)
            .cloned();

        let expression_node = match prefix_fn_option {
            Some(p_fn) => {
                // The prefix function (e.g., parse_identifier_as_expression_prefix)
                // creates an AST node based on the current_token.
                // It DOES NOT advance the token itself.
                p_fn(self)
            }
            None => {
                let msg = format!(
                    "no prefix parse function for token type {} found. Token: {:?}",
                    self.current_token.token_type, self.current_token
                );
                self.errors.push(msg);
                return None;
            }
        }?; // If p_fn returns None (error during its execution), propagate it.

        // After the prefix function has successfully created the initial part of the expression (the "left" node),
        // parse_expression advances the token. This consumes the token(s) used by the prefix function.
        self.next_token();

        // TODO: Implement infix operator parsing loop here
        // while !_precedence_is_semicolon && precedence < self.peek_precedence() {
        //    ... look up infix_parse_fn ...
        //    ... self.next_token() to consume operator ...
        //    ... expression_node = infix_fn(self, expression_node) ...
        // }

        Some(expression_node)
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

    pub fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    #[allow(dead_code)] // Kept for future use
    pub fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Node; // For token_literal
                   // use lexer::Lexer; // Already imported at the top of parser/src/lib.rs
                   // use ast::{Statement, LetStatement, Identifier, ReturnStatement, ExpressionStatement, Expression}; // Partially imported at top

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.is_empty() {
            return;
        }
        // eprintln!("Parser has {} errors", errors.len()); // allow-println
        for _msg in errors {
            // eprintln!("Parser error: {}", msg); // allow-println
        }
        panic!("Parser errors encountered");
    }

    #[test]
    fn test_let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 121212;
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
            None => panic!(
                "Statement is not a LetStatement. Got: {:?}",
                statement.as_any()
            ),
        };

        match let_stmt.name {
            Some(ref ident_box) => {
                // ident_box is Box<Identifier>
                let ident = ident_box.as_ref(); // Get &Identifier from Box<Identifier>
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
                Some(s) => s,
                #[allow(non_snake_case)]
                None => panic!(
                    "Statement is not a ReturnStatement. Got: {:?}",
                    statement.as_any()
                ),
            };

            assert_eq!(
                return_statement.token_literal(),
                "return",
                "Return statement token literal should be 'return'"
            );
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statement. got={}",
            program.statements.len()
        );

        let stmt_trait_object = &program.statements.first().unwrap(); // This is &Box<dyn Statement>
        let stmt_as_any = stmt_trait_object.as_any(); // This is &dyn Any

        match stmt_as_any.downcast_ref::<ExpressionStatement>() {
            Some(expr_stmt) => {
                assert!(
                    expr_stmt.expression.is_some(),
                    "ExpressionStatement's expression is None"
                );

                // Get the inner Box<dyn Expression> and then downcast it
                let dyn_expr = expr_stmt.expression.as_ref().unwrap(); // This is &Box<dyn Expression>
                match dyn_expr.as_any().downcast_ref::<Identifier>() {
                    Some(ident) => {
                        assert_eq!(
                            ident.value, "foobar",
                            "ident.value not {}. got={}",
                            "foobar", ident.value
                        );
                        assert_eq!(
                            ident.token_literal(),
                            "foobar",
                            "ident.token_literal not {}. got={}",
                            "foobar",
                            ident.token_literal()
                        );
                    }
                    None => panic!(
                        "stmt.expression is not ast::Identifier. got={:?}",
                        expr_stmt.token_literal()
                    ),
                }
            }
            None => panic!(
                "program.statements[0] is not ast::ExpressionStatement. got={:?}",
                stmt_trait_object.token_literal()
            ),
        }
    }
}
