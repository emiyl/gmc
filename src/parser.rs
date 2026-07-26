use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();

        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.current != Token::EOF {
            statements.push(self.parse_statement());
        }

        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match &self.current {
            Token::Identifier(name) => {
                let name = name.clone();

                self.advance();

                match self.current {
                    Token::Equals => {
                        self.advance();

                        let value = self.parse_expression();

                        if self.current != Token::Semicolon {
                            panic!("Expected ;");
                        }

                        self.advance();

                        Statement::Assignment { name, value }
                    }

                    _ => panic!("Expected ="),
                }
            }

            _ => panic!("Expected statement"),
        }
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while self.current == Token::Plus {
            self.advance();

            let right = self.parse_primary();

            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOp::Add,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary(&mut self) -> Expr {
        match &self.current {
            Token::Number(value) => {
                let value = *value;

                self.advance();

                Expr::Integer(value)
            }

            Token::Identifier(name) => {
                let name = name.clone();

                self.advance();

                Expr::Variable(name)
            }

            _ => panic!("Unexpected token {:?}", self.current),
        }
    }
}
