use crate::{
    context::{AngleUnit, Context},
    types::*,
};

fn factorial(n: f64) -> f64 {
    let result = (1..=n.round() as usize).try_fold(1, |acc, x| usize::checked_mul(acc, x));

    match result {
        Some(f) => f as f64,
        None => f64::INFINITY,
    }
}

pub fn eval(expr: Expr, context: &Context) -> f64 {
    match expr {
        Expr::Number(f) => f,
        Expr::BinaryOperation { operator, lhs, rhs } => {
            use BinaryOperator::*;
            match operator {
                Add => eval(*lhs, context) + eval(*rhs, context),
                Sub => eval(*lhs, context) - eval(*rhs, context),
                Mul => eval(*lhs, context) * eval(*rhs, context),
                Div => eval(*lhs, context) / eval(*rhs, context),
                Rem => eval(*lhs, context).rem_euclid(eval(*rhs, context)),
                Pow => eval(*lhs, context).powf(eval(*rhs, context)),
            }
        }
        Expr::UnaryOperation { operator, arg } => {
            use UnaryOperator::*;
            match operator {
                Neg => -eval(*arg, context),
                Fac => factorial(eval(*arg, context)),
            }
        }
        Expr::UnaryFunctionCall { function, arg } => {
            use AngleUnit::*;
            use UnaryFunction::*;
            match (function, &context.angle_unit) {
                (Sin, Radian) => eval(*arg, context).sin(),
                (Sin, Degree) => eval(*arg, context).to_radians().sin(),
                (Cos, Radian) => eval(*arg, context).cos(),
                (Cos, Degree) => eval(*arg, context).to_radians().cos(),
                (Tan, Radian) => eval(*arg, context).tan(),
                (Tan, Degree) => eval(*arg, context).to_radians().tan(),
                (Sec, Radian) => 1.0 / eval(*arg, context).cos(),
                (Sec, Degree) => 1.0 / eval(*arg, context).to_radians().cos(),
                (Csc, Radian) => 1.0 / eval(*arg, context).sin(),
                (Csc, Degree) => 1.0 / eval(*arg, context).to_radians().sin(),
                (Cot, Radian) => 1.0 / eval(*arg, context).tan(),
                (Cot, Degree) => 1.0 / eval(*arg, context).to_radians().tan(),
                (Asin, Radian) => eval(*arg, context).asin(),
                (Asin, Degree) => eval(*arg, context).asin().to_degrees(),
                (Acos, Radian) => eval(*arg, context).acos(),
                (Acos, Degree) => eval(*arg, context).acos().to_degrees(),
                (Atan, Radian) => eval(*arg, context).atan(),
                (Atan, Degree) => eval(*arg, context).atan().to_degrees(),
                (Asec, Radian) => eval(*arg, context).recip().acos(),
                (Asec, Degree) => eval(*arg, context).recip().acos().to_degrees(),
                (Acsc, Radian) => eval(*arg, context).recip().asin(),
                (Acsc, Degree) => eval(*arg, context).recip().asin().to_degrees(),
                (Acot, Radian) => eval(*arg, context).recip().atan(),
                (Acot, Degree) => eval(*arg, context).recip().atan().to_degrees(),
                (Sinh, _) => eval(*arg, context).sinh(),
                (Cosh, _) => eval(*arg, context).cosh(),
                (Tanh, _) => eval(*arg, context).tanh(),
                (Floor, _) => eval(*arg, context).floor(),
                (Ceil, _) => eval(*arg, context).ceil(),
                (Round, _) => eval(*arg, context).round(),
                (Abs, _) => eval(*arg, context).abs(),
                (Sqrt, _) => eval(*arg, context).sqrt(),
                (Exp, _) => eval(*arg, context).exp(),
                (Exp2, _) => eval(*arg, context).exp2(),
                (Ln, _) => eval(*arg, context).ln(),
                (Log10, _) => eval(*arg, context).log10(),
                (Rad, _) => eval(*arg, context).to_radians(),
                (Deg, _) => eval(*arg, context).to_degrees(),
            }
        }
        Expr::BinaryFunctionCall {
            function,
            arg1,
            arg2,
        } => {
            use BinaryFunction::*;
            match (function, &context.angle_unit) {
                (Log, _) => eval(*arg1, context).log(eval(*arg2, context)),
                (NRoot, _) => eval(*arg1, context).powf(eval(*arg2, context).recip()),
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
        assert_eq!(eval(Expr::Number(5.0), &RADIAN_CONTEXT), 5.0);
    }

    #[test]
    fn test_eval_addition() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Add,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 5.0);
    }

    #[test]
    fn test_eval_subtraction() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Sub,
            lhs: Box::new(Expr::Number(5.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 2.0);
    }

    #[test]
    fn test_eval_multiplication() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Mul,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 6.0);
    }

    #[test]
    fn test_eval_division() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Div,
            lhs: Box::new(Expr::Number(6.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 2.0);
    }

    #[test]
    fn test_eval_remainder() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Rem,
            lhs: Box::new(Expr::Number(7.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_power() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Pow,
            lhs: Box::new(Expr::Number(2.0)),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 8.0);
    }

    #[test]
    fn test_eval_negation() {
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Neg,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), -5.0);
    }

    #[test]
    fn test_eval_factorial() {
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Fac,
            arg: Box::new(Expr::Number(5.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 120.0);
    }

    #[test]
    fn test_eval_sin_radian() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Sin,
            arg: Box::new(Expr::Number(std::f64::consts::PI / 2.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_sin_degree() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Sin,
            arg: Box::new(Expr::Number(90.0)),
        };
        assert_eq!(eval(expr, &DEGREE_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_log() {
        let expr = Expr::BinaryFunctionCall {
            function: BinaryFunction::Log,
            arg1: Box::new(Expr::Number(8.0)),
            arg2: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 3.0);
    }

    #[test]
    fn test_eval_mixed_addition_and_multiplication() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Add,
            lhs: Box::new(Expr::BinaryOperation {
                operator: BinaryOperator::Mul,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
            rhs: Box::new(Expr::Number(4.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 10.0);
    }

    #[test]
    fn test_eval_mixed_subtraction_and_division() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Sub,
            lhs: Box::new(Expr::Number(10.0)),
            rhs: Box::new(Expr::BinaryOperation {
                operator: BinaryOperator::Div,
                lhs: Box::new(Expr::Number(6.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 8.0);
    }

    #[test]
    fn test_eval_mixed_power_and_remainder() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Rem,
            lhs: Box::new(Expr::BinaryOperation {
                operator: BinaryOperator::Pow,
                lhs: Box::new(Expr::Number(2.0)),
                rhs: Box::new(Expr::Number(3.0)),
            }),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 2.0);
    }

    #[test]
    fn test_eval_mixed_negation_and_addition() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Add,
            lhs: Box::new(Expr::UnaryOperation {
                operator: UnaryOperator::Neg,
                arg: Box::new(Expr::Number(5.0)),
            }),
            rhs: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), -2.0);
    }

    #[test]
    fn test_eval_mixed_factorial_and_subtraction() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Sub,
            lhs: Box::new(Expr::UnaryOperation {
                operator: UnaryOperator::Fac,
                arg: Box::new(Expr::Number(5.0)),
            }),
            rhs: Box::new(Expr::Number(119.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_mixed_sin_and_multiplication() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Mul,
            lhs: Box::new(Expr::UnaryFunctionCall {
                function: UnaryFunction::Sin,
                arg: Box::new(Expr::Number(std::f64::consts::PI / 2.0)),
            }),
            rhs: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 2.0);
    }

    #[test]
    fn test_eval_mixed_log_and_addition() {
        let expr = Expr::BinaryOperation {
            operator: BinaryOperator::Add,
            lhs: Box::new(Expr::BinaryFunctionCall {
                function: BinaryFunction::Log,
                arg1: Box::new(Expr::Number(8.0)),
                arg2: Box::new(Expr::Number(2.0)),
            }),
            rhs: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 4.0);
    }
    #[test]
    fn test_eval_cos_radian() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Cos,
            arg: Box::new(Expr::Number(0.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_cos_degree() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Cos,
            arg: Box::new(Expr::Number(0.0)),
        };
        assert_eq!(eval(expr, &DEGREE_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_tan_radian() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Tan,
            arg: Box::new(Expr::Number(std::f64::consts::PI / 4.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_tan_degree() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Tan,
            arg: Box::new(Expr::Number(45.0)),
        };
        assert_eq!(eval(expr, &DEGREE_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_sqrt() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Sqrt,
            arg: Box::new(Expr::Number(16.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 4.0);
    }

    #[test]
    fn test_eval_exp() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Exp,
            arg: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), std::f64::consts::E);
    }

    #[test]
    fn test_eval_ln() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Ln,
            arg: Box::new(Expr::Number(std::f64::consts::E)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 1.0);
    }

    #[test]
    fn test_eval_log10() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Log10,
            arg: Box::new(Expr::Number(1000.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 3.0);
    }

    #[test]
    fn test_eval_exp2() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Exp2,
            arg: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 8.0);
    }

    #[test]
    fn test_eval_floor() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Floor,
            arg: Box::new(Expr::Number(3.7)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 3.0);
    }

    #[test]
    fn test_eval_ceil() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Ceil,
            arg: Box::new(Expr::Number(3.3)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 4.0);
    }

    #[test]
    fn test_eval_round() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Round,
            arg: Box::new(Expr::Number(3.5)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 4.0);
    }

    #[test]
    fn test_eval_abs() {
        let expr = Expr::UnaryFunctionCall {
            function: UnaryFunction::Abs,
            arg: Box::new(Expr::Number(-3.5)),
        };
        assert_eq!(eval(expr, &RADIAN_CONTEXT), 3.5);
    }
}
