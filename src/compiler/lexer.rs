#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    StringLiteral(String),

    ShiftLeft,
    ShiftRight,
    ShiftLeftEquals,
    ShiftRightEquals,

    Asterisk,
    AsteriskEquals,
    Slash,
    SlashEquals,
    Percent,
    PercentEquals,
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
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let c = self.input[self.position];

        let token = match c {
            '*' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::AsteriskEquals
                } else {
                    Token::Asterisk
                }
            }
            '/' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::SlashEquals
                } else {
                    Token::Slash
                }
            }
            '%' => {
                if self.peek_char(1) == Some('=') {
                    self.position += 1;
                    Token::PercentEquals
                } else {
                    Token::Percent
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
            '<' => {
                if self.peek_char(1) == Some('<') {
                    if self.peek_char(2) == Some('=') {
                        self.position += 2;
                        Token::ShiftLeftEquals
                    } else {
                        self.position += 1;
                        Token::ShiftLeft
                    }
                } else {
                    Token::LeftAngle
                }
            }
            '>' => {
                if self.peek_char(1) == Some('>') {
                    if self.peek_char(2) == Some('=') {
                        self.position += 2;
                        Token::ShiftRightEquals
                    } else {
                        self.position += 1;
                        Token::ShiftRight
                    }
                } else {
                    Token::RightAngle
                }
            }
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

        let text: String = self.input[start..self.position].iter().collect();

        Token::Number(text.parse().unwrap())
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
