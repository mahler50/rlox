use crate::{
    ast::{
        expr::{self, Expr},
        stmt::{self, Stmt},
    },
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

    /// Wrapping statements in block.
    fn block(&mut self, statements: &[Stmt]) -> String {
        let mut s = String::new();
        s.push('[');
        for stmt in statements {
            s.push_str(&stmt.accept(self));
            s.push(';');
        }
        if s.len() > 1 {
            s.pop();
        }
        s.push(']');
        s
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_assignment_expr(&mut self, name: &crate::token::Token, value: &Expr) -> String {
        let mut s = String::new();
        s.push_str("(= ");
        s.push_str(&name.lexeme);
        s.push(' ');
        s.push_str(&value.accept(self));
        s.push(')');
        s
    }

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

    fn visit_ternary(&mut self, condition: &Expr, truepart: &Expr, falsepart: &Expr) -> String {
        self.parenthesize("?", vec![condition, truepart, falsepart])
    }

    fn visit_variable(&mut self, name: &crate::token::Token) -> String {
        name.lexeme.clone()
    }
}

impl stmt::Visitor<String> for AstPrinter {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> String {
        self.block(statements)
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) -> String {
        expression.accept(self)
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> String {
        self.parenthesize("print", vec![expression])
    }

    fn visit_program_stmt(&mut self, declarations: &[stmt::Stmt]) -> String {
        self.block(declarations)
    }

    fn visit_var_stmt(&mut self, name: &crate::token::Token, initializer: &Option<Expr>) -> String {
        let mut s = String::new();
        s.push_str("(var ");
        s.push_str(&name.lexeme);
        if let Some(expr) = initializer {
            s.push_str(" = ");
            s.push_str(&expr.accept(self));
        }
        s.push(')');

        s
    }
}
