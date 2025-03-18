use std::collections::HashMap;

use crate::{args::Args, errors::EvalError, types::Expr};

pub struct Context {
    functions: HashMap<String, Function>,
}

impl Context {
    pub fn new(args: Args) -> Context {
        let mut functions = [(
            "sin".into(),
            Function::External {
                arity: 1,
                body: |x| x.get(0).unwrap().sin(),
            },
        )]
        .into();
        Context { functions }
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}

pub enum Function {
    External {
        arity: usize,
        body: fn(Vec<f64>) -> f64,
    },
    Internal {
        arity: usize,
        body: Expr,
    },
}

impl Function {
    pub fn call(&self, args: Vec<f64>) -> Result<f64, EvalError> {
        match self {
            Function::External { arity, body } => {
                if args.len() == *arity {
                    Ok(body(args))
                } else {
                    Err(EvalError::InvalidNumberOfArgumentsError {
                        expected: *arity,
                        found: args.len(),
                    })
                }
            }
            Function::Internal { arity, body } => {
                todo!()
            }
        }
    }
}
