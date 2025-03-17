use crate::lexer::Token;
use crate::types::*;

use chumsky::input::ValueInput;
use chumsky::prelude::*;

fn unary_function_to_enum(name: &str) -> UnaryFn {
    match name {
        "sin" => UnaryFn::Sin,
        "cos" => UnaryFn::Cos,
        "tan" => UnaryFn::Tan,
        "sec" => UnaryFn::Sec,
        "csc" => UnaryFn::Csc,
        "cot" => UnaryFn::Cot,
        "asin" => UnaryFn::Asin,
        "acos" => UnaryFn::Acos,
        "atan" => UnaryFn::Atan,
        "asec" => UnaryFn::Asec,
        "acsc" => UnaryFn::Acsc,
        "acot" => UnaryFn::Acot,
        "sinh" => UnaryFn::Sinh,
        "cosh" => UnaryFn::Cosh,
        "tanh" => UnaryFn::Tanh,
        "floor" => UnaryFn::Floor,
        "ceil" => UnaryFn::Ceil,
        "round" => UnaryFn::Round,
        "abs" => UnaryFn::Abs,
        "sqrt" => UnaryFn::Sqrt,
        "exp" => UnaryFn::Exp,
        "exp2" => UnaryFn::Exp2,
        "ln" => UnaryFn::Ln,
        "log10" => UnaryFn::Log10,
        "rad" => UnaryFn::Rad,
        "deg" => UnaryFn::Deg,
        _ => unreachable!("unimplemented unary function"),
    }
}

fn binary_function_to_enum(name: &str) -> BinaryFn {
    match name {
        "log" => BinaryFn::Log,
        "nroot" => BinaryFn::NRoot,
        _ => unreachable!("unknown binary function"),
    }
}

// pub fn parse(input: &str) -> Result<Expr, String> {
//     let (expr, errs) = parser().parse(input).into_output_errors();

//     errs.iter().for_each(|e| {
//         Report::build(ReportKind::Error, ("sample.tao", e.span().into_range()))
//             .with_message(e.to_string())
//             .with_label(
//                 Label::new(("", e.span().into_range()))
//                     .with_message(e.to_string())
//                     .with_color(Color::Red),
//             )
//             .finish()
//             .print(("", Source::from(input)))
//             .unwrap();
//     });

//     println!("{:?}", expr);

//     expr.ok_or(format!("{:?}", errs))
// }

#[allow(clippy::let_and_return)]
pub fn parser<'a, I>() -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let number = select! {
            Token::Number(n) => Expr::Number(n)
        };

        // let unary_fn_call = just(Token::Ident("sin"))
        //     .to(UnaryFn::Sin)
        //     .then_ignore(Token::Ctrl('('))
        //     .then(expr.clone())
        //     .then_ignore(Token::Ctrl(')'))
        //     .map(|(func, arg)| Expr::UnaryFnCall {
        //         function: func,
        //         arg: Box::new(arg),
        //     });

        // let atomic = choice((
        //     unary_fn_call,
        //     expr.clone()
        //         .delimited_by(just('(').padded(), just(')').padded()),
        //     expr.clone().delimited_by(just('|'), just('|')),
        // ));

        // let numbers = number
        //     .foldl(
        //         whitespace().at_least(1).ignore_then(number).repeated(),
        //         |acc, rhs| Expr::BinaryOp {
        //             op: BinaryOp::Mul,
        //             lhs: Box::new(acc),
        //             rhs: Box::new(rhs),
        //         },
        //     )
        //     .padded();

        // let polyatomic = numbers.or(atomic.clone()).foldl(
        //     atomic.clone().padded().repeated().at_least(1),
        //     |acc, rhs| Expr::BinaryOp {
        //         op: BinaryOp::Mul,
        //         lhs: Box::new(acc),
        //         rhs: Box::new(rhs),
        //     },
        // );

        // let op = |c| just(c).padded();

        // let prefixed = just('-')
        //     .then(atomic.clone())
        //     .map(|(_, rhs)| Expr::UnaryOp {
        //         op: UnaryOp::Neg,
        //         arg: Box::new(rhs),
        //     });

        // let postfixed = prefixed
        //     .clone()
        //     .or(number)
        //     .then(just('!'))
        //     .map(|(lhs, _)| Expr::UnaryOp {
        //         op: UnaryOp::Fac,
        //         arg: Box::new(lhs),
        //     });

        // let term = postfixed.or(prefixed).or(atomic).or(number).padded();

        // let power = term
        //     .clone()
        //     .then(op('^').to(BinaryOp::Pow))
        //     .repeated()
        //     .foldr(term, |(lhs, op), rhs| Expr::BinaryOp {
        //         op,
        //         lhs: Box::new(lhs),
        //         rhs: Box::new(rhs),
        //     });

        // let product = polyatomic.clone().or(power.clone()).foldl(
        //     choice((
        //         op('*').to(BinaryOp::Mul),
        //         op('/').to(BinaryOp::Div),
        //         op('%').to(BinaryOp::Rem),
        //     ))
        //     .then(polyatomic.or(power))
        //     .repeated(),
        //     |lhs, (op, rhs)| Expr::BinaryOp {
        //         op,
        //         lhs: Box::new(lhs),
        //         rhs: Box::new(rhs),
        //     },
        // );

        // let sum = product.clone().foldl(
        //     choice((op('+').to(BinaryOp::Add), op('-').to(BinaryOp::Sub)))
        //         .then(product)
        //         .repeated(),
        //     |lhs, (op, rhs)| Expr::BinaryOp {
        //         op,
        //         lhs: Box::new(lhs),
        //         rhs: Box::new(rhs),
        //     },
        // );

        number
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use Expr::*;

