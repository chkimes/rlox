use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use std::convert::TryInto;
use std::iter::*;

pub struct Scanner<'a>{
    pub source: &'a [u8],
    start: usize,
    current: usize,
    line: u16,
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: u16,
}

#[derive(FromPrimitive)]
#[derive(ToPrimitive)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, EOF
  }

impl Scanner<'_> {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance().try_into().unwrap();
        if is_alpha(c) {
            return self.identifier();
        }
        if is_digit(c) {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                let tt = if self.match_char('=') { TokenType::BangEqual } else { TokenType::Bang };
                return self.make_token(tt);
            },
            '=' => {
                let tt = if self.match_char('=') { TokenType::EqualEqual } else { TokenType::Equal };
                return self.make_token(tt);
            },
            '<' => {
                let tt = if self.match_char('=') { TokenType::LessEqual } else { TokenType::Less };
                return self.make_token(tt);
            },
            '>' => {
                let tt = if self.match_char('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                return self.make_token(tt);
            },
            '"' => return self.string(),
            _   => { }
        }

        return self.error_token();
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                  self.advance();
                  break;
                },
                '\n'=> {
                    self.line += 1;
                    self.advance();
                },
                '/' => {
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line.
                        while self.peek() != '\n' && !self.at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        return self.source[self.current].try_into().unwrap();
    }

    fn peek_next(&self) -> char {
        if self.at_end() {
            return '\0';
        } else {
            return self.source[self.current+1].try_into().unwrap();
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            return self.error_token();
        }

          // The closing quote.
          self.advance();
          return self.make_token(TokenType::String);
    }

    fn number(&mut self) -> Token {
        while is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && is_digit(self.peek_next()) {
            // Consume the ".".
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start].try_into().unwrap() {
            'a' => return self.check_keyword(1, "nd", TokenType::And),
            'c' => return self.check_keyword(1, "lass", TokenType::Class),
            'e' => return self.check_keyword(1, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1].try_into().unwrap() {
                        'a' => return self.check_keyword(2, "lse", TokenType::False),
                        'o' => return self.check_keyword(2, "r", TokenType::For),
                        'u' => return self.check_keyword(2, "n", TokenType::Fun),
                        _   => return TokenType::Identifier,
                    }
                }
                return TokenType::Identifier;
            },
            'i' => return self.check_keyword(1, "f", TokenType::If),
            'n' => return self.check_keyword(1, "il", TokenType::Nil),
            'o' => return self.check_keyword(1, "r", TokenType::Or),
            'p' => return self.check_keyword(1, "rint", TokenType::Print),
            'r' => return self.check_keyword(1, "eturn", TokenType::Return),
            's' => return self.check_keyword(1, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1].try_into().unwrap() {
                        'h' => return self.check_keyword(2, "is", TokenType::This),
                        'r' => return self.check_keyword(2, "ue", TokenType::True),
                        _   => return TokenType::Identifier,
                    }
                }
                return TokenType::Identifier;
            },
            'v' => return self.check_keyword(1, "ar", TokenType::Var),
            'w' => return self.check_keyword(1, "hile", TokenType::While),
            _   => return TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + rest.len() {
            for (i, c) in rest.chars().enumerate() {
                let b = self.source[self.start + start + i];
                if c as u8 != b {
                    return TokenType::Identifier;
                }
            }

            return token_type;
        }

        TokenType::Identifier
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn error_token(&self) -> Token {
        Token {
            token_type: TokenType::Error,
            start: 0,
            length: 0,
            line: self.line,
        }
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}