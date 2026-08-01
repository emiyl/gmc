use core::panic;

use super::ast::*;
use super::lexer::Token::Identifier;
use super::lexer::{Lexer, Token};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Vec<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let next = vec![lexer.next_token(), lexer.next_token()];

        Self {
            lexer,
            current,
            next,
        }
    }

    fn advance(&mut self) {
        self.current = self.next.remove(0);
        self.next.push(self.lexer.next_token());
    }

    fn peek(&self, n: usize) -> &Token {
        if n == 0 {
            &self.current
        } else if n > 0 && n <= self.next.len() {
            &self.next[n - 1]
        } else {
            panic!("Peek out of bounds: {}", n);
        }
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
            let block = self.parse_block();
            statements.extend(block);
        }

        statements
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        if self.current != Token::LeftBrace {
            return vec![self.parse_statement()];
        }

        self.expect(Token::LeftBrace);
        let mut statements = Vec::new();

        while self.current != Token::RightBrace {
            statements.push(self.parse_statement());
        }

        self.expect(Token::RightBrace);
        statements
    }

    fn parse_if_statement(&mut self) -> Statement {
        self.advance(); // consume 'if'
        self.expect(Token::LeftParen);
        let condition = self.parse_expression();
        self.expect(Token::RightParen);

        let then_branch = Box::new(self.parse_block());

        let else_branch = if self.current == Token::Identifier("else".to_string()) {
            self.advance(); // consume 'else'
            Some(Box::new(self.parse_block()))
        } else {
            None
        };

        Statement::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_assignment(&mut self, name: String) -> Statement {
        self.advance(); // consume identifier
        self.expect(Token::Equals);
        let value = self.parse_expression();
        self.expect(Token::Semicolon);

        Statement::Assignment { name, value }
    }

    fn parse_compound_assignment(&mut self, name: String, op: BinaryOp) -> Statement {
        self.advance(); // consume identifier
        self.advance(); // consume compound assignment token

        let right = self.parse_expression();
        self.expect(Token::Semicolon);

        let value = Expr::Binary {
            left: Box::new(Expr::Variable(name.clone())),
            operator: op,
            right: Box::new(right),
        };

        Statement::Assignment { name, value }
    }

    fn parse_increment_statement(&mut self, name: String, operator: BinaryOp) -> Statement {
        self.advance(); // consume identifier
        self.advance(); // consume ++ or --
        self.expect(Token::Semicolon);

        let value = Expr::Binary {
            left: Box::new(Expr::Variable(name.clone())),
            operator,
            right: Box::new(Expr::Integer(1)),
        };

        Statement::Assignment { name, value }
    }

    fn parse_prefix_increment_statement(&mut self, operator: BinaryOp) -> Statement {
        self.advance(); // consume ++ or --

        let Token::Identifier(name) = &self.current else {
            panic!("Expected identifier after prefix increment/decrement");
        };
        let name = name.clone();

        self.advance(); // consume identifier
        self.expect(Token::Semicolon);

        let value = Expr::Binary {
            left: Box::new(Expr::Variable(name.clone())),
            operator,
            right: Box::new(Expr::Integer(1)),
        };

        Statement::Assignment { name, value }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current {
            Token::PlusPlus => return self.parse_prefix_increment_statement(BinaryOp::Add),
            Token::MinusMinus => return self.parse_prefix_increment_statement(BinaryOp::Sub),
            _ => {}
        }

        if let Token::Identifier(name) = &self.current {
            let name = name.clone();

            match name.as_str() {
                "if" => return self.parse_if_statement(),
                _ => match self.peek(1) {
                    Token::Equals => return self.parse_assignment(name),
                    Token::PlusEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Add);
                    }
                    Token::MinusEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Sub);
                    }
                    Token::AsteriskEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Mul);
                    }
                    Token::SlashEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Div);
                    }
                    Token::PercentEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Rem);
                    }
                    Token::AmpersandEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::And);
                    }
                    Token::VerticalBarEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Or);
                    }
                    Token::CaretEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Xor);
                    }
                    Token::ShiftLeftEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Shl);
                    }
                    Token::ShiftRightEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Shr);
                    }
                    Token::PlusPlus => return self.parse_increment_statement(name, BinaryOp::Add),
                    Token::MinusMinus => {
                        return self.parse_increment_statement(name, BinaryOp::Sub);
                    }
                    Token::LeftParen => {
                        let expr = self.parse_expression();
                        self.expect(Token::Semicolon);
                        return Statement::Expression(expr);
                    }
                    _ => {}
                },
            };

            panic!("Unexpected identifier: {}", name);
        }

        // Otherwise, it's an expression statement.
        let expr = self.parse_expression();
        self.expect(Token::Semicolon);

        Statement::Expression(expr)
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_primary();

        loop {
            match &self.current {
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
                Token::Percent => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Rem,
                        right: Box::new(right),
                    };
                }
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
                Token::Ampersand => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::And,
                        right: Box::new(right),
                    };
                }
                Token::VerticalBar => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Or,
                        right: Box::new(right),
                    };
                }
                Token::Caret => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Xor,
                        right: Box::new(right),
                    };
                }
                Token::LeftAngle => {
                    self.advance();

                    match self.current {
                        Token::LeftAngle => {
                            self.advance();

                            let right = self.parse_primary();

                            left = Expr::Binary {
                                left: Box::new(left),
                                operator: BinaryOp::Shl,
                                right: Box::new(right),
                            };
                        }
                        Token::Equals => {
                            self.advance();

                            let right = self.parse_primary();

                            left = Expr::Binary {
                                left: Box::new(left),
                                operator: BinaryOp::Lte,
                                right: Box::new(right),
                            };
                        }
                        _ => {
                            let right = self.parse_primary();

                            left = Expr::Binary {
                                left: Box::new(left),
                                operator: BinaryOp::Lt,
                                right: Box::new(right),
                            };
                        }
                    }
                }
                Token::RightAngle => {
                    self.advance();

                    match self.current {
                        Token::RightAngle => {
                            self.advance();

                            let right = self.parse_primary();

                            left = Expr::Binary {
                                left: Box::new(left),
                                operator: BinaryOp::Shr,
                                right: Box::new(right),
                            };
                        }
                        Token::Equals => {
                            self.advance();

                            let right = self.parse_primary();

                            left = Expr::Binary {
                                left: Box::new(left),
                                operator: BinaryOp::Gte,
                                right: Box::new(right),
                            };
                        }
                        _ => {
                            let right = self.parse_primary();

                            left = Expr::Binary {
                                left: Box::new(left),
                                operator: BinaryOp::Gt,
                                right: Box::new(right),
                            };
                        }
                    }
                }
                Token::ShiftLeft => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Shl,
                        right: Box::new(right),
                    };
                }
                Token::ShiftRight => {
                    self.advance();

                    let right = self.parse_primary();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator: BinaryOp::Shr,
                        right: Box::new(right),
                    };
                }
                Token::Equals => {
                    self.advance();

                    if self.current == Token::Equals {
                        self.advance();

                        let right = self.parse_primary();

                        left = Expr::Binary {
                            left: Box::new(left),
                            operator: BinaryOp::Eq,
                            right: Box::new(right),
                        };
                    }
                }
                Token::Exclamation => {
                    self.advance();

                    if self.current == Token::Equals {
                        self.advance();

                        let right = self.parse_primary();

                        left = Expr::Binary {
                            left: Box::new(left),
                            operator: BinaryOp::Neq,
                            right: Box::new(right),
                        };
                    }
                }
                Identifier(str) => match str.as_str() {
                    "mod" => {
                        self.advance();

                        let right = self.parse_primary();

                        left = Expr::Binary {
                            left: Box::new(left),
                            operator: BinaryOp::Mod,
                            right: Box::new(right),
                        }
                    }
                    _ => break,
                },
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

            Token::Exclamation => {
                self.advance();
                let operand = self.parse_primary();
                Expr::Unary {
                    operator: UnaryOp::Not,
                    operand: Box::new(operand),
                }
            }

            Token::Tilde => {
                self.advance();
                let operand = self.parse_primary();
                Expr::Unary {
                    operator: UnaryOp::Neg,
                    operand: Box::new(operand),
                }
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
