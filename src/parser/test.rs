use super::*;
use crate::models::operators;
use Expr::*;

macro_rules! preop {
    ($op_name:ident, $val:expr) => {
        Expr::PrefixOp {
            op: operators::PrefixOp::$op_name,
            arg: $val.into(),
        }
    };
}

macro_rules! postop {
    ($op_name:ident, $val:expr) => {
        Expr::PostfixOp {
            op: operators::PostfixOp::$op_name,
            arg: $val.into(),
        }
    };
}

macro_rules! binop {
    ($op_name:ident, $lhs:expr, $rhs:expr) => {
        Expr::InfixOp {
            op: operators::InfixOp::$op_name,
            lhs: $lhs.into(),
            rhs: $rhs.into(),
        }
    };
}

fn parse_expr(input: &str) -> Result<Expr, String> {
    use crate::models::Token;
    use chumsky::input::Stream;
    use logos::Logos;

    let token_iter = Token::lexer(input)
        .spanned()
        .map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        })
        .filter(|(tok, _)| !matches!(tok, Token::Space));

    let token_stream = Stream::from_iter(token_iter.clone()).map((0..input.len()).into(), |x| x);

    expression()
        .parse(token_stream)
        .into_result()
        .map_err(|e| format!("parse error {:?}", e))
}

fn parse_stmt(input: &str) -> Result<Stmt, String> {
    crate::lex_and_parse(input).map_err(|e| format!("parse error {:?}", e))
}

#[test]
fn number() {
    assert_eq!(parse_expr("1"), Ok(Int(1)));
    assert_eq!(parse_expr("   1"), Ok(Int(1)));
    assert_eq!(parse_expr("0"), Ok(Int(0)));
    assert_eq!(parse_expr("2.5"), Ok(Float(2.5)));
    assert_eq!(parse_expr("1e3"), Ok(Float(1e3)));
    assert_eq!(parse_expr("1e-3"), Ok(Float(1e-3)));
    assert_eq!(parse_expr("2.5e2"), Ok(Float(2.5e2)));
    assert_eq!(parse_expr("2.5e-2"), Ok(Float(2.5e-2)));

    // Tests that should fail
    assert!(parse_expr("1..2").is_err());
    assert!(parse_expr("2.5.2").is_err());
    assert!(parse_expr("1e3.5").is_err());
    assert!(parse_expr("1e3 .5").is_err());
    assert!(parse_expr("1e3. 5").is_err());
    assert!(parse_expr("1e3 . 5").is_err());
}

