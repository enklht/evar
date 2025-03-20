use crate::lexer::Token;
use crate::types::*;

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
    .padded_by(just(Token::Space).or_not())
    .boxed()
    .labelled("ident");

    just(Token::Let)
        .ignore_then(just(Token::Space))
        .ignore_then(ident.clone())
        .then_ignore(just(Token::LParen))
        .then(ident.separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::RParen))
        .then_ignore(just(Token::Equal).padded_by(just(Token::Space).or_not()))
        .then(expression())
        .map(|((name, args), body)| Stmt::DefFun { name, args, body })
        .labelled("function definition")
        .as_context()
}

pub fn variable_definition<'a, I>() -> impl Parser<'a, I, Stmt, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    just(Token::Let)
        .ignore_then(just(Token::Space))
        .ignore_then(select! {
            Token::Ident(ident) => ident.to_string()
        })
        .then_ignore(just(Token::Equal).padded_by(just(Token::Space).or_not()))
        .then(expression())
        .map(|(name, expr)| Stmt::DefVar { name, expr })
        .labelled("variable definition")
        .as_context()
}

pub fn expression<'a, I>() -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let whitespace = just(Token::Space).or_not();

    let number = just(Token::Minus)
        .or_not()
        .then(select! {
            Token::Number(n) => n
        })
        .map(|(sign, n)| match sign {
            Some(_) => Expr::Number(-n),
            None => Expr::Number(n),
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

        let term = prefixed
            .padded_by(whitespace.clone())
            .labelled("term")
            .boxed();

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
                power.and_is(just(Token::Minus).not()).repeated(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;

    macro_rules! preop {
        ($op_name:ident, $val:expr) => {
            Expr::PrefixOp {
                op: super::PrefixOp::$op_name,
                arg: $val.into(),
            }
        };
    }

    macro_rules! proop {
        ($op_name:ident, $val:expr) => {
            Expr::PostfixOp {
                op: super::PostfixOp::$op_name,
                arg: $val.into(),
            }
        };
    }

    macro_rules! binop {
        ($op_name:ident, $lhs:expr, $rhs:expr) => {
            Expr::InfixOp {
                op: super::InfixOp::$op_name,
                lhs: $lhs.into(),
                rhs: $rhs.into(),
            }
        };
    }

    fn parse_expr(input: &str) -> Result<Expr, String> {
        use crate::lexer::Token;
        use chumsky::input::Stream;
        use logos::Logos;

        let token_iter = Token::lexer(input).spanned().map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

        let token_stream =
            Stream::from_iter(token_iter.clone()).map((0..input.len()).into(), |x| x);

        expression().parse(token_stream).into_result().map_err(|_| {
            format!(
                "failed to parse {:?}",
                token_iter.map(|(tok, _span)| tok).collect::<Vec<_>>()
            )
        })
    }

    fn parse_stmt(input: &str) -> Result<Stmt, String> {
        use crate::lexer::Token;
        use chumsky::input::Stream;
        use logos::Logos;

        let token_iter = Token::lexer(input).spanned().map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

        let token_stream =
            Stream::from_iter(token_iter.clone()).map((0..input.len()).into(), |x| x);

        parser().parse(token_stream).into_result().map_err(|_| {
            format!(
                "failed to parse {:?}",
                token_iter.map(|(tok, _span)| tok).collect::<Vec<_>>()
            )
        })
    }

    #[test]
    fn number() {
        assert_eq!(parse_expr("1"), Ok(Number(1.)));
        assert_eq!(parse_expr("   1"), Ok(Number(1.)));
        assert_eq!(parse_expr("0"), Ok(Number(0.)));
        assert_eq!(parse_expr("2.5"), Ok(Number(2.5)));
        assert_eq!(parse_expr("1e3"), Ok(Number(1e3)));
        assert_eq!(parse_expr("1e-3"), Ok(Number(1e-3)));
        assert_eq!(parse_expr("2.5e2"), Ok(Number(2.5e2)));
        assert_eq!(parse_expr("2.5e-2"), Ok(Number(2.5e-2)));
        assert_eq!(parse_expr("-1"), Ok(Number(-1.)));
        assert_eq!(parse_expr("-2.5"), Ok(Number(-2.5)));
        assert_eq!(parse_expr("-1e3"), Ok(Number(-1e3)));
        assert_eq!(parse_expr("-1e-3"), Ok(Number(-1e-3)));
        assert_eq!(parse_expr("-2.5e2"), Ok(Number(-2.5e2)));
        assert_eq!(parse_expr("-2.5e-2"), Ok(Number(-2.5e-2)));

        // Tests that should fail
        assert!(parse_expr("0.").is_err());
        assert!(parse_expr("1..2").is_err());
        assert!(parse_expr("2.5.2").is_err());
        assert!(parse_expr("1e3.5").is_err());
        assert!(parse_expr("1e3 .5").is_err());
        assert!(parse_expr("1e3. 5").is_err());
        assert!(parse_expr("1e3 . 5").is_err());
    }

    #[test]
    fn basic_ops() {
        assert_eq!(parse_expr("6*3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse_expr("6 * 3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse_expr("6* 3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse_expr("6 *3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse_expr("6+3"), Ok(binop!(Add, Number(6.), Number(3.))));
        assert_eq!(parse_expr("6-3"), Ok(binop!(Sub, Number(6.), Number(3.))));
        assert_eq!(parse_expr("6/3"), Ok(binop!(Div, Number(6.), Number(3.))));
        assert_eq!(parse_expr("6%3"), Ok(binop!(Rem, Number(6.), Number(3.))));
        assert_eq!(parse_expr("2^3"), Ok(binop!(Pow, Number(2.), Number(3.))));

        // Different number notations
        assert_eq!(
            parse_expr("1e3 + 2.5"),
            Ok(binop!(Add, Number(1e3), Number(2.5)))
        );
        assert_eq!(
            parse_expr("1e-3 * 2.5"),
            Ok(binop!(Mul, Number(1e-3), Number(2.5)))
        );
        assert_eq!(
            parse_expr("2.5e2 - 1"),
            Ok(binop!(Sub, Number(2.5e2), Number(1.)))
        );
        assert_eq!(
            parse_expr("2.5e-2 / 1e3"),
            Ok(binop!(Div, Number(2.5e-2), Number(1e3)))
        );
        assert_eq!(
            parse_expr("-1e3 + 2.5"),
            Ok(binop!(Add, Number(-1e3), Number(2.5)))
        );
        assert_eq!(
            parse_expr("2.5e2 % 1e3"),
            Ok(binop!(Rem, Number(2.5e2), Number(1e3)))
        );

        assert_eq!(
            parse_expr("2 + 3 * 4"),
            Ok(binop!(Add, Number(2.), binop!(Mul, Number(3.), Number(4.))))
        );
        assert_eq!(
            parse_expr("(2 + 3) * 4"),
            Ok(binop!(Mul, binop!(Add, Number(2.), Number(3.)), Number(4.)))
        );
        assert_eq!(
            parse_expr("2 * (3 + 4)"),
            Ok(binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))))
        );
        assert_eq!(
            parse_expr("2 * 3 + 4"),
            Ok(binop!(Add, binop!(Mul, Number(2.), Number(3.)), Number(4.)))
        );
        assert_eq!(
            parse_expr("2 + 3 * 4 - 5 / 6"),
            Ok(binop!(
                Sub,
                binop!(Add, Number(2.), binop!(Mul, Number(3.), Number(4.))),
                binop!(Div, Number(5.), Number(6.))
            ))
        );
        assert_eq!(
            parse_expr("2 * (3 + 4) - 5 % 6"),
            Ok(binop!(
                Sub,
                binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))),
                binop!(Rem, Number(5.), Number(6.))
            ))
        );
        assert_eq!(parse_expr("5!"), Ok(proop!(Fac, Number(5.))));
        assert_eq!(
            parse_expr("-(2 + 3)"),
            Ok(preop!(Neg, binop!(Add, Number(2.), Number(3.))))
        );
        assert_eq!(
            parse_expr("-(2 * 3) + 4"),
            Ok(binop!(
                Add,
                preop!(Neg, binop!(Mul, Number(2.), Number(3.))),
                Number(4.)
            ))
        );
        assert_eq!(
            parse_expr("2 * -(3 + 4)"),
            Ok(binop!(
                Mul,
                Number(2.),
                preop!(Neg, binop!(Add, Number(3.), Number(4.)))
            ))
        );
        assert_eq!(
            parse_expr("-(2 * 3 + 4)"),
            Ok(preop!(
                Neg,
                binop!(Add, binop!(Mul, Number(2.), Number(3.)), Number(4.))
            ))
        );
        assert_eq!(
            parse_expr("3! + 4"),
            Ok(binop!(Add, proop!(Fac, Number(3.)), Number(4.)))
        );
        assert_eq!(
            parse_expr("-(3!)"),
            Ok(preop!(Neg, proop!(Fac, Number(3.))))
        );
        assert_eq!(parse_expr("-3!"), Ok(proop!(Fac, Number(-3.))));
        assert_eq!(
            parse_expr("2 ^ 3!"),
            Ok(binop!(Pow, Number(2.), proop!(Fac, Number(3.))))
        );
        assert_eq!(
            parse_expr("-(2 ^ 3)"),
            Ok(preop!(Neg, binop!(Pow, Number(2.), Number(3.))))
        );
        assert_eq!(parse_expr("-2^3"), Ok(binop!(Pow, Number(-2.), Number(3.))));
        assert_eq!(
            parse_expr("2 ^ -3"),
            Ok(binop!(Pow, Number(2.), Number(-3.)))
        );
        assert_eq!(
            parse_expr("-(2 ^ -3)"),
            Ok(preop!(Neg, binop!(Pow, Number(2.), Number(-3.))))
        );
        assert_eq!(parse_expr("-(-3)"), Ok(preop!(Neg, Number(-3.))));
        assert_eq!(
            parse_expr("-2 (-3)"),
            Ok(binop!(Mul, Number(-2.), Number(-3.)))
        );
        assert_eq!(
            parse_expr("(5 + 3)  (-3)"),
            Ok(binop!(
                Mul,
                binop!(Add, Number(5.), Number(3.)),
                Number(-3.)
            ))
        );
        assert_eq!(
            parse_expr("(5 % 3)  2"),
            Ok(binop!(Mul, binop!(Rem, Number(5.), Number(3.)), Number(2.)))
        );
        assert_eq!(parse_expr("--1"), Ok(preop!(Neg, Number(-1.))));
        assert_eq!(parse_expr("--3"), Ok(preop!(Neg, Number(-3.))));

        // Failing tests
        assert!(parse_expr("2 ** 3").is_err());
        assert!(parse_expr("2 // 3").is_err());
        assert!(parse_expr("2 %% 3").is_err());
        assert!(parse_expr("2 ^^ 3").is_err());
        assert!(parse_expr("2 +* 3").is_err());
        assert!(parse_expr("2 *+ 3").is_err());
        assert!(parse_expr("2 + 3 *").is_err());
        assert!(parse_expr("2 * (3 + 4").is_err());
        assert!(parse_expr("2 * 3 + 4)").is_err());
        assert!(parse_expr("2 * (3 + )").is_err());
        assert!(parse_expr("2 * (3 + 4))").is_err());
        assert!(parse_expr("2 * ((3 + 4)").is_err());
        assert!(parse_expr("2 * (3 + (4)").is_err());
        assert!(parse_expr("2 * (3 + 4))").is_err());
        assert!(parse_expr("2 * (3 + 4) -").is_err());
        assert!(parse_expr("2 * (3 + 4) - 5 %").is_err());
        assert!(parse_expr("2 * (3 + 4) - 5 % 6)").is_err());
    }

    #[test]
    fn function_calls() {
        // Unary function calls
        assert_eq!(
            parse_expr("sin(0)"),
            Ok(Expr::FnCall {
                name: "sin".into(),
                args: vec![Number(0.)],
            })
        );

        assert_eq!(
            parse_expr("sin(3)"),
            Ok(Expr::FnCall {
                name: "sin".into(),
                args: vec![Number(3.)],
            })
        );

        // Binary function calls
        assert_eq!(
            parse_expr("log(1, 10)"),
            Ok(Expr::FnCall {
                name: "log".into(),
                args: vec![Number(1.), Number(10.)],
            })
        );
        assert_eq!(
            parse_expr("log(2.5, 10)"),
            Ok(Expr::FnCall {
                name: "log".into(),
                args: vec![Number(2.5), Number(10.)],
            })
        );
        assert_eq!(
            parse_expr("log(2.5, 2.5)"),
            Ok(Expr::FnCall {
                name: "log".into(),
                args: vec![Number(2.5), Number(2.5)],
            })
        );

        // Failing tests for calls
        assert!(parse_expr("sin(-3.)").is_err());
        assert!(parse_expr("log(1, )").is_err()); // Missing second argument with trailing comma
        assert!(parse_expr("log(, 10)").is_err()); // Missing first argument
        assert!(parse_expr("log(1, 10").is_err()); // Missing closing parenthesis
        assert!(parse_expr("log 1, 10)").is_err()); // Missing opening parenthesis
        assert!(parse_expr("log(1, 10))").is_err()); // Extra closing parenthesis
        assert!(parse_expr("log((1, 10)").is_err()); // Extra opening parenthesis
        assert!(parse_expr("log(1, (10)").is_err()); // Unmatched parentheses
        assert!(parse_expr("log(1, 10))").is_err()); // Extra closing parenthesis
        assert!(parse_expr("log(1, 10) +").is_err()); // Trailing operator
        assert!(parse_expr("log(1, 10) !").is_err()); // Trailing operator
    }

    #[test]
    fn mathematical_notations() {
        assert_eq!(
            parse_expr("2 sin(3)"),
            Ok(binop!(
                Mul,
                Number(2.),
                Expr::FnCall {
                    name: "sin".into(),
                    args: vec![Number(3.)],
                }
            ))
        );
        assert_eq!(
            parse_expr("2 (5 + 2)"),
            Ok(binop!(Mul, Number(2.), binop!(Add, Number(5.), Number(2.))))
        );
        assert_eq!(
            parse_expr("3 (4 + 5) sin(6)"),
            Ok(binop!(
                Mul,
                binop!(Mul, Number(3.), binop!(Add, Number(4.), Number(5.))),
                Expr::FnCall {
                    name: "sin".into(),
                    args: vec![Number(6.)],
                }
            ))
        );
        assert_eq!(
            parse_expr("2 (3 + 4) (5 + 6)"),
            Ok(binop!(
                Mul,
                binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))),
                binop!(Add, Number(5.), Number(6.))
            ))
        );
        assert_eq!(
            parse_expr("2 sin(3 + 4)"),
            Ok(binop!(
                Mul,
                Number(2.),
                Expr::FnCall {
                    name: "sin".into(),
                    args: vec![binop!(Add, Number(3.), Number(4.))],
                }
            ))
        );
        assert_eq!(
            parse_expr("2 (3 + sin(4))"),
            Ok(binop!(
                Mul,
                Number(2.),
                binop!(
                    Add,
                    Number(3.),
                    Expr::FnCall {
                        name: "sin".into(),
                        args: vec![Number(4.)],
                    }
                )
            ))
        );
        assert_eq!(
            parse_expr("2 (3 + 4) 5"),
            Ok(binop!(
                Mul,
                binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))),
                Number(5.)
            ))
        );

        // Failing tests
        assert!(parse_expr("2 (3 + 4").is_err());
        assert!(parse_expr("2 3 + 4)").is_err());
        assert!(parse_expr("2 (3 + sin(4)").is_err());
        assert!(parse_expr("2 (3 + 4))").is_err());
        assert!(parse_expr("2 (3 + (4)").is_err());
        assert!(parse_expr("2 (3 + 4))").is_err());
        assert!(parse_expr("2 (3 + 4) -").is_err());
        assert!(parse_expr("2 (3 + 4) - 5 %").is_err());
        assert!(parse_expr("2 (3 + 4) - 5 % 6)").is_err());
    }

    #[test]
    fn mixed_operations_and_number_notations() {
        // Mixed operations
        assert_eq!(
            parse_expr("sin(2 + 3) * 4"),
            Ok(binop!(
                Mul,
                Expr::FnCall {
                    name: "sin".into(),
                    args: vec![binop!(Add, Number(2.), Number(3.))],
                },
                Number(4.)
            ))
        );
        assert_eq!(
            parse_expr("2 * log(3 + 4, 10)"),
            Ok(binop!(
                Mul,
                Number(2.),
                Expr::FnCall {
                    name: "log".into(),
                    args: vec![binop!(Add, Number(3.), Number(4.)), Number(10.)],
                }
            ))
        );

        assert_eq!(
            parse_expr("2 * sin(3 + 4) - log(5, 6)"),
            Ok(binop!(
                Sub,
                binop!(
                    Mul,
                    Number(2.),
                    Expr::FnCall {
                        name: "sin".into(),
                        args: vec![binop!(Add, Number(3.), Number(4.))],
                    }
                ),
                Expr::FnCall {
                    name: "log".into(),
                    args: vec![Number(5.), Number(6.)],
                }
            ))
        );
        assert_eq!(
            parse_expr("2 * (3 + sin(4))"),
            Ok(binop!(
                Mul,
                Number(2.),
                binop!(
                    Add,
                    Number(3.),
                    Expr::FnCall {
                        name: "sin".into(),
                        args: vec![Number(4.)],
                    }
                )
            ))
        );

        // Combining scientific notation and function calls
        assert_eq!(
            parse_expr("sin(1e3) + 2.5"),
            Ok(binop!(
                Add,
                Expr::FnCall {
                    name: "sin".into(),
                    args: vec![Number(1e3)],
                },
                Number(2.5)
            ))
        );
        assert_eq!(
            parse_expr("log(1e-3, 2.5) * 10"),
            Ok(binop!(
                Mul,
                Expr::FnCall {
                    name: "log".into(),
                    args: vec![Number(1e-3), Number(2.5)],
                },
                Number(10.)
            ))
        );
        assert_eq!(
            parse_expr("2 * sin(2.5e2) - log(1, 1e3)"),
            Ok(binop!(
                Sub,
                binop!(
                    Mul,
                    Number(2.),
                    Expr::FnCall {
                        name: "sin".into(),
                        args: vec![Number(2.5e2)],
                    }
                ),
                Expr::FnCall {
                    name: "log".into(),
                    args: vec![Number(1.), Number(1e3)],
                }
            ))
        );
        assert_eq!(
            parse_expr("2 3 + sin(4)"),
            Ok(binop!(
                Add,
                binop!(Mul, Number(2.), Number(3.)),
                Expr::FnCall {
                    name: "sin".into(),
                    args: vec![Number(4.)],
                }
            ))
        );
        assert_eq!(
            parse_expr("sin(4) 2 + 3"),
            Ok(binop!(
                Add,
                binop!(
                    Mul,
                    Expr::FnCall {
                        name: "sin".into(),
                        args: vec![Number(4.)],
                    },
                    Number(2.)
                ),
                Number(3.)
            ))
        );

        // Failing tests for mixed operations

        // Missing closing parenthesis
        assert!(parse_expr("2 * (3 + sin(4)").is_err());

        // Extra comma in log function
        assert!(parse_expr("log(2, 3,) + sin(4)").is_err());

        // Invalid character in expression
        assert!(parse_expr("2 * (3 + sin(4) @)").is_err());

        // Unmatched parentheses
        assert!(parse_expr("2 * (3 + sin(4)) + (5").is_err());

        // Extra closing parenthesis
        assert!(parse_expr("2 * (3 + sin(4))) + 5").is_err());

        // Invalid number format
        assert!(parse_expr("2 * (3 + sin(4.5.6))").is_err());
    }

    #[test]
    fn variable_definition_test() {
        assert_eq!(
            parse_stmt("let x = 42"),
            Ok(Stmt::DefVar {
                name: "x".into(),
                expr: Expr::Number(42.),
            })
        );

        assert_eq!(
            parse_stmt("let y = 3.14"),
            Ok(Stmt::DefVar {
                name: "y".into(),
                expr: Expr::Number(3.14),
            })
        );

        assert_eq!(
            parse_stmt("let z = x + y"),
            Ok(Stmt::DefVar {
                name: "z".into(),
                expr: binop!(Add, Expr::Variable("x".into()), Expr::Variable("y".into()))
            })
        );

        // Failing tests
        assert!(parse_stmt("let = 42").is_err());
        assert!(parse_stmt("let x 42").is_err());
        assert!(parse_stmt("let x = ").is_err());
        assert!(parse_stmt("let 42 = x").is_err());
    }

    #[test]
    fn function_definition_test() {
        assert_eq!(
            parse_stmt("let add(a, b) = a + b"),
            Ok(Stmt::DefFun {
                name: "add".into(),
                args: vec!["a".into(), "b".into()],
                body: binop!(Add, Expr::Variable("a".into()), Expr::Variable("b".into())),
            })
        );

        assert_eq!(
            parse_stmt("let square(x) = x * x"),
            Ok(Stmt::DefFun {
                name: "square".into(),
                args: vec!["x".into()],
                body: binop!(Mul, Expr::Variable("x".into()), Expr::Variable("x".into())),
            })
        );

        assert_eq!(
            parse_stmt("let negate(x) = -x"),
            Ok(Stmt::DefFun {
                name: "negate".into(),
                args: vec!["x".into()],
                body: preop!(Neg, Expr::Variable("x".into())),
            })
        );

        // Failing tests
        assert!(parse_stmt("let add(a, b) = ").is_err());
        assert!(parse_stmt("let add(a, b = a + b").is_err());
        assert!(parse_stmt("let add(a b) = a + b").is_err());
        assert!(parse_stmt("let add(a, b) a + b").is_err());
    }
}
