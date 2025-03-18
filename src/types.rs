#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    FnCall {
        fname: String,
        args: Vec<Expr>,
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
