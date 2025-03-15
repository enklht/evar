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
}

#[derive(Debug, PartialEq)]
pub enum BinaryFunction {
    Log,
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
