use crate::lexer::Token;
use crate::types::*;

use chumsky::input::ValueInput;
use chumsky::prelude::*;

#[allow(clippy::let_and_return)]
pub fn parser<'a, I>() -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    recursive(|expr| {
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

        let fn_call = select! {
            Token::Ident(ident) => ident
        }
        .labelled("ident")
        .then_ignore(just(Token::LParen))
        .then(expr.clone().separated_by(just(Token::Comma)).collect())
        .then_ignore(just(Token::RParen))
        .map(|(fname, args)| Expr::FnCall {
            fname: fname.into(),
            args,
        })
        .boxed();

        let atomic = choice((
            number.clone(),
            fn_call,
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
            .as_context()
            .boxed();

        let power = term
            .clone()
            .then(just(Token::Caret).to(BinaryOp::Pow))
            .repeated()
            .foldr(term, |(lhs, op), rhs| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
            .boxed();

        let powers = power
            .clone()
            .foldl(
                power.and_is(just(Token::Minus).not()).repeated(),
                |lhs, rhs| Expr::BinaryOp {
                    op: BinaryOp::Mul,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            )
            .boxed();

        let product = powers
            .clone()
            .foldl(
                choice((
                    just(Token::Asterisk).to(BinaryOp::Mul),
                    just(Token::Slash).to(BinaryOp::Div),
                    just(Token::Percent).to(BinaryOp::Rem),
                ))
                .then(powers)
                .repeated(),
                |lhs, (op, rhs)| Expr::BinaryOp {
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
                    just(Token::Plus).to(BinaryOp::Add),
                    just(Token::Minus).to(BinaryOp::Sub),
                ))
                .then(product)
                .repeated(),
                |lhs, (op, rhs)| Expr::BinaryOp {
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
            Expr::BinaryOp {
                op: super::BinaryOp::$op_name,
                lhs: $lhs.into(),
                rhs: $rhs.into(),
            }
        };
    }

    fn parse(input: &str) -> Result<Expr, String> {
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
        assert_eq!(parse("1"), Ok(Number(1.)));
        assert_eq!(parse("   1"), Ok(Number(1.)));
        assert_eq!(parse("0"), Ok(Number(0.)));
        assert_eq!(parse("2.5"), Ok(Number(2.5)));
        assert_eq!(parse("1e3"), Ok(Number(1e3)));
        assert_eq!(parse("1e-3"), Ok(Number(1e-3)));
        assert_eq!(parse("2.5e2"), Ok(Number(2.5e2)));
        assert_eq!(parse("2.5e-2"), Ok(Number(2.5e-2)));
        assert_eq!(parse("-1"), Ok(Number(-1.)));
        assert_eq!(parse("-2.5"), Ok(Number(-2.5)));
        assert_eq!(parse("-1e3"), Ok(Number(-1e3)));
        assert_eq!(parse("-1e-3"), Ok(Number(-1e-3)));
        assert_eq!(parse("-2.5e2"), Ok(Number(-2.5e2)));
        assert_eq!(parse("-2.5e-2"), Ok(Number(-2.5e-2)));

        // Tests that should fail
        assert!(parse("abc").is_err());
        assert!(parse("0.").is_err());
        assert!(parse("1..2").is_err());
        assert!(parse("1e").is_err());
        assert!(parse("1e--3").is_err());
        assert!(parse("2.5.2").is_err());
        assert!(parse("1e3.5").is_err());
        assert!(parse("1 e3").is_err());
        assert!(parse("1e 3").is_err());
        assert!(parse("1e3 .5").is_err());
        assert!(parse("1e3. 5").is_err());
        assert!(parse("1e3 . 5").is_err());
        assert!(parse("1 e 3").is_err());
    }

    #[test]
    fn basic_ops() {
        assert_eq!(parse("6*3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse("6 * 3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse("6* 3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse("6 *3"), Ok(binop!(Mul, Number(6.), Number(3.))));
        assert_eq!(parse("6+3"), Ok(binop!(Add, Number(6.), Number(3.))));
        assert_eq!(parse("6-3"), Ok(binop!(Sub, Number(6.), Number(3.))));
        assert_eq!(parse("6/3"), Ok(binop!(Div, Number(6.), Number(3.))));
        assert_eq!(parse("6%3"), Ok(binop!(Rem, Number(6.), Number(3.))));
        assert_eq!(parse("2^3"), Ok(binop!(Pow, Number(2.), Number(3.))));

        // Different number notations
        assert_eq!(
            parse("1e3 + 2.5"),
            Ok(binop!(Add, Number(1e3), Number(2.5)))
        );
        assert_eq!(
            parse("1e-3 * 2.5"),
            Ok(binop!(Mul, Number(1e-3), Number(2.5)))
        );
        assert_eq!(
            parse("2.5e2 - 1"),
            Ok(binop!(Sub, Number(2.5e2), Number(1.)))
        );
        assert_eq!(
            parse("2.5e-2 / 1e3"),
            Ok(binop!(Div, Number(2.5e-2), Number(1e3)))
        );
        assert_eq!(
            parse("-1e3 + 2.5"),
            Ok(binop!(Add, Number(-1e3), Number(2.5)))
        );
        assert_eq!(
            parse("2.5e2 % 1e3"),
            Ok(binop!(Rem, Number(2.5e2), Number(1e3)))
        );

        assert_eq!(
            parse("2 + 3 * 4"),
            Ok(binop!(Add, Number(2.), binop!(Mul, Number(3.), Number(4.))))
        );
        assert_eq!(
            parse("(2 + 3) * 4"),
            Ok(binop!(Mul, binop!(Add, Number(2.), Number(3.)), Number(4.)))
        );
        assert_eq!(
            parse("2 * (3 + 4)"),
            Ok(binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))))
        );
        assert_eq!(
            parse("2 * 3 + 4"),
            Ok(binop!(Add, binop!(Mul, Number(2.), Number(3.)), Number(4.)))
        );
        assert_eq!(
            parse("2 + 3 * 4 - 5 / 6"),
            Ok(binop!(
                Sub,
                binop!(Add, Number(2.), binop!(Mul, Number(3.), Number(4.))),
                binop!(Div, Number(5.), Number(6.))
            ))
        );
        assert_eq!(
            parse("2 * (3 + 4) - 5 % 6"),
            Ok(binop!(
                Sub,
                binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))),
                binop!(Rem, Number(5.), Number(6.))
            ))
        );
        assert_eq!(parse("5!"), Ok(proop!(Fac, Number(5.))));
        assert_eq!(
            parse("-(2 + 3)"),
            Ok(preop!(Neg, binop!(Add, Number(2.), Number(3.))))
        );
        assert_eq!(
            parse("-(2 * 3) + 4"),
            Ok(binop!(
                Add,
                preop!(Neg, binop!(Mul, Number(2.), Number(3.))),
                Number(4.)
            ))
        );
        assert_eq!(
            parse("2 * -(3 + 4)"),
            Ok(binop!(
                Mul,
                Number(2.),
                preop!(Neg, binop!(Add, Number(3.), Number(4.)))
            ))
        );
        assert_eq!(
            parse("-(2 * 3 + 4)"),
            Ok(preop!(
                Neg,
                binop!(Add, binop!(Mul, Number(2.), Number(3.)), Number(4.))
            ))
        );
        assert_eq!(
            parse("3! + 4"),
            Ok(binop!(Add, proop!(Fac, Number(3.)), Number(4.)))
        );
        assert_eq!(parse("-(3!)"), Ok(preop!(Neg, proop!(Fac, Number(3.)))));
        assert_eq!(parse("-3!"), Ok(proop!(Fac, Number(-3.))));
        assert_eq!(
            parse("2 ^ 3!"),
            Ok(binop!(Pow, Number(2.), proop!(Fac, Number(3.))))
        );
        assert_eq!(
            parse("-(2 ^ 3)"),
            Ok(preop!(Neg, binop!(Pow, Number(2.), Number(3.))))
        );
        assert_eq!(parse("-2^3"), Ok(binop!(Pow, Number(-2.), Number(3.))));
        assert_eq!(parse("2 ^ -3"), Ok(binop!(Pow, Number(2.), Number(-3.))));
        assert_eq!(
            parse("-(2 ^ -3)"),
            Ok(preop!(Neg, binop!(Pow, Number(2.), Number(-3.))))
        );
        assert_eq!(parse("-(-3)"), Ok(preop!(Neg, Number(-3.))));
        assert_eq!(parse("-2 (-3)"), Ok(binop!(Mul, Number(-2.), Number(-3.))));
        assert_eq!(
            parse("(5 + 3)  (-3)"),
            Ok(binop!(
                Mul,
                binop!(Add, Number(5.), Number(3.)),
                Number(-3.)
            ))
        );
        assert_eq!(
            parse("(5 % 3)  2"),
            Ok(binop!(Mul, binop!(Rem, Number(5.), Number(3.)), Number(2.)))
        );
        assert_eq!(parse("--1"), Ok(preop!(Neg, Number(-1.))));
        assert_eq!(parse("--3"), Ok(preop!(Neg, Number(-3.))));

        // Failing tests
        assert!(parse("2 ** 3").is_err());
        assert!(parse("2 // 3").is_err());
        assert!(parse("2 %% 3").is_err());
        assert!(parse("2 ^^ 3").is_err());
        assert!(parse("2 +* 3").is_err());
        assert!(parse("2 *+ 3").is_err());
        assert!(parse("2 + 3 *").is_err());
        assert!(parse("2 * (3 + 4").is_err());
        assert!(parse("2 * 3 + 4)").is_err());
        assert!(parse("2 * (3 + )").is_err());
        assert!(parse("2 * (3 + 4))").is_err());
        assert!(parse("2 * ((3 + 4)").is_err());
        assert!(parse("2 * (3 + (4)").is_err());
        assert!(parse("2 * (3 + 4))").is_err());
        assert!(parse("2 * (3 + 4) -").is_err());
        assert!(parse("2 * (3 + 4) - 5 %").is_err());
        assert!(parse("2 * (3 + 4) - 5 % 6)").is_err());
    }

    #[test]
    fn function_calls() {
        // Unary function calls
        assert_eq!(
            parse("sin(0)"),
            Ok(Expr::FnCall {
                fname: "sin".into(),
                args: vec![Number(0.)],
            })
        );

        assert_eq!(
            parse("sin(3)"),
            Ok(Expr::FnCall {
                fname: "sin".into(),
                args: vec![Number(3.)],
            })
        );

        // Failing tests for unary function calls
        assert!(parse("sin(-3.)").is_err());
        assert!(parse("sin(abc)").is_err());

        // Binary function calls
        assert_eq!(
            parse("log(1, 10)"),
            Ok(Expr::FnCall {
                fname: "log".into(),
                args: vec![Number(1.), Number(10.)],
            })
        );
        assert_eq!(
            parse("log(2.5, 10)"),
            Ok(Expr::FnCall {
                fname: "log".into(),
                args: vec![Number(2.5), Number(10.)],
            })
        );
        assert_eq!(
            parse("log(2.5, 2.5)"),
            Ok(Expr::FnCall {
                fname: "log".into(),
                args: vec![Number(2.5), Number(2.5)],
            })
        );

        // Failing tests for binary function calls
        assert!(parse("log(abc, 10)").is_err());
        assert!(parse("log(1, abc)").is_err());
        assert!(parse("log(1, )").is_err()); // Missing second argument with trailing comma
        assert!(parse("log(, 10)").is_err()); // Missing first argument
        assert!(parse("log(1, 10").is_err()); // Missing closing parenthesis
        assert!(parse("log 1, 10)").is_err()); // Missing opening parenthesis
        assert!(parse("log(1, 10))").is_err()); // Extra closing parenthesis
        assert!(parse("log((1, 10)").is_err()); // Extra opening parenthesis
        assert!(parse("log(1, (10)").is_err()); // Unmatched parentheses
        assert!(parse("log(1, 10))").is_err()); // Extra closing parenthesis
        assert!(parse("log(1, 10) +").is_err()); // Trailing operator
        assert!(parse("log(1, 10) !").is_err()); // Trailing operator
    }

    #[test]
    fn mathematical_notations() {
        assert_eq!(
            parse("2 sin(3)"),
            Ok(binop!(
                Mul,
                Number(2.),
                Expr::FnCall {
                    fname: "sin".into(),
                    args: vec![Number(3.)],
                }
            ))
        );
        assert_eq!(
            parse("2 (5 + 2)"),
            Ok(binop!(Mul, Number(2.), binop!(Add, Number(5.), Number(2.))))
        );
        assert_eq!(
            parse("3 (4 + 5) sin(6)"),
            Ok(binop!(
                Mul,
                binop!(Mul, Number(3.), binop!(Add, Number(4.), Number(5.))),
                Expr::FnCall {
                    fname: "sin".into(),
                    args: vec![Number(6.)],
                }
            ))
        );
        assert_eq!(
            parse("2 (3 + 4) (5 + 6)"),
            Ok(binop!(
                Mul,
                binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))),
                binop!(Add, Number(5.), Number(6.))
            ))
        );
        assert_eq!(
            parse("2 sin(3 + 4)"),
            Ok(binop!(
                Mul,
                Number(2.),
                Expr::FnCall {
                    fname: "sin".into(),
                    args: vec![binop!(Add, Number(3.), Number(4.))],
                }
            ))
        );
        assert_eq!(
            parse("2 (3 + sin(4))"),
            Ok(binop!(
                Mul,
                Number(2.),
                binop!(
                    Add,
                    Number(3.),
                    Expr::FnCall {
                        fname: "sin".into(),
                        args: vec![Number(4.)],
                    }
                )
            ))
        );
        assert_eq!(
            parse("2 (3 + 4) 5"),
            Ok(binop!(
                Mul,
                binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))),
                Number(5.)
            ))
        );

        // Failing tests
        assert!(parse("2 (3 + 4").is_err());
        assert!(parse("2 3 + 4)").is_err());
        assert!(parse("2 sin 3)").is_err());
        assert!(parse("2 (3 + sin(4)").is_err());
        assert!(parse("2 (3 + sin 4)").is_err());
        assert!(parse("2 (3 + 4))").is_err());
        assert!(parse("2 (3 + (4)").is_err());
        assert!(parse("2 (3 + 4))").is_err());
        assert!(parse("2 (3 + 4) -").is_err());
        assert!(parse("2 (3 + 4) - 5 %").is_err());
        assert!(parse("2 (3 + 4) - 5 % 6)").is_err());
    }

    #[test]
    fn mixed_operations_and_number_notations() {
        // Mixed operations
        assert_eq!(
            parse("sin(2 + 3) * 4"),
            Ok(binop!(
                Mul,
                Expr::FnCall {
                    fname: "sin".into(),
                    args: vec![binop!(Add, Number(2.), Number(3.))],
                },
                Number(4.)
            ))
        );
        assert_eq!(
            parse("2 * log(3 + 4, 10)"),
            Ok(binop!(
                Mul,
                Number(2.),
                Expr::FnCall {
                    fname: "log".into(),
                    args: vec![binop!(Add, Number(3.), Number(4.)), Number(10.)],
                }
            ))
        );

        assert_eq!(
            parse("2 * sin(3 + 4) - log(5, 6)"),
            Ok(binop!(
                Sub,
                binop!(
                    Mul,
                    Number(2.),
                    Expr::FnCall {
                        fname: "sin".into(),
                        args: vec![binop!(Add, Number(3.), Number(4.))],
                    }
                ),
                Expr::FnCall {
                    fname: "log".into(),
                    args: vec![Number(5.), Number(6.)],
                }
            ))
        );
        assert_eq!(
            parse("2 * (3 + sin(4))"),
            Ok(binop!(
                Mul,
                Number(2.),
                binop!(
                    Add,
                    Number(3.),
                    Expr::FnCall {
                        fname: "sin".into(),
                        args: vec![Number(4.)],
                    }
                )
            ))
        );

        // Combining scientific notation and function calls
        assert_eq!(
            parse("sin(1e3) + 2.5"),
            Ok(binop!(
                Add,
                Expr::FnCall {
                    fname: "sin".into(),
                    args: vec![Number(1e3)],
                },
                Number(2.5)
            ))
        );
        assert_eq!(
            parse("log(1e-3, 2.5) * 10"),
            Ok(binop!(
                Mul,
                Expr::FnCall {
                    fname: "log".into(),
                    args: vec![Number(1e-3), Number(2.5)],
                },
                Number(10.)
            ))
        );
        assert_eq!(
            parse("2 * sin(2.5e2) - log(1, 1e3)"),
            Ok(binop!(
                Sub,
                binop!(
                    Mul,
                    Number(2.),
                    Expr::FnCall {
                        fname: "sin".into(),
                        args: vec![Number(2.5e2)],
                    }
                ),
                Expr::FnCall {
                    fname: "log".into(),
                    args: vec![Number(1.), Number(1e3)],
                }
            ))
        );
        assert_eq!(
            parse("2 3 + sin(4)"),
            Ok(binop!(
                Add,
                binop!(Mul, Number(2.), Number(3.)),
                Expr::FnCall {
                    fname: "sin".into(),
                    args: vec![Number(4.)],
                }
            ))
        );
        assert_eq!(
            parse("sin(4) 2 + 3"),
            Ok(binop!(
                Add,
                binop!(
                    Mul,
                    Expr::FnCall {
                        fname: "sin".into(),
                        args: vec![Number(4.)],
                    },
                    Number(2.)
                ),
                Number(3.)
            ))
        );

        // Failing tests for mixed operations

        // Missing closing parenthesis
        assert!(parse("2 * (3 + sin(4)").is_err());

        // Missing opening parenthesis
        assert!(parse("2 * 3 + sin 4)").is_err());

        // Extra comma in log function
        assert!(parse("log(2, 3,) + sin(4)").is_err());

        // Invalid character in expression
        assert!(parse("2 * (3 + sin(4) @)").is_err());

        // Unmatched parentheses
        assert!(parse("2 * (3 + sin(4)) + (5").is_err());

        // Extra closing parenthesis
        assert!(parse("2 * (3 + sin(4))) + 5").is_err());

        // Invalid number format
        assert!(parse("2 * (3 + sin(4.5.6))").is_err());

        // Invalid character in function argument
        assert!(parse("sin(2 + 3a) + 4").is_err());
    }
}
