use crate::{
    args::AngleUnit,
    models::{Context, EvalError, Function, Variable},
};
use std::collections::HashMap;

macro_rules! unary_fn {
    ($fname:expr, $body:expr) => {
        (String::from($fname), Function::new_external(1, $body))
    };
}

macro_rules! binary_fn {
    ($fname:expr, $body:expr) => {
        (String::from($fname), Function::new_external(2, $body))
    };
}

#[rustfmt::skip]
pub fn create_context(angle_unit: &AngleUnit) -> Context {
    let mut functions = HashMap::from(match angle_unit {
        AngleUnit::Radian => [
            unary_fn!("sin", |x| Ok(x[0].to_float()?.sin().into())),
            unary_fn!("cos", |x| Ok(x[0].to_float()?.cos().into())),
            unary_fn!("tan", |x| Ok(x[0].to_float()?.tan().into())),
            unary_fn!("sec", |x| Ok(x[0].to_float()?.sin().recip().into())),
            unary_fn!("csc", |x| Ok(x[0].to_float()?.cos().recip().into())),
            unary_fn!("cot", |x| Ok(x[0].to_float()?.tan().recip().into())),
            unary_fn!("asin", |x| match x[0].to_float()? {
                n if (-1. ..=1.).contains(&n) => Ok(n.asin().into()),
                _ => Err(EvalError::MathDomain("the domain of asin is [-1, 1]".to_string()))
            }),
            unary_fn!("acos", |x| match x[0].to_float()? {
                n if (-1. ..=1.).contains(&n) => Ok(n.acos().into()),
                _ => Err(EvalError::MathDomain("the domain of asin is [-1, 1]".to_string()))
            }),
            unary_fn!("atan", |x| Ok(x[0].to_float()?.atan().into())),
        ],
        AngleUnit::Degree => [
            unary_fn!("sin", |x| Ok(x[0].to_float()?.to_radians().sin().into())),
            unary_fn!("cos", |x| Ok(x[0].to_float()?.to_radians().cos().into())),
            unary_fn!("tan", |x| Ok(x[0].to_float()?.to_radians().tan().into())),
            unary_fn!("sec", |x| Ok(x[0].to_float()?.to_radians().sin().recip().into())),
            unary_fn!("csc", |x| Ok(x[0].to_float()?.to_radians().cos().recip().into())),
            unary_fn!("cot", |x| Ok(x[0].to_float()?.to_radians().tan().recip().into())),
            unary_fn!("asin", |x| match x[0].to_float()? {
                n if (-1. ..=1.).contains(&n) => Ok(n.asin().to_degrees().into()),
                _ => Err(EvalError::MathDomain("the domain of asin is [-1, 1]".to_string()))
            }),
            unary_fn!("acos", |x| match x[0].to_float()? {
                n if (-1. ..=1.).contains(&n) => Ok(n.acos().to_degrees().into()),
                _ => Err(EvalError::MathDomain("the domain of asin is [-1, 1]".to_string()))
            }),
            unary_fn!("atan", |x| Ok(x[0].to_float()?.atan().to_degrees().into())),
        ],
    });

    for (name, function) in [
        unary_fn!("sinh", |x| Ok(x[0].to_float()?.sinh().into())),
        unary_fn!("cosh", |x| Ok(x[0].to_float()?.cosh().into())),
        unary_fn!("tanh", |x| Ok(x[0].to_float()?.tanh().into())),
        unary_fn!("sqrt", |x| match x[0].to_float()? {
            n if 0. <= n => Ok(n.sqrt().into()),
            _ => Err(EvalError::MathDomain("the domain of sqrt is [0, infinity)".to_string()))
        }),
        unary_fn!("exp", |x| Ok(x[0].to_float()?.exp().into())),
        unary_fn!("exp2", |x| Ok(x[0].to_float()?.exp2().into())),
        unary_fn!("ln", |x| match x[0].to_float()? {
            n if 0. < n => Ok(n.ln().into()),
            _ => Err(EvalError::MathDomain("the domain of ln is (0, infinity)".to_string()))
        }),
        unary_fn!("log2", |x| match x[0].to_float()? {
            n if 0. < n => Ok(n.log2().into()),
            _ => Err(EvalError::MathDomain("the domain of log2 is (0, infinity)".to_string()))
        }),
        unary_fn!("log10", |x| match x[0].to_float()? {
            n if 0. < n => Ok(n.log10().into()),
            _ => Err(EvalError::MathDomain("the domain of log10 is (0, infinity)".to_string()))
        }),
        unary_fn!("rad", |x| Ok(x[0].to_float()?.to_radians().into())),
        unary_fn!("deg", |x| Ok(x[0].to_float()?.to_degrees().into())),
        unary_fn!("floor", |x| Ok((x[0].to_float()?.floor() as i32).into())),
        unary_fn!("ceil", |x| Ok((x[0].to_float()?.ceil() as i32).into())),
        unary_fn!("round", |x| Ok((x[0].to_float()?.round() as i32).into())),
        unary_fn!("abs", |x| Ok(x[0].abs())),
        binary_fn!("nroot", |x| match (x[0].to_float()?, x[1].to_float()?) {
            (x, n) if 0. <= x && n != 0. => Ok(x.powf(n.recip()).into()),
            _ => Err(EvalError::MathDomain("the domain of nroot is [0, infinity) x (R \\ {0})".to_string()))
        }),
        binary_fn!("log", |x| match x[0].to_float()? {
            n if 0. < n => Ok(n.log(x[1].to_float()?).into()),
            _ => Err(EvalError::MathDomain("the domain of log10 is (0, infinity) x R".to_string()))
        }),
    ] {
        functions.insert(name, function);
    }

    use std::f64::consts::{E, PI, TAU};
    let variables = [
        (String::from("e"), Variable::External(E.into())),
        (String::from("pi"), Variable::External(PI.into())),
        (String::from("tau"), Variable::External(TAU.into())),
    ]
    .into();

    Context::new(functions, variables)
}
