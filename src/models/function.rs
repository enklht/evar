use super::{Context, EvalError, Expr, Value};
use std::rc::Rc;

#[derive(Clone)]
pub struct Function(Rc<FunctionInner>);

impl Function {
    pub fn call(&self, args: Vec<Value>, context: &mut Context) -> Result<Value, EvalError> {
        self.0.call(args, context)
    }

    pub fn new_internal(arg_names: Vec<String>, body: Expr) -> Function {
        Function(Rc::new(FunctionInner::Internal {
            arity: arg_names.len(),
            arg_names,
            body,
        }))
    }

    pub fn new_external(
        arity: usize,
        body: fn(Vec<Value>) -> Result<Value, EvalError>,
    ) -> Function {
        Function(Rc::new(FunctionInner::External { arity, body }))
    }

    pub fn is_external(&self) -> bool {
        matches!(&*self.0, FunctionInner::External { arity: _, body: _ })
    }
}

enum FunctionInner {
    External {
        arity: usize,
        body: fn(Vec<Value>) -> Result<Value, EvalError>,
    },
    Internal {
        arity: usize,
        arg_names: Vec<String>,
        body: Expr,
    },
}

impl FunctionInner {
    pub fn call(&self, args: Vec<Value>, context: &mut Context) -> Result<Value, EvalError> {
        match self {
            FunctionInner::External { arity, body } => {
                if args.len() == *arity {
                    body(args)
                } else {
                    Err(EvalError::InvalidNumberOfArguments(*arity, args.len()))
                }
            }
            FunctionInner::Internal {
                arity,
                arg_names,
                body,
            } => {
                if args.len() == *arity {
                    context.extend();

                    for (arg_name, arg) in arg_names.iter().zip(args.into_iter()) {
                        context.set_variable(arg_name, arg);
                    }
                    body.eval(context)
                } else {
                    Err(EvalError::InvalidNumberOfArguments(*arity, args.len()))
                }
            }
        }
    }
}
