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

    fn parse_while_statement(&mut self) -> Statement {
        self.advance(); // consume 'while'
        self.expect(Token::LeftParen);
        let condition = self.parse_expression();
        self.expect(Token::RightParen);

        Statement::While {
            condition,
            body: Box::new(self.parse_block()),
        }
    }

    fn parse_repeat_statement(&mut self) -> Statement {
        self.advance(); // consume 'repeat'
        self.expect(Token::LeftParen);
        let count = self.parse_expression();
        self.expect(Token::RightParen);

        Statement::Repeat {
            count,
            body: Box::new(self.parse_block()),
        }
    }

    fn parse_do_until_statement(&mut self) -> Statement {
        self.advance(); // consume 'do'
        let body = Box::new(self.parse_block());

        if !self.is_keyword("until") {
            panic!("Expected 'until' after do-block");
        }

        self.advance(); // consume 'until'
        self.expect(Token::LeftParen);
        let condition = self.parse_expression();
        self.expect(Token::RightParen);
        self.expect(Token::Semicolon);

        Statement::DoUntil { body, condition }
    }

    fn parse_for_statement(&mut self) -> Statement {
        self.advance(); // consume 'for'
        self.expect(Token::LeftParen);

        let init = if self.current == Token::Semicolon {
            self.advance();
            None
        } else {
            let stmt = self.parse_for_clause_statement();
            self.expect(Token::Semicolon);
            Some(Box::new(stmt))
        };

        let condition = if self.current == Token::Semicolon {
            self.advance();
            None
        } else {
            let expr = self.parse_expression();
            self.expect(Token::Semicolon);
            Some(expr)
        };

        let update = if self.current == Token::RightParen {
            None
        } else {
            Some(Box::new(self.parse_for_clause_statement()))
        };

        self.expect(Token::RightParen);

        Statement::For {
            init,
            condition,
            update,
            body: Box::new(self.parse_block()),
        }
    }

    fn parse_switch_statement(&mut self) -> Statement {
        self.advance(); // consume 'switch'
        self.expect(Token::LeftParen);
        let value = self.parse_expression();
        self.expect(Token::RightParen);
        self.expect(Token::LeftBrace);

        let mut cases = Vec::new();
        let mut default = None;

        while self.current != Token::RightBrace {
            if self.is_keyword("case") {
                self.advance(); // consume 'case'
                let case_value = self.parse_expression();
                self.expect(Token::Colon);
                let body = self.parse_switch_case_body();
                cases.push((case_value, body));
                continue;
            }

            if self.is_keyword("default") {
                self.advance(); // consume 'default'
                self.expect(Token::Colon);
                default = Some(self.parse_switch_case_body());
                continue;
            }

            panic!(
                "Expected 'case' or 'default' in switch block, got {:?}",
                self.current
            );
        }

        self.expect(Token::RightBrace);

        Statement::Switch {
            value,
            cases,
            default,
        }
    }

    fn parse_switch_case_body(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.current != Token::RightBrace
            && !self.is_keyword("case")
            && !self.is_keyword("default")
        {
            statements.push(self.parse_statement());
        }

        statements
    }

    fn parse_assignment(&mut self, name: String, needs_semicolon: bool) -> Statement {
        self.advance(); // consume identifier
        self.expect(Token::Equals);
        let value = self.parse_expression();
        if needs_semicolon {
            self.expect(Token::Semicolon);
        }

        Statement::Assignment { name, value }
    }

    fn parse_compound_assignment(
        &mut self,
        name: String,
        op: BinaryOp,
        needs_semicolon: bool,
    ) -> Statement {
        self.advance(); // consume identifier
        self.advance(); // consume compound assignment token

        let right = self.parse_expression();
        if needs_semicolon {
            self.expect(Token::Semicolon);
        }

        let value = Expr::Binary {
            left: Box::new(Expr::Variable(name.clone())),
            operator: op,
            right: Box::new(right),
        };

        Statement::Assignment { name, value }
    }

    fn parse_increment_statement(
        &mut self,
        name: String,
        operator: BinaryOp,
        needs_semicolon: bool,
    ) -> Statement {
        self.advance(); // consume identifier
        self.advance(); // consume ++ or --
        if needs_semicolon {
            self.expect(Token::Semicolon);
        }

        let value = Expr::Binary {
            left: Box::new(Expr::Variable(name.clone())),
            operator,
            right: Box::new(Expr::Integer(1)),
        };

        Statement::Assignment { name, value }
    }

    fn parse_prefix_increment_statement(
        &mut self,
        operator: BinaryOp,
        needs_semicolon: bool,
    ) -> Statement {
        self.advance(); // consume ++ or --

        let Token::Identifier(name) = &self.current else {
            panic!("Expected identifier after prefix increment/decrement");
        };
        let name = name.clone();

        self.advance(); // consume identifier
        if needs_semicolon {
            self.expect(Token::Semicolon);
        }

        let value = Expr::Binary {
            left: Box::new(Expr::Variable(name.clone())),
            operator,
            right: Box::new(Expr::Integer(1)),
        };

        Statement::Assignment { name, value }
    }

    fn parse_return_statement(&mut self) -> Statement {
        self.advance(); // consume 'return'

        if self.current == Token::Semicolon {
            self.advance();
            return Statement::Return(None);
        }

        let expr = self.parse_expression();
        self.expect(Token::Semicolon);
        Statement::Return(Some(expr))
    }

    fn parse_for_clause_statement(&mut self) -> Statement {
        match self.current {
            Token::PlusPlus => return self.parse_prefix_increment_statement(BinaryOp::Add, false),
            Token::MinusMinus => {
                return self.parse_prefix_increment_statement(BinaryOp::Sub, false);
            }
            _ => {}
        }

        if let Token::Identifier(name) = &self.current {
            let name = name.clone();

            return match self.peek(1) {
                Token::Equals => self.parse_assignment(name, false),
                Token::PlusEquals => self.parse_compound_assignment(name, BinaryOp::Add, false),
                Token::MinusEquals => self.parse_compound_assignment(name, BinaryOp::Sub, false),
                Token::AsteriskEquals => self.parse_compound_assignment(name, BinaryOp::Mul, false),
                Token::SlashEquals => self.parse_compound_assignment(name, BinaryOp::Div, false),
                Token::PercentEquals => self.parse_compound_assignment(name, BinaryOp::Rem, false),
                Token::AmpersandEquals => {
                    self.parse_compound_assignment(name, BinaryOp::And, false)
                }
                Token::VerticalBarEquals => {
                    self.parse_compound_assignment(name, BinaryOp::Or, false)
                }
                Token::CaretEquals => self.parse_compound_assignment(name, BinaryOp::Xor, false),
                Token::ShiftLeftEquals => {
                    self.parse_compound_assignment(name, BinaryOp::Shl, false)
                }
                Token::ShiftRightEquals => {
                    self.parse_compound_assignment(name, BinaryOp::Shr, false)
                }
                Token::PlusPlus => self.parse_increment_statement(name, BinaryOp::Add, false),
                Token::MinusMinus => self.parse_increment_statement(name, BinaryOp::Sub, false),
                _ => {
                    let expr = self.parse_expression();
                    Statement::Expression(expr)
                }
            };
        }

        let expr = self.parse_expression();
        Statement::Expression(expr)
    }

    fn is_keyword(&self, keyword: &str) -> bool {
        matches!(&self.current, Token::Identifier(value) if value == keyword)
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current {
            Token::PlusPlus => return self.parse_prefix_increment_statement(BinaryOp::Add, true),
            Token::MinusMinus => return self.parse_prefix_increment_statement(BinaryOp::Sub, true),
            _ => {}
        }

        if let Token::Identifier(name) = &self.current {
            let name = name.clone();

            match name.as_str() {
                "if" => return self.parse_if_statement(),
                "while" => return self.parse_while_statement(),
                "repeat" => return self.parse_repeat_statement(),
                "for" => return self.parse_for_statement(),
                "do" => return self.parse_do_until_statement(),
                "switch" => return self.parse_switch_statement(),
                "break" => {
                    self.advance();
                    self.expect(Token::Semicolon);
                    return Statement::Break;
                }
                "continue" => {
                    self.advance();
                    self.expect(Token::Semicolon);
                    return Statement::Continue;
                }
                "return" => return self.parse_return_statement(),
                _ => match self.peek(1) {
                    Token::Equals => return self.parse_assignment(name, true),
                    Token::PlusEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Add, true);
                    }
                    Token::MinusEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Sub, true);
                    }
                    Token::AsteriskEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Mul, true);
                    }
                    Token::SlashEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Div, true);
                    }
                    Token::PercentEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Rem, true);
                    }
                    Token::AmpersandEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::And, true);
                    }
                    Token::VerticalBarEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Or, true);
                    }
                    Token::CaretEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Xor, true);
                    }
                    Token::ShiftLeftEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Shl, true);
                    }
                    Token::ShiftRightEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Shr, true);
                    }
                    Token::PlusPlus => {
                        return self.parse_increment_statement(name, BinaryOp::Add, true);
                    }
                    Token::MinusMinus => {
                        return self.parse_increment_statement(name, BinaryOp::Sub, true);
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

        if self.current == Token::Question {
            self.advance();
            let then_expr = self.parse_expression();
            self.expect(Token::Colon);
            let else_expr = self.parse_expression();
            left = Expr::Ternary {
                condition: Box::new(left),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
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

            Token::StringLiteral(value) => {
                let value = value.clone();
                self.advance();
                Expr::String(value)
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
