use crate::{context::Context, errors::EvalError, types::*};

fn factorial(n: f64) -> Result<f64, EvalError> {
    let n = n.round();
    if n < 0. {
        return Err(EvalError::DomainError);
    }
    let result = (1..=n as u128).try_fold(1_u128, |acc, x| acc.checked_mul(x));

    match result {
        Some(n) => Ok(n as f64),
        None => Err(EvalError::OverFlowError),
    }
}

pub fn eval(expr: Expr, context: &Context) -> Result<f64, EvalError> {
    match expr {
        Expr::Number(f) => Ok(f),
        Expr::BinaryOp {
            op: operator,
            lhs,
            rhs,
        } => {
            use BinaryOp::*;
            match operator {
                Add => Ok(eval(*lhs, context)? + eval(*rhs, context)?),
                Sub => Ok(eval(*lhs, context)? - eval(*rhs, context)?),
                Mul => Ok(eval(*lhs, context)? * eval(*rhs, context)?),
                Div => Ok(eval(*lhs, context)? / eval(*rhs, context)?),
                Rem => Ok(eval(*lhs, context)?.rem_euclid(eval(*rhs, context)?)),
                Pow => Ok(eval(*lhs, context)?.powf(eval(*rhs, context)?)),
            }
        }
        Expr::UnaryOp { op: operator, arg } => {
            use UnaryOp::*;
            match operator {
                Neg => Ok(-eval(*arg, context)?),
                Fac => factorial(eval(*arg, context)?),
            }
        }
        Expr::FnCall { fname, args } => {
            let function = context
                .get_function(&fname)
                .ok_or(EvalError::FunctionNotFoundError(fname))?;
            let mut evaluated_args = Vec::new();
            for arg in args {
                evaluated_args.push(eval(arg, context)?);
            }
            function.call(evaluated_args)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        args::{AngleUnit, Args},
        context::Context,
    };

    use super::*;

    const RADIAN_ARGS: Args = Args {
        fix: 10,
        base: 10,
        angle_unit: AngleUnit::Radian,
    };

    const DEGREE_ARGS: Args = Args {
        fix: 10,
        base: 10,
        angle_unit: AngleUnit::Degree,
    };

    #[test]
    fn test_eval_number() {
        let radian_context: Context = Context::new(RADIAN_ARGS);

        let expr = Expr::Number(5.0);
        assert_eq!(eval(expr, &radian_context).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_basic_operations() {
        let radian_context: Context = Context::new(RADIAN_ARGS);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 2.0 + 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Sub,
            lhs: Box::new(Expr::Number(5.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 5.0 - 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Mul,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 2.0 * 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Div,
            lhs: Box::new(Expr::Number(6.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 6.0 / 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Rem,
            lhs: Box::new(Expr::Number(7.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 7.0 % 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Pow,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 2.0_f64.powf(3.0));

        let expr = Expr::UnaryOp {
            op: UnaryOp::Neg,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), -5.0);

        let expr = Expr::UnaryOp {
            op: UnaryOp::Fac,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 120.0); // 5! = 120
    }

    #[test]
    fn test_eval_functions() {
        let radian_context: Context = Context::new(RADIAN_ARGS);
        let degree_context: Context = Context::new(DEGREE_ARGS);

        let expr = Expr::FnCall {
            fname: "sin".into(),
            args: vec![Expr::Number(std::f64::consts::PI / 2.0)],
        };
        assert_eq!(
            eval(expr, &radian_context).unwrap(),
            (std::f64::consts::PI / 2.0).sin()
        );

        let expr = Expr::FnCall {
            fname: "sin".into(),
            args: vec![Expr::Number(90.0)],
        };
        assert_eq!(
            eval(expr, &degree_context).unwrap(),
            90.0_f64.to_radians().sin()
        );

        let expr = Expr::FnCall {
            fname: "log".into(),
            args: vec![Expr::Number(8.0), Expr::Number(2.0)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 8.0_f64.log(2.0));

        let expr = Expr::FnCall {
            fname: "cos".into(),
            args: vec![Expr::Number(0.0)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 0.0_f64.cos());

        let expr = Expr::FnCall {
            fname: "cos".into(),
            args: vec![Expr::Number(0.0)],
        };
        assert_eq!(
            eval(expr, &degree_context).unwrap(),
            0.0_f64.to_radians().cos()
        );

        let expr = Expr::FnCall {
            fname: "tan".into(),
            args: vec![Expr::Number(std::f64::consts::PI / 4.0)],
        };
        assert_eq!(
            eval(expr, &radian_context).unwrap(),
            (std::f64::consts::PI / 4.0).tan()
        );

        let expr = Expr::FnCall {
            fname: "tan".into(),
            args: vec![Expr::Number(45.0)],
        };
        assert_eq!(
            eval(expr, &degree_context).unwrap(),
            45.0_f64.to_radians().tan()
        );

        let expr = Expr::FnCall {
            fname: "sqrt".into(),
            args: vec![Expr::Number(16.0)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 16.0_f64.sqrt());

        let expr = Expr::FnCall {
            fname: "exp".into(),
            args: vec![Expr::Number(1.0)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 1.0_f64.exp());

        let expr = Expr::FnCall {
            fname: "ln".into(),
            args: vec![Expr::Number(std::f64::consts::E)],
        };
        assert_eq!(
            eval(expr, &radian_context).unwrap(),
            std::f64::consts::E.ln()
        );

        let expr = Expr::FnCall {
            fname: "log10".into(),
            args: vec![Expr::Number(1000.0)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 1000.0_f64.log10());

        let expr = Expr::FnCall {
            fname: "exp2".into(),
            args: vec![Expr::Number(3.0)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 3.0_f64.exp2());

        let expr = Expr::FnCall {
            fname: "floor".into(),
            args: vec![Expr::Number(3.7)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 3.7_f64.floor());

        let expr = Expr::FnCall {
            fname: "ceil".into(),
            args: vec![Expr::Number(3.3)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 3.3_f64.ceil());

        let expr = Expr::FnCall {
            fname: "round".into(),
            args: vec![Expr::Number(3.5)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 3.5_f64.round());

        let expr = Expr::FnCall {
            fname: "abs".into(),
            args: vec![Expr::Number(-3.5)],
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), (-3.5_f64).abs());
    }

    #[test]
    fn test_eval_mixed_operations() {
        let radian_context: Context = Context::new(RADIAN_ARGS);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            lhs: Box::new(Expr::BinaryOp {
                op: BinaryOp::Mul,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
            rhs: Box::new(Expr::Number(4.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), (2.0 * 3.0) + 4.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Sub,
            lhs: Box::new(Expr::Number(10.0)),
            rhs: Box::new(Expr::BinaryOp {
                op: BinaryOp::Div,
                lhs: Box::new(Expr::Number(6.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 10.0 - (6.0 / 3.0));

        let expr = Expr::BinaryOp {
            op: BinaryOp::Rem,
            lhs: Box::new(Expr::BinaryOp {
                op: BinaryOp::Pow,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(
            eval(expr, &radian_context).unwrap(),
            2.0_f64.powf(3.0) % 3.0
        );

        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            lhs: Box::new(Expr::UnaryOp {
                op: UnaryOp::Neg,
                arg: Box::new(Expr::Number(5.0)),
            }),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), -5.0 + 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Sub,
            lhs: Box::new(Expr::UnaryOp {
                op: UnaryOp::Fac,
                arg: Box::new(Expr::Number(5.0)),
            }),
            rhs: Box::new(Expr::Number(119.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 120.0 - 119.0); // 5! - 119

        let expr = Expr::BinaryOp {
            op: BinaryOp::Mul,
            lhs: Box::new(Expr::FnCall {
                fname: "sin".into(),
                args: vec![Expr::Number(std::f64::consts::PI / 2.0)],
            }),
            rhs: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(
            eval(expr, &radian_context).unwrap(),
            (std::f64::consts::PI / 2.0).sin() * 2.0
        );

        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            lhs: Box::new(Expr::FnCall {
                fname: "log".into(),
                args: vec![Expr::Number(8.0), Expr::Number(2.0)],
            }),
            rhs: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(eval(expr, &radian_context).unwrap(), 8.0_f64.log(2.0) + 1.0);
    }
}
