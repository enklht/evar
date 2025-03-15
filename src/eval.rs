use crate::types::*;

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
        _ => todo!(),
    }
}
