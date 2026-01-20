use crate::token::{LiteralType, Token};

/// Enum of lox's expression.
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralType,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Ternary {
        condition: Box<Expr>,
        expr1: Box<Expr>,
        expr2: Box<Expr>,
    },
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, expression: &Expr) -> T;
    fn visit_literal(&mut self, value: &LiteralType) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_ternary(&mut self, condition: &Expr, expr1: &Expr, expr2: &Expr) -> T;
}

impl Expr {
    pub fn accept<T, V>(&self, visitor: &mut V) -> T
    where
        V: Visitor<T>,
    {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Ternary {
                condition,
                expr1,
                expr2,
            } => visitor.visit_ternary(condition, expr1, expr2),
        }
    }
}
