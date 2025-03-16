use crate::{
    context::{AngleUnit, Context},
    types::*,
};

fn factorial(n: f64) -> f64 {
    let result = (1..=n.round() as usize).try_fold(1, |acc, x| usize::checked_mul(acc, x));

    match result {
        Some(f) => f as f64,
        None => f64::MAX,
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
                (Sin, Degree) => eval(*arg, context).to_degrees().sin(),
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
            }
        }
    }
}
