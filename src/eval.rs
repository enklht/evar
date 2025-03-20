use crate::{context::Context, errors::EvalError};

// #[cfg(test)]
// mod tests {
//     use crate::{
//         args::{AngleUnit, Args},
//         context::Context,
//     };

//     use super::*;

//     const RADIAN_ARGS: Args = Args {
//         // fix: 10,
//         // base: 10,
//         angle_unit: AngleUnit::Radian,
//         debug: false,
//         no_color: false,
//     };

//     const DEGREE_ARGS: Args = Args {
//         // fix: 10,
//         // base: 10,
//         angle_unit: AngleUnit::Degree,
//         debug: false,
//         no_color: false,
//     };

//     #[test]
//     fn test_eval_number() {
//         let radian_context = &mut Context::new(&RADIAN_ARGS);

//         let expr = Expr::Number(5.0);
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 5.0);
//     }

//     #[test]
//     fn test_eval_basic_operations() {
//         let radian_context = &mut Context::new(&RADIAN_ARGS);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Add,
//             lhs: Box::new(Expr::Number(2.0)),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 2.0 + 3.0);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Sub,
//             lhs: Box::new(Expr::Number(5.0)),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 5.0 - 3.0);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Mul,
//             lhs: Box::new(Expr::Number(2.0)),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 2.0 * 3.0);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Div,
//             lhs: Box::new(Expr::Number(6.0)),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 6.0 / 3.0);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Rem,
//             lhs: Box::new(Expr::Number(7.0)),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 7.0 % 3.0);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Pow,
//             lhs: Box::new(Expr::Number(2.0)),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 2.0_f64.powf(3.0));

//         let expr = Expr::PrefixOp {
//             op: PrefixOp::Neg,
//             arg: Box::new(Expr::Number(5.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), -5.0);

//         let expr = Expr::PostfixOp {
//             op: PostfixOp::Fac,
//             arg: Box::new(Expr::Number(5.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 120.0); // 5! = 120
//     }

//     #[test]
//     fn test_eval_functions() {
//         let radian_context = &mut Context::new(&RADIAN_ARGS);
//         let degree_context = &mut Context::new(&DEGREE_ARGS);

//         let expr = Expr::FnCall {
//             name: "sin".into(),
//             args: vec![Expr::Number(std::f64::consts::PI / 2.0)],
//         };
//         assert_eq!(
//             eval_expr(expr, radian_context).unwrap(),
//             (std::f64::consts::PI / 2.0).sin()
//         );

//         let expr = Expr::FnCall {
//             name: "sin".into(),
//             args: vec![Expr::Number(90.0)],
//         };
//         assert_eq!(
//             eval_expr(expr, degree_context).unwrap(),
//             90.0_f64.to_radians().sin()
//         );

//         let expr = Expr::FnCall {
//             name: "log".into(),
//             args: vec![Expr::Number(8.0), Expr::Number(2.0)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 8.0_f64.log(2.0));

//         let expr = Expr::FnCall {
//             name: "cos".into(),
//             args: vec![Expr::Number(0.0)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 0.0_f64.cos());

//         let expr = Expr::FnCall {
//             name: "cos".into(),
//             args: vec![Expr::Number(0.0)],
//         };
//         assert_eq!(
//             eval_expr(expr, degree_context).unwrap(),
//             0.0_f64.to_radians().cos()
//         );

//         let expr = Expr::FnCall {
//             name: "tan".into(),
//             args: vec![Expr::Number(std::f64::consts::PI / 4.0)],
//         };
//         assert_eq!(
//             eval_expr(expr, radian_context).unwrap(),
//             (std::f64::consts::PI / 4.0).tan()
//         );

//         let expr = Expr::FnCall {
//             name: "tan".into(),
//             args: vec![Expr::Number(45.0)],
//         };
//         assert_eq!(
//             eval_expr(expr, degree_context).unwrap(),
//             45.0_f64.to_radians().tan()
//         );

//         let expr = Expr::FnCall {
//             name: "sqrt".into(),
//             args: vec![Expr::Number(16.0)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 16.0_f64.sqrt());

//         let expr = Expr::FnCall {
//             name: "exp".into(),
//             args: vec![Expr::Number(1.0)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 1.0_f64.exp());