//     macro_rules! unop {
//         ($op_name:ident, $val:expr) => {
//             Expr::UnaryOp {
//                 op: super::UnaryOp::$op_name,
//                 arg: $val.into(),
//             }
//         };
//     }

//     macro_rules! binop {
//         ($op_name:ident, $lhs:expr, $rhs:expr) => {
//             Expr::BinaryOp {
//                 op: super::BinaryOp::$op_name,
//                 lhs: $lhs.into(),
//                 rhs: $rhs.into(),
//             }
//         };
//     }

//     #[test]
//     fn number() {
//         assert_eq!(parse("1"), Ok(Number(1.)));
//         assert_eq!(parse("   1"), Ok(Number(1.)));
//         assert_eq!(parse("0"), Ok(Number(0.)));
//         assert_eq!(parse("2.5"), Ok(Number(2.5)));
//         assert_eq!(parse("1e3"), Ok(Number(1e3)));
//         assert_eq!(parse("1e-3"), Ok(Number(1e-3)));
//         assert_eq!(parse("2.5e2"), Ok(Number(2.5e2)));
//         assert_eq!(parse("2.5e-2"), Ok(Number(2.5e-2)));
//         assert_eq!(parse("-1"), Ok(Number(-1.)));
//         assert_eq!(parse("-2.5"), Ok(Number(-2.5)));
//         assert_eq!(parse("-1e3"), Ok(Number(-1e3)));
//         assert_eq!(parse("-1e-3"), Ok(Number(-1e-3)));
//         assert_eq!(parse("-2.5e2"), Ok(Number(-2.5e2)));
//         assert_eq!(parse("-2.5e-2"), Ok(Number(-2.5e-2)));

//         // Tests that should fail
//         assert!(parse("abc").is_err());
//         assert!(parse("0.").is_err());
//         assert!(parse("- 3").is_err());
//         assert!(parse("1..2").is_err());
//         assert!(parse("1e").is_err());
//         assert!(parse("1e--3").is_err());
//         assert!(parse("--1").is_err());
//         assert!(parse("2.5.2").is_err());
//         assert!(parse("1e3.5").is_err());
//         assert!(parse("1 e3").is_err());
//         assert!(parse("1e 3").is_err());
//         assert!(parse("1e3 .5").is_err());
//         assert!(parse("1e3. 5").is_err());
//         assert!(parse("1e3 . 5").is_err());
//         assert!(parse("1 e 3").is_err());
//     }

