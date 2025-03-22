use super::EvalError;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Value(Rc<ValueInner>);

impl Value {
    pub fn null() -> Value {
        Value(Rc::new(ValueInner::Null))
    }

    pub fn print(&self, fix: Option<usize>) {
        use ValueInner::*;
        match &*self.0 {
            Int(n) => println!("{}", n),
            Float(n) => {
                if let Some(fix) = fix {
                    println!("{:.*}", fix, n);
                } else {
                    println!("{}", n);
                }
            }
            Null => {}
        }
    }

    pub fn to_float(&self) -> Result<f64, EvalError> {
        match &*self.0.clone() {
            ValueInner::Int(x) => Ok(f64::from(*x)),
            ValueInner::Float(x) => Ok(*x),
            v => Err(EvalError::InvalidConversion(
                v.type_name(),
                String::from("Float"),
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ValueInner {
    Null,
    Int(i32),
    Float(f64),
}

impl ValueInner {
    pub fn type_name(&self) -> String {
        use ValueInner::*;
        match self {
            Null => String::from("Null"),
            Int(_) => String::from("Integer"),
            Float(_) => String::from("Float"),
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value(Rc::new(ValueInner::Int(value)))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value(Rc::new(ValueInner::Float(value)))
    }
}

macro_rules! define_binop {
    ($trait:ident, $fname:ident) => {
        impl $trait<Value> for Value {
            type Output = Value;
            fn $fname(self, rhs: Value) -> Self::Output {
                use ValueInner::*;
                match (&*self.0, &*rhs.0) {
                    (Int(x), Int(y)) => x.$fname(y).into(),
                    (Float(x), Int(y)) => x.$fname(f64::from(*y)).into(),
                    (Int(x), Float(y)) => f64::from(*x).$fname(y).into(),
                    (Float(x), Float(y)) => x.$fname(y).into(),
                    _ => unimplemented!(),
                }
            }
        }
    };
}

use std::ops::{Add, Div, Mul, Neg, Sub};
define_binop!(Add, add);
define_binop!(Sub, sub);
define_binop!(Mul, mul);

impl Div<Value> for Value {
    type Output = Result<Value, EvalError>;
    fn div(self, rhs: Value) -> Self::Output {
        use ValueInner::*;
        match (&*self.0, &*rhs.0) {
            (Int(x), Int(y)) => {
                if *y == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(f64::from(*x).div(f64::from(*y)).into())
            }
            (Float(x), Int(y)) => {
                if *y == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.div(f64::from(*y)).into())
            }
            (Int(x), Float(y)) => {
                if *y == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(f64::from(*x).div(y).into())
            }
            (Float(x), Float(y)) => {
                if *y == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.div(y).into())
            }
            _ => unimplemented!(),
        }
    }
}

impl Value {
    pub fn rem_euclid(self, rhs: Value) -> Result<Value, EvalError> {
        use ValueInner::*;
        match (&*self.0, &*rhs.0) {
            (Int(x), Int(y)) => {
                if *y == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.rem_euclid(*y).into())
            }
            (Float(x), Int(y)) => {
                if *y == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.rem_euclid(f64::from(*y)).into())
            }
            (Int(x), Float(y)) => {
                if *y == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(f64::from(*x).rem_euclid(*y).into())
            }
            (Float(x), Float(y)) => {
                if *y == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.rem_euclid(*y).into())
            }
            _ => unimplemented!(),
        }
    }

    pub fn pow(self, rhs: Value) -> Self {
        use ValueInner::*;
        match (&*self.0, &*rhs.0) {
            (Int(x), Int(y)) => {
                if *y < 0 {
                    f64::from(*x).powi(*y).into()
                } else {
                    (*x).pow(*y as u32).into()
                }
            }
            (Float(x), Int(y)) => x.powi(*y).into(),
            (Int(x), Float(y)) => f64::from(*x).powf(*y).into(),
            (Float(x), Float(y)) => x.powf(*y).into(),
            _ => unimplemented!(),
        }
    }

    pub fn factorial(&self) -> Result<Value, EvalError> {
        use ValueInner::*;
        match &*self.0 {
            Int(n) => {
                let n = *n;
                if n < 0 {
                    return Err(EvalError::MathDomain(
                        "the argument must be a non-negative integer".to_string(),
                    ));
                }
                let result = (1..=n).try_fold(1_i32, |acc, x| acc.checked_mul(x));
                match result {
                    Some(n) => Ok(n.into()),
                    None => Err(EvalError::Overflow),
                }
            }
            v => Err(EvalError::TypeError(String::from("Integer"), v.type_name())),
        }
    }

    pub fn abs(&self) -> Value {
        use ValueInner::*;
        match &*self.0 {
            Int(n) => n.abs().into(),
            Float(n) => n.abs().into(),
            _ => unimplemented!(),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        use ValueInner::*;
        match &*self.0 {
            Int(x) => x.neg().into(),
            Float(x) => x.neg().into(),
            _ => unimplemented!(),
        }
    }
}
