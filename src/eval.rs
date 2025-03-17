use crate::{
    context::{AngleUnit, Context},
    errors::EvalError,
    types::*,
};

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
        Expr::UnaryFnCall { function, arg } => {
            use AngleUnit::*;
            use UnaryFn::*;
            match (function, &context.angle_unit) {
                (Sin, Radian) => Ok(eval(*arg, context)?.sin()),
                (Sin, Degree) => Ok(eval(*arg, context)?.to_radians().sin()),
                (Cos, Radian) => Ok(eval(*arg, context)?.cos()),
                (Cos, Degree) => Ok(eval(*arg, context)?.to_radians().cos()),
                (Tan, Radian) => Ok(eval(*arg, context)?.tan()),
                (Tan, Degree) => Ok(eval(*arg, context)?.to_radians().tan()),
                (Sec, Radian) => Ok(1.0 / eval(*arg, context)?.cos()),
                (Sec, Degree) => Ok(1.0 / eval(*arg, context)?.to_radians().cos()),
                (Csc, Radian) => Ok(1.0 / eval(*arg, context)?.sin()),
                (Csc, Degree) => Ok(1.0 / eval(*arg, context)?.to_radians().sin()),
                (Cot, Radian) => Ok(1.0 / eval(*arg, context)?.tan()),
                (Cot, Degree) => Ok(1.0 / eval(*arg, context)?.to_radians().tan()),
                (Asin, Radian) => Ok(eval(*arg, context)?.asin()),
                (Asin, Degree) => Ok(eval(*arg, context)?.asin().to_degrees()),
                (Acos, Radian) => Ok(eval(*arg, context)?.acos()),
                (Acos, Degree) => Ok(eval(*arg, context)?.acos().to_degrees()),
                (Atan, Radian) => Ok(eval(*arg, context)?.atan()),
                (Atan, Degree) => Ok(eval(*arg, context)?.atan().to_degrees()),
                (Asec, Radian) => Ok(eval(*arg, context)?.recip().acos()),
                (Asec, Degree) => Ok(eval(*arg, context)?.recip().acos().to_degrees()),
                (Acsc, Radian) => Ok(eval(*arg, context)?.recip().asin()),
                (Acsc, Degree) => Ok(eval(*arg, context)?.recip().asin().to_degrees()),
                (Acot, Radian) => Ok(eval(*arg, context)?.recip().atan()),
                (Acot, Degree) => Ok(eval(*arg, context)?.recip().atan().to_degrees()),
                (Sinh, _) => Ok(eval(*arg, context)?.sinh()),
                (Cosh, _) => Ok(eval(*arg, context)?.cosh()),
                (Tanh, _) => Ok(eval(*arg, context)?.tanh()),
                (Floor, _) => Ok(eval(*arg, context)?.floor()),
                (Ceil, _) => Ok(eval(*arg, context)?.ceil()),
                (Round, _) => Ok(eval(*arg, context)?.round()),
                (Abs, _) => Ok(eval(*arg, context)?.abs()),
                (Sqrt, _) => Ok(eval(*arg, context)?.sqrt()),
                (Exp, _) => Ok(eval(*arg, context)?.exp()),
                (Exp2, _) => Ok(eval(*arg, context)?.exp2()),
                (Ln, _) => Ok(eval(*arg, context)?.ln()),
                (Log10, _) => Ok(eval(*arg, context)?.log10()),
                (Rad, _) => Ok(eval(*arg, context)?.to_radians()),
                (Deg, _) => Ok(eval(*arg, context)?.to_degrees()),
            }
        }
        Expr::BinaryFnCall {
            function,
            arg1,
            arg2,
        } => {
            use BinaryFn::*;
            match (function, &context.angle_unit) {
                (Log, _) => Ok(eval(*arg1, context)?.log(eval(*arg2, context)?)),
                (NRoot, _) => Ok(eval(*arg1, context)?.powf(eval(*arg2, context)?.recip())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RADIAN_CONTEXT: Context = Context {
        fix: 10,
        base: 10,
        angle_unit: AngleUnit::Radian,
    };

    const DEGREE_CONTEXT: Context = Context {
        fix: 10,
        base: 10,
        angle_unit: AngleUnit::Degree,
    };

    #[test]
    fn test_eval_number() {
        let expr = Expr::Number(5.0);
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_basic_operations() {
        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 2.0 + 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Sub,
            lhs: Box::new(Expr::Number(5.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 5.0 - 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Mul,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 2.0 * 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Div,
            lhs: Box::new(Expr::Number(6.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 6.0 / 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Rem,
            lhs: Box::new(Expr::Number(7.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 7.0 % 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Pow,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 2.0_f64.powf(3.0));

        let expr = Expr::UnaryOp {
            op: UnaryOp::Neg,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), -5.0);

        let expr = Expr::UnaryOp {
            op: UnaryOp::Fac,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 120.0); // 5! = 120
    }

    #[test]
    fn test_eval_functions() {
        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Sin,
            arg: Box::new(Expr::Number(std::f64::consts::PI / 2.0)),
        };
        assert_eq!(
            eval(expr, &RADIAN_CONTEXT).unwrap(),
            (std::f64::consts::PI / 2.0).sin()
        );

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Sin,
            arg: Box::new(Expr::Number(90.0)),
        };
        assert_eq!(
            eval(expr, &DEGREE_CONTEXT).unwrap(),
            90.0_f64.to_radians().sin()
        );

        let expr = Expr::BinaryFnCall {
            function: BinaryFn::Log,
            arg1: Box::new(Expr::Number(8.0)),
            arg2: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 8.0_f64.log(2.0));

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Cos,
            arg: Box::new(Expr::Number(0.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 0.0_f64.cos());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Cos,
            arg: Box::new(Expr::Number(0.0)),
        };
        assert_eq!(
            eval(expr, &DEGREE_CONTEXT).unwrap(),
            0.0_f64.to_radians().cos()
        );

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Tan,
            arg: Box::new(Expr::Number(std::f64::consts::PI / 4.0)),
        };
        assert_eq!(
            eval(expr, &RADIAN_CONTEXT).unwrap(),
            (std::f64::consts::PI / 4.0).tan()
        );

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Tan,
            arg: Box::new(Expr::Number(45.0)),
        };
        assert_eq!(
            eval(expr, &DEGREE_CONTEXT).unwrap(),
            45.0_f64.to_radians().tan()
        );

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Sqrt,
            arg: Box::new(Expr::Number(16.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 16.0_f64.sqrt());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Exp,
            arg: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 1.0_f64.exp());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Ln,
            arg: Box::new(Expr::Number(std::f64::consts::E)),
        };
        assert_eq!(
            eval(expr, &RADIAN_CONTEXT).unwrap(),
            std::f64::consts::E.ln()
        );

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Log10,
            arg: Box::new(Expr::Number(1000.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 1000.0_f64.log10());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Exp2,
            arg: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 3.0_f64.exp2());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Floor,
            arg: Box::new(Expr::Number(3.7)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 3.7_f64.floor());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Ceil,
            arg: Box::new(Expr::Number(3.3)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 3.3_f64.ceil());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Round,
            arg: Box::new(Expr::Number(3.5)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 3.5_f64.round());

        let expr = Expr::UnaryFnCall {
            function: UnaryFn::Abs,
            arg: Box::new(Expr::Number(-3.5)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), (-3.5_f64).abs());
    }
    #[test]
    fn test_eval_mixed_operations() {
        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            lhs: Box::new(Expr::BinaryOp {
                op: BinaryOp::Mul,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
            rhs: Box::new(Expr::Number(4.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), (2.0 * 3.0) + 4.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Sub,
            lhs: Box::new(Expr::Number(10.0)),
            rhs: Box::new(Expr::BinaryOp {
                op: BinaryOp::Div,
                lhs: Box::new(Expr::Number(6.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 10.0 - (6.0 / 3.0));

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
            eval(expr, &RADIAN_CONTEXT).unwrap(),
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
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), -5.0 + 3.0);

        let expr = Expr::BinaryOp {
            op: BinaryOp::Sub,
            lhs: Box::new(Expr::UnaryOp {
                op: UnaryOp::Fac,
                arg: Box::new(Expr::Number(5.0)),
            }),
            rhs: Box::new(Expr::Number(119.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 120.0 - 119.0); // 5! - 119

        let expr = Expr::BinaryOp {
            op: BinaryOp::Mul,
            lhs: Box::new(Expr::UnaryFnCall {
                function: UnaryFn::Sin,
                arg: Box::new(Expr::Number(std::f64::consts::PI / 2.0)),
            }),
            rhs: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(
            eval(expr, &RADIAN_CONTEXT).unwrap(),
            (std::f64::consts::PI / 2.0).sin() * 2.0
        );

        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            lhs: Box::new(Expr::BinaryFnCall {
                function: BinaryFn::Log,
                arg1: Box::new(Expr::Number(8.0)),
                arg2: Box::new(Expr::Number(2.0)),
            }),
            rhs: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT).unwrap(), 8.0_f64.log(2.0) + 1.0);
    }
}
