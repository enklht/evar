use super::EvalError;
use super::{Context, operators::*};

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
    pub fn eval(&self, context: &Context) -> Result<f64, EvalError> {
        match self {
            Expr::Number(f) => Ok(*f),
            Expr::InfixOp { op, lhs, rhs } => {
                use InfixOp::*;
                match op {
                    Add => Ok(lhs.eval(context)? + rhs.eval(context)?),
                    Sub => Ok(lhs.eval(context)? - rhs.eval(context)?),
                    Mul => Ok(lhs.eval(context)? * rhs.eval(context)?),
                    Div => Ok(lhs.eval(context)? / rhs.eval(context)?),
                    Rem => Ok(lhs.eval(context)?.rem_euclid(rhs.eval(context)?)),
                    Pow => Ok(lhs.eval(context)?.powf(rhs.eval(context)?)),
                }
            }
            Expr::PrefixOp { op, arg } => {
                use PrefixOp::*;
                match op {
                    Neg => Ok(-arg.eval(context)?),
                }
            }
            Expr::PostfixOp { op, arg } => {
                use PostfixOp::*;
                match op {
                    Fac => factorial(arg.eval(context)?),
                }
            }
            Expr::FnCall { name: fname, args } => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(arg.eval(context)?);
                }

                let function = context
                    .get_function(&fname)
                    .ok_or(EvalError::FunctionNotFound(fname.to_string()))?;

                function.call(evaluated_args, context)
            }
            Expr::Variable(name) => {
                let variable = context
                    .get_variable(&name)
                    .ok_or(EvalError::VariableNotFound(name.to_string()))?;
                Ok(variable.get())
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
