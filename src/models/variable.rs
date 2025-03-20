#[derive(Clone)]
pub enum Variable {
    External(f64),
    Internal(f64),
}

impl Variable {
    pub fn get(&self) -> f64 {
        use Variable::*;
        match self {
            External(n) => *n,
            Internal(n) => *n,
        }
    }
}
