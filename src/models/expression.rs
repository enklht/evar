use std::cell::RefCell;
use std::rc::Rc;

use super::EvalError;
use super::{FunctionContext, VariableContext, operators::*};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
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
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Variable(n) => write!(f, "{}", n),
            Expr::FnCall { name: fname, args } => {
                let args_str = args
                    .iter()
                    .map(|x| x.to_string())
                    .reduce(|acc, x| acc + ", " + &x)
                    .unwrap_or_else(|| "".into());
                write!(f, "{}({})", fname, args_str)
            }
            Expr::PrefixOp { op, arg } => write!(f, "({}{})", op, arg),
            Expr::PostfixOp { op, arg } => write!(f, "({}{})", arg, op),
            Expr::InfixOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

impl Expr {
    pub fn eval(
        &self,
        fcontext: &FunctionContext,
        vcontext: Rc<RefCell<VariableContext>>,
    ) -> Result<f64, EvalError> {
        match self {
            Expr::Number(f) => Ok(*f),
            Expr::InfixOp { op, lhs, rhs } => {
                use InfixOp::*;
                match op {
                    Add => {
                        Ok(lhs.eval(fcontext, vcontext.clone())? + rhs.eval(fcontext, vcontext)?)
                    }
                    Sub => {
                        Ok(lhs.eval(fcontext, vcontext.clone())? - rhs.eval(fcontext, vcontext)?)
                    }
                    Mul => {
                        Ok(lhs.eval(fcontext, vcontext.clone())? * rhs.eval(fcontext, vcontext)?)
                    }
                    Div => {
                        Ok(lhs.eval(fcontext, vcontext.clone())? / rhs.eval(fcontext, vcontext)?)
                    }
                    Rem => Ok(lhs
                        .eval(fcontext, vcontext.clone())?
                        .rem_euclid(rhs.eval(fcontext, vcontext)?)),
                    Pow => Ok(lhs
                        .eval(fcontext, vcontext.clone())?
                        .powf(rhs.eval(fcontext, vcontext)?)),
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
                    Fac => factorial(arg.eval(fcontext, vcontext)?),
                }
            }
            Expr::FnCall { name: fname, args } => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(arg.eval(fcontext, vcontext.clone())?);
                }

                let function = fcontext
                    .get_function(fname)
                    .ok_or(EvalError::FunctionNotFound(fname.to_string()))?;

                function.call(evaluated_args, fcontext, vcontext)
            }
            Expr::Variable(name) => {
                let variable = vcontext
                    .borrow()
                    .get_variable(name)
                    .ok_or(EvalError::VariableNotFound(name.to_string()))?
                    .get();
                Ok(variable)
            }
        }
    }
}

fn factorial(n: f64) -> Result<f64, EvalError> {
    let n = n.round();
    if n < 0. {
        return Err(EvalError::MathDomain);
    }
    let result = (1..=n as u128).try_fold(1_u128, |acc, x| acc.checked_mul(x));

    match result {
        Some(n) => Ok(n as f64),
        None => Err(EvalError::Overflow),
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
        let expr = Expr::Number(42.0);
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), 42.0);
    }

    #[test]
    fn test_infix_op_add() {
        let (fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::InfixOp {
            op: InfixOp::Add,
            lhs: Box::new(Expr::Number(1.0)),
            rhs: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), 3.0);
    }

    #[test]
    fn test_prefix_op_neg() {
        let (fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::PrefixOp {
            op: PrefixOp::Neg,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), -5.0);
    }

    #[test]
    fn test_postfix_op_fac() {
        let (fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::PostfixOp {
            op: PostfixOp::Fac,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), 120.0);
    }

    #[test]
    fn test_fn_call() {
        let (mut fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let expr = Expr::FnCall {
            name: "mock_fn".to_string(),
            args: vec![Expr::Number(2.0), Expr::Number(3.0)],
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
        assert_eq!(expr.eval(&fcontext, vcontext).unwrap(), 5.0);
    }
}
