use crate::ast::expr::Expr;
use crate::token::Token;

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Program(Vec<Stmt>),
    Var(Token, Option<Expr>),
    Expression(Expr),
    Print(Expr),
}

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
    fn visit_program_stmt(&mut self, declarations: &[Stmt]) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
}

impl Stmt {
    pub fn accept<T, V>(&self, visitor: &mut V) -> T
    where
        V: Visitor<T>,
    {
        match self {
            Stmt::Block(statements) => visitor.visit_block_stmt(statements),
            Stmt::Expression(expression) => visitor.visit_expression_stmt(expression),
            Stmt::Print(expression) => visitor.visit_print_stmt(expression),
            Stmt::Program(declarations) => visitor.visit_program_stmt(declarations),
            Stmt::Var(name, initializer) => visitor.visit_var_stmt(name, initializer),
        }
    }
}
