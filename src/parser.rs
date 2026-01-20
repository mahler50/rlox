use unescape::unescape;

use crate::ast::expr::Expr;
use crate::error::{RloxError, report};
use crate::token::{LiteralType, Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }
}

/// Methods for parsing tokens.
impl Parser {
    fn expression(&mut self) -> Result<Expr, RloxError> {
        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::QuestionMark]) {
            let expr1 = self.expression()?;
            self.consume(TokenType::Colon, "Expect ':' after '?' in ternary operator")?;
            let expr2 = self.expression()?;
            expr = Expr::Ternary {
                condition: Box::new(expr),
                expr1: Box::new(expr1),
                expr2: Box::new(expr2),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RloxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, RloxError> {
        match self.peek().token_type {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralType::Bool(false),
                })
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralType::Bool(true),
                })
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralType::Nil,
                })
            }
            TokenType::Number => {
                let lexeme = &self.peek().lexeme;
                let number = lexeme.as_str().parse::<f64>().unwrap();
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralType::Number(number),
                })
            }
            TokenType::String => {
                let lexeme = self.peek().lexeme.clone();
                let lexeme = lexeme[1..lexeme.len() - 1].to_string();
                match unescape(&lexeme) {
                    Some(unescaped) => {
                        self.advance();
                        Ok(Expr::Literal {
                            value: LiteralType::String(unescaped),
                        })
                    }
                    None => {
                        report(&RloxError::LexicalError(
                            self.peek().line,
                            "Invalid escape string sequence".to_string(),
                            lexeme.clone(),
                        ));
                        self.advance();
                        self.had_error = true;
                        Ok(Expr::Literal {
                            value: LiteralType::String(lexeme),
                        })
                    }
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => Err(self.error(&format!(
                "Unexpected token type: {:?}.",
                self.peek().token_type
            ))),
        }
    }
}

/// Helper methods for parsing.
impl Parser {
    /// Return whether current token matches expected token types.
    fn matches(&mut self, types: &[TokenType]) -> bool {
        for &type_ in types {
            if self.check(type_) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Check whether current token matches expected token type.
    /// If is at end of token stream, return false.
    fn check(&self, type_: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == type_
        }
    }

    /// Advance cursor and return current token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Return whether is at end of token stream.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Return current token.
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Return previous token.
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Consume an expected token or raise an error.
    fn consume(&mut self, type_: TokenType, message: &str) -> Result<&Token, RloxError> {
        if self.check(type_) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    /// Return a syntax error.
    fn error(&mut self, message: &str) -> RloxError {
        self.had_error = true;
        let error = RloxError::SyntaxError(
            self.peek().line,
            self.peek().lexeme.clone(),
            message.to_owned(),
        );
        report(&error);

        error
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.previous().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
        }

        self.advance();
    }
}
