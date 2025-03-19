use derive_more::Display;

#[derive(Debug, PartialEq, Display)]
pub enum Expr {
    Number(f64),

    #[display(
        "{}({})",
        fname,
        args.iter()
            .map(|x| x.to_string())
            .reduce(|acc, x| acc + ", " + &x)
            .unwrap_or("".into())
    )]
    FnCall {
        fname: String,
        args: Vec<Expr>,
    },

    #[display("({} {})", arg, op)]
    UnaryOp {
        op: UnaryOp,
        arg: Box<Expr>,
    },

    #[display("({} {} {})", lhs, op, rhs)]
    BinaryOp {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone, Display)]
pub enum UnaryOp {
    #[display("neg")]
    Neg,
    #[display("!")]
    Fac,
}

#[derive(Debug, PartialEq, Clone, Display)]
pub enum BinaryOp {
    #[display("+")]
    Add,
    #[display("-")]
    Sub,
    #[display("*")]
    Mul,
    #[display("/")]
    Div,
    #[display("%")]
    Rem,
    #[display("^")]
    Pow,
}
