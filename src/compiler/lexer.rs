#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    StringLiteral(String),

    ShiftLeft,
    ShiftRight,
    ShiftLeftEquals,
    ShiftRightEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,

    Multiply,
    MultiplyEquals,
    Divide,
    DivideEquals,
    Remainder,
    RemainderEquals,
    Plus,
    PlusEquals,
    PlusPlus,
    Minus,
    MinusEquals,
    MinusMinus,
    Ampersand,
    AmpersandEquals,
    VerticalBar,
    VerticalBarEquals,
    Caret,
    CaretEquals,
    Exclamation,
    Tilde,
    LeftAngle,
    RightAngle,

    Equals,
    Colon,
    Question,
    Semicolon,

    LeftBracket,
    RightBracket,
    Dot,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,

    CommentSingleLine,
    Newline,

    EOF,
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while self.position < self.input.len() {
            match self.input[self.position] {
                ' ' | '\t' | '\r' => self.position += 1,
                _ => break,
            }
        }

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let c = self.input[self.position];

        let token = match c {
            '*' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::MultiplyEquals
                } else {
                    Token::Multiply
                }
            }
            '/' => {
                match self.peek_char(1) {
                    Some('/') => {
                        // Consume the rest of the line as a comment, but keep the newline
                        // in the input so it can be emitted as a separate token.
                        while self.position < self.input.len() && self.input[self.position] != '\n'
                        {
                            self.position += 1;
                        }
                        return Token::CommentSingleLine;
                    }
                    Some('=') => {
                        self.position += 1;
                        Token::DivideEquals
                    }
                    _ => Token::Divide,
                }
            }
            '%' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::RemainderEquals
                } else {
                    Token::Remainder
                }
            }
            '+' => {
                if self.peek_char(1) == Some('+') {
                    self.position += 1;
                    Token::PlusPlus
                } else if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::PlusEquals
                } else {
                    Token::Plus
                }
            }
            '-' => {
                if self.peek_char(1) == Some('-') {
                    self.position += 1;
                    Token::MinusMinus
                } else if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::MinusEquals
                } else {
                    Token::Minus
                }
            }
            '&' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::AmpersandEquals
                } else {
                    Token::Ampersand
                }
            }
            '|' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::VerticalBarEquals
                } else {
                    Token::VerticalBar
                }
            }
            '^' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::CaretEquals
                } else {
                    Token::Caret
                }
            }
            '~' => Token::Tilde,
            '<' => match self.peek_char(1) {
                Some('=') => {
                    self.position += 1;
                    Token::LessThanEquals
                }
                Some('<') => {
                    if self.peek_char(2) == Some('=') {
                        self.position += 2;
                        Token::ShiftLeftEquals
                    } else {
                        self.position += 1;
                        Token::ShiftLeft
                    }
                }
                _ => Token::LessThan,
            },
            '>' => match self.peek_char(1) {
                Some('=') => {
                    self.position += 1;
                    Token::GreaterThanEquals
                }
                Some('>') => {
                    if self.peek_char(2) == Some('=') {
                        self.position += 2;
                        Token::ShiftRightEquals
                    } else {
                        self.position += 1;
                        Token::ShiftRight
                    }
                }
                _ => Token::GreaterThan,
            },
            '=' => Token::Equals,
            ':' => Token::Colon,
            '?' => Token::Question,
            '!' => Token::Exclamation,
            ';' => Token::Semicolon,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '.' => Token::Dot,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ',' => Token::Comma,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '"' | '\'' => return self.read_string(c),

            '0'..='9' => return self.read_number(),
            'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(),

            '\n' => Token::Newline,

            _ => panic!("Unknown character {}", c),
        };

        self.position += 1;
        token
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }

        if self.peek_char(0) == Some('.') && self.peek_char(1) != Some('.') {
            self.position += 1;
            while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
                self.position += 1;
            }
        }

        if let Some('e') | Some('E') = self.peek_char(0) {
            self.position += 1;
            if let Some('+') | Some('-') = self.peek_char(0) {
                self.position += 1;
            }
            while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
                self.position += 1;
            }
        }

        let text: String = self.input[start..self.position].iter().collect();

        Token::Number(text)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len()
            && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
        {
            self.position += 1;
        }

        let text: String = self.input[start..self.position].iter().collect();

        Token::Identifier(text)
    }

    fn read_string(&mut self, quote: char) -> Token {
        // Consume opening quote.
        self.position += 1;
        let start = self.position;

        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch == quote {
                let text: String = self.input[start..self.position].iter().collect();
                self.position += 1; // consume closing quote
                return Token::StringLiteral(text);
            }

            // Preserve escaped characters literally for now; runtime string semantics
            // are handled by VM string functions.
            if ch == '\\' && self.peek_char(1).is_some() {
                self.position += 1;
            }

            self.position += 1;
        }

        panic!("Unterminated string literal");
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }
}
