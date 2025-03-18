use std::collections::HashMap;

use crate::{args::Args, errors::EvalError, types::Expr};

pub struct Context {
    functions: HashMap<String, Function>,
}

macro_rules! unary_fn {
    ($fname:ident) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 1,
                body: |x| x.first().unwrap().$fname(),
            },
        )
    };
    ($fname:ident, $body:expr) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 1,
                body: $body,
            },
        )
    };
}

impl Context {
    pub fn new(args: Args) -> Context {
        let functions = [
            unary_fn!(sin),
            unary_fn!(cos),
            unary_fn!(tan),
            unary_fn!(sec, |x| x.first().unwrap().sin().recip()),
            unary_fn!(csc, |x| x.first().unwrap().cos().recip()),
            unary_fn!(cot, |x| x.first().unwrap().tan().recip()),
            unary_fn!(asin),
            unary_fn!(acos),
            unary_fn!(atan),
            unary_fn!(sinh),
            unary_fn!(cosh),
            unary_fn!(tanh),
            unary_fn!(floor),
            unary_fn!(ceil),
            unary_fn!(round),
            unary_fn!(abs),
            unary_fn!(sqrt),
            unary_fn!(exp),
            unary_fn!(exp2),
            unary_fn!(ln),
            unary_fn!(log2),
            unary_fn!(log10),
            unary_fn!(rad, |x| x.first().unwrap().to_radians()),
            unary_fn!(deg, |x| x.first().unwrap().to_degrees()),
        ]
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
