use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let next = lexer.next_token();

        Self {
            lexer,
            current,
            next,
        }
    }

    fn advance(&mut self) {
        self.current = std::mem::replace(&mut self.next, self.lexer.next_token());
    }

    fn peek(&self) -> &Token {
        &self.next
    }

    fn expect(&mut self, expected: Token) {
        if self.current != expected {
            panic!("Expected {:?}, got {:?}", expected, self.current);
        }

        self.advance();
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.current != Token::EOF {
            statements.push(self.parse_statement());
        }

        statements
    }

    fn parse_statement(&mut self) -> Statement {
        if let Token::Identifier(name) = &self.current {
            let name = name.clone();

            if *self.peek() == Token::Equals {
                self.advance(); // identifier
                self.advance(); // '='

                let value = self.parse_expression();
                self.expect(Token::Semicolon);

                return Statement::Assignment { name, value };
            }
        }

        // Otherwise, it's an expression statement.
        let expr = self.parse_expression();
        self.expect(Token::Semicolon);

        Statement::Expression(expr)
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_primary();

        loop {
            match self.current {
                Token::Plus => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Sub,
                        right: Box::new(right),
                    };
                }
                Token::Asterisk => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Mul,
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Div,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
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

                if self.current == Token::LeftParen {
                    self.advance(); // consume '('

                    let mut args = Vec::new();

                    if self.current != Token::RightParen {
                        loop {
                            args.push(self.parse_expression());

                            if self.current == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    if self.current != Token::RightParen {
                        panic!("Expected ')'");
                    }

                    self.advance(); // consume ')'

                    Expr::Call { name, args }
                } else {
                    Expr::Variable(name)
                }
            }

            _ => panic!("Unexpected token {:?}", self.current),
        }
    }
}
