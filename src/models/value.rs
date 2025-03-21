use std::{
    ops::{Add, Neg},
    rc::Rc,
};

use super::EvalError;

#[derive(Debug, Clone, PartialEq)]
pub struct Value(Rc<ValueInner>);

impl Value {
    pub fn null() -> Value {
        Value(Rc::new(ValueInner::Null))
    }

    pub fn factorial(&self) -> Result<Value, EvalError> {
        use ValueInner::*;
        match &*self.0 {
            // todo this factorial is not correct at all
            Int(n) => Ok(Value::from(*n)),
            v => Err(EvalError::TypeError {
                expected: "Integer".into(),
                found: v.type_name(),
            }),
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
            Null => "Null".into(),
            Int(_) => "Integer".into(),
            Float(_) => "Float".into(),
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

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Self::Output {
        use ValueInner::*;
        match (&*self.0, &*rhs.0) {
            (Int(x), Int(y)) => (x + y).into(),
            (Float(x), Int(y)) => (x + *y as f64).into(),
            (Int(x), Float(y)) => (*x as f64 + y).into(),
            (Float(x), Float(y)) => (x + y).into(),
            _ => unimplemented!(),
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        use ValueInner::*;
        match &*self.0 {
            Int(x) => (-x).into(),
            Float(x) => (-x).into(),
            _ => unimplemented!(),
        }
    }
}
