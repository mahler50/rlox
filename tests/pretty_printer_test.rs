use rlox::{
    ast::{expr::Expr, pretty_printer::AstPrinter},
    token::{LiteralType, Token, TokenType},
};

#[test]
fn test_pretty_printer() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), LiteralType::Nil, 1),
            right: Box::new(Expr::Literal {
                value: LiteralType::Number(123.0),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), LiteralType::Nil, 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralType::Number(45.67),
            }),
        }),
    };
    let mut printer = AstPrinter();

    assert_eq!("(* (- 123) (group 45.67))", printer.fmt(&expr));
}

#[test]
fn test_ternary_operator() {
    let expr = Expr::Ternary {
        condition: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralType::Number(1.0),
            }),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), LiteralType::Nil, 1),
            right: Box::new(Expr::Literal {
                value: LiteralType::Number(2.0),
            }),
        }),
        expr1: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralType::Number(1.0),
            }),
            operator: Token::new(TokenType::Plus, "+".to_string(), LiteralType::Nil, 1),
            right: Box::new(Expr::Literal {
                value: LiteralType::Number(2.0),
            }),
        }),
        expr2: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: LiteralType::Number(3.0),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), LiteralType::Nil, 1),
            right: Box::new(Expr::Literal {
                value: LiteralType::Number(4.0),
            }),
        }),
    };
    let mut printer = AstPrinter();

    assert_eq!("(? (== 1 2) (+ 1 2) (* 3 4))", printer.fmt(&expr));
}
