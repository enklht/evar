use pest::Parser;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};
use pest_derive::Parser;
use std::sync::LazyLock;

static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    use Rule::*;
    use pest::pratt_parser::{Assoc::*, Op};

    PrattParser::new()
        .op(Op::infix(add, Left) | Op::infix(sub, Left))
        .op(Op::infix(mul, Left) | Op::infix(div, Left) | Op::infix(rem, Left))
        .op(Op::infix(pow, Right))
        .op(Op::postfix(fac))
        .op(Op::prefix(Rule::neg))
});

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum UnaryFunction {
    Sin,
}

#[derive(Debug, PartialEq)]
pub enum BinaryFunction {
    Log,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Neg,
    Fac,
    Power,
}

#[derive(Debug, PartialEq)]
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
        Expr::UnaryOperation {
            operator: UnaryOperator::$op_name,
            arg: $val.into(),
        }
    };
}

macro_rules! binop {
    ($op_name:ident, $lhs:expr, $rhs:expr) => {
        Expr::BinaryOperation {
            operator: BinaryOperator::$op_name,
            lhs: $lhs.into(),
            rhs: $rhs.into(),
        }
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
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Ok(parse_number(primary.as_str())),
            Rule::binaryfncall => parse_binary_function(primary),
            Rule::unaryfncall => parse_unary_function(primary),
            Rule::expr => parse_expr(primary.into_inner()),
            _ => unreachable!(),
        })
        .map_prefix(|op, val| match op.as_rule() {
            Rule::neg => Ok(unop!(Neg, val?)),
            _ => unreachable!(),
        })
        .map_postfix(|val, op| match op.as_rule() {
            Rule::fac => Ok(unop!(Fac, val?)),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => Ok(binop!(Add, lhs?, rhs?)),
            Rule::sub => Ok(binop!(Sub, lhs?, rhs?)),
            Rule::mul => Ok(binop!(Mul, lhs?, rhs?)),
            Rule::div => Ok(binop!(Div, lhs?, rhs?)),
            Rule::rem => Ok(binop!(Rem, lhs?, rhs?)),
            Rule::pow => Ok(binop!(Pow, lhs?, rhs?)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;

    #[test]
    fn number() {
        assert_eq!(parse("1"), Ok(Int(1)));
        assert_eq!(parse("0"), Ok(Int(0)));
        assert_eq!(parse("0."), Ok(Float(0.)));
        assert_eq!(parse("2.5"), Ok(Float(2.5)));
        assert_eq!(parse("1e3"), Ok(Float(1e3)));
        assert_eq!(parse("1e-3"), Ok(Float(1e-3)));
        assert_eq!(parse("2.5e2"), Ok(Float(2.5e2)));
        assert_eq!(parse("2.5e-2"), Ok(Float(2.5e-2)));
        assert_eq!(parse("-1"), Ok(Int(-1)));
        assert_eq!(parse("-2.5"), Ok(Float(-2.5)));
        assert_eq!(parse("-1e3"), Ok(Float(-1e3)));
        assert_eq!(parse("-1e-3"), Ok(Float(-1e-3)));
        assert_eq!(parse("-2.5e2"), Ok(Float(-2.5e2)));
        assert_eq!(parse("-2.5e-2"), Ok(Float(-2.5e-2)));

        // Tests that should fail
        assert!(parse("abc").is_err());
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
        assert_eq!(parse("6*3"), Ok(binop!(Mul, Int(6), Int(3))));
        assert_eq!(parse("6 * 3"), Ok(binop!(Mul, Int(6), Int(3))));
        assert_eq!(parse("6* 3"), Ok(binop!(Mul, Int(6), Int(3))));
        assert_eq!(parse("6 *3"), Ok(binop!(Mul, Int(6), Int(3))));
        assert_eq!(parse("6+3"), Ok(binop!(Add, Int(6), Int(3))));
        assert_eq!(parse("6-3"), Ok(binop!(Sub, Int(6), Int(3))));
        assert_eq!(parse("6/3"), Ok(binop!(Div, Int(6), Int(3))));
        assert_eq!(parse("6%3"), Ok(binop!(Rem, Int(6), Int(3))));
        assert_eq!(parse("2^3"), Ok(binop!(Pow, Int(2), Int(3))));
        assert_eq!(
            parse("2 + 3 * 4"),
            Ok(binop!(Add, Int(2), binop!(Mul, Int(3), Int(4))))
        );
        assert_eq!(
            parse("(2 + 3) * 4"),
            Ok(binop!(Mul, binop!(Add, Int(2), Int(3)), Int(4)))
        );
        assert_eq!(
            parse("2 * (3 + 4)"),
            Ok(binop!(Mul, Int(2), binop!(Add, Int(3), Int(4))))
        );
        assert_eq!(
            parse("2 * 3 + 4"),
            Ok(binop!(Add, binop!(Mul, Int(2), Int(3)), Int(4)))
        );
        assert_eq!(
            parse("2 + 3 * 4 - 5 / 6"),
            Ok(binop!(
                Sub,
                binop!(Add, Int(2), binop!(Mul, Int(3), Int(4))),
                binop!(Div, Int(5), Int(6))
            ))
        );
        assert_eq!(
            parse("2 * (3 + 4) - 5 % 6"),
            Ok(binop!(
                Sub,
                binop!(Mul, Int(2), binop!(Add, Int(3), Int(4))),
                binop!(Rem, Int(5), Int(6))
            ))
        );
        assert_eq!(parse("5!"), Ok(unop!(Fac, Int(5))));
        assert_eq!(
            parse("-(2 + 3)"),
            Ok(unop!(Neg, binop!(Add, Int(2), Int(3))))
        );
        assert_eq!(
            parse("-(2 * 3) + 4"),
            Ok(binop!(Add, unop!(Neg, binop!(Mul, Int(2), Int(3))), Int(4)))
        );
        assert_eq!(
            parse("2 * -(3 + 4)"),
            Ok(binop!(Mul, Int(2), unop!(Neg, binop!(Add, Int(3), Int(4)))))
        );
        assert_eq!(
            parse("-(2 * 3 + 4)"),
            Ok(unop!(Neg, binop!(Add, binop!(Mul, Int(2), Int(3)), Int(4))))
        );
        assert_eq!(parse("3! + 4"), Ok(binop!(Add, unop!(Fac, Int(3)), Int(4))));
        assert_eq!(parse("-(3!)"), Ok(unop!(Neg, unop!(Fac, Int(3)))));
        assert_eq!(parse("-3!"), Ok(unop!(Fac, Int(-3))));
        assert_eq!(parse("2 ^ 3!"), Ok(binop!(Pow, Int(2), unop!(Fac, Int(3)))));
        assert_eq!(
            parse("-(2 ^ 3)"),
            Ok(unop!(Neg, binop!(Pow, Int(2), Int(3))))
        );
        assert_eq!(parse("-2^3"), Ok(binop!(Pow, Int(-2), Int(3))));
        assert_eq!(parse("2 ^ -3"), Ok(binop!(Pow, Int(2), Int(-3))));
        assert_eq!(
            parse("-(2 ^ -3)"),
            Ok(unop!(Neg, binop!(Pow, Int(2), Int(-3))))
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
    }

    #[test]
    fn unary_function_calls() {
        assert_eq!(
            parse("sin(0)"),
            Ok(Expr::UnaryFunctionCall {
                function: UnaryFunction::Sin,
                arg: Box::new(Int(0)),
            })
        );
        assert_eq!(
            parse("sin(3.14)"),
            Ok(Expr::UnaryFunctionCall {
                function: UnaryFunction::Sin,
                arg: Box::new(Float(3.14)),
            })
        );
        assert_eq!(
            parse("sin(-3.14)"),
            Ok(Expr::UnaryFunctionCall {
                function: UnaryFunction::Sin,
                arg: Box::new(Float(-3.14)),
            })
        );

        // Failing tests
        assert!(parse("sin()").is_err());
        assert!(parse("sin(3, 4)").is_err());
        assert!(parse("sin(abc)").is_err());
    }

    #[test]
    fn binary_function_calls() {
        assert_eq!(
            parse("log(1, 10)"),
            Ok(Expr::BinaryFunctionCall {
                function: BinaryFunction::Log,
                arg1: Box::new(Int(1)),
                arg2: Box::new(Int(10)),
            })
        );
        assert_eq!(
            parse("log(2.5, 10)"),
            Ok(Expr::BinaryFunctionCall {
                function: BinaryFunction::Log,
                arg1: Box::new(Float(2.5)),
                arg2: Box::new(Int(10)),
            })
        );
        assert_eq!(
            parse("log(2.5, 2.5)"),
            Ok(Expr::BinaryFunctionCall {
                function: BinaryFunction::Log,
                arg1: Box::new(Float(2.5)),
                arg2: Box::new(Float(2.5)),
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
                Expr::UnaryFunctionCall {
                    function: UnaryFunction::Sin,
                    arg: Box::new(binop!(Add, Int(2), Int(3))),
                },
                Int(4)
            ))
        );
        assert_eq!(
            parse("2 * log(3 + 4, 10)"),
            Ok(binop!(
                Mul,
                Int(2),
                Expr::BinaryFunctionCall {
                    function: BinaryFunction::Log,
                    arg1: Box::new(binop!(Add, Int(3), Int(4))),
                    arg2: Box::new(Int(10)),
                }
            ))
        );
        assert_eq!(
            parse("2 * sin(3 + 4) - log(5, 6)"),
            Ok(binop!(
                Sub,
                binop!(
                    Mul,
                    Int(2),
                    Expr::UnaryFunctionCall {
                        function: UnaryFunction::Sin,
                        arg: Box::new(binop!(Add, Int(3), Int(4))),
                    }
                ),
                Expr::BinaryFunctionCall {
                    function: BinaryFunction::Log,
                    arg1: Box::new(Int(5)),
                    arg2: Box::new(Int(6)),
                }
            ))
        );
        assert_eq!(
            parse("2 * (3 + sin(4))"),
            Ok(binop!(
                Mul,
                Int(2),
                binop!(
                    Add,
                    Int(3),
                    Expr::UnaryFunctionCall {
                        function: UnaryFunction::Sin,
                        arg: Box::new(Int(4)),
                    }
                )
            ))
        );
        assert_eq!(
            parse("log(2, 3) + sin(4)"),
            Ok(binop!(
                Add,
                Expr::BinaryFunctionCall {
                    function: BinaryFunction::Log,
                    arg1: Box::new(Int(2)),
                    arg2: Box::new(Int(3)),
                },
                Expr::UnaryFunctionCall {
                    function: UnaryFunction::Sin,
                    arg: Box::new(Int(4)),
                }
            ))
        );

        // Different number notations
        assert_eq!(parse("1e3 + 2.5"), Ok(binop!(Add, Float(1e3), Float(2.5))));
        assert_eq!(
            parse("1e-3 * 2.5"),
            Ok(binop!(Mul, Float(1e-3), Float(2.5)))
        );
        assert_eq!(parse("2.5e2 - 1"), Ok(binop!(Sub, Float(2.5e2), Int(1))));
        assert_eq!(
            parse("2.5e-2 / 1e3"),
            Ok(binop!(Div, Float(2.5e-2), Float(1e3)))
        );
        assert_eq!(
            parse("-1e3 + 2.5"),
            Ok(binop!(Add, Float(-1e3), Float(2.5)))
        );
        assert_eq!(
            parse("2.5e2 % 1e3"),
            Ok(binop!(Rem, Float(2.5e2), Float(1e3)))
        );

        // Combining scientific notation and function calls
        assert_eq!(
            parse("sin(1e3) + 2.5"),
            Ok(binop!(
                Add,
                Expr::UnaryFunctionCall {
                    function: UnaryFunction::Sin,
                    arg: Box::new(Float(1e3)),
                },
                Float(2.5)
            ))
        );
        assert_eq!(
            parse("log(1e-3, 2.5) * 10"),
            Ok(binop!(
                Mul,
                Expr::BinaryFunctionCall {
                    function: BinaryFunction::Log,
                    arg1: Box::new(Float(1e-3)),
                    arg2: Box::new(Float(2.5)),
                },
                Int(10)
            ))
        );
        assert_eq!(
            parse("2 * sin(2.5e2) - log(1, 1e3)"),
            Ok(binop!(
                Sub,
                binop!(
                    Mul,
                    Int(2),
                    Expr::UnaryFunctionCall {
                        function: UnaryFunction::Sin,
                        arg: Box::new(Float(2.5e2)),
                    }
                ),
                Expr::BinaryFunctionCall {
                    function: BinaryFunction::Log,
                    arg1: Box::new(Int(1)),
                    arg2: Box::new(Float(1e3)),
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

        // Missing operator between function calls
        assert!(parse("sin(2) log(3, 4)").is_err());

        // Invalid character in function argument
        assert!(parse("sin(2 + 3a) + 4").is_err());
    }
}
