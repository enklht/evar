use pest::Parser;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};
use pest_derive::Parser;
use std::sync::OnceLock;

static PRATT_PARSER: OnceLock<PrattParser<Rule>> = OnceLock::new();

pub fn init_parser() -> Result<(), PrattParser<Rule>> {
    PRATT_PARSER.set({
        use Rule::*;
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left) | Op::infix(rem, Left))
            .op(Op::infix(pow, Right))
            .op(Op::postfix(fac))
            .op(Op::prefix(Rule::neg))
    })
}

#[derive(Debug)]
pub enum Expr {
    Int(isize),
    Float(f64),
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

#[derive(Debug)]
pub enum UnaryFunction {
    Sin,
}

#[derive(Debug)]
pub enum BinaryFunction {
    Log,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Neg,
    Fac,
    Power,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

macro_rules! unop {
    ($op_name:ident, $val:expr) => {
        Ok(Expr::UnaryOperation {
            operator: UnaryOperator::$op_name,
            arg: $val?.into(),
        })
    };
}

macro_rules! binop {
    ($op_name:ident, $lhs:expr, $rhs:expr) => {
        Ok(Expr::BinaryOperation {
            operator: BinaryOperator::$op_name,
            lhs: $lhs?.into(),
            rhs: $rhs?.into(),
        })
    };
}

fn parse_number(s: &str) -> Expr {
    match s.parse::<isize>() {
        Ok(i) => Expr::Int(i),
        Err(_) => Expr::Float(s.parse().unwrap()),
    }
}

fn parse_unary_function(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let name = pair.next().unwrap().as_str();
    let arg = parse_expr(pair.next().unwrap().into_inner())?;

    Ok(Expr::UnaryFunctionCall {
        function: match name {
            "sin" => UnaryFunction::Sin,
            _ => todo!("unimplemented unary function"),
        },
        arg: Box::new(arg),
    })
}

fn parse_binary_function(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    let mut pair = pair.into_inner();
    let name = pair.next().unwrap().as_str();
    let arg1 = parse_expr(pair.next().unwrap().into_inner())?;
    let arg2 = parse_expr(pair.next().unwrap().into_inner())?;

    Ok(Expr::BinaryFunctionCall {
        function: match name {
            "log" => BinaryFunction::Log,
            _ => todo!("unimplemented unary function"),
        },
        arg1: Box::new(arg1),
        arg2: Box::new(arg2),
    })
}

fn parse_expr(pairs: Pairs<Rule>) -> Result<Expr, Error<Rule>> {
    PRATT_PARSER
        .get()
        .unwrap()
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Ok(parse_number(primary.as_str())),
            Rule::binaryfncall => parse_binary_function(primary),
            Rule::unaryfncall => parse_unary_function(primary),
            Rule::expr => parse_expr(primary.into_inner()),
            _ => unreachable!(),
        })
        .map_prefix(|op, val| match op.as_rule() {
            Rule::neg => unop!(Neg, val),
            _ => unreachable!(),
        })
        .map_postfix(|val, op| match op.as_rule() {
            Rule::fac => unop!(Fac, val),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => binop!(Add, lhs, rhs),
            Rule::sub => binop!(Sub, lhs, rhs),
            Rule::mul => binop!(Mul, lhs, rhs),
            Rule::div => binop!(Div, lhs, rhs),
            Rule::rem => binop!(Rem, lhs, rhs),
            Rule::pow => binop!(Pow, lhs, rhs),
            _ => unreachable!(),
        })
        .parse(pairs)
}

pub fn parse(input: &str) -> Result<Expr, Error<Rule>> {
    parse_expr(
        ExprParser::parse(Rule::equation, input)?
            .next()
            .unwrap()
            .into_inner(),
    )
}
