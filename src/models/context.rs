use super::{Function, Variable};
use crate::models::Expr;
use std::collections::HashMap;

pub struct Context {
    functions: HashMap<String, Function>,
    variables: HashMap<String, Variable>,
}

impl Context {
    pub fn new(
        functions: HashMap<String, Function>,
        variables: HashMap<String, Variable>,
    ) -> Context {
        Context {
            functions,
            variables,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }

    pub fn set_function(&mut self, name: &str, args: Vec<String>, body: Expr) -> Option<()> {
        self.functions.insert(
            name.to_string(),
            Function::Internal {
                arity: args.len(),
                args,
                body,
            },
        );
        Some(())
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
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
}
