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

        let number = just(Token::Operator("-"))
            .or_not()
            .then(select! {
                Token::Number(n) => n
            })
            .map(|(sign, n)| match sign {
                Some(_) => Expr::Number(-n),
                None => Expr::Number(n),
            })
            .labelled("number");

        let unary_fn_call = select! {
            Token::Ident("sin") => UnaryFn::Sin,
            Token::Ident("cos") => UnaryFn::Cos,
            Token::Ident("tan") => UnaryFn::Tan,
            Token::Ident("sec") => UnaryFn::Sec,
            Token::Ident("csc") => UnaryFn::Csc,
            Token::Ident("cot") => UnaryFn::Cot,
            Token::Ident("asin") => UnaryFn::Asin,
            Token::Ident("acos") => UnaryFn::Acos,
            Token::Ident("atan") => UnaryFn::Atan,
            Token::Ident("asec") => UnaryFn::Asec,
            Token::Ident("acsc") => UnaryFn::Acsc,
            Token::Ident("acot") => UnaryFn::Acot,
            Token::Ident("sinh") => UnaryFn::Sinh,
            Token::Ident("cosh") => UnaryFn::Cosh,
            Token::Ident("tanh") => UnaryFn::Tanh,
            Token::Ident("floor") => UnaryFn::Floor,
            Token::Ident("ceil") => UnaryFn::Ceil,
            Token::Ident("round") => UnaryFn::Round,
            Token::Ident("abs") => UnaryFn::Abs,
            Token::Ident("sqrt") => UnaryFn::Sqrt,
            Token::Ident("exp") => UnaryFn::Exp,
            Token::Ident("exp2") => UnaryFn::Exp2,
            Token::Ident("ln") => UnaryFn::Ln,
            Token::Ident("log10") => UnaryFn::Log10,
            Token::Ident("rad") => UnaryFn::Rad,
            Token::Ident("deg") => UnaryFn::Deg,
        }
        .labelled("ident")
        .then(
            expr.clone()
                .padded_by(whitespace.clone())
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
        )
        .map(|(func, arg)| Expr::UnaryFnCall {
            function: func,
            arg: Box::new(arg),
        });

        let binary_fn_call = select! {
            Token::Ident("log") => BinaryFn::Log,
            Token::Ident("nroot") => BinaryFn::NRoot,
        }
        .labelled("ident")
        .then(
            expr.clone()
                .then_ignore(just(Token::Ctrl(',')))
                .then(expr.clone())
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
        )
        .map(|(func, (arg1, arg2))| Expr::BinaryFnCall {
            function: func,
            arg1: Box::new(arg1),
            arg2: Box::new(arg2),
        });

        let atomic = choice((
            number.clone(),
            unary_fn_call,
            binary_fn_call,
            expr.clone()
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
        ));

        let prefixed = select! {
            Token::Operator("-") => UnaryOp::Neg
        }
        .labelled("prefix operator")
        .then(atomic.clone().and_is(number.clone().not()))
        .map(|(op, rhs)| Expr::UnaryOp {
            op,
            arg: Box::new(rhs),
        })
        .or(atomic.clone());

        let postfixed = prefixed
            .clone()
            .then(
                select! {
                    Token::Operator("!") => UnaryOp::Fac
                }
                .labelled("postfix operator"),
            )
            .map(|(lhs, op)| Expr::UnaryOp {
                op,
                arg: Box::new(lhs),
            })
            .or(prefixed);

        let term = postfixed.padded_by(whitespace.clone()).boxed();

        let power = term
            .clone()
            .then(
                select! {
                    Token::Operator("^") => BinaryOp::Pow
                }
                .labelled("infix operator"),
            )
            .repeated()
            .foldr(term, |(lhs, op), rhs| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });

        let powers = power
            .clone()
            .foldl(power.and_is(number.not()).repeated(), |lhs, rhs| {
                Expr::BinaryOp {
                    op: BinaryOp::Mul,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            });

        let product = powers.clone().foldl(
            select! {
                Token::Operator("*") => BinaryOp::Mul,
                Token::Operator("/") => BinaryOp::Div,
                Token::Operator("%") => BinaryOp::Rem
            }
            .labelled("infix operator")
            .then(powers)
            .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        let sum = product.clone().foldl(
            select! {
                Token::Operator("+") => BinaryOp::Add,
                Token::Operator("-") => BinaryOp::Sub
            }
            .labelled("infix operator")
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        );

        sum.labelled("expression").as_context()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;

    macro_rules! unop {
        ($op_name:ident, $val:expr) => {
            Expr::UnaryOp {
                op: super::UnaryOp::$op_name,
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
        assert!(parse("- 3").is_err());
        assert!(parse("1..2").is_err());
        assert!(parse("1e").is_err());
        assert!(parse("1e--3").is_err());
        assert!(parse("--1").is_err());
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
        assert_eq!(parse("5!"), Ok(unop!(Fac, Number(5.))));
        assert_eq!(
            parse("-(2 + 3)"),
            Ok(unop!(Neg, binop!(Add, Number(2.), Number(3.))))
        );
        assert_eq!(
            parse("-(2 * 3) + 4"),
            Ok(binop!(
                Add,
                unop!(Neg, binop!(Mul, Number(2.), Number(3.))),
                Number(4.)
            ))
        );
        assert_eq!(
            parse("2 * -(3 + 4)"),
            Ok(binop!(
                Mul,
                Number(2.),
                unop!(Neg, binop!(Add, Number(3.), Number(4.)))
            ))
        );
        assert_eq!(
            parse("-(2 * 3 + 4)"),
            Ok(unop!(
                Neg,
                binop!(Add, binop!(Mul, Number(2.), Number(3.)), Number(4.))
            ))
        );
        assert_eq!(
            parse("3! + 4"),
            Ok(binop!(Add, unop!(Fac, Number(3.)), Number(4.)))
        );
        assert_eq!(parse("-(3!)"), Ok(unop!(Neg, unop!(Fac, Number(3.)))));
        assert_eq!(parse("-3!"), Ok(unop!(Fac, Number(-3.))));
        assert_eq!(
            parse("2 ^ 3!"),
            Ok(binop!(Pow, Number(2.), unop!(Fac, Number(3.))))
        );
        assert_eq!(
            parse("-(2 ^ 3)"),
            Ok(unop!(Neg, binop!(Pow, Number(2.), Number(3.))))
        );
        assert_eq!(parse("-2^3"), Ok(binop!(Pow, Number(-2.), Number(3.))));
        assert_eq!(parse("2 ^ -3"), Ok(binop!(Pow, Number(2.), Number(-3.))));
        assert_eq!(
            parse("-(2 ^ -3)"),
            Ok(unop!(Neg, binop!(Pow, Number(2.), Number(-3.))))
        );
        assert_eq!(parse("-(-3)"), Ok(unop!(Neg, Number(-3.))));
        assert_eq!(parse("-2 (-3)"), Ok(binop!(Mul, Number(-2.), Number(-3.))));
        assert_eq!(
            parse("(5 + 3)  (-3)"),
            Ok(binop!(
                Mul,
                binop!(Add, Number(5.), Number(3.)),
                Number(-3.)
            ))
        );

        // Failing tests
        assert!(parse("- 6*3").is_err());
        assert!(parse("2 ** 3").is_err());
        assert!(parse("2 // 3").is_err());
        assert!(parse("2 %% 3").is_err());
        assert!(parse("2 ^^ 3").is_err());
        assert!(parse("2 +* 3").is_err());
        assert!(parse("2 *+ 3").is_err());
        assert!(parse("2 + 3 *").is_err());
        assert!(parse("2 * (3 + 4").is_err());
        assert!(parse("2 * 3 + 4)").is_err());
        assert!(parse("2 * - 3").is_err());
        assert!(parse("2 * (3 + )").is_err());
        assert!(parse("2 * (3 + 4))").is_err());
        assert!(parse("2 * ((3 + 4)").is_err());
        assert!(parse("2 * (3 + (4)").is_err());
        assert!(parse("2 * (3 + 4))").is_err());
        assert!(parse("2 * (3 + 4) -").is_err());
        assert!(parse("2 * (3 + 4) - 5 %").is_err());
        assert!(parse("2 * (3 + 4) - 5 % 6)").is_err());
        assert!(parse("--3").is_err());
        assert!(parse("(5 % 3)  2").is_err(),);
    }

    #[test]
    fn unary_function_calls() {
        assert_eq!(
            parse("sin(0)"),
            Ok(Expr::UnaryFnCall {
                function: UnaryFn::Sin,
                arg: Box::new(Number(0.)),
            })
        );
        assert_eq!(
            parse("sin(3)"),
            Ok(Expr::UnaryFnCall {
                function: UnaryFn::Sin,
                arg: Box::new(Number(3.)),
            })
        );

        // Failing tests
        assert!(parse("sin()").is_err());
        assert!(parse("sin(-3.)").is_err());
        assert!(parse("sin(3, 4)").is_err());
        assert!(parse("sin(abc)").is_err());
    }

    #[test]
    fn binary_function_calls() {
        assert_eq!(
            parse("log(1, 10)"),
            Ok(Expr::BinaryFnCall {
                function: BinaryFn::Log,
                arg1: Box::new(Number(1.)),
                arg2: Box::new(Number(10.)),
            })
        );
        assert_eq!(
            parse("log(2.5, 10)"),
            Ok(Expr::BinaryFnCall {
                function: BinaryFn::Log,
                arg1: Box::new(Number(2.5)),
                arg2: Box::new(Number(10.)),
            })
        );
        assert_eq!(
            parse("log(2.5, 2.5)"),
            Ok(Expr::BinaryFnCall {
                function: BinaryFn::Log,
                arg1: Box::new(Number(2.5)),
                arg2: Box::new(Number(2.5)),
            })
        );

        // Failing tests
        assert!(parse("log(1)").is_err());
        assert!(parse("log(1, 2, 3)").is_err());
        assert!(parse("log(abc, 10)").is_err());
    }

    #[test]
    fn mixed_operations_and_number_notations() {
        // Mixed operations
        assert_eq!(
            parse("sin(2 + 3) * 4"),
            Ok(binop!(
                Mul,
                Expr::UnaryFnCall {
                    function: UnaryFn::Sin,
                    arg: Box::new(binop!(Add, Number(2.), Number(3.))),
                },
                Number(4.)
            ))
        );
        assert_eq!(
            parse("2 * log(3 + 4, 10)"),
            Ok(binop!(
                Mul,
                Number(2.),
                Expr::BinaryFnCall {
                    function: BinaryFn::Log,
                    arg1: Box::new(binop!(Add, Number(3.), Number(4.))),
                    arg2: Box::new(Number(10.)),
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
                    Expr::UnaryFnCall {
                        function: UnaryFn::Sin,
                        arg: Box::new(binop!(Add, Number(3.), Number(4.))),
                    }
                ),
                Expr::BinaryFnCall {
                    function: BinaryFn::Log,
                    arg1: Box::new(Number(5.)),
                    arg2: Box::new(Number(6.)),
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
                    Expr::UnaryFnCall {
                        function: UnaryFn::Sin,
                        arg: Box::new(Number(4.)),
                    }
                )
            ))
        );
        assert_eq!(
            parse("log(2, 3) + sin(4)"),
            Ok(binop!(
                Add,
                Expr::BinaryFnCall {
                    function: BinaryFn::Log,
                    arg1: Box::new(Number(2.)),
                    arg2: Box::new(Number(3.)),
                },
                Expr::UnaryFnCall {
                    function: UnaryFn::Sin,
                    arg: Box::new(Number(4.)),
                }
            ))
        );

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

        // Combining scientific notation and function calls
        assert_eq!(
            parse("sin(1e3) + 2.5"),
            Ok(binop!(
                Add,
                Expr::UnaryFnCall {
                    function: UnaryFn::Sin,
                    arg: Box::new(Number(1e3)),
                },
                Number(2.5)
            ))
        );
        assert_eq!(
            parse("log(1e-3, 2.5) * 10"),
            Ok(binop!(
                Mul,
                Expr::BinaryFnCall {
                    function: BinaryFn::Log,
                    arg1: Box::new(Number(1e-3)),
                    arg2: Box::new(Number(2.5)),
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
                    Expr::UnaryFnCall {
                        function: UnaryFn::Sin,
                        arg: Box::new(Number(2.5e2)),
                    }
                ),
                Expr::BinaryFnCall {
                    function: BinaryFn::Log,
                    arg1: Box::new(Number(1.)),
                    arg2: Box::new(Number(1e3)),
                }
            ))
        );

        // Failing tests for mixed operations

        // Missing closing parenthesis
        assert!(parse("2 * (3 + sin(4)").is_err());

        // Missing opening parenthesis
        assert!(parse("2 * 3 + sin 4)").is_err());

        // Missing argument for sin function
        assert!(parse("2 * sin()").is_err());

        // Extra comma in log function
        assert!(parse("log(2, 3,) + sin(4)").is_err());

        // Invalid character in expression
        assert!(parse("2 * (3 + sin(4) @)").is_err());

        // Missing operator between numbers
        assert!(parse("2 3 + sin(4)").is_err());

        // Missing operator between function call and number
        assert!(parse("sin(4) 2 + 3").is_err());

        // Invalid function name
        assert!(parse("invalid(2, 3) + sin(4)").is_err());

        // Unmatched parentheses
        assert!(parse("2 * (3 + sin(4)) + (5").is_err());

        // Extra closing parenthesis
        assert!(parse("2 * (3 + sin(4))) + 5").is_err());

        // Missing argument for log function
        assert!(parse("log(2) + sin(4)").is_err());

        // Extra argument for sin function
        assert!(parse("sin(2, 3) + 4").is_err());

        // Invalid number format
        assert!(parse("2 * (3 + sin(4.5.6))").is_err());

        // Invalid character in function argument
        assert!(parse("sin(2 + 3a) + 4").is_err());
    }
}
