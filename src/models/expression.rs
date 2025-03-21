use std::cell::RefCell;
use std::rc::Rc;

use super::{EvalError, Value};
use super::{FunctionContext, VariableContext, operators::*};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i32),
    Float(f64),
    Variable(String),
    FnCall {
        name: String,
        args: Vec<Expr>,
    },
    PrefixOp {
        op: PrefixOp,
        arg: Box<Expr>,
    },
    PostfixOp {
        op: PostfixOp,
        arg: Box<Expr>,
    },
    InfixOp {
        op: InfixOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    PrevAnswer,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Float(n) => write!(f, "{}", n),
            Expr::Variable(n) => write!(f, "{}", n),
            Expr::FnCall { name, args } => {
                let args_str = args
                    .iter()
                    .map(|x| x.to_string())
                    .reduce(|acc, x| acc + ", " + &x)
                    .unwrap_or_else(|| "".into());
                write!(f, "{}({})", name, args_str)
            }
            Expr::PrefixOp { op, arg } => write!(f, "({}{})", op, arg),
            Expr::PostfixOp { op, arg } => write!(f, "({}{})", arg, op),
            Expr::InfixOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::PrevAnswer => write!(f, "_"),
        }
    }
}

impl Expr {
    pub fn eval(
        &self,
        fcontext: &FunctionContext,
        vcontext: Rc<RefCell<VariableContext>>,
    ) -> Result<Value, EvalError> {
        match self {
            Expr::Int(f) => Ok(Value::from(*f)),
            Expr::Float(f) => Ok(Value::from(*f)),
            Expr::InfixOp { op, lhs, rhs } => {
                use InfixOp::*;
                match op {
                    Add => {
                        Ok(lhs.eval(fcontext, vcontext.clone())? + rhs.eval(fcontext, vcontext)?)
                    }
                    _ => unimplemented!(), // Sub => {
                                           //     Ok(lhs.eval(fcontext, vcontext.clone())? - rhs.eval(fcontext, vcontext)?)
                                           // }
                                           // Mul => {
                                           //     Ok(lhs.eval(fcontext, vcontext.clone())? * rhs.eval(fcontext, vcontext)?)
                                           // }
                                           // Div => {
                                           //     Ok(lhs.eval(fcontext, vcontext.clone())? / rhs.eval(fcontext, vcontext)?)
                                           // }
                                           // Rem => Ok(lhs
                                           //     .eval(fcontext, vcontext.clone())?
                                           //     .rem_euclid(rhs.eval(fcontext, vcontext)?)),
                                           // Pow => Ok(lhs
                                           //     .eval(fcontext, vcontext.clone())?
                                           //     .pow(rhs.eval(fcontext, vcontext)?)),
                }
            }
            Expr::PrefixOp { op, arg } => {
                use PrefixOp::*;
                match op {
                    Neg => Ok(-arg.eval(fcontext, vcontext)?),
                }
            }
            Expr::PostfixOp { op, arg } => {
                use PostfixOp::*;
                match op {
                    Fac => arg.eval(fcontext, vcontext)?.factorial(),
                }
            }
            Expr::FnCall { name, args } => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(arg.eval(fcontext, vcontext.clone())?);
                }

                let function = fcontext
                    .get_function(name)
                    .ok_or(EvalError::FunctionNotFound(name.to_string()))?;

                function.call(evaluated_args, fcontext, vcontext)
            }
            Expr::Variable(name) => {
                let variable = vcontext
                    .borrow()
                    .get_variable(name)
                    .ok_or(EvalError::VariableNotFound(name.to_string()))?
                    .get();
                Ok(variable.into())
            }
            Expr::PrevAnswer => fcontext.get_prev_answer().ok_or(EvalError::NoHistory),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        args::{AngleUnit, Args},
        create_context,
    };

    const RADIAN_ARGS: Args = Args {
        angle_unit: AngleUnit::Radian,
        debug: false,
        no_color: false,
    };

    #[test]
    fn test_number() {
        let (fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::Float(42.0);
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), Value::from(42.0));
    }

    #[test]
    fn test_infix_op_add() {
        let (fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::InfixOp {
            op: InfixOp::Add,
            lhs: Box::new(Expr::Float(1.0)),
            rhs: Box::new(Expr::Float(2.0)),
        };
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), Value::from(3.0));
    }

    #[test]
    fn test_prefix_op_neg() {
        let (fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::PrefixOp {
            op: PrefixOp::Neg,
            arg: Box::new(Expr::Float(5.0)),
        };
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), Value::from(-5.0));
    }

    #[test]
    fn test_postfix_op_fac() {
        let (fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::PostfixOp {
            op: PostfixOp::Fac,
            arg: Box::new(Expr::Float(5.0)),
        };
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), Value::from(120.0));
    }

    #[test]
    fn test_fn_call() {
        let (mut fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::FnCall {
            name: "mock_fn".to_string(),
            args: vec![Expr::Float(2.0), Expr::Float(3.0)],
        };
        fcontext.set_function(
            "mock_fn",
            vec!["x".into(), "y".into()],
            Expr::InfixOp {
                op: InfixOp::Add,
                lhs: Expr::Variable("x".into()).into(),
                rhs: Expr::Variable("y".into()).into(),
            },
        );
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), Value::from(5.0));
    }
}
