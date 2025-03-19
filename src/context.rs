use std::collections::HashMap;

use crate::{args::Args, errors::EvalError, types::Expr};

pub struct Context {
    functions: HashMap<String, Function>,
    variables: HashMap<String, Variable>,
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

macro_rules! binary_fn {
    ($fname:ident) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 2,
                body: |x| x.first().unwrap().$fname(*x.get(1).unwrap()),
            },
        )
    };
    ($fname:ident, $body:expr) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 2,
                body: $body,
            },
        )
    };
}

impl Context {
    pub fn new(args: &Args) -> Context {
        use crate::args::AngleUnit;

        let mut functions = HashMap::from(match args.angle_unit {
            AngleUnit::Radian => [
                unary_fn!(sin),
                unary_fn!(cos),
                unary_fn!(tan),
                unary_fn!(sec, |x| x.first().unwrap().sin().recip()),
                unary_fn!(csc, |x| x.first().unwrap().cos().recip()),
                unary_fn!(cot, |x| x.first().unwrap().tan().recip()),
                unary_fn!(asin),
                unary_fn!(acos),
                unary_fn!(atan),
            ],
            AngleUnit::Degree => [
                unary_fn!(sin, |x| x.first().unwrap().to_radians().sin()),
                unary_fn!(cos, |x| x.first().unwrap().to_radians().cos()),
                unary_fn!(tan, |x| x.first().unwrap().to_radians().tan()),
                unary_fn!(sec, |x| x.first().unwrap().to_radians().sin().recip()),
                unary_fn!(csc, |x| x.first().unwrap().to_radians().cos().recip()),
                unary_fn!(cot, |x| x.first().unwrap().to_radians().tan().recip()),
                unary_fn!(asin, |x| x.first().unwrap().asin().to_degrees()),
                unary_fn!(acos, |x| x.first().unwrap().acos().to_degrees()),
                unary_fn!(atan, |x| x.first().unwrap().atan().to_degrees()),
            ],
        });

        for (fname, function) in [
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
            binary_fn!(log),
            binary_fn!(nroot, |x| x
                .first()
                .unwrap()
                .powf(x.get(1).unwrap().recip())),
        ] {
            functions.insert(fname, function);
        }

        let variables = [
            ("e".to_string(), Variable::External(std::f64::consts::E)),
            ("pi".to_string(), Variable::External(std::f64::consts::PI)),
            ("tau".to_string(), Variable::External(std::f64::consts::TAU)),
        ]
        .into();

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
        use Variable::*;
        use std::collections::hash_map::Entry::*;

        match self.variables.entry(name.to_string()) {
            Occupied(mut e) => match e.get() {
                External(_) => return None,
                Internal(_) => {
                    e.insert(Variable::Internal(n));
                }
            },
            Vacant(e) => {
                e.insert(Variable::Internal(n));
            }
        }
        Some(n)
    }
}

pub enum Variable {
    External(f64),
    Internal(f64),
}

impl Variable {
    pub fn get(&self) -> f64 {
        use Variable::*;
        match self {
            External(n) => *n,
            Internal(n) => *n,
        }
    }
}

pub enum Function {
    External {
        arity: usize,
        body: fn(Vec<f64>) -> f64,
    },
    Internal {
        arity: usize,
        args: Vec<String>,
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
                    Err(EvalError::InvalidNumberOfArguments {
                        expected: *arity,
                        found: args.len(),
                    })
                }
            }
            Function::Internal {
                arity: _,
                args: _,
                body: _,
            } => {
                todo!()
            }
        }
    }
}
