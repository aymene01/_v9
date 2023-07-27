#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Function,
    Let,
    If,
    Else,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    // Literals
    Identifier(String),
    Number(f64),
    // End of Input
    Eof,
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    pub fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.input.chars().nth(self.position) {
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn is_whitespace(ch: char) -> bool {
        ch.is_whitespace() || ch == '\n'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_digit(10)
    }

    // fn is_identifier_start(ch: char) -> bool {
    //     ch.is_alphabetic() || ch == '_'
    // }

    fn is_identifier_continue(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    fn read_identifier(&mut self, start: char) -> String {
        let mut ident = start.to_string();
        while let Some(ch) = self.peek() {
            if Self::is_identifier_continue(ch) {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self, start: char) -> f64 {
        let mut number = start.to_string();
        while let Some(ch) = self.peek() {
            if Self::is_digit(ch) || ch == '.' {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.advance() {
            if Self::is_whitespace(ch) {
                continue;
            }

            match ch {
                '(' => return Token::LeftParen,
                ')' => return Token::RightParen,
                '{' => return Token::LeftBrace,
                '}' => return Token::RightBrace,
                ',' => return Token::Comma,
                ';' => return Token::Semicolon,
                '+' => return Token::Plus,
                '-' => return Token::Minus,
                '*' => return Token::Star,
                '/' => return Token::Slash,
                '0'..='9' => return Token::Number(self.read_number(ch)),
                'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(ch).into(),
                _ => {
                    //Invalid character
                }
            }
        }

        Token::Eof
    }
}

impl From<String> for Token {
    fn from(identifier: String) -> Self {
        match identifier.as_str() {
            "function" => Token::Function,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Identifier(identifier),
        }
    }
}
