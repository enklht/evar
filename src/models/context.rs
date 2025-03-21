use super::{Function, Variable};
use crate::models::Expr;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct FunctionContext {
    previous_answer: Option<f64>,
    functions: HashMap<String, Function>,
}

impl FunctionContext {
    pub fn new(functions: HashMap<String, Function>) -> FunctionContext {
        FunctionContext {
            previous_answer: None,
            functions,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }

    pub fn set_function(&mut self, name: &str, arg_names: Vec<String>, body: Expr) {
        self.functions.insert(
            name.to_string(),
            Function::Internal {
                arity: arg_names.len(),
                arg_names,
                body,
            },
        );
    }

    pub fn get_prev_answer(&self) -> Option<f64> {
        self.previous_answer
    }

    pub fn set_prev_answer(&mut self, value: f64) {
        self.previous_answer = Some(value);
    }
}

pub struct VariableContext {
    parent: Option<Rc<RefCell<VariableContext>>>,
    variables: HashMap<String, Variable>,
}

impl VariableContext {
    pub fn new(variables: HashMap<String, Variable>) -> VariableContext {
        VariableContext {
            parent: None,
            variables,
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Variable> {
        if let Some(val) = self.variables.get(name) {
            Some(val.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_variable(name)
        } else {
            None
        }
    }

    pub fn set_variable(&mut self, name: &str, n: f64) -> Option<f64> {
        use super::Variable::*;
        use std::collections::hash_map::Entry::*;

        match self.variables.entry(name.to_string()) {
            Occupied(mut e) => match e.get() {
                External(_) => return None,
                Internal(_) => {
                    e.insert(Internal(n));
                }
            },
            Vacant(e) => {
                e.insert(Internal(n));
            }
        }
        Some(n)
    }

    pub fn extend(parent: Rc<RefCell<VariableContext>>) -> VariableContext {
        VariableContext {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }
}
