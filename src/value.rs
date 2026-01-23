use std::fmt::Display;

/// Lox builtin value types.
#[derive(Debug, PartialEq, Clone)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LoxValue::Nil => false,
            LoxValue::Bool(b) => *b,
            _ => true,
        }
    }
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::Nil => write!(f, "nil"),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::Number(num) => write!(f, "{}", num),
            LoxValue::String(s) => write!(f, "{}", s),
        }
    }
}
