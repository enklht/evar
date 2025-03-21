use std::collections::HashMap;

use crate::{
    args::AngleUnit,
    models::{Context, Function, Value, Variable},
};

macro_rules! unary_fn {
    ($fname:ident) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 1,
                body: |x| x.first().unwrap().$fname(),
            },
        )
    };
    ($fname:ident, $body:expr) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 1,
                body: $body,
            },
        )
    };
}

macro_rules! binary_fn {
    ($fname:ident) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 2,
                body: |x| x.first().unwrap().$fname(*x.get(1).unwrap()),
            },
        )
    };
    ($fname:ident, $body:expr) => {
        (
            stringify!($fname).into(),
            Function::External {
                arity: 2,
                body: $body,
            },
        )
    };
}

pub fn create_context(angle_unit: &AngleUnit) -> Context {
    let mut functions = HashMap::from(match angle_unit {
        AngleUnit::Radian => [
            // unary_fn!(sin),
            // unary_fn!(cos),
            // unary_fn!(tan),
            // unary_fn!(sec, |x| x.first().unwrap().sin().recip()),
            // unary_fn!(csc, |x| x.first().unwrap().cos().recip()),
            // unary_fn!(cot, |x| x.first().unwrap().tan().recip()),
            // unary_fn!(asin),
            // unary_fn!(acos),
            // unary_fn!(atan),
        ],
        AngleUnit::Degree => [
            // unary_fn!(sin, |x| x.first().unwrap().to_radians().sin()),
            // unary_fn!(cos, |x| x.first().unwrap().to_radians().cos()),
            // unary_fn!(tan, |x| x.first().unwrap().to_radians().tan()),
            // unary_fn!(sec, |x| x.first().unwrap().to_radians().sin().recip()),
            // unary_fn!(csc, |x| x.first().unwrap().to_radians().cos().recip()),
            // unary_fn!(cot, |x| x.first().unwrap().to_radians().tan().recip()),
            // unary_fn!(asin, |x| x.first().unwrap().asin().to_degrees()),
            // unary_fn!(acos, |x| x.first().unwrap().acos().to_degrees()),
            // unary_fn!(atan, |x| x.first().unwrap().atan().to_degrees()),
        ],
    });

    for (name, function) in [
        // unary_fn!(sinh),
        // unary_fn!(cosh),
        // unary_fn!(tanh),
        // unary_fn!(floor),
        // unary_fn!(ceil),
        // unary_fn!(round),
        // unary_fn!(abs),
        // unary_fn!(sqrt),
        // unary_fn!(exp),
        // unary_fn!(exp2),
        // unary_fn!(ln),
        // unary_fn!(log2),
        // unary_fn!(log10),
        // unary_fn!(rad, |x| x.first().unwrap().to_radians()),
        // unary_fn!(deg, |x| x.first().unwrap().to_degrees()),
        // binary_fn!(log),
        // binary_fn!(nroot, |x| x
        //     .first()
        //     .unwrap()
        //     .powf(x.get(1).unwrap().recip())),
    ] {
        functions.insert(name, function);
    }

    let variables = [
        (
            "e".to_string(),
            Variable::External(std::f64::consts::E.into()),
        ),
        (
            "pi".to_string(),
            Variable::External(std::f64::consts::PI.into()),
        ),
        (
            "tau".to_string(),
            Variable::External(std::f64::consts::TAU.into()),
        ),
    ]
    .into();

    Context::new(functions, variables)
}