#[test]
fn basic_ops() {
    assert_eq!(parse_expr("6*3"), Ok(binop!(Mul, Int(6), Int(3))));
    assert_eq!(parse_expr("6 * 3"), Ok(binop!(Mul, Int(6), Int(3))));
    assert_eq!(parse_expr("6* 3"), Ok(binop!(Mul, Int(6), Int(3))));
    assert_eq!(parse_expr("6 *3"), Ok(binop!(Mul, Int(6), Int(3))));
    assert_eq!(parse_expr("6+3"), Ok(binop!(Add, Int(6), Int(3))));
    assert_eq!(parse_expr("6-3"), Ok(binop!(Sub, Int(6), Int(3))));
    assert_eq!(parse_expr("6/3"), Ok(binop!(Div, Int(6), Int(3))));
    assert_eq!(parse_expr("6%3"), Ok(binop!(Rem, Int(6), Int(3))));
    assert_eq!(parse_expr("2^3"), Ok(binop!(Pow, Int(2), Int(3))));
    assert_eq!(parse_expr("6//3"), Ok(binop!(IntDiv, Int(6), Int(3))));

    // Different number notations
    assert_eq!(
        parse_expr("1e3 + 2.5"),
        Ok(binop!(Add, Float(1e3), Float(2.5)))
    );
    assert_eq!(
        parse_expr("1e-3 * 2.5"),
        Ok(binop!(Mul, Float(1e-3), Float(2.5)))
    );
    assert_eq!(
        parse_expr("2.5e2 - 1"),
        Ok(binop!(Sub, Float(2.5e2), Int(1)))
    );
    assert_eq!(
        parse_expr("2.5e-2 / 1e3"),
        Ok(binop!(Div, Float(2.5e-2), Float(1e3)))
    );
    assert_eq!(
        parse_expr("-1e3 + 2.5"),
        Ok(binop!(Add, preop!(Neg, Float(1e3)), Float(2.5)))
    );
    assert_eq!(
        parse_expr("2.5e2 % 1e3"),
        Ok(binop!(Rem, Float(2.5e2), Float(1e3)))
    );

    assert_eq!(
        parse_expr("2 + 3 * 4"),
        Ok(binop!(Add, Int(2), binop!(Mul, Int(3), Int(4))))
    );
    assert_eq!(
        parse_expr("(2 + 3) * 4"),
        Ok(binop!(Mul, binop!(Add, Int(2), Int(3)), Int(4)))
    );
    assert_eq!(
        parse_expr("2 * (3 + 4)"),
        Ok(binop!(Mul, Int(2), binop!(Add, Int(3), Int(4))))
    );
    assert_eq!(
        parse_expr("2 * 3 + 4"),
        Ok(binop!(Add, binop!(Mul, Int(2), Int(3)), Int(4)))
    );
    assert_eq!(
        parse_expr("2 + 3 * 4 - 5 / 6"),
        Ok(binop!(
            Sub,
            binop!(Add, Int(2), binop!(Mul, Int(3), Int(4))),
            binop!(Div, Int(5), Int(6))
        ))
    );
    assert_eq!(
        parse_expr("2 * (3 + 4) - 5 % 6"),
        Ok(binop!(
            Sub,
            binop!(Mul, Int(2), binop!(Add, Int(3), Int(4))),
            binop!(Rem, Int(5), Int(6))
        ))
    );
    assert_eq!(parse_expr("5!"), Ok(postop!(Fac, Int(5))));
    assert_eq!(
        parse_expr("-(2 + 3)"),
        Ok(preop!(Neg, binop!(Add, Int(2), Int(3))))
    );
    assert_eq!(
        parse_expr("-(2 * 3) + 4"),
        Ok(binop!(
            Add,
            preop!(Neg, binop!(Mul, Int(2), Int(3))),
            Int(4)
        ))
    );
    assert_eq!(
        parse_expr("2 * -(3 + 4)"),
        Ok(binop!(
            Mul,
            Int(2),
            preop!(Neg, binop!(Add, Int(3), Int(4)))
        ))
    );
    assert_eq!(
        parse_expr("-(2 * 3 + 4)"),
        Ok(preop!(
            Neg,
            binop!(Add, binop!(Mul, Int(2), Int(3)), Int(4))
        ))
    );
    assert_eq!(
        parse_expr("3! + 4"),
        Ok(binop!(Add, postop!(Fac, Int(3)), Int(4)))
    );
    assert_eq!(parse_expr("-(3!)"), Ok(preop!(Neg, postop!(Fac, Int(3)))));
    assert_eq!(parse_expr("-3!"), Ok(preop!(Neg, postop!(Fac, Int(3)))));
    assert_eq!(
        parse_expr("2 ^ 3!"),
        Ok(binop!(Pow, Int(2), postop!(Fac, Int(3))))
    );
    assert_eq!(
        parse_expr("-(2 ^ 3)"),
        Ok(preop!(Neg, binop!(Pow, Int(2), Int(3))))
    );
    assert_eq!(
        parse_expr("-2^3"),
        Ok(preop!(Neg, binop!(Pow, Int(2), Int(3))))
    );
    assert_eq!(
        parse_expr("2 ^ (-3)"),
        Ok(binop!(Pow, Int(2), preop!(Neg, Int(3))))
    );
    assert_eq!(
        parse_expr("-(2 ^ (-3))"),
        Ok(preop!(Neg, binop!(Pow, Int(2), preop!(Neg, Int(3)))))
    );
    assert_eq!(parse_expr("-(-3)"), Ok(preop!(Neg, preop!(Neg, Int(3)))));
    assert_eq!(
        parse_expr("-2 (-3)"),
        Ok(binop!(Mul, preop!(Neg, Int(2)), preop!(Neg, Int(3))))
    );
    assert_eq!(
        parse_expr("(5 + 3)  (-3)"),
        Ok(binop!(
            Mul,
            binop!(Add, Int(5), Int(3)),
            preop!(Neg, Int(3))
        ))
    );

    // Failing tests
    assert!(parse_expr("2 %% 3").is_err());
    assert!(parse_expr("2 ^^ 3").is_err());
    assert!(parse_expr("2 ** 3").is_err());
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
    assert!(parse_expr("--1").is_err());
    assert!(parse_expr("--3").is_err());
}

