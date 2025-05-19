use lexer::Token;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String;
    fn as_string(&self) -> String;
}

pub trait Statement: Node + Any {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(first) = self.statements.first() {
            first.token_literal()
        } else {
            String::new()
        }
    }

    fn as_string(&self) -> String {
        self.statements
            .iter()
            .map(|statement| statement.as_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_string(&self) -> String {
        self.value.clone()
    }
}

#[allow(dead_code)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Box<Identifier>>,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_string(&self) -> String {
        let mut output = String::new();

        if let Some(name) = &self.name {
            output.push_str("let ");
            output.push_str(&name.as_string());
            output.push_str(" = ");

            if let Some(value) = &self.value {
                output.push_str(&value.as_string());
            }
        } else {
            output.push_str("let _ = _");
        }

        output.push(';');
        output
    }
}

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_string(&self) -> String {
        "return ...;".to_string()
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ExpressionStatement {
    token: Token,
    expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_string(&self) -> String {
        if let Some(expression) = &self.expression {
            expression.as_string()
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::TokenType;

    #[test]
    fn test_string() {
        // Create an identifier for the variable name
        let name = Identifier {
            token: Token {
                token_type: TokenType::Identifier,
                literal: "myVar".to_string(),
            },
            value: "myVar".to_string(),
        };

        // Create an identifier for the value
        let value = Identifier {
            token: Token {
                token_type: TokenType::Identifier,
                literal: "anotherVar".to_string(),
            },
            value: "anotherVar".to_string(),
        };

        // Create a let statement
        let let_stmt = LetStatement {
            token: Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            name: Some(Box::new(name)),
            value: Some(Box::new(value)),
        };

        // Create a program with the let statement
        let program = Program {
            statements: vec![Box::new(let_stmt)],
        };

        // Test the string representation
        assert_eq!(program.as_string(), "let myVar = anotherVar;");
    }
}
