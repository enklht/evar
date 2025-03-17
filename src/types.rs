#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    UnaryFnCall {
        function: UnaryFn,
        arg: Box<Expr>,
    },
    BinaryFnCall {
        function: BinaryFn,
        arg1: Box<Expr>,
        arg2: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        arg: Box<Expr>,
    },
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryFn {
    Sin,
    Cos,
    Tan,
    Sec,
    Csc,
    Cot,
    Asin,
    Acos,
    Atan,
    Asec,
    Acsc,
    Acot,
    Sinh,
    Cosh,
    Tanh,
    Floor,
    Ceil,
    Round,
    Abs,
    Sqrt,
    Exp,
    Exp2,
    Ln,
    Log10,
    Rad,
    Deg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryFn {
    Log,
    NRoot,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg,
    Fac,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}
