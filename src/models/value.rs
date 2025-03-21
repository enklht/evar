use rug::{Complete, Float, Integer, Rational, ops::CompleteRound};
use std::rc::Rc;

use super::EvalError;

#[derive(Debug, Clone, PartialEq)]
pub struct Value(Rc<ValueInner>);

impl Value {
    pub fn null() -> Value {
        Value(Rc::new(ValueInner::Null))
    }
}

#[derive(Debug, PartialEq)]
enum ValueInner {
    Null,
    Integer(Integer),
    Rational(Rational),
    Float(Float),
}

impl ValueInner {
    pub fn type_name(&self) -> String {
        use ValueInner::*;
        match self {
            Null => String::from("Null"),
            Integer(_) => String::from("Integer"),
            Rational(_) => String::from("Rational"),
            Float(_) => String::from("Float"),
        }
    }
}

impl From<Integer> for Value {
    fn from(value: Integer) -> Self {
        Value(Rc::new(ValueInner::Integer(value)))
    }
}

impl From<Rational> for Value {
    fn from(value: Rational) -> Self {
        Value(Rc::new(ValueInner::Rational(value)))
    }
}

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Value(Rc::new(ValueInner::Float(value)))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value(Rc::new(ValueInner::Integer(Integer::from(value))))
    }
}

macro_rules! define_binop {
    ($trait:ident, $fname:ident) => {
        impl $trait<Value> for Value {
            type Output = Value;
            fn $fname(self, rhs: Value) -> Self::Output {
                use ValueInner::*;
                match (&*self.0, &*rhs.0) {
                    (Integer(x), Integer(y)) => x.$fname(y).complete().into(),
                    // (Float(x), Integer(y)) => x.$fname(f64::from(*y)).into(),
                    // (Integer(x), Float(y)) => f64::from(*x).$fname(y).into(),
                    // (Float(x), Float(y)) => x.$fname(y).into(),
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
        use ValueInner as V;
        match (&*self.0, &*rhs.0) {
            (V::Integer(x), V::Integer(y)) => {
                if *y == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(Rational::from((x, y)).into())
            }
            // (V::Float(x), V::Integer(y)) => {
            //     if *y == 0 {
            //         return Err(EvalError::DivisionByZero);
            //     }
            //     Ok(x.div(Float::unwrapped_cast_from(y)).into())
            // }
            // (V::Integer(x), V::Float(y)) => {
            //     if *y == 0.0 {
            //         return Err(EvalError::DivisionByZero);
            //     }
            //     Ok(x.to_f64().div(y).into())
            // }
            (V::Float(x), V::Float(y)) => {
                if *y == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.div(y).complete(53).into())
            }
            _ => unimplemented!(),
        }
    }
}

impl Value {
    pub fn rem_euclid(self, rhs: Value, prec: ) -> Result<Value, EvalError> {
        use ValueInner::*;
        match (&*self.0, &*rhs.0) {
            (Integer(x), Integer(y)) => {
                if *y == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                let (_, rem) = x.clone().div_rem_euc(y.clone());
                Ok(rem.into())
            }
            (Float(x), Integer(y)) => {
                if *y == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.rem_euclid(y.to_f64()).into())
            }
            (Integer(x), Float(y)) => {
                if *y == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(x.to_f64().rem_euclid(*y).into())
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
            (Integer(x), Integer(y)) => {
                if *y < 0 {
                    x.to_f64().powi(y.to_i32().unwrap()).into()
                } else {
                    x.pow(y.to_u32().unwrap()).into()
                }
            }
            (Float(x), Integer(y)) => x.powi(y.to_i32().unwrap()).into(),
            (Integer(x), Float(y)) => f64::from(*x).powf(*y).into(),
            (Float(x), Float(y)) => x.powf(*y).into(),
            _ => unimplemented!(),
        }
    }

    pub fn factorial(&self) -> Result<Value, EvalError> {
        use ValueInner::*;
        match &*self.0 {
            Integer(n) => {
                let n = *n;
                if n < 0 {
                    return Err(EvalError::MathDomain);
                }
                let result = (1..=n).try_fold(1_i32, |acc, x| acc.checked_mul(x));
                match result {
                    Some(n) => Ok(n.into()),
                    None => Err(EvalError::Overflow),
                }
            }
            v => Err(EvalError::TypeError {
                expected: String::from("Integer"),
                found: v.type_name(),
            }),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        use ValueInner::*;
        match &*self.0 {
            Integer(x) => x.neg().complete().into(),
            Float(x) => x.neg().into(),
            _ => unimplemented!(),
        }
    }
}
