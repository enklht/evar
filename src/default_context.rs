use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    args::Args,
    models::{Function, FunctionContext, Variable, VariableContext},
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

pub fn create_context(args: &Args) -> (FunctionContext, Rc<RefCell<VariableContext>>) {
    use crate::args::AngleUnit;

    let mut functions = HashMap::from(match args.angle_unit {
        AngleUnit::Radian => [
            unary_fn!(sin),
            unary_fn!(cos),
            unary_fn!(tan),
            unary_fn!(sec, |x| x.first().unwrap().sin().recip()),
            unary_fn!(csc, |x| x.first().unwrap().cos().recip()),
            unary_fn!(cot, |x| x.first().unwrap().tan().recip()),
            unary_fn!(asin),
            unary_fn!(acos),
            unary_fn!(atan),
        ],
        AngleUnit::Degree => [
            unary_fn!(sin, |x| x.first().unwrap().to_radians().sin()),
            unary_fn!(cos, |x| x.first().unwrap().to_radians().cos()),
            unary_fn!(tan, |x| x.first().unwrap().to_radians().tan()),
            unary_fn!(sec, |x| x.first().unwrap().to_radians().sin().recip()),
            unary_fn!(csc, |x| x.first().unwrap().to_radians().cos().recip()),
            unary_fn!(cot, |x| x.first().unwrap().to_radians().tan().recip()),
            unary_fn!(asin, |x| x.first().unwrap().asin().to_degrees()),
            unary_fn!(acos, |x| x.first().unwrap().acos().to_degrees()),
            unary_fn!(atan, |x| x.first().unwrap().atan().to_degrees()),
        ],
    });

    for (fname, function) in [
        unary_fn!(sinh),
        unary_fn!(cosh),
        unary_fn!(tanh),
        unary_fn!(floor),
        unary_fn!(ceil),
        unary_fn!(round),
        unary_fn!(abs),
        unary_fn!(sqrt),
        unary_fn!(exp),
        unary_fn!(exp2),
        unary_fn!(ln),
        unary_fn!(log2),
        unary_fn!(log10),
        unary_fn!(rad, |x| x.first().unwrap().to_radians()),
        unary_fn!(deg, |x| x.first().unwrap().to_degrees()),
        binary_fn!(log),
        binary_fn!(nroot, |x| x
            .first()
            .unwrap()
            .powf(x.get(1).unwrap().recip())),
    ] {
        functions.insert(fname, function);
    }

    let variables = [
        ("e".to_string(), Variable::External(std::f64::consts::E)),
        ("pi".to_string(), Variable::External(std::f64::consts::PI)),
        ("tau".to_string(), Variable::External(std::f64::consts::TAU)),
    ]
    .into();

    (
        FunctionContext::new(functions),
        Rc::new(RefCell::new(VariableContext::new(variables))),
    )
}