#[test]
fn function_calls() {
    // Unary function calls
    assert_eq!(
        parse_expr("sin(0.)"),
        Ok(Expr::FnCall {
            name: String::from("sin"),
            args: vec![Float(0.)],
        })
    );

    assert_eq!(
        parse_expr("sin(3)"),
        Ok(Expr::FnCall {
            name: String::from("sin"),
            args: vec![Int(3)],
        })
    );

    // Binary function calls
    assert_eq!(
        parse_expr("log(1, 10)"),
        Ok(Expr::FnCall {
            name: String::from("log"),
            args: vec![Int(1), Int(10)],
        })
    );
    assert_eq!(
        parse_expr("log(2.5, 10)"),
        Ok(Expr::FnCall {
            name: String::from("log"),
            args: vec![Float(2.5), Int(10)],
        })
    );
    assert_eq!(
        parse_expr("log(2.5, 2.5)"),
        Ok(Expr::FnCall {
            name: String::from("log"),
            args: vec![Float(2.5), Float(2.5)],
        })
    );
    assert_eq!(
        parse_expr("log(1, 10) !"),
        Ok(postop!(
            Fac,
            Expr::FnCall {
                name: String::from("log"),
                args: vec![Int(1), Int(10)],
            }
        ))
    );

    // Failing tests for calls
    assert!(parse_expr("log(1, )").is_err()); // Missing second argument with trailing comma
    assert!(parse_expr("log(, 10)").is_err()); // Missing first argument
    assert!(parse_expr("log(1, 10").is_err()); // Missing closing parenthesis
    assert!(parse_expr("log 1, 10)").is_err()); // Missing opening parenthesis
    assert!(parse_expr("log(1, 10))").is_err()); // Extra closing parenthesis
    assert!(parse_expr("log((1, 10)").is_err()); // Extra opening parenthesis
    assert!(parse_expr("log(1, (10)").is_err()); // Unmatched parentheses
    assert!(parse_expr("log(1, 10))").is_err()); // Extra closing parenthesis
    assert!(parse_expr("log(1, 10) +").is_err()); // Trailing operator
}

