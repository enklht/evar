use super::Expr;

pub enum Function {
    External {
        arity: usize,
        body: fn(Vec<f64>) -> f64,
    },
    Internal {
        arity: usize,
        args: Vec<String>,
        body: Expr,
    },
}

// impl Function {
//     pub fn call(&self, args: Vec<f64>) -> Result<f64, EvalError> {
//         match self {
//             Function::External { arity, body } => {
//                 if args.len() == *arity {
//                     Ok(body(args))
//                 } else {
//                     Err(EvalError::InvalidNumberOfArguments {
//                         expected: *arity,
//                         found: args.len(),
//                     })
//                 }
//             }
//             Function::Internal {
//                 arity: _,
//                 args: _,
//                 body: _,
//             } => {
//                 todo!()
//             }
//         }
//     }
// }
