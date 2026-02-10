use crate::token::{LiteralType, Token};

/// Enum of lox's expression.
#[derive(Debug)]
pub enum Expr {
    Assignment {
        name: Token,
        value: Box<Expr>,
    },
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
        truepart: Box<Expr>,
        falsepart: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

pub trait Visitor<T> {
    fn visit_assignment_expr(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, expression: &Expr) -> T;
    fn visit_literal(&mut self, value: &LiteralType) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_ternary(&mut self, condition: &Expr, truepart: &Expr, falsepart: &Expr) -> T;
    fn visit_variable(&mut self, name: &Token) -> T;
}

impl Expr {
    pub fn accept<T, V>(&self, visitor: &mut V) -> T
    where
        V: Visitor<T>,
    {
        match self {
            Expr::Assignment { name, value } => visitor.visit_assignment_expr(name, value),
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
                truepart,
                falsepart,
            } => visitor.visit_ternary(condition, truepart, falsepart),
            Expr::Variable { name } => visitor.visit_variable(name),
        }
    }
}