//     #[test]
//     fn basic_ops() {
//         assert_eq!(parse("6*3"), Ok(binop!(Mul, Number(6.), Number(3.))));
//         assert_eq!(parse("6 * 3"), Ok(binop!(Mul, Number(6.), Number(3.))));
//         assert_eq!(parse("6* 3"), Ok(binop!(Mul, Number(6.), Number(3.))));
//         assert_eq!(parse("6 *3"), Ok(binop!(Mul, Number(6.), Number(3.))));
//         assert_eq!(parse("6+3"), Ok(binop!(Add, Number(6.), Number(3.))));
//         assert_eq!(parse("6-3"), Ok(binop!(Sub, Number(6.), Number(3.))));
//         assert_eq!(parse("6/3"), Ok(binop!(Div, Number(6.), Number(3.))));
//         assert_eq!(parse("6%3"), Ok(binop!(Rem, Number(6.), Number(3.))));
//         assert_eq!(parse("2^3"), Ok(binop!(Pow, Number(2.), Number(3.))));
//         assert_eq!(
//             parse("2 + 3 * 4"),
//             Ok(binop!(Add, Number(2.), binop!(Mul, Number(3.), Number(4.))))
//         );
//         assert_eq!(
//             parse("(2 + 3) * 4"),
//             Ok(binop!(Mul, binop!(Add, Number(2.), Number(3.)), Number(4.)))
//         );
//         assert_eq!(
//             parse("2 * (3 + 4)"),
//             Ok(binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))))
//         );
//         assert_eq!(
//             parse("2 * 3 + 4"),
//             Ok(binop!(Add, binop!(Mul, Number(2.), Number(3.)), Number(4.)))
//         );
//         assert_eq!(
//             parse("2 + 3 * 4 - 5 / 6"),
//             Ok(binop!(
//                 Sub,
//                 binop!(Add, Number(2.), binop!(Mul, Number(3.), Number(4.))),
//                 binop!(Div, Number(5.), Number(6.))
//             ))
//         );
//         assert_eq!(
//             parse("2 * (3 + 4) - 5 % 6"),
//             Ok(binop!(
//                 Sub,
//                 binop!(Mul, Number(2.), binop!(Add, Number(3.), Number(4.))),
//                 binop!(Rem, Number(5.), Number(6.))
//             ))
//         );
//         assert_eq!(parse("5!"), Ok(unop!(Fac, Number(5.))));
//         assert_eq!(
//             parse("-(2 + 3)"),
//             Ok(unop!(Neg, binop!(Add, Number(2.), Number(3.))))
//         );
//         assert_eq!(
//             parse("-(2 * 3) + 4"),
//             Ok(binop!(
//                 Add,
//                 unop!(Neg, binop!(Mul, Number(2.), Number(3.))),
//                 Number(4.)
//             ))
//         );
//         assert_eq!(
//             parse("2 * -(3 + 4)"),
//             Ok(binop!(
//                 Mul,
//                 Number(2.),
//                 unop!(Neg, binop!(Add, Number(3.), Number(4.)))
//             ))
//         );
//         assert_eq!(
//             parse("-(2 * 3 + 4)"),
//             Ok(unop!(
//                 Neg,
//                 binop!(Add, binop!(Mul, Number(2.), Number(3.)), Number(4.))
//             ))
//         );
//         assert_eq!(
//             parse("3! + 4"),
//             Ok(binop!(Add, unop!(Fac, Number(3.)), Number(4.)))
//         );
//         assert_eq!(parse("-(3!)"), Ok(unop!(Neg, unop!(Fac, Number(3.)))));
//         assert_eq!(parse("-3!"), Ok(unop!(Fac, Number(-3.))));
//         assert_eq!(
//             parse("2 ^ 3!"),
//             Ok(binop!(Pow, Number(2.), unop!(Fac, Number(3.))))
//         );
//         assert_eq!(
//             parse("-(2 ^ 3)"),
//             Ok(unop!(Neg, binop!(Pow, Number(2.), Number(3.))))
//         );
//         assert_eq!(parse("-2^3"), Ok(binop!(Pow, Number(-2.), Number(3.))));
//         assert_eq!(parse("2 ^ -3"), Ok(binop!(Pow, Number(2.), Number(-3.))));
//         assert_eq!(
//             parse("-(2 ^ -3)"),
//             Ok(unop!(Neg, binop!(Pow, Number(2.), Number(-3.))))
//         );
//         assert_eq!(parse("-(-3)"), Ok(unop!(Neg, Number(-3.))));
//         assert_eq!(parse("-2 (-3)"), Ok(binop!(Mul, Number(-2.), Number(-3.))));
//         assert_eq!(
//             parse("(5 + 3)  (-3)"),
//             Ok(binop!(
//                 Mul,
//                 binop!(Add, Number(5.), Number(3.)),
//                 Number(-3.)
//             ))
//         );

