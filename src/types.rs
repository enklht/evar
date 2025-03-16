#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    UnaryFunctionCall {
        function: UnaryFunction,
        arg: Box<Expr>,
    },
    BinaryFunctionCall {
        function: BinaryFunction,
        arg1: Box<Expr>,
        arg2: Box<Expr>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        arg: Box<Expr>,
    },
    BinaryOperation {
        operator: BinaryOperator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum UnaryFunction {
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

#[derive(Debug, PartialEq)]
pub enum BinaryFunction {
    Log,
    NRoot,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Neg,
    Fac,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}
