#[cfg(test)]
mod test;

use crate::models::{Expr, Stmt, operators::*};

use winnow::{
    ascii::{alpha1, alphanumeric0, dec_int, float, multispace0, multispace1},
    combinator::{
        alt, cut_err, delimited, not, preceded, repeat, separated, separated_foldl1,
        separated_foldr1, seq, terminated,
    },
    error::StrContext,
    prelude::*,
    token::one_of,
};

fn number(input: &mut &str) -> ModalResult<Expr> {
    alt((
        terminated(dec_int.map(Expr::Int), not(one_of(['.', 'e', 'E']))),
        float.map(Expr::Float),
    ))
    .parse_next(input)
}

fn ident(input: &mut &str) -> ModalResult<String> {
    (alpha1, alphanumeric0).take().parse_to().parse_next(input)
}

fn fn_call(input: &mut &str) -> ModalResult<Expr> {
    (ident, delimited('(', separated(0.., expression, ","), ')'))
        .map(|(name, args)| Expr::FnCall { name, args })
        .parse_next(input)
}

fn atomic(input: &mut &str) -> ModalResult<Expr> {
    alt((
        number,
        fn_call,
        ident.map(Expr::Variable),
        '_'.map(|_| Expr::PrevAnswer),
        delimited('(', expression, ')'),
    ))
    .parse_next(input)
}

fn postfixed(input: &mut &str) -> ModalResult<Expr> {
    alt((
        terminated(atomic, '!').map(|expr| Expr::PostfixOp {
            op: PostfixOp::Fac,
            arg: Box::new(expr),
        }),
        atomic,
    ))
    .parse_next(input)
}

fn prefixed(input: &mut &str) -> ModalResult<Expr> {
    alt((
        postfixed,
        preceded('-', postfixed).map(|expr| Expr::PrefixOp {
            op: PrefixOp::Neg,
            arg: Box::new(expr),
        }),
    ))
    .parse_next(input)
}

fn term(input: &mut &str) -> ModalResult<Expr> {
    delimited(multispace0, prefixed, multispace0).parse_next(input)
}

fn power(input: &mut &str) -> ModalResult<Expr> {
    separated_foldr1(
        term,
        alt(("^".value(InfixOp::Pow), "**".value(InfixOp::Pow))),
        |lhs, op, rhs| Expr::InfixOp {
            op: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
    )
    .parse_next(input)
}

fn powers(input: &mut &str) -> ModalResult<Expr> {
    (
        power,
        repeat(0.., preceded(not(alt(('-'.void(), number.void()))), power)),
    )
        .map(|(head, tail): (Expr, Vec<Expr>)| {
            tail.into_iter().fold(head, |acc, rhs| Expr::InfixOp {
                op: InfixOp::Mul,
                lhs: Box::new(acc),
                rhs: Box::new(rhs),
            })
        })
        .parse_next(input)
}

fn product(input: &mut &str) -> ModalResult<Expr> {
    separated_foldl1(
        powers,
        alt((
            '*'.value(InfixOp::Mul),
            '/'.value(InfixOp::Div),
            '%'.value(InfixOp::Rem),
        )),
        |lhs, op, rhs| Expr::InfixOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
    )
    .parse_next(input)
}

fn expression(input: &mut &str) -> ModalResult<Expr> {
    separated_foldl1(
        product,
        alt(('+'.value(InfixOp::Add), '-'.value(InfixOp::Sub))),
        |lhs, op, rhs| Expr::InfixOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
    )
    .parse_next(input)
}

fn function_definition(input: &mut &str) -> ModalResult<Stmt> {
    seq! (
        _: "let",
        _: multispace1,
        ident.context(StrContext::Expected("ident".into())),
        _: '(',
        separated(0.., ident, delimited(multispace0, ",", multispace0)),
        _: ')',
        _: multispace0,
        _: '=',
        _: multispace0,
        expression.context(StrContext::Expected("expression".into()))
    )
    .map(|(name, arg_names, body)| Stmt::DefFun {
        name,
        arg_names,
        body,
    })
    .parse_next(input)
}

fn variable_definition(input: &mut &str) -> ModalResult<Stmt> {
    preceded(
        "let",
        cut_err(seq! (
            _: multispace0,
            ident.context(StrContext::Expected("ident".into())),
            _: multispace0,
            _: '=',
            _: multispace0,
            expression.context(StrContext::Expected("expression".into()))
        )),
    )
    .context(StrContext::Label("variable definition".into()))
    .map(|(name, expr)| Stmt::DefVar { name, expr })
    .parse_next(input)
}

pub fn parser(input: &mut &str) -> ModalResult<Stmt> {
    delimited(
        multispace0,
        alt((
            function_definition,
            variable_definition,
            expression.map(Stmt::Expr),
        )),
        multispace0,
    )
    .parse_next(input)
}
