use std::collections::HashMap;

use crate::{
    args::AngleUnit,
    models::{Context, Function, Variable},
};

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
            unary_fn!("sin", |x| f64::from(&x[0]).sin().into()),
            unary_fn!("cos", |x| f64::from(&x[0]).cos().into()),
            unary_fn!("tan", |x| f64::from(&x[0]).tan().into()),
            unary_fn!("sec", |x| f64::from(&x[0]).sin().recip().into()),
            unary_fn!("csc", |x| f64::from(&x[0]).cos().recip().into()),
            unary_fn!("cot", |x| f64::from(&x[0]).tan().recip().into()),
            unary_fn!("asin", |x| f64::from(&x[0]).asin().into()),
            unary_fn!("acos", |x| f64::from(&x[0]).acos().into()),
            unary_fn!("atan", |x| f64::from(&x[0]).atan().into()),
            unary_fn!("asec", |x| f64::from(&x[0]).recip().asin().into()),
            unary_fn!("acsc", |x| f64::from(&x[0]).recip().acos().into()),
            unary_fn!("acot", |x| f64::from(&x[0]).recip().atan().into()),
        ],
        AngleUnit::Degree => [
            unary_fn!("sin", |x| f64::from(&x[0]).to_radians().sin().into()),
            unary_fn!("cos", |x| f64::from(&x[0]).to_radians().cos().into()),
            unary_fn!("tan", |x| f64::from(&x[0]).to_radians().tan().into()),
            unary_fn!("sec", |x| f64::from(&x[0]).to_radians().sin().recip().into()),
            unary_fn!("csc", |x| f64::from(&x[0]).to_radians().cos().recip().into()),
            unary_fn!("cot", |x| f64::from(&x[0]).to_radians().tan().recip().into()),
            unary_fn!("asin", |x| f64::from(&x[0]).asin().to_degrees().into()),
            unary_fn!("acos", |x| f64::from(&x[0]).acos().to_degrees().into()),
            unary_fn!("atan", |x| f64::from(&x[0]).atan().to_degrees().into()),
            unary_fn!("asec", |x| f64::from(&x[0]).recip().asin().to_degrees().into()),
            unary_fn!("acsc", |x| f64::from(&x[0]).recip().acos().to_degrees().into()),
            unary_fn!("acot", |x| f64::from(&x[0]).recip().atan().to_degrees().into()),
        ],
    });

    for (name, function) in [
        unary_fn!("sinh", |x| f64::from(&x[0]).sinh().into()),
        unary_fn!("cosh", |x| f64::from(&x[0]).cosh().into()),
        unary_fn!("tanh", |x| f64::from(&x[0]).tanh().into()),
        unary_fn!("sqrt", |x| f64::from(&x[0]).sqrt().into()),
        unary_fn!("exp", |x| f64::from(&x[0]).exp().into()),
        unary_fn!("exp2", |x| f64::from(&x[0]).exp2().into()),
        unary_fn!("ln", |x| f64::from(&x[0]).ln().into()),
        unary_fn!("log2", |x| f64::from(&x[0]).log2().into()),
        unary_fn!("log10", |x| f64::from(&x[0]).log10().into()),
        unary_fn!("rad", |x| f64::from(&x[0]).to_radians().into()),
        unary_fn!("deg", |x| f64::from(&x[0]).to_degrees().into()),
        unary_fn!("floor", |x| (f64::from(&x[0]).floor() as i32).into()),
        unary_fn!("ceil", |x| (f64::from(&x[0]).ceil() as i32).into()),
        unary_fn!("round", |x| (f64::from(&x[0]).round() as i32).into()),
        unary_fn!("abs", |x| x[0].abs()),
        binary_fn!("log", |x| f64::from(&x[0]).log(f64::from(&x[1])).into()),
        binary_fn!("nroot", |x| f64::from(&x[0]).powf(f64::from(&x[1]).recip()).into()),
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
