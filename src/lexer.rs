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
    Exclamation,
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

        let token = match c {
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '%' => Token::Percent,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '&' => Token::Ampersand,
            '|' => Token::VerticalBar,
            '^' => Token::Caret,
            '~' => Token::Tilde,
            '<' => Token::LeftAngle,
            '>' => Token::RightAngle,
            '=' => Token::Equals,
            '!' => Token::Exclamation,
            ';' => Token::Semicolon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ',' => Token::Comma,

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
}
