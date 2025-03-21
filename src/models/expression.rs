use super::{Context, operators::*};
use super::{EvalError, Value};

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
                    .unwrap_or_else(|| String::from(""));
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
    pub fn eval(&self, context: &mut Context) -> Result<Value, EvalError> {
        match self {
            Expr::Int(f) => Ok(Value::from(*f)),
            Expr::Float(f) => Ok(Value::from(*f)),
            Expr::InfixOp { op, lhs, rhs } => {
                use InfixOp::*;
                match op {
                    Add => Ok(lhs.eval(context)? + rhs.eval(context)?),
                    Sub => Ok(lhs.eval(context)? - rhs.eval(context)?),
                    Mul => Ok(lhs.eval(context)? * rhs.eval(context)?),
                    Div => lhs.eval(context)? / rhs.eval(context)?,
                    Rem => lhs.eval(context)?.rem_euclid(rhs.eval(context)?),
                    Pow => Ok(lhs.eval(context)?.pow(rhs.eval(context)?)),
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
                    Fac => arg.eval(context)?.factorial(),
                }
            }
            Expr::FnCall { name, args } => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(arg.eval(context)?);
                }

                let function = context
                    .get_function(name)
                    .ok_or(EvalError::FunctionNotFound(name.to_string()))?
                    .clone();

                function.call(evaluated_args, context)
            }
            Expr::Variable(name) => {
                let variable = context
                    .get_variable(name)
                    .ok_or(EvalError::VariableNotFound(name.to_string()))?
                    .get();
                Ok(variable.into())
            }
            Expr::PrevAnswer => context.get_prev_answer().ok_or(EvalError::NoHistory),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{args::AngleUnit::*, create_context};

    #[test]
    fn test_number() {
        let mut context = create_context(&Radian);
        let expr = Expr::Float(42.0);
        assert_eq!(expr.eval(&mut context,).unwrap(), Value::from(42.0));
    }

    #[test]
    fn test_infix_op_add() {
        let mut context = create_context(&Radian);
        let expr = Expr::InfixOp {
            op: InfixOp::Add,
            lhs: Box::new(Expr::Float(1.0)),
            rhs: Box::new(Expr::Float(2.0)),
        };
        assert_eq!(expr.eval(&mut context,).unwrap(), Value::from(3.0));
    }

    #[test]
    fn test_prefix_op_neg() {
        let mut context = create_context(&Radian);
        let expr = Expr::PrefixOp {
            op: PrefixOp::Neg,
            arg: Box::new(Expr::Float(5.0)),
        };
        assert_eq!(expr.eval(&mut context,).unwrap(), Value::from(-5.0));
    }

    #[test]
    fn test_postfix_op_fac() {
        let mut context = create_context(&Radian);
        let expr = Expr::PostfixOp {
            op: PostfixOp::Fac,
            arg: Box::new(Expr::Float(5.0)),
        };
        assert_eq!(expr.eval(&mut context,).unwrap(), Value::from(120.0));
    }

    #[test]
    fn test_fn_call() {
        let mut context = create_context(&Radian);
        let expr = Expr::FnCall {
            name: "mock_fn".to_string(),
            args: vec![Expr::Float(2.0), Expr::Float(3.0)],
        };
        context.set_function(
            "mock_fn",
            vec![String::from("x"), String::from("y")],
            Expr::InfixOp {
                op: InfixOp::Add,
                lhs: Expr::Variable(String::from("x")).into(),
                rhs: Expr::Variable(String::from("y")).into(),
            },
        );
        assert_eq!(expr.eval(&mut context,).unwrap(), Value::from(5.0));
    }
}
