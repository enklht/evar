use crate::types::*;

fn factorial(n: f64) -> f64 {
    let result = (1..=n.round() as usize).try_fold(1, |acc, x| usize::checked_mul(acc, x));

    match result {
        Some(f) => f as f64,
        None => f64::MAX,
    }
}

pub fn eval(expr: Expr) -> f64 {
    match expr {
        Expr::Number(f) => f,
        Expr::BinaryOperation {
            operator: op,
            lhs,
            rhs,
        } => {
            use BinaryOperator::*;
            match op {
                Add => eval(*lhs) + eval(*rhs),
                Sub => eval(*lhs) - eval(*rhs),
                Mul => eval(*lhs) * eval(*rhs),
                Div => eval(*lhs) / eval(*rhs),
                Rem => eval(*lhs).rem_euclid(eval(*rhs)),
                Pow => eval(*lhs).powf(eval(*rhs)),
            }
        }
        Expr::UnaryOperation { operator: op, arg } => {
            use UnaryOperator::*;
            match op {
                Neg => -eval(*arg),
                Fac => factorial(eval(*arg)),
            }
        }
        _ => todo!(),
    }
}
