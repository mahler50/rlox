use crate::{
    ast::{
        expr::{self, Expr},
        stmt::{self, Stmt},
    },
    environment::Environment,
    error::RloxError,
    token::{LiteralType, Token, TokenType},
    value::LoxValue,
};

#[derive(Debug, Default)]
pub struct Interpreter {
    pub had_error: bool,
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            had_error: false,
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: Stmt) {
        self.had_error = false;
        if let Stmt::Program(_) = program {
            if let Err(e) = program.accept(self) {
                eprintln!("{}", e);
            }
        } else {
            println!("Input is not a valid program!");
            self.had_error = true;
        }
    }
}

/// Visitor for expression.
impl expr::Visitor<Result<LoxValue, RloxError>> for Interpreter {
    fn visit_assignment_expr(&mut self, name: &Token, value: &Expr) -> Result<LoxValue, RloxError> {
        let value = value.accept(self)?;
        self.environment.assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_literal(&mut self, value: &LiteralType) -> Result<LoxValue, RloxError> {
        Ok(match value {
            LiteralType::String(s) => LoxValue::String(s.clone()),
            LiteralType::Number(num) => LoxValue::Number(*num),
            LiteralType::Bool(b) => LoxValue::Bool(*b),
            LiteralType::Nil => LoxValue::Nil,
        })
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Result<LoxValue, RloxError> {
        expression.accept(self)
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<LoxValue, RloxError> {
        let right = right.accept(self)?;
        match operator.token_type {
            TokenType::Minus => {
                if let LoxValue::Number(n) = right {
                    Ok(LoxValue::Number(-n))
                } else {
                    Err(RloxError::RuntimeError(
                        "Operand must be a number".to_owned(),
                    ))
                }
            }
            TokenType::Bang => Ok(LoxValue::Bool(!right.is_truthy())),
            _ => Err(RloxError::RuntimeError(
                "Unknown unary operator.".to_owned(),
            )),
        }
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<LoxValue, RloxError> {
        let lhs = left.accept(self)?;
        let rhs = right.accept(self)?;

        match operator.token_type {
            TokenType::Plus => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => Ok(LoxValue::Number(lhs + rhs)),
                (LoxValue::String(lhs), LoxValue::String(rhs)) => {
                    Ok(LoxValue::String(format!("{}{}", lhs, rhs)))
                }
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers or strings.".to_owned(),
                )),
            },
            TokenType::Minus => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => Ok(LoxValue::Number(lhs - rhs)),
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers.".to_owned(),
                )),
            },
            TokenType::Star => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => Ok(LoxValue::Number(lhs * rhs)),
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers.".to_owned(),
                )),
            },
            TokenType::Slash => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => {
                    if rhs == 0.0 {
                        Err(RloxError::RuntimeError(
                            "can not divided by zero".to_owned(),
                        ))
                    } else {
                        Ok(LoxValue::Number(lhs / rhs))
                    }
                }
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers.".to_owned(),
                )),
            },
            TokenType::Greater => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => Ok(LoxValue::Bool(lhs > rhs)),
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers.".to_owned(),
                )),
            },
            TokenType::GreaterEqual => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => Ok(LoxValue::Bool(lhs >= rhs)),
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers.".to_owned(),
                )),
            },
            TokenType::Less => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => Ok(LoxValue::Bool(lhs < rhs)),
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers.".to_owned(),
                )),
            },
            TokenType::LessEqual => match (lhs, rhs) {
                (LoxValue::Number(lhs), LoxValue::Number(rhs)) => Ok(LoxValue::Bool(lhs <= rhs)),
                _ => Err(RloxError::RuntimeError(
                    "Operands must be two numbers.".to_owned(),
                )),
            },
            TokenType::EqualEqual => Ok(LoxValue::Bool(lhs == rhs)),
            TokenType::BangEqual => Ok(LoxValue::Bool(lhs != rhs)),
            _ => unimplemented!("operator: {:?} not support yet", operator),
        }
    }

    fn visit_ternary(
        &mut self,
        condition: &Expr,
        truepart: &Expr,
        falsepart: &Expr,
    ) -> Result<LoxValue, RloxError> {
        let condition = condition.accept(self)?;
        match condition {
            LoxValue::Bool(b) => {
                if b {
                    truepart.accept(self)
                } else {
                    falsepart.accept(self)
                }
            }
            _ => Err(RloxError::RuntimeError(
                "invalid ternary expression".to_owned(),
            )),
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<LoxValue, RloxError> {
        self.environment.get(name)
    }
}

/// Visitor for statement.
impl stmt::Visitor<Result<(), RloxError>> for Interpreter {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), RloxError> {
        self.environment.enter_scope();
        for stmt in statements {
            stmt.accept(self)?;
        }
        self.environment.exit_scope();
        Ok(())
    }

    fn visit_program_stmt(&mut self, declarations: &[Stmt]) -> Result<(), RloxError> {
        for statement in declarations {
            statement.accept(self)?;
        }
        Ok(())
    }

    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
    ) -> Result<(), RloxError> {
        let mut value = LoxValue::Nil;
        if let Some(expr) = initializer {
            value = expr.accept(self)?;
        }
        self.environment.define(&name.lexeme, value);
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), RloxError> {
        expression.accept(self)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), RloxError> {
        let value = expression.accept(self)?;
        println!("{}", value);
        Ok(())
    }
}
