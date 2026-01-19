use crate::{
    ast::expr::{self, Expr},
    token::LiteralType,
};

pub struct AstPrinter();

impl AstPrinter {
    /// Format a given `Expr` to a readable string.
    pub fn fmt(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    /// Parenthesize the given expression list.
    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut s = String::new();
        s.push('(');
        s.push_str(name);
        for expr in exprs {
            s.push(' ');
            s.push_str(&expr.accept(self));
        }
        s.push(')');

        s
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(
        &mut self,
        left: &expr::Expr,
        operator: &crate::token::Token,
        right: &expr::Expr,
    ) -> String {
        self.parenthesize(&operator.lexeme, vec![left, right])
    }

    fn visit_grouping(&mut self, expression: &expr::Expr) -> String {
        self.parenthesize("group", vec![expression])
    }

    fn visit_literal(&mut self, value: &crate::token::LiteralType) -> String {
        match value {
            LiteralType::Number(n) => n.to_string(),
            LiteralType::String(s) => s.to_owned(),
            LiteralType::Bool(b) => b.to_string(),
            LiteralType::Nil => "nil".to_string(),
        }
    }

    fn visit_unary(&mut self, operator: &crate::token::Token, right: &expr::Expr) -> String {
        self.parenthesize(&operator.lexeme, vec![right])
    }
}
