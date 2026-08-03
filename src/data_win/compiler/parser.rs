use super::ast::*;
use super::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
enum AccessSegment {
    Index(Expr),
    Member(String),
}

#[derive(Debug, Clone)]
struct AccessTarget {
    base: Expr,
    segments: Vec<AccessSegment>,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Vec<Token>,
    current_line: usize,
    line_token_position: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let next = vec![lexer.next_token(), lexer.next_token()];

        Self {
            lexer,
            current,
            next,
            current_line: 1,
            line_token_position: 1,
        }
    }

    fn increment_line(&mut self) {
        self.current_line += 1;
        self.line_token_position = 1;
    }

    fn advance(&mut self) {
        if self.current == Token::Newline {
            self.increment_line();
        } else {
            self.line_token_position += 1;
        }
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
            panic!(
                "Expected {:?}, got {:?} at line {}, token {}",
                expected, self.current, self.current_line, self.line_token_position
            );
        }

        self.advance();
    }

    fn consume_statement_terminator(&mut self) {
        if self.current == Token::Semicolon {
            self.advance();
            return;
        }

        if self.current == Token::CommentSingleLine {
            self.advance();
            if self.current == Token::Newline {
                self.advance();
            }
            return;
        }

        if self.current == Token::Newline {
            self.advance();
            return;
        }

        panic!(
            "Expected ';' or newline, got {:?} at line {}, token {}",
            self.current, self.current_line, self.line_token_position
        );
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.current != Token::EOF {
            while self.current == Token::Newline || self.current == Token::CommentSingleLine {
                self.advance();
            }
            if self.current == Token::EOF {
                break;
            }
            let block = self.parse_block();
            statements.extend(block);
        }

        statements
    }

    fn skip_newlines(&mut self) {
        while self.current == Token::Newline || self.current == Token::CommentSingleLine {
            self.advance();
        }
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        self.skip_newlines();

        if self.current != Token::LeftBrace {
            return vec![self.parse_statement()];
        }

        self.expect(Token::LeftBrace);
        let mut statements = Vec::new();

        while self.current != Token::RightBrace {
            if self.current == Token::Newline || self.current == Token::CommentSingleLine {
                self.advance();
                continue;
            }
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
        self.consume_statement_terminator();

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
            self.consume_statement_terminator();
            Some(Box::new(stmt))
        };

        let condition = if self.current == Token::Semicolon {
            self.advance();
            None
        } else {
            let expr = self.parse_expression();
            self.consume_statement_terminator();
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
            self.skip_newlines();
            if self.current == Token::RightBrace {
                break;
            }

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
            if self.current == Token::Newline || self.current == Token::CommentSingleLine {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement());
        }

        statements
    }

    fn build_call(&self, name: &str, args: Vec<CallArg>) -> Expr {
        Expr::Call {
            name: name.to_string(),
            args,
        }
    }

    fn parse_access_target_from_identifier(&mut self, name: String) -> AccessTarget {
        self.advance(); // consume identifier

        let mut target = AccessTarget {
            base: Expr::Variable(name),
            segments: Vec::new(),
        };

        loop {
            match self.current {
                Token::LeftBracket => {
                    self.advance(); // consume '['
                    let index = self.parse_expression();
                    self.expect(Token::RightBracket);
                    target.segments.push(AccessSegment::Index(index));
                }
                Token::Dot => {
                    self.advance(); // consume '.'

                    let Token::Identifier(field) = &self.current else {
                        panic!("Expected identifier after '.'");
                    };
                    let field = field.clone();
                    self.advance();
                    target.segments.push(AccessSegment::Member(field));
                }
                _ => break,
            }
        }

        // Normalize `global.field`, `self.field`, `other.field` into a scoped
        // variable name so they are emitted with the correct instance type
        // rather than being lowered to variable_struct_get/set calls.
        if let Expr::Variable(ref base_name) = target.base {
            if matches!(base_name.as_str(), "global" | "self" | "other")
                && matches!(target.segments.first(), Some(AccessSegment::Member(_)))
            {
                let scope = base_name.clone();
                let first = target.segments.remove(0);
                if let AccessSegment::Member(field) = first {
                    target.base = Expr::Variable(format!("{}.{}", scope, field));
                }
            }
        }

        target
    }

    fn build_access_read_expr(&self, target: &AccessTarget) -> Expr {
        let mut expr = target.base.clone();

        for segment in &target.segments {
            expr = match segment {
                AccessSegment::Index(index) => self.build_call(
                    "array_get",
                    vec![
                        CallArg::Positional(expr),
                        CallArg::Positional(index.clone()),
                    ],
                ),
                AccessSegment::Member(name) => self.build_call(
                    "variable_struct_get",
                    vec![
                        CallArg::Positional(expr),
                        CallArg::Positional(Expr::String(name.clone())),
                    ],
                ),
            };
        }

        expr
    }

    fn build_access_write_expr(
        &self,
        target: &AccessTarget,
        value: Expr,
        compound_operator: Option<BinaryOp>,
    ) -> Expr {
        let mut expr = if let Some(operator) = compound_operator {
            Expr::Binary {
                left: Box::new(self.build_access_read_expr(target)),
                operator,
                right: Box::new(value),
            }
        } else {
            value
        };

        for segment_index in (0..target.segments.len()).rev() {
            let segment = &target.segments[segment_index];
            let container_expr = if segment_index == 0 {
                target.base.clone()
            } else {
                let prefix = AccessTarget {
                    base: target.base.clone(),
                    segments: target.segments[..segment_index].to_vec(),
                };
                self.build_access_read_expr(&prefix)
            };

            expr = match segment {
                AccessSegment::Index(index) => self.build_call(
                    "array_set",
                    vec![
                        CallArg::Positional(container_expr),
                        CallArg::Positional(index.clone()),
                        CallArg::Positional(expr),
                    ],
                ),
                AccessSegment::Member(name) => self.build_call(
                    "variable_struct_set",
                    vec![
                        CallArg::Positional(container_expr),
                        CallArg::Positional(Expr::String(name.clone())),
                        CallArg::Positional(expr),
                    ],
                ),
            };
        }

        expr
    }

    fn parse_array_literal(&mut self) -> Expr {
        self.expect(Token::LeftBracket);

        let mut values = Vec::new();
        loop {
            self.skip_newlines();
            if self.current == Token::RightBracket {
                break;
            }

            values.push(CallArg::Positional(self.parse_expression()));
            self.skip_newlines();

            if self.current == Token::Comma {
                self.advance();
                continue;
            }

            break;
        }

        self.expect(Token::RightBracket);

        self.build_call("@@NewGMLArray@@", values)
    }

    fn parse_struct_literal(&mut self) -> Expr {
        self.expect(Token::LeftBrace);

        let mut fields = Vec::new();

        if self.current != Token::RightBrace {
            loop {
                if self.current == Token::Newline {
                    self.advance();
                    continue;
                }

                let key = match &self.current {
                    Token::Identifier(name) => {
                        let key = name.clone();
                        self.advance();
                        key
                    }
                    Token::StringLiteral(value) => {
                        let key = value.clone();
                        self.advance();
                        key
                    }
                    _ => panic!(
                        "Expected identifier or string literal in struct literal, got {:?} at line {}, token {}",
                        self.current, self.current_line, self.line_token_position
                    ),
                };

                self.expect(Token::Colon);
                let value = self.parse_expression();
                fields.push((key, value));

                if self.current == Token::Comma {
                    self.advance();
                    if self.current == Token::RightBrace {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        self.expect(Token::RightBrace);
        Expr::StructLiteral(fields)
    }

    fn parse_postfix(&mut self, mut expr: Expr) -> Expr {
        loop {
            match self.current {
                Token::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression();
                    self.expect(Token::RightBracket);
                    expr = self.build_call(
                        "array_get",
                        vec![CallArg::Positional(expr), CallArg::Positional(index)],
                    );
                }
                Token::Dot => {
                    self.advance();
                    let Token::Identifier(field) = &self.current else {
                        panic!("Expected identifier after '.'");
                    };
                    let field = field.clone();
                    self.advance();
                    expr = Expr::MemberAccess {
                        target: Box::new(expr),
                        field,
                    };
                }
                _ => break,
            }
        }

        expr
    }

    fn parse_access_statement_from_identifier(
        &mut self,
        name: String,
        needs_semicolon: bool,
    ) -> Statement {
        let target = self.parse_access_target_from_identifier(name);
        let simple_variable_name = if target.segments.is_empty() {
            if let Expr::Variable(name) = &target.base {
                Some(name.clone())
            } else {
                None
            }
        } else {
            None
        };

        let statement = match self.current {
            Token::Equals => {
                self.advance();
                let value = self.parse_expression();
                if let Some(name) = simple_variable_name {
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(&target, value, None))
                }
            }
            Token::PlusEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Add,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Add),
                    ))
                }
            }
            Token::MinusEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Sub,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Sub),
                    ))
                }
            }
            Token::MultiplyEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Mul,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Mul),
                    ))
                }
            }
            Token::DivideEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Div,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Div),
                    ))
                }
            }
            Token::RemainderEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Rem,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Rem),
                    ))
                }
            }
            Token::AmpersandEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::And,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::And),
                    ))
                }
            }
            Token::VerticalBarEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Or,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Or),
                    ))
                }
            }
            Token::CaretEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Xor,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Xor),
                    ))
                }
            }
            Token::ShiftLeftEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Shl,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Shl),
                    ))
                }
            }
            Token::ShiftRightEquals => {
                self.advance();
                let right = self.parse_expression();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Shr,
                        right: Box::new(right),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        right,
                        Some(BinaryOp::Shr),
                    ))
                }
            }
            Token::PlusPlus => {
                self.advance();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Add,
                        right: Box::new(Expr::Integer(1)),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        Expr::Integer(1),
                        Some(BinaryOp::Add),
                    ))
                }
            }
            Token::MinusMinus => {
                self.advance();
                if let Some(name) = simple_variable_name.clone() {
                    let value = Expr::Binary {
                        left: Box::new(Expr::Variable(name.clone())),
                        operator: BinaryOp::Sub,
                        right: Box::new(Expr::Integer(1)),
                    };
                    Statement::Assignment { name, value }
                } else {
                    Statement::Expression(self.build_access_write_expr(
                        &target,
                        Expr::Integer(1),
                        Some(BinaryOp::Sub),
                    ))
                }
            }
            Token::Semicolon => {
                if needs_semicolon {
                    self.consume_statement_terminator();
                }
                Statement::Expression(self.build_access_read_expr(&target))
            }
            _ => {
                let expr = self.build_access_read_expr(&target);
                let expr = self.parse_expression_with_left(expr);
                if needs_semicolon {
                    self.consume_statement_terminator();
                }
                return Statement::Expression(expr);
            }
        };

        if needs_semicolon {
            self.consume_statement_terminator();
        }
        statement
    }

    fn parse_assignment(&mut self, name: String, needs_semicolon: bool) -> Statement {
        self.advance(); // consume identifier
        self.expect(Token::Equals);
        let value = self.parse_expression();
        if needs_semicolon {
            self.consume_statement_terminator();
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
            self.consume_statement_terminator();
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
            self.consume_statement_terminator();
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
            self.consume_statement_terminator();
        }

        let value = Expr::Binary {
            left: Box::new(Expr::Variable(name.clone())),
            operator,
            right: Box::new(Expr::Integer(1)),
        };

        Statement::Assignment { name, value }
    }

    fn parse_function_declaration_statement(&mut self) -> Statement {
        self.advance(); // consume 'function'

        let name = match &self.current {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => panic!("Expected function name"),
        };

        self.expect(Token::LeftParen);
        let mut params = Vec::new();
        if self.current != Token::RightParen {
            loop {
                let Token::Identifier(name) = &self.current else {
                    panic!("Expected parameter name");
                };
                let name = name.clone();
                self.advance();

                let default_value = if self.current == Token::Equals {
                    self.advance();
                    Some(self.parse_expression())
                } else {
                    None
                };

                params.push(FunctionParameter {
                    name,
                    default_value,
                });

                if self.current == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RightParen);
        self.skip_newlines();

        let body = if self.current == Token::LeftBrace {
            self.parse_block()
        } else {
            vec![self.parse_statement()]
        };

        Statement::FunctionDeclaration { name, params, body }
    }

    fn parse_function_expression(&mut self) -> Expr {
        self.advance(); // consume 'function'

        let name = if let Token::Identifier(candidate) = &self.current {
            let candidate = candidate.clone();
            self.advance();
            if self.current == Token::LeftParen {
                Some(candidate)
            } else {
                None
            }
        } else {
            None
        };

        self.expect(Token::LeftParen);
        let mut params = Vec::new();
        if self.current != Token::RightParen {
            loop {
                let Token::Identifier(name) = &self.current else {
                    panic!("Expected parameter name");
                };
                let name = name.clone();
                self.advance();

                let default_value = if self.current == Token::Equals {
                    self.advance();
                    Some(self.parse_expression())
                } else {
                    None
                };

                params.push(FunctionParameter {
                    name,
                    default_value,
                });

                if self.current == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RightParen);
        self.skip_newlines();

        let body = if self.current == Token::LeftBrace {
            self.parse_block()
        } else {
            vec![self.parse_statement()]
        };

        Expr::Function { name, params, body }
    }

    fn parse_return_statement(&mut self) -> Statement {
        self.advance(); // consume 'return'

        if self.current == Token::Semicolon {
            self.advance();
            return Statement::Return(None);
        }

        if self.current == Token::Newline || self.current == Token::CommentSingleLine {
            self.consume_statement_terminator();
            return Statement::Return(None);
        }

        let expr = self.parse_expression();
        self.consume_statement_terminator();
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

        if self.is_keyword("var") {
            return self.parse_var_declaration(false);
        }

        if let Token::Identifier(name) = &self.current {
            let name = name.clone();

            return match self.peek(1) {
                Token::Equals => self.parse_assignment(name, false),
                Token::PlusEquals => self.parse_compound_assignment(name, BinaryOp::Add, false),
                Token::MinusEquals => self.parse_compound_assignment(name, BinaryOp::Sub, false),
                Token::MultiplyEquals => self.parse_compound_assignment(name, BinaryOp::Mul, false),
                Token::DivideEquals => self.parse_compound_assignment(name, BinaryOp::Div, false),
                Token::RemainderEquals => {
                    self.parse_compound_assignment(name, BinaryOp::Rem, false)
                }
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
                Token::LeftBracket | Token::Dot => {
                    self.parse_access_statement_from_identifier(name, false)
                }
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

    fn parse_var_declaration(&mut self, needs_semicolon: bool) -> Statement {
        self.advance(); // consume 'var'

        let mut declarations = Vec::new();
        loop {
            let Token::Identifier(var_name) = &self.current else {
                panic!(
                    "Expected identifier in var declaration, got {:?}",
                    self.current
                );
            };
            let var_name = var_name.clone();
            self.advance();

            let value = if self.current == Token::Equals {
                self.advance();
                Some(self.parse_expression())
            } else {
                None
            };

            declarations.push((var_name, value));

            if self.current == Token::Comma {
                self.advance();
            } else {
                break;
            }
        }

        if needs_semicolon {
            self.consume_statement_terminator();
        }
        Statement::VarDeclaration { declarations }
    }

    fn parse_globalvar_declaration(&mut self) -> Statement {
        self.advance(); // consume 'globalvar'
        let Token::Identifier(var_name) = &self.current else {
            panic!("Expected identifier in globalvar declaration");
        };
        let var_name = var_name.clone();
        self.advance();
        self.consume_statement_terminator();
        Statement::GlobalVarDeclaration { name: var_name }
    }

    fn parse_static_declaration(&mut self, needs_semicolon: bool) -> Statement {
        self.advance(); // consume 'static'
        let Token::Identifier(var_name) = &self.current else {
            panic!("Expected identifier after 'static'");
        };
        let var_name = var_name.clone();
        self.advance();

        let value = if self.current == Token::Equals {
            self.advance();
            Some(self.parse_expression())
        } else {
            None
        };

        if needs_semicolon {
            self.consume_statement_terminator();
        }
        Statement::StaticDeclaration {
            name: var_name,
            value,
        }
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
                    self.consume_statement_terminator();
                    return Statement::Break;
                }
                "continue" => {
                    self.advance();
                    self.consume_statement_terminator();
                    return Statement::Continue;
                }
                "return" => return self.parse_return_statement(),
                "function" => return self.parse_function_declaration_statement(),
                "var" => return self.parse_var_declaration(true),
                "globalvar" => return self.parse_globalvar_declaration(),
                "static" => return self.parse_static_declaration(true),
                _ => match self.peek(1) {
                    Token::Equals => return self.parse_assignment(name, true),
                    Token::PlusEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Add, true);
                    }
                    Token::MinusEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Sub, true);
                    }
                    Token::MultiplyEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Mul, true);
                    }
                    Token::DivideEquals => {
                        return self.parse_compound_assignment(name, BinaryOp::Div, true);
                    }
                    Token::RemainderEquals => {
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
                    Token::LeftBracket | Token::Dot => {
                        return self.parse_access_statement_from_identifier(name, true);
                    }
                    Token::LeftParen => {
                        let expr = self.parse_expression();
                        self.consume_statement_terminator();
                        return Statement::Expression(expr);
                    }
                    _ => {}
                },
            };

            panic!(
                "Unexpected identifier: {} at line {}, token {}",
                name, self.current_line, self.line_token_position
            );
        }

        // Otherwise, it's an expression statement.
        let expr = self.parse_expression();
        self.consume_statement_terminator();

        Statement::Expression(expr)
    }

    fn parse_expression(&mut self) -> Expr {
        let left = self.parse_primary();
        self.parse_expression_with_left(left)
    }

    fn parse_binary_expression(&mut self, left: Expr, operator: BinaryOp) -> Expr {
        let right = self.parse_primary();
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn parse_expression_with_left(&mut self, mut left: Expr) -> Expr {
        loop {
            match &self.current {
                Token::Multiply => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Mul);
                }
                Token::Divide => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Div);
                }
                Token::Remainder => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Rem);
                }
                Token::Modulo => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Mod);
                }
                Token::Plus => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Add);
                }
                Token::Minus => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Sub);
                }
                Token::Ampersand => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::And);
                }
                Token::VerticalBar => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Or);
                }
                Token::Caret => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Xor);
                }
                Token::ShiftLeft => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Shl);
                }
                Token::ShiftRight => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Shr);
                }
                Token::LessThan => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Lt);
                }
                Token::LessThanEquals => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Lte);
                }
                Token::GreaterThan => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Gt);
                }
                Token::GreaterThanEquals => {
                    self.advance();
                    left = self.parse_binary_expression(left, BinaryOp::Gte);
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
            Token::Number(text) => {
                let value = text.clone();
                self.advance();
                if value.contains('.') || value.contains('e') || value.contains('E') {
                    Expr::Float(value.parse().unwrap())
                } else {
                    Expr::Integer(value.parse().unwrap())
                }
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

            Token::Plus => {
                self.advance();
                self.parse_primary()
            }

            Token::Minus => {
                self.advance();
                let operand = self.parse_primary();
                Expr::Unary {
                    operator: UnaryOp::Neg,
                    operand: Box::new(operand),
                }
            }

            Token::LeftBracket => {
                let expr = self.parse_array_literal();
                self.parse_postfix(expr)
            }

            Token::LeftBrace => {
                let expr = self.parse_struct_literal();
                self.parse_postfix(expr)
            }

            Token::Identifier(name) if name == "true" => {
                self.advance();
                Expr::Bool(true)
            }

            Token::Identifier(name) if name == "false" => {
                self.advance();
                Expr::Bool(false)
            }

            Token::Identifier(name) if name == "function" => self.parse_function_expression(),

            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                let expr = if self.current == Token::LeftParen {
                    self.advance(); // consume '('

                    let mut args = Vec::new();

                    if self.current != Token::RightParen {
                        loop {
                            if let Token::Identifier(candidate) = &self.current {
                                let is_named = self.peek(1) == &Token::Colon;
                                if is_named {
                                    let arg_name = candidate.clone();
                                    self.advance();
                                    self.expect(Token::Colon);
                                    args.push(CallArg::Named {
                                        name: arg_name,
                                        value: self.parse_expression(),
                                    });
                                } else {
                                    args.push(CallArg::Positional(self.parse_expression()));
                                }
                            } else {
                                args.push(CallArg::Positional(self.parse_expression()));
                            }

                            if self.current == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    if self.current != Token::RightParen {
                        panic!(
                            "Expected ')', got {:?} at line {}, token {}",
                            self.current, self.current_line, self.line_token_position
                        );
                    }

                    self.advance(); // consume ')'

                    Expr::Call { name, args }
                } else {
                    Expr::Variable(name)
                };

                self.parse_postfix(expr)
            }

            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression();
                self.expect(Token::RightParen);
                self.parse_postfix(expr)
            }

            Token::CommentSingleLine => {
                self.advance();
                while self.current != Token::EOF && self.current != Token::Newline {
                    self.advance();
                }
                if self.current == Token::Newline {
                    self.advance();
                }
                self.parse_primary()
            }

            Token::Newline => {
                self.advance();
                self.parse_primary()
            }

            _ => panic!(
                "Unexpected token {:?} at line {}, token {}",
                self.current, self.current_line, self.line_token_position
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_increments_line_number() {
        let input = "// comment\nx = 1;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        assert_eq!(parser.current_line, 2);
    }

    #[test]
    fn parse_negative_integer_literal() {
        let input = "var x = -5;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        assert_eq!(
            format!("{:?}", program[0]),
            "VarDeclaration { declarations: [(\"x\", Some(Unary { operator: Neg, operand: Integer(5) }))] }"
        );
    }

    #[test]
    fn parse_negative_float_literal() {
        let input = "var x = -5.5;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        assert_eq!(
            format!("{:?}", program[0]),
            "VarDeclaration { declarations: [(\"x\", Some(Unary { operator: Neg, operand: Float(5.5) }))] }"
        );
    }

    #[test]
    fn parse_array_literal_trailing_comma() {
        let input = "var x = [1, 2,];";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        assert_eq!(
            format!("{:?}", program[0]),
            "VarDeclaration { declarations: [(\"x\", Some(Call { name: \"@@NewGMLArray@@\", args: [Positional(Integer(1)), Positional(Integer(2))] }))] }"
        );
    }

    #[test]
    fn parse_array_literal_newlines_and_trailing_comma() {
        let input = "var x = [1,\n2,\n];";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        assert_eq!(
            format!("{:?}", program[0]),
            "VarDeclaration { declarations: [(\"x\", Some(Call { name: \"@@NewGMLArray@@\", args: [Positional(Integer(1)), Positional(Integer(2))] }))] }"
        );
    }

    #[test]
    fn parse_array_literal_newline_before_closing_bracket() {
        let input = "var x = [1,\n2\n];";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        assert_eq!(
            format!("{:?}", program[0]),
            "VarDeclaration { declarations: [(\"x\", Some(Call { name: \"@@NewGMLArray@@\", args: [Positional(Integer(1)), Positional(Integer(2))] }))] }"
        );
    }

    #[test]
    fn parse_assignment_without_semicolon_when_followed_by_newline() {
        let input = "x = 1\ny = 2;";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 2);
        assert_eq!(
            format!("{:?}", program[0]),
            "Assignment { name: \"x\", value: Integer(1) }"
        );
        assert_eq!(
            format!("{:?}", program[1]),
            "Assignment { name: \"y\", value: Integer(2) }"
        );
    }

    #[test]
    fn parse_expression_statement_without_semicolon_when_followed_by_newline() {
        let input = "show_debug_message(\"hi\")\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(program.len(), 1);
        assert!(matches!(program[0], Statement::Expression(_)));
    }
}
