#[derive(Debug, PartialEq, Clone)]
pub enum PrefixOp {
    Neg,
}

impl std::fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            PrefixOp::Neg => "-",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOp {
    Fac,
}

impl std::fmt::Display for PostfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            PostfixOp::Fac => "!",
        };
        write!(f, "{}", op_str)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}

impl std::fmt::Display for InfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            InfixOp::Add => "+",
            InfixOp::Sub => "-",
            InfixOp::Mul => "*",
            InfixOp::Div => "/",
            InfixOp::Rem => "%",
            InfixOp::Pow => "^",
        };
        write!(f, "{}", op_str)
    }
}
