use crate::types::*;

use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric0, char, one_of, space0};
use nom::combinator::{all_consuming, map_res, not, peek, recognize};
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::{Err, IResult, Parser};

fn unary_function_to_enum(name: &str) -> Result<UnaryFn, ()> {
    match name {
        "sin" => Ok(UnaryFn::Sin),
        "cos" => Ok(UnaryFn::Cos),
        "tan" => Ok(UnaryFn::Tan),
        "sec" => Ok(UnaryFn::Sec),
        "csc" => Ok(UnaryFn::Csc),
        "cot" => Ok(UnaryFn::Cot),
        "asin" => Ok(UnaryFn::Asin),
        "acos" => Ok(UnaryFn::Acos),
        "atan" => Ok(UnaryFn::Atan),
        "asec" => Ok(UnaryFn::Asec),
        "acsc" => Ok(UnaryFn::Acsc),
        "acot" => Ok(UnaryFn::Acot),
        "sinh" => Ok(UnaryFn::Sinh),
        "cosh" => Ok(UnaryFn::Cosh),
        "tanh" => Ok(UnaryFn::Tanh),
        "floor" => Ok(UnaryFn::Floor),
        "ceil" => Ok(UnaryFn::Ceil),
        "round" => Ok(UnaryFn::Round),
        "abs" => Ok(UnaryFn::Abs),
        "sqrt" => Ok(UnaryFn::Sqrt),
        "exp" => Ok(UnaryFn::Exp),
        "exp2" => Ok(UnaryFn::Exp2),
        "ln" => Ok(UnaryFn::Ln),
        "log10" => Ok(UnaryFn::Log10),
        "rad" => Ok(UnaryFn::Rad),
        "deg" => Ok(UnaryFn::Deg),
        _ => Err(()),
    }
}

fn number(input: &str) -> IResult<&str, Expr> {
    let (input, n) = double(input)?;
    Ok((input, Expr::Number(n)))
}

fn ident(input: &str) -> IResult<&str, &str> {
    let (input, ident) = recognize(alpha1.and_then(alphanumeric0)).parse(input)?;
    Ok((input, ident))
}

fn unary_fn_call(input: &str) -> IResult<&str, Expr> {
    let (input, function) = map_res(ident, unary_function_to_enum).parse(input)?;
    let (input, arg) = delimited(char('('), expr, char(')')).parse(input)?;
    Ok((
        input,
        Expr::UnaryFnCall {
            function,
            arg: Box::new(arg),
        },
    ))
}

fn binary_fn_call(input: &str) -> IResult<&str, Expr> {
    todo!()
}

fn atom(input: &str) -> IResult<&str, Expr> {
    alt((
        number,
        unary_fn_call,
        binary_fn_call,
        delimited(char('('), expr, char(')')),
    ))
    .parse(input)
}

fn prefixed(input: &str) -> IResult<&str, Expr> {
    alt((
        preceded(char('-'), atom).map(|e| Expr::UnaryOp {
            op: UnaryOp::Neg,
            arg: Box::new(e),
        }),
        atom,
    ))
    .parse(input)
}

fn postfixed(input: &str) -> IResult<&str, Expr> {
    alt((
        terminated(atom, char('!')).map(|e| Expr::UnaryOp {
            op: UnaryOp::Fac,
            arg: Box::new(e),
        }),
        prefixed,
    ))
    .parse(input)
}

fn term(input: &str) -> IResult<&str, Expr> {
    delimited(space0, postfixed, space0).parse(input)
}

fn power(input: &str) -> IResult<&str, Expr> {
    let (input, (lhs, rhs)) = separated_pair(term, char('^'), power).parse(input)?;
    Ok((
        input,
        Expr::BinaryOp {
            op: BinaryOp::Pow,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
    ))
}

fn powers(input: &str) -> IResult<&str, Expr> {
    let (input, first) = power(input)?;
    let (input, tail) = many0(preceded(peek(not(number)), power)).parse(input)?;
    Ok((
        input,
        tail.into_iter().fold(first, |acc, rhs| Expr::BinaryOp {
            op: BinaryOp::Mul,
            lhs: Box::new(acc),
            rhs: Box::new(rhs),
        }),
    ))
}

fn product(input: &str) -> IResult<&str, Expr> {
    let (input, first) = powers(input)?;
    let (input, tail) = many0((
        one_of("*/").map(|c| match c {
            '*' => BinaryOp::Mul,
            '/' => BinaryOp::Div,
            _ => unreachable!(),
        }),
        powers,
    ))
    .parse(input)?;

    Ok((
        input,
        tail.into_iter()
            .fold(first, |acc, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(acc),
                rhs: Box::new(rhs),
            }),
    ))
}

fn sum(input: &str) -> IResult<&str, Expr> {
    let (input, first) = product(input)?;
    let (input, tail) = many0((
        one_of("+-").map(|c| match c {
            '+' => BinaryOp::Add,
            '-' => BinaryOp::Sub,
            _ => unreachable!(),
        }),
        product,
    ))
    .parse(input)?;

    Ok((
        input,
        tail.into_iter()
            .fold(first, |acc, (op, rhs)| Expr::BinaryOp {
                op,
                lhs: Box::new(acc),
                rhs: Box::new(rhs),
            }),
    ))
}

fn expr(input: &str) -> IResult<&str, Expr> {
    sum.parse(input)
}

pub fn parse(input: &str) -> Result<Expr, String> {
    match term.parse(input) {
        Ok((_, expr)) => Ok(expr),
        Err(e) => Err(format!("{}", e)),
    }
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