#[test]
fn mathematical_notations() {
    assert_eq!(
        parse_expr("2 sin(3)"),
        Ok(binop!(
            Mul,
            Int(2),
            Expr::FnCall {
                name: String::from("sin"),
                args: vec![Int(3)],
            }
        ))
    );
    assert_eq!(
        parse_expr("2 (5 + 2)"),
        Ok(binop!(Mul, Int(2), binop!(Add, Int(5), Int(2))))
    );
    assert_eq!(
        parse_expr("3 (4 + 5) sin(6)"),
        Ok(binop!(
            Mul,
            binop!(Mul, Int(3), binop!(Add, Int(4), Int(5))),
            Expr::FnCall {
                name: String::from("sin"),
                args: vec![Int(6)],
            }
        ))
    );
    assert_eq!(
        parse_expr("2 (3 + 4) (5 + 6)"),
        Ok(binop!(
            Mul,
            binop!(Mul, Int(2), binop!(Add, Int(3), Int(4))),
            binop!(Add, Int(5), Int(6))
        ))
    );
    assert_eq!(
        parse_expr("2 sin(3 + 4)"),
        Ok(binop!(
            Mul,
            Int(2),
            Expr::FnCall {
                name: String::from("sin"),
                args: vec![binop!(Add, Int(3), Int(4))],
            }
        ))
    );
    assert_eq!(
        parse_expr("2 (3 + sin(4))"),
        Ok(binop!(
            Mul,
            Int(2),
            binop!(
                Add,
                Int(3),
                Expr::FnCall {
                    name: String::from("sin"),
                    args: vec![Int(4)],
                }
            )
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
    assert!(parse_expr("2^2 3^3 4^4").is_err());
    assert!(parse_expr("2 ^ 2 3 ^ 3  4 ^ 4").is_err());
}

#[test]
fn mixed_operations_and_number_notations() {
    // Mixed operations
    assert_eq!(
        parse_expr("sin(2 + 3) * 4"),
        Ok(binop!(
            Mul,
            Expr::FnCall {
                name: String::from("sin"),
                args: vec![binop!(Add, Int(2), Int(3))],
            },
            Int(4)
        ))
    );
    assert_eq!(
        parse_expr("2 * log(3 + 4, 10)"),
        Ok(binop!(
            Mul,
            Int(2),
            Expr::FnCall {
                name: String::from("log"),
                args: vec![binop!(Add, Int(3), Int(4)), Int(10)],
            }
        ))
    );

    assert_eq!(
        parse_expr("2 * sin(3 + 4) - log(5, 6)"),
        Ok(binop!(
            Sub,
            binop!(
                Mul,
                Int(2),
                Expr::FnCall {
                    name: String::from("sin"),
                    args: vec![binop!(Add, Int(3), Int(4))],
                }
            ),
            Expr::FnCall {
                name: String::from("log"),
                args: vec![Int(5), Int(6)],
            }
        ))
    );
    assert_eq!(
        parse_expr("2 * (3 + sin(4))"),
        Ok(binop!(
            Mul,
            Int(2),
            binop!(
                Add,
                Int(3),
                Expr::FnCall {
                    name: String::from("sin"),
                    args: vec![Int(4)],
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
                name: String::from("sin"),
                args: vec![Float(1e3)],
            },
            Float(2.5)
        ))
    );
    assert_eq!(
        parse_expr("log(1e-3, 2.5) * 10"),
        Ok(binop!(
            Mul,
            Expr::FnCall {
                name: String::from("log"),
                args: vec![Float(1e-3), Float(2.5)],
            },
            Int(10)
        ))
    );
    assert_eq!(
        parse_expr("2 * sin(2.5e2) - log(1, 1e3)"),
        Ok(binop!(
            Sub,
            binop!(
                Mul,
                Int(2),
                Expr::FnCall {
                    name: String::from("sin"),
                    args: vec![Float(2.5e2)],
                }
            ),
            Expr::FnCall {
                name: String::from("log"),
                args: vec![Int(1), Float(1e3)],
            }
        ))
    );

    // Failing tests for mixed operations
    assert!(parse_expr("2 * (3 + sin(4)").is_err()); // Missing closing parenthesis
    assert!(parse_expr("log(2, 3,) + sin(4)").is_err()); // Extra comma in log function
    assert!(parse_expr("2 * (3 + sin(4) @)").is_err()); // Invalid character in expression
    assert!(parse_expr("2 * (3 + sin(4)) + (5").is_err()); // Unmatched parentheses
    assert!(parse_expr("2 * (3 + sin(4))) + 5").is_err()); // Extra closing parenthesis
    assert!(parse_expr("2 * (3 + sin(4.5.6))").is_err()); // Invalid number format
}

#[test]
fn variable_definition_test() {
    assert_eq!(
        parse_stmt("let x = 42"),
        Ok(Stmt::DefVar {
            name: String::from("x"),
            expr: Expr::Int(42),
        })
    );

    assert_eq!(
        parse_stmt("let y = 3.15"),
        Ok(Stmt::DefVar {
            name: String::from("y"),
            expr: Expr::Float(3.15),
        })
    );

    assert_eq!(
        parse_stmt("let z = x + y"),
        Ok(Stmt::DefVar {
            name: String::from("z"),
            expr: binop!(
                Add,
                Expr::Variable(String::from("x")),
                Expr::Variable(String::from("y"))
            )
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
            name: String::from("add"),
            arg_names: vec![String::from("a"), String::from("b")],
            body: binop!(
                Add,
                Expr::Variable(String::from("a")),
                Expr::Variable(String::from("b"))
            ),
        })
    );

    assert_eq!(
        parse_stmt("let square(x) = x * x"),
        Ok(Stmt::DefFun {
            name: String::from("square"),
            arg_names: vec![String::from("x")],
            body: binop!(
                Mul,
                Expr::Variable(String::from("x")),
                Expr::Variable(String::from("x"))
            ),
        })
    );

    assert_eq!(
        parse_stmt("let negate(x) = -x"),
        Ok(Stmt::DefFun {
            name: String::from("negate"),
            arg_names: vec![String::from("x")],
            body: preop!(Neg, Expr::Variable(String::from("x"))),
        })
    );

    // Failing tests
    assert!(parse_stmt("let add(a, b) = ").is_err());
    assert!(parse_stmt("let add(a, b = a + b").is_err());
    assert!(parse_stmt("let add(a b) = a + b").is_err());
    assert!(parse_stmt("let add(a, b) a + b").is_err());
}