//         let expr = Expr::FnCall {
//             name: "ln".into(),
//             args: vec![Expr::Number(std::f64::consts::E)],
//         };
//         assert_eq!(
//             eval_expr(expr, radian_context).unwrap(),
//             std::f64::consts::E.ln()
//         );

//         let expr = Expr::FnCall {
//             name: "log10".into(),
//             args: vec![Expr::Number(1000.0)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 1000.0_f64.log10());

//         let expr = Expr::FnCall {
//             name: "exp2".into(),
//             args: vec![Expr::Number(3.0)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.0_f64.exp2());

//         let expr = Expr::FnCall {
//             name: "floor".into(),
//             args: vec![Expr::Number(3.7)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.7_f64.floor());

//         let expr = Expr::FnCall {
//             name: "ceil".into(),
//             args: vec![Expr::Number(3.3)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.3_f64.ceil());

//         let expr = Expr::FnCall {
//             name: "round".into(),
//             args: vec![Expr::Number(3.5)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 3.5_f64.round());

//         let expr = Expr::FnCall {
//             name: "abs".into(),
//             args: vec![Expr::Number(-3.5)],
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), (-3.5_f64).abs());
//     }

//     #[test]
//     fn test_eval_mixed_operations() {
//         let radian_context = &mut Context::new(&RADIAN_ARGS);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Add,
//             lhs: Box::new(Expr::InfixOp {
//                 op: InfixOp::Mul,
//                 lhs: Box::new(Expr::Number(2.0)),
//                 rhs: Box::new(Expr::Number(3.0)),
//             }),
//             rhs: Box::new(Expr::Number(4.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), (2.0 * 3.0) + 4.0);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Sub,
//             lhs: Box::new(Expr::Number(10.0)),
//             rhs: Box::new(Expr::InfixOp {
//                 op: InfixOp::Div,
//                 lhs: Box::new(Expr::Number(6.0)),
//                 rhs: Box::new(Expr::Number(3.0)),
//             }),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 10.0 - (6.0 / 3.0));

//         let expr = Expr::InfixOp {
//             op: InfixOp::Rem,
//             lhs: Box::new(Expr::InfixOp {
//                 op: InfixOp::Pow,
//                 lhs: Box::new(Expr::Number(2.0)),
//                 rhs: Box::new(Expr::Number(3.0)),
//             }),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(
//             eval_expr(expr, radian_context).unwrap(),
//             2.0_f64.powf(3.0) % 3.0
//         );

//         let expr = Expr::InfixOp {
//             op: InfixOp::Add,
//             lhs: Box::new(Expr::PrefixOp {
//                 op: PrefixOp::Neg,
//                 arg: Box::new(Expr::Number(5.0)),
//             }),
//             rhs: Box::new(Expr::Number(3.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), -5.0 + 3.0);

//         let expr = Expr::InfixOp {
//             op: InfixOp::Sub,
//             lhs: Box::new(Expr::PostfixOp {
//                 op: PostfixOp::Fac,
//                 arg: Box::new(Expr::Number(5.0)),
//             }),
//             rhs: Box::new(Expr::Number(119.0)),
//         };
//         assert_eq!(eval_expr(expr, radian_context).unwrap(), 120.0 - 119.0); // 5! - 119

//         let expr = Expr::InfixOp {
//             op: InfixOp::Mul,
//             lhs: Box::new(Expr::FnCall {
//                 name: "sin".into(),
//                 args: vec![Expr::Number(std::f64::consts::PI / 2.0)],
//             }),
//             rhs: Box::new(Expr::Number(2.0)),
//         };
//         assert_eq!(
//             eval_expr(expr, radian_context).unwrap(),
//             (std::f64::consts::PI / 2.0).sin() * 2.0
//         );

//         let expr = Expr::InfixOp {
//             op: InfixOp::Add,
//             lhs: Box::new(Expr::FnCall {
//                 name: "log".into(),
//                 args: vec![Expr::Number(8.0), Expr::Number(2.0)],
//             }),
//             rhs: Box::new(Expr::Number(1.0)),
//         };
//         assert_eq!(
//             eval_expr(expr, radian_context).unwrap(),
//             8.0_f64.log(2.0) + 1.0
//         );
//     }
// }
