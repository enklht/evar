use std::{cell::RefCell, rc::Rc};

use super::{EvalError, Expr, FunctionContext, Value, VariableContext};

pub enum Function {
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

impl Function {
    pub fn call(
        &self,
        args: Vec<Value>,
        fcontext: &FunctionContext,
        vcontext: Rc<RefCell<VariableContext>>,
    ) -> Result<Value, EvalError> {
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
                arity,
                arg_names,
                body,
            } => {
                if args.len() == *arity {
                    let mut vcontext = VariableContext::extend(vcontext);

                    for (arg_name, arg) in arg_names.iter().zip(args.into_iter()) {
                        vcontext.set_variable(arg_name, arg);
                    }
                    body.eval(fcontext, Rc::new(RefCell::new(vcontext)))
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