//         // Failing tests
//         assert!(parse("- 6*3").is_err());
//         assert!(parse("2 ** 3").is_err());
//         assert!(parse("2 // 3").is_err());
//         assert!(parse("2 %% 3").is_err());
//         assert!(parse("2 ^^ 3").is_err());
//         assert!(parse("2 +* 3").is_err());
//         assert!(parse("2 *+ 3").is_err());
//         assert!(parse("2 + 3 *").is_err());
//         assert!(parse("2 * (3 + 4").is_err());
//         assert!(parse("2 * 3 + 4)").is_err());
//         assert!(parse("2 * - 3").is_err());
//         assert!(parse("2 * (3 + )").is_err());
//         assert!(parse("2 * (3 + 4))").is_err());
//         assert!(parse("2 * ((3 + 4)").is_err());
//         assert!(parse("2 * (3 + (4)").is_err());
//         assert!(parse("2 * (3 + 4))").is_err());
//         assert!(parse("2 * (3 + 4) -").is_err());
//         assert!(parse("2 * (3 + 4) - 5 %").is_err());
//         assert!(parse("2 * (3 + 4) - 5 % 6)").is_err());
//         assert!(parse("--3").is_err());
//         assert!(parse("(5 % 3)  2").is_err(),);
//     }

//     #[test]
//     fn unary_function_calls() {
//         assert_eq!(
//             parse("sin(0)"),
//             Ok(Expr::UnaryFnCall {
//                 function: UnaryFn::Sin,
//                 arg: Box::new(Number(0.)),
//             })
//         );
//         assert_eq!(
//             parse("sin(3)"),
//             Ok(Expr::UnaryFnCall {
//                 function: UnaryFn::Sin,
//                 arg: Box::new(Number(3.)),
//             })
//         );
//         assert_eq!(
//             parse("sin(-3.)"),
//             Ok(Expr::UnaryFnCall {
//                 function: UnaryFn::Sin,
//                 arg: Box::new(Number(-3.)),
//             })
//         );

//         // Failing tests
//         assert!(parse("sin()").is_err());
//         assert!(parse("sin(3, 4)").is_err());
//         assert!(parse("sin(abc)").is_err());
//     }

//     #[test]
//     fn binary_function_calls() {
//         assert_eq!(
//             parse("log(1, 10)"),
//             Ok(Expr::BinaryFnCall {
//                 function: BinaryFn::Log,
//                 arg1: Box::new(Number(1.)),
//                 arg2: Box::new(Number(10.)),
//             })
//         );
//         assert_eq!(
//             parse("log(2.5, 10)"),
//             Ok(Expr::BinaryFnCall {
//                 function: BinaryFn::Log,
//                 arg1: Box::new(Number(2.5)),
//                 arg2: Box::new(Number(10.)),
//             })
//         );
//         assert_eq!(
//             parse("log(2.5, 2.5)"),
//             Ok(Expr::BinaryFnCall {
//                 function: BinaryFn::Log,
//                 arg1: Box::new(Number(2.5)),
//                 arg2: Box::new(Number(2.5)),
//             })
//         );

//         // Failing tests
//         assert!(parse("log(1)").is_err());
//         assert!(parse("log(1, 2, 3)").is_err());
//         assert!(parse("log(abc, 10)").is_err());
//     }

//     #[test]
//     fn mixed_operations_and_number_notations() {
//         // Mixed operations
//         assert_eq!(
//             parse("sin(2 + 3) * 4"),
//             Ok(binop!(
//                 Mul,
//                 Expr::UnaryFnCall {
//                     function: UnaryFn::Sin,
//                     arg: Box::new(binop!(Add, Number(2.), Number(3.))),
//                 },
//                 Number(4.)
//             ))
//         );
//         assert_eq!(
//             parse("2 * log(3 + 4, 10)"),
//             Ok(binop!(
//                 Mul,
//                 Number(2.),
//                 Expr::BinaryFnCall {
//                     function: BinaryFn::Log,
//                     arg1: Box::new(binop!(Add, Number(3.), Number(4.))),
//                     arg2: Box::new(Number(10.)),
//                 }
//             ))
//         );
//         assert_eq!(
//             parse("2 * sin(3 + 4) - log(5, 6)"),
//             Ok(binop!(
//                 Sub,
//                 binop!(
//                     Mul,
//                     Number(2.),
//                     Expr::UnaryFnCall {
//                         function: UnaryFn::Sin,
//                         arg: Box::new(binop!(Add, Number(3.), Number(4.))),
//                     }
//                 ),
//                 Expr::BinaryFnCall {
//                     function: BinaryFn::Log,
//                     arg1: Box::new(Number(5.)),
//                     arg2: Box::new(Number(6.)),
//                 }
//             ))
//         );
//         assert_eq!(
//             parse("2 * (3 + sin(4))"),
//             Ok(binop!(
//                 Mul,
//                 Number(2.),
//                 binop!(
//                     Add,
//                     Number(3.),
//                     Expr::UnaryFnCall {
//                         function: UnaryFn::Sin,
//                         arg: Box::new(Number(4.)),
//                     }
//                 )
//             ))
//         );
//         assert_eq!(
//             parse("log(2, 3) + sin(4)"),
//             Ok(binop!(
//                 Add,
//                 Expr::BinaryFnCall {
//                     function: BinaryFn::Log,
//                     arg1: Box::new(Number(2.)),
//                     arg2: Box::new(Number(3.)),
//                 },
//                 Expr::UnaryFnCall {
//                     function: UnaryFn::Sin,
//                     arg: Box::new(Number(4.)),
//                 }
//             ))
//         );

