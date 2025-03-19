use crate::{context::Context, errors::EvalError, types::*};

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

pub fn eval(stmt: Stmt, context: &mut Context) -> Result<f64, EvalError> {
    match stmt {
        Stmt::DefVar { name, expr } => {
            let val = eval_expr(expr, context)?;
            let variable = context
                .set_variable(&name, val)
                .ok_or(EvalError::InvalidVariableDefinition(name))?;
            Ok(variable)
        }
        Stmt::DefFun { name, args, body } => {
            context.set_function(&name, args, body);
            Ok(f64::NAN)
        }
        Stmt::Expr(expr) => eval_expr(expr, context),
    }
}

pub fn eval_expr(expr: Expr, context: &mut Context) -> Result<f64, EvalError> {
    match expr {
        Expr::Number(f) => Ok(f),
        Expr::InfixOp { op, lhs, rhs } => {
            use InfixOp::*;
            match op {
                Add => Ok(eval_expr(*lhs, context)? + eval_expr(*rhs, context)?),
                Sub => Ok(eval_expr(*lhs, context)? - eval_expr(*rhs, context)?),
                Mul => Ok(eval_expr(*lhs, context)? * eval_expr(*rhs, context)?),
                Div => Ok(eval_expr(*lhs, context)? / eval_expr(*rhs, context)?),
                Rem => Ok(eval_expr(*lhs, context)?.rem_euclid(eval_expr(*rhs, context)?)),
                Pow => Ok(eval_expr(*lhs, context)?.powf(eval_expr(*rhs, context)?)),
            }
        }
        Expr::PrefixOp { op, arg } => {
            use PrefixOp::*;
            match op {
                Neg => Ok(-eval_expr(*arg, context)?),
            }
        }
        Expr::PostfixOp { op, arg } => {
            use PostfixOp::*;
            match op {
                Fac => factorial(eval_expr(*arg, context)?),
            }
        }
        Expr::FnCall { name: fname, args } => {
            let mut evaluated_args = Vec::new();
            for arg in args {
                evaluated_args.push(eval_expr(arg, context)?);
            }

            let function = context
                .get_function(&fname)
                .ok_or(EvalError::FunctionNotFound(fname))?;
            function.call(evaluated_args)
        }
        Expr::Variable(name) => {
            let variable = context
                .get_variable(&name)
                .ok_or(EvalError::VariableNotFound(name))?;
            Ok(variable.get())
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
        // fix: 10,
        // base: 10,
        angle_unit: AngleUnit::Radian,
        debug: false,
        no_color: false,
    };

    const DEGREE_ARGS: Args = Args {
        // fix: 10,
        // base: 10,
        angle_unit: AngleUnit::Degree,
        debug: false,
        no_color: false,
    };

    #[test]
    fn test_eval_number() {
        let radian_context = &mut Context::new(&RADIAN_ARGS);

        let expr = Expr::Number(5.0);
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_basic_operations() {
        let radian_context = &mut Context::new(&RADIAN_ARGS);

        let expr = Expr::InfixOp {
            op: InfixOp::Add,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 2.0 + 3.0);

        let expr = Expr::InfixOp {
            op: InfixOp::Sub,
            lhs: Box::new(Expr::Number(5.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 5.0 - 3.0);

        let expr = Expr::InfixOp {
            op: InfixOp::Mul,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 2.0 * 3.0);

        let expr = Expr::InfixOp {
            op: InfixOp::Div,
            lhs: Box::new(Expr::Number(6.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 6.0 / 3.0);

        let expr = Expr::InfixOp {
            op: InfixOp::Rem,
            lhs: Box::new(Expr::Number(7.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 7.0 % 3.0);

        let expr = Expr::InfixOp {
            op: InfixOp::Pow,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 2.0_f64.powf(3.0));

        let expr = Expr::PrefixOp {
            op: PrefixOp::Neg,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), -5.0);

        let expr = Expr::PostfixOp {
            op: PostfixOp::Fac,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 120.0); // 5! = 120
    }

    #[test]
    fn test_eval_functions() {
        let radian_context = &mut Context::new(&RADIAN_ARGS);
        let degree_context = &mut Context::new(&DEGREE_ARGS);

        let expr = Expr::FnCall {
            name: "sin".into(),
            args: vec![Expr::Number(std::f64::consts::PI / 2.0)],
        };
        assert_eq!(
            eval_expr(expr, radian_context).unwrap(),
            (std::f64::consts::PI / 2.0).sin()
        );

        let expr = Expr::FnCall {
            name: "sin".into(),
            args: vec![Expr::Number(90.0)],
        };
        assert_eq!(
            eval_expr(expr, degree_context).unwrap(),
            90.0_f64.to_radians().sin()
        );

        let expr = Expr::FnCall {
            name: "log".into(),
            args: vec![Expr::Number(8.0), Expr::Number(2.0)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 8.0_f64.log(2.0));

        let expr = Expr::FnCall {
            name: "cos".into(),
            args: vec![Expr::Number(0.0)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 0.0_f64.cos());

        let expr = Expr::FnCall {
            name: "cos".into(),
            args: vec![Expr::Number(0.0)],
        };
        assert_eq!(
            eval_expr(expr, degree_context).unwrap(),
            0.0_f64.to_radians().cos()
        );

        let expr = Expr::FnCall {
            name: "tan".into(),
            args: vec![Expr::Number(std::f64::consts::PI / 4.0)],
        };
        assert_eq!(
            eval_expr(expr, radian_context).unwrap(),
            (std::f64::consts::PI / 4.0).tan()
        );

        let expr = Expr::FnCall {
            name: "tan".into(),
            args: vec![Expr::Number(45.0)],
        };
        assert_eq!(
            eval_expr(expr, degree_context).unwrap(),
            45.0_f64.to_radians().tan()
        );

        let expr = Expr::FnCall {
            name: "sqrt".into(),
            args: vec![Expr::Number(16.0)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 16.0_f64.sqrt());

        let expr = Expr::FnCall {
            name: "exp".into(),
            args: vec![Expr::Number(1.0)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 1.0_f64.exp());

        let expr = Expr::FnCall {
            name: "ln".into(),
            args: vec![Expr::Number(std::f64::consts::E)],
        };
        assert_eq!(
            eval_expr(expr, radian_context).unwrap(),
            std::f64::consts::E.ln()
        );

        let expr = Expr::FnCall {
            name: "log10".into(),
            args: vec![Expr::Number(1000.0)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 1000.0_f64.log10());

        let expr = Expr::FnCall {
            name: "exp2".into(),
            args: vec![Expr::Number(3.0)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.0_f64.exp2());

        let expr = Expr::FnCall {
            name: "floor".into(),
            args: vec![Expr::Number(3.7)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.7_f64.floor());

        let expr = Expr::FnCall {
            name: "ceil".into(),
            args: vec![Expr::Number(3.3)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.3_f64.ceil());

        let expr = Expr::FnCall {
            name: "round".into(),
            args: vec![Expr::Number(3.5)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.5_f64.round());

        let expr = Expr::FnCall {
            name: "abs".into(),
            args: vec![Expr::Number(-3.5)],
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), (-3.5_f64).abs());
    }

    #[test]
    fn test_eval_mixed_operations() {
        let radian_context = &mut Context::new(&RADIAN_ARGS);

        let expr = Expr::InfixOp {
            op: InfixOp::Add,
            lhs: Box::new(Expr::InfixOp {
                op: InfixOp::Mul,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
            rhs: Box::new(Expr::Number(4.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), (2.0 * 3.0) + 4.0);

        let expr = Expr::InfixOp {
            op: InfixOp::Sub,
            lhs: Box::new(Expr::Number(10.0)),
            rhs: Box::new(Expr::InfixOp {
                op: InfixOp::Div,
                lhs: Box::new(Expr::Number(6.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 10.0 - (6.0 / 3.0));

        let expr = Expr::InfixOp {
            op: InfixOp::Rem,
            lhs: Box::new(Expr::InfixOp {
                op: InfixOp::Pow,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(
            eval_expr(expr, radian_context).unwrap(),
            2.0_f64.powf(3.0) % 3.0
        );

        let expr = Expr::InfixOp {
            op: InfixOp::Add,
            lhs: Box::new(Expr::PrefixOp {
                op: PrefixOp::Neg,
                arg: Box::new(Expr::Number(5.0)),
            }),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), -5.0 + 3.0);

        let expr = Expr::InfixOp {
            op: InfixOp::Sub,
            lhs: Box::new(Expr::PostfixOp {
                op: PostfixOp::Fac,
                arg: Box::new(Expr::Number(5.0)),
            }),
            rhs: Box::new(Expr::Number(119.0)),
        };
        assert_eq!(eval_expr(expr, radian_context).unwrap(), 120.0 - 119.0); // 5! - 119

        let expr = Expr::InfixOp {
            op: InfixOp::Mul,
            lhs: Box::new(Expr::FnCall {
                name: "sin".into(),
                args: vec![Expr::Number(std::f64::consts::PI / 2.0)],
            }),
            rhs: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(
            eval_expr(expr, radian_context).unwrap(),
            (std::f64::consts::PI / 2.0).sin() * 2.0
        );

        let expr = Expr::InfixOp {
            op: InfixOp::Add,
            lhs: Box::new(Expr::FnCall {
                name: "log".into(),
                args: vec![Expr::Number(8.0), Expr::Number(2.0)],
            }),
            rhs: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(
            eval_expr(expr, radian_context).unwrap(),
            8.0_f64.log(2.0) + 1.0
        );
    }
}
