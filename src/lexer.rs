#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i32),

    Asterisk,
    Slash,
    Percent,
    Plus,
    Minus,
    Ampersand,
    VerticalBar,
    Caret,
    Tilde,
    LeftAngle,
    RightAngle,

    Equals,
    Semicolon,

    LeftParen,
    RightParen,
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

        match c {
            '*' => {
                self.position += 1;
                Token::Asterisk
            }
            '/' => {
                self.position += 1;
                Token::Slash
            }
            '%' => {
                self.position += 1;
                Token::Percent
            }
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '-' => {
                self.position += 1;
                Token::Minus
            }
            '&' => {
                self.position += 1;
                Token::Ampersand
            }
            '|' => {
                self.position += 1;
                Token::VerticalBar
            }
            '^' => {
                self.position += 1;
                Token::Caret
            }
            '~' => {
                self.position += 1;
                Token::Tilde
            }
            '<' => {
                self.position += 1;
                Token::LeftAngle
            }
            '>' => {
                self.position += 1;
                Token::RightAngle
            }
            '=' => {
                self.position += 1;
                Token::Equals
            }

            ';' => {
                self.position += 1;
                Token::Semicolon
            }

            '(' => {
                self.position += 1;
                Token::LeftParen
            }

            ')' => {
                self.position += 1;
                Token::RightParen
            }

            ',' => {
                self.position += 1;
                Token::Comma
            }

            '0'..='9' => self.read_number(),

            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),

            _ => panic!("Unknown character {}", c),
        }
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
}