//         // Different number notations
//         assert_eq!(
//             parse("1e3 + 2.5"),
//             Ok(binop!(Add, Number(1e3), Number(2.5)))
//         );
//         assert_eq!(
//             parse("1e-3 * 2.5"),
//             Ok(binop!(Mul, Number(1e-3), Number(2.5)))
//         );
//         assert_eq!(
//             parse("2.5e2 - 1"),
//             Ok(binop!(Sub, Number(2.5e2), Number(1.)))
//         );
//         assert_eq!(
//             parse("2.5e-2 / 1e3"),
//             Ok(binop!(Div, Number(2.5e-2), Number(1e3)))
//         );
//         assert_eq!(
//             parse("-1e3 + 2.5"),
//             Ok(binop!(Add, Number(-1e3), Number(2.5)))
//         );
//         assert_eq!(
//             parse("2.5e2 % 1e3"),
//             Ok(binop!(Rem, Number(2.5e2), Number(1e3)))
//         );

//         // Combining scientific notation and function calls
//         assert_eq!(
//             parse("sin(1e3) + 2.5"),
//             Ok(binop!(
//                 Add,
//                 Expr::UnaryFnCall {
//                     function: UnaryFn::Sin,
//                     arg: Box::new(Number(1e3)),
//                 },
//                 Number(2.5)
//             ))
//         );
//         assert_eq!(
//             parse("log(1e-3, 2.5) * 10"),
//             Ok(binop!(
//                 Mul,
//                 Expr::BinaryFnCall {
//                     function: BinaryFn::Log,
//                     arg1: Box::new(Number(1e-3)),
//                     arg2: Box::new(Number(2.5)),
//                 },
//                 Number(10.)
//             ))
//         );
//         assert_eq!(
//             parse("2 * sin(2.5e2) - log(1, 1e3)"),
//             Ok(binop!(
//                 Sub,
//                 binop!(
//                     Mul,
//                     Number(2.),
//                     Expr::UnaryFnCall {
//                         function: UnaryFn::Sin,
//                         arg: Box::new(Number(2.5e2)),
//                     }
//                 ),
//                 Expr::BinaryFnCall {
//                     function: BinaryFn::Log,
//                     arg1: Box::new(Number(1.)),
//                     arg2: Box::new(Number(1e3)),
//                 }
//             ))
//         );

//         // Failing tests for mixed operations

//         // Missing closing parenthesis
//         assert!(parse("2 * (3 + sin(4)").is_err());

//         // Missing opening parenthesis
//         assert!(parse("2 * 3 + sin 4)").is_err());

//         // Missing argument for sin function
//         assert!(parse("2 * sin()").is_err());

//         // Extra comma in log function
//         assert!(parse("log(2, 3,) + sin(4)").is_err());

//         // Invalid character in expression
//         assert!(parse("2 * (3 + sin(4) @)").is_err());

//         // Missing operator between numbers
//         assert!(parse("2 3 + sin(4)").is_err());

//         // Missing operator between function call and number
//         assert!(parse("sin(4) 2 + 3").is_err());

//         // Invalid function name
//         assert!(parse("invalid(2, 3) + sin(4)").is_err());

//         // Unmatched parentheses
//         assert!(parse("2 * (3 + sin(4)) + (5").is_err());

//         // Extra closing parenthesis
//         assert!(parse("2 * (3 + sin(4))) + 5").is_err());

//         // Missing argument for log function
//         assert!(parse("log(2) + sin(4)").is_err());

//         // Extra argument for sin function
//         assert!(parse("sin(2, 3) + 4").is_err());

//         // Invalid number format
//         assert!(parse("2 * (3 + sin(4.5.6))").is_err());

//         // Missing operator between function calls
//         assert!(parse("sin(2) log(3, 4)").is_err());

//         // Invalid character in function argument
//         assert!(parse("sin(2 + 3a) + 4").is_err());
//     }
// }
