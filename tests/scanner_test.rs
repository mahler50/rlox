use rlox::{
    scanner::Scanner,
    token::{LiteralType, Token, TokenType},
};

fn scan(source: &str) -> Vec<Token> {
    let mut scanner = Scanner::new(source.to_owned());
    scanner.scan_tokens().unwrap()
}

#[test]
fn test_empty_source() {
    let tokens = scan("");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Eof);
}

#[test]
fn test_unterminated_string_with_escape() {
    let mut scanner = Scanner::new("\"unterminated\\n".to_string());
    let result = scanner.scan_tokens();
    assert!(result.is_err());
}

#[test]
fn test_string_with_escape_sequences() {
    let tokens = scan("\"hello \\\"world\\\"\"");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(
        tokens[0].literal,
        LiteralType::String("hello \\\"world\\\"".to_string())
    );
}

#[test]
fn test_multiline_string() {
    let tokens = scan("\"hello\nworld\"");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(
        tokens[0].literal,
        LiteralType::String("hello\nworld".to_string())
    );
}

#[test]
fn test_numbers_with_leading_zeros() {
    let tokens = scan("007 0.123");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].literal, LiteralType::Number(7.0));
    assert_eq!(tokens[1].token_type, TokenType::Number);
    assert_eq!(tokens[1].literal, LiteralType::Number(0.123));
}

#[test]
fn test_invalid_number() {
    let mut scanner = Scanner::new("123abc".to_string());
    let result = scanner.scan_tokens();
    assert!(result.is_err());
}

#[test]
fn test_identifier_with_underscore() {
    let tokens = scan("foo_bar");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[0].lexeme, "foo_bar");
}

#[test]
fn test_keywords_with_identifiers() {
    let tokens = scan("var variable = true;");
    let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();
    assert_eq!(
        token_types,
        vec![
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::True,
            TokenType::Semicolon,
            TokenType::Eof,
        ]
    );
}

#[test]
fn test_unterminated_comment() {
    let tokens = scan("// this is a comment");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Eof);
}

#[test]
fn test_nested_comments() {
    let tokens = scan("// outer comment // inner comment\nvar x = 10;");
    let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();
    assert_eq!(
        token_types,
        vec![
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::Eof,
        ]
    );
}

#[test]
fn test_complex_expression() {
    let tokens = scan("if (a > 10) { print a + 1; } else { print \"done\"; }");
    let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();
    assert_eq!(
        token_types,
        vec![
            TokenType::If,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::Greater,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Print,
            TokenType::Identifier,
            TokenType::Plus,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::Else,
            TokenType::LeftBrace,
            TokenType::Print,
            TokenType::String,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::Eof,
        ]
    );
}

#[test]
fn test_ternary_operator() {
    let tokens = scan("var c = a == b ? 1 : 2");
    let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type).collect();
    assert_eq!(
        token_types,
        vec![
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::EqualEqual,
            TokenType::Identifier,
            TokenType::QuestionMark,
            TokenType::Number,
            TokenType::Colon,
            TokenType::Number,
            TokenType::Eof
        ]
    );
}
