use super::{Function, Value, Variable};
use crate::models::Expr;
use std::collections::HashMap;

pub struct Context {
    previous_answer: Option<Value>,
    functions: HashMap<String, Function>,
    variables: Option<Box<VariableContext>>,
}

impl Context {
    pub fn new(
        functions: HashMap<String, Function>,
        variables: HashMap<String, Variable>,
    ) -> Context {
        Context {
            previous_answer: None,
            functions,
            variables: Some(Box::new(VariableContext::new(variables))),
        }
    }

    pub fn extend(&mut self) {
        let variables = self.variables.take();
        self.variables = Some(Box::new(VariableContext {
            parent: variables,
            variables: HashMap::new(),
        }))
    }

    pub fn detach(&mut self) {
        let variables = self.variables.take().unwrap().parent.take();
        self.variables = variables
    }

    pub fn get_variable(&self, name: &str) -> Option<Variable> {
        match &self.variables {
            Some(variables) => variables.get_variable(name),
            None => unreachable!(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) -> Option<Value> {
        match &mut self.variables {
            Some(variables) => variables.set_variable(name, value),
            None => unreachable!(),
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }

    pub fn set_function(&mut self, name: &str, arg_names: Vec<String>, body: Expr) {
        self.functions
            .insert(name.to_string(), Function::new_internal(arg_names, body));
    }

    pub fn get_prev_answer(&self) -> Option<Value> {
        self.previous_answer.clone()
    }

    pub fn set_prev_answer(&mut self, value: &Value) {
        self.previous_answer = Some(value.clone());
    }
}

pub struct VariableContext {
    parent: Option<Box<VariableContext>>,
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
            parent.get_variable(name)
        } else {
            None
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) -> Option<Value> {
        use super::Variable::*;
        use std::collections::hash_map::Entry::*;

        match self.variables.entry(name.to_string()) {
            Occupied(mut e) => match e.get() {
                External(_) => return None,
                Internal(_) => {
                    e.insert(Internal(value.clone()));
                }
            },
            Vacant(e) => {
                e.insert(Internal(value.clone()));
            }
        }
        Some(value)
    }
}
