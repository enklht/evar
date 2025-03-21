use super::{Context, EvalError, Expr, Value};
use std::rc::Rc;

#[derive(Clone)]
pub struct Function(Rc<FunctionInner>);

impl Function {
    pub fn call(&self, args: Vec<Value>, fcontext: &mut Context) -> Result<Value, EvalError> {
        self.0.call(args, fcontext)
    }

    pub fn new_internal(arg_names: Vec<String>, body: Expr) -> Function {
        Function(Rc::new(FunctionInner::Internal {
            arity: arg_names.len(),
            arg_names,
            body,
        }))
    }

    pub fn new_external(arity: usize, body: fn(Vec<Value>) -> Value) -> Function {
        Function(Rc::new(FunctionInner::External { arity, body }))
    }
}

pub enum FunctionInner {
    External {
        arity: usize,
        body: fn(Vec<Value>) -> Value,
    },
    Internal {
        arity: usize,
        arg_names: Vec<String>,
        body: Expr,
    },
}

impl FunctionInner {
    pub fn call(&self, args: Vec<Value>, fcontext: &mut Context) -> Result<Value, EvalError> {
        match self {
            FunctionInner::External { arity, body } => {
                if args.len() == *arity {
                    Ok(body(args))
                } else {
                    Err(EvalError::InvalidNumberOfArguments {
                        expected: *arity,
                        found: args.len(),
                    })
                }
            }
            FunctionInner::Internal {
                arity,
                arg_names,
                body,
            } => {
                if args.len() == *arity {
                    fcontext.extend();

                    for (arg_name, arg) in arg_names.iter().zip(args.into_iter()) {
                        fcontext.set_variable(arg_name, arg);
                    }
                    body.eval(fcontext)
                } else {
                    Err(EvalError::InvalidNumberOfArguments {
                        expected: *arity,
                        found: args.len(),
                    })
                }
            }
        }
    }
}
