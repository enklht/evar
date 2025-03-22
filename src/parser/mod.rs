#[cfg(test)]
mod test;

use crate::models::{Expr, Stmt, Token, operators::*};

use chumsky::input::ValueInput;
use chumsky::prelude::*;

pub fn parser<'a, I>() -> impl Parser<'a, I, Stmt, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    function_definition()
        .or(variable_definition())
        .or(expression().map(Stmt::Expr))
}

pub fn function_definition<'a, I>() -> impl Parser<'a, I, Stmt, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let ident = select! {
        Token::Ident(ident) => ident.to_string()
    }
    .boxed()
    .labelled("ident");

    just(Token::Let)
        .ignore_then(ident.clone())
        .then_ignore(just(Token::LParen))
        .then(ident.separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::RParen))
        .then_ignore(just(Token::Equal))
        .then(expression())
        .map(|((name, arg_names), body)| Stmt::DefFun {
            name,
            arg_names,
            body,
        })
        .labelled("function definition")
        .as_context()
}

pub fn variable_definition<'a, I>() -> impl Parser<'a, I, Stmt, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let ident = select! {
        Token::Ident(ident) => ident.to_string()
    }
    .boxed()
    .labelled("ident");

    just(Token::Let)
        .ignore_then(ident)
        .then_ignore(just(Token::Equal))
        .then(expression())
        .map(|(name, expr)| Stmt::DefVar { name, expr })
        .labelled("variable definition")
        .as_context()
}

pub fn expression<'a, I>() -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let number = just(Token::Minus)
        .ignore_then(select! {
            Token::Int(n) => Expr::Int(-n),
            Token::Float(n) => Expr::Float(-n)
        })
        .or(select! {
        Token::Int(n) => Expr::Int(n),
        Token::Float(n) => Expr::Float(n)
        })
        .labelled("number")
        .boxed();

    recursive(|expr| {
        let fn_call = select! {
            Token::Ident(ident) => ident.to_string()
        }
        .labelled("ident")
        .then_ignore(just(Token::LParen))
        .then(expr.clone().separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::RParen))
        .map(|(name, args)| Expr::FnCall { name, args })
        .boxed();

        let variable = select! {
            Token::Ident(ident) => Expr::Variable(ident.to_string())
        }
        .labelled("ident");

        let atomic = choice((
            number.clone(),
            fn_call,
            variable,
            just(Token::Underscore).map(|_| Expr::PrevAnswer),
            expr.clone()
                .delimited_by(just(Token::LParen), just(Token::RParen)),
        ))
        .boxed();

        let postfixed = atomic
            .clone()
            .then(choice((just(Token::Exclamation).to(PostfixOp::Fac),)))
            .map(|(lhs, op)| Expr::PostfixOp {
                op,
                arg: Box::new(lhs),
            })
            .or(atomic.clone())
            .boxed();

        let prefixed = postfixed
            .clone()
            .or(choice((just(Token::Minus).to(PrefixOp::Neg),))
                .then(postfixed.clone())
                .map(|(op, rhs)| Expr::PrefixOp {
                    op,
                    arg: Box::new(rhs),
                }))
            .boxed();

        let term = prefixed.labelled("term").boxed();

        let power = term
            .clone()
            .then(just(Token::Caret).to(InfixOp::Pow))
            .repeated()
            .foldr(term, |(lhs, op), rhs| Expr::InfixOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
            .boxed();

        let powers = power
            .clone()
            .foldl(
                select! {
                            Token::Minus | Token::Int(_) | Token::Float(_)
                }
                .not()
                .rewind()
                .ignore_then(power)
                .repeated(),
                |lhs, rhs| Expr::InfixOp {
                    op: InfixOp::Mul,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            )
            .boxed();

        let product = powers
            .clone()
            .foldl(
                choice((
                    just(Token::Asterisk).to(InfixOp::Mul),
                    just(Token::Slash).to(InfixOp::Div),
                    just(Token::Percent).to(InfixOp::Rem),
                ))
                .then(powers)
                .repeated(),
                |lhs, (op, rhs)| Expr::InfixOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            )
            .boxed();

        let sum = product
            .clone()
            .foldl(
                choice((
                    just(Token::Plus).to(InfixOp::Add),
                    just(Token::Minus).to(InfixOp::Sub),
                ))
                .then(product)
                .repeated(),
                |lhs, (op, rhs)| Expr::InfixOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            )
            .boxed();

        sum.labelled("expression").as_context()
    })
}
