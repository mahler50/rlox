use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RloxError, token::Token, value::LoxValue};

#[derive(Debug, Default)]
pub struct EnvInner {
    parent: Option<Rc<RefCell<EnvInner>>>,
    table: HashMap<String, LoxValue>,
}

impl EnvInner {
    pub fn from_parent(parent: Rc<RefCell<EnvInner>>) -> Self {
        Self {
            parent: Some(parent),
            table: HashMap::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Environment {
    value: Rc<RefCell<EnvInner>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            value: Rc::new(RefCell::new(EnvInner::default())),
        }
    }

    /// Define a variable in current scope.
    pub fn define(&mut self, name: &str, value: LoxValue) {
        self.value
            .borrow_mut()
            .table
            .insert(name.to_string(), value);
    }

    /// Get identifier's value or if identifier does not exist, throw a runtime error.
    pub fn get(&self, name: &Token) -> Result<LoxValue, RloxError> {
        match Self::recursive_get(&self.value, name) {
            Some(value) => Ok(value),
            None => Err(RloxError::RuntimeError(format!(
                "Undefined variable: {}.",
                name.lexeme
            ))),
        }
    }

    fn recursive_get(environment: &Rc<RefCell<EnvInner>>, name: &Token) -> Option<LoxValue> {
        let environment = environment.borrow();
        if let Some(value) = environment.table.get(&name.lexeme) {
            Some(value.clone())
        } else if let Some(enclosing) = &environment.parent {
            Self::recursive_get(enclosing, name)
        } else {
            None
        }
    }

    /// Assign a variable with new value.
    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RloxError> {
        let mut current = Rc::clone(&self.value);
        loop {
            if let Some(old_value) = current.borrow_mut().table.get_mut(&name.lexeme) {
                *old_value = value;
                return Ok(());
            }
            let outer;
            if let Some(parent) = &current.borrow().parent {
                outer = Rc::clone(parent);
            } else {
                break;
            }
            current = outer;
        }

        Err(RloxError::RuntimeError(format!(
            "Can't assign undefined variable: {}.",
            name.lexeme
        )))
    }

    /// Enter a new inner scope.
    pub fn enter_scope(&mut self) {
        self.value = Rc::new(RefCell::new(EnvInner {
            parent: Some(Rc::clone(&self.value)),
            table: HashMap::default(),
        }));
    }

    /// Exit current scope or panic if trying to exit global scope.
    pub fn exit_scope(&mut self) {
        let parent = self.value.borrow_mut().parent.clone();
        if let Some(parent) = parent {
            self.value = parent;
        } else {
            panic!("Already in the global scope, can't exit.");
        }
    }
}
