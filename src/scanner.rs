use std::{collections::HashMap, sync::LazyLock};

use crate::{
    error::{RloxError, report},
    token::{LiteralType, Token, TokenType},
};

// Lzay init keywords map.
static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut keywords = HashMap::with_capacity(30);
    keywords.insert("and", TokenType::And);
    keywords.insert("class", TokenType::Class);
    keywords.insert("else", TokenType::Else);
    keywords.insert("false", TokenType::False);
    keywords.insert("for", TokenType::For);
    keywords.insert("fun", TokenType::Fun);
    keywords.insert("if", TokenType::If);
    keywords.insert("nil", TokenType::Nil);
    keywords.insert("or", TokenType::Or);
    keywords.insert("print", TokenType::Print);
    keywords.insert("return", TokenType::Return);
    keywords.insert("super", TokenType::Super);
    keywords.insert("this", TokenType::This);
    keywords.insert("true", TokenType::True);
    keywords.insert("var", TokenType::Var);
    keywords.insert("while", TokenType::While);

    keywords
});

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub had_error: bool,
}

impl Scanner {
    /// Create a sacnner instance.
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, RloxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        if self.had_error {
            return Err(RloxError::ScannerError);
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            LiteralType::Nil,
            self.line,
        ));

        Ok(self.tokens.to_owned())
    }

    /// Whether is at the end of file.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let b = self.advance();
        match b {
            // Single character tokens.
            b'(' => self.add_token(TokenType::LeftParen, LiteralType::Nil),
            b')' => self.add_token(TokenType::RightParen, LiteralType::Nil),
            b'{' => self.add_token(TokenType::LeftBrace, LiteralType::Nil),
            b'}' => self.add_token(TokenType::RightBrace, LiteralType::Nil),
            b',' => self.add_token(TokenType::Comma, LiteralType::Nil),
            b'.' => self.add_token(TokenType::Dot, LiteralType::Nil),
            b'-' => self.add_token(TokenType::Minus, LiteralType::Nil),
            b'+' => self.add_token(TokenType::Plus, LiteralType::Nil),
            b';' => self.add_token(TokenType::Semicolon, LiteralType::Nil),
            b'*' => self.add_token(TokenType::Star, LiteralType::Nil),

            // One or two characters tokens.
            b'!' if self.r#match(b'=') => self.add_token(TokenType::BangEqual, LiteralType::Nil),
            b'!' => self.add_token(TokenType::Bang, LiteralType::Nil),
            b'=' if self.r#match(b'=') => self.add_token(TokenType::EqualEqual, LiteralType::Nil),
            b'=' => self.add_token(TokenType::Equal, LiteralType::Nil),
            b'<' if self.r#match(b'=') => self.add_token(TokenType::LessEqual, LiteralType::Nil),
            b'<' => self.add_token(TokenType::Less, LiteralType::Nil),
            b'>' if self.r#match(b'=') => self.add_token(TokenType::GreaterEqual, LiteralType::Nil),
            b'>' => self.add_token(TokenType::Greater, LiteralType::Nil),

            // Ternary.
            b'?' => self.add_token(TokenType::QuestionMark, LiteralType::Nil),
            b':' => self.add_token(TokenType::Colon, LiteralType::Nil),

            // Slash or comment.
            b'/' => {
                if self.r#match(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, LiteralType::Nil);
                }
            }

            // Ignore whitespace.
            b' ' | b'\r' | b'\t' => (),

            // Next line.
            b'\n' => self.line += 1,

            // String.
            b'"' => self.string(),
            _ => {
                if is_digit(b) {
                    self.number();
                } else if is_alpha(b) {
                    self.identifier();
                } else {
                    report(&RloxError::LexicalError(
                        self.line,
                        (b as char).to_string(),
                        "invalid token".to_string(),
                    ))
                }
            }
        }
    }

    /// Advacne current index and return current byte.
    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    /// Add a new token to token list.
    fn add_token(&mut self, token_type: TokenType, literal: LiteralType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    /// Return whether next char matches the expected char.
    fn r#match(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    /// Get current character.
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    // Get next character.
    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }

    /// Handle string token.
    fn string(&mut self) {
        let mut escaped = false;
        while (self.peek() != b'"' || escaped) && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            if self.peek() == b'\\' {
                escaped = !escaped;
            } else {
                escaped = false;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.had_error = true;
            report(&RloxError::LexicalError(
                self.line,
                self.source[self.start..self.current].to_string(),
                "unterminated string".to_string(),
            ));
            return;
        }

        // The closing '"'.
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, LiteralType::String(value));
    }

    /// Handle number token.
    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == b'.' && is_digit(self.peek_next()) {
            // Consume the '.'.
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        // Make sure number does not have trailing alpha characters.
        if is_alpha(self.peek()) {
            while is_alpha_numeric(self.peek()) {
                self.advance();
            }
            self.had_error = true;
            report(&RloxError::LexicalError(
                self.line,
                self.source[self.start..self.current].to_string(),
                "invalid number".to_string(),
            ));
        }

        match self.source[self.start..self.current].parse::<f64>() {
            Ok(number) => self.add_token(TokenType::Number, LiteralType::Number(number)),
            Err(_) => {
                self.had_error = true;
                report(&RloxError::LexicalError(
                    self.line,
                    self.source[self.start..self.current].to_string(),
                    "invalid number".to_string(),
                ));
            }
        }
    }

    /// Handle keyword and identifier token.
    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if let Some(&token_type) = KEYWORDS.get(text) {
            self.add_token(token_type, LiteralType::Nil);
        } else {
            self.add_token(TokenType::Identifier, LiteralType::Nil);
        }
    }
}

fn is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

fn is_alpha(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_uppercase() || b == b'_'
}

fn is_alpha_numeric(b: u8) -> bool {
    is_digit(b) || is_alpha(b)
}
