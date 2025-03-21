use std::cell::RefCell;
use std::rc::Rc;

use super::{EvalError, Expr, FunctionContext, Value, VariableContext};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    DefVar {
        name: String,
        expr: Expr,
    },
    DefFun {
        name: String,
        arg_names: Vec<String>,
        body: Expr,
    },
    Expr(Expr),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::DefVar { name, expr } => write!(f, "let {} := {}", name, expr),
            Stmt::DefFun {
                name,
                arg_names,
                body,
            } => write!(f, "let {}({:?}) := {}", name, arg_names, body),
            Stmt::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

impl Stmt {
    pub fn eval(
        self,
        fcontext: &mut FunctionContext,
        vcontext: Rc<RefCell<VariableContext>>,
    ) -> Result<Value, EvalError> {
        match self {
            Stmt::DefVar { name, expr } => {
                let val = expr.eval(fcontext, vcontext.clone())?;
                let variable = vcontext
                    .borrow_mut()
                    .set_variable(&name, val)
                    .ok_or(EvalError::InvalidVariableDefinition(name))?;
                Ok(variable)
            }
            Stmt::DefFun {
                name,
                arg_names: args,
                body,
            } => {
                fcontext.set_function(&name, args, body);
                Ok(Value::null())
            }
            Stmt::Expr(expr) => {
                let answer = expr.eval(fcontext, vcontext)?;
                fcontext.set_prev_answer(&answer);
                Ok(answer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Expr;
    use super::*;
    use crate::{
        args::{AngleUnit, Args},
        create_context,
    };

    const RADIAN_ARGS: Args = Args {
        angle_unit: AngleUnit::Radian,
        debug: false,
        no_color: false,
    };

    #[test]
    fn test_def_var_eval() {
        let (mut fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let stmt = Stmt::DefVar {
            name: "x".to_string(),
            expr: Expr::Number(42.0),
        };
        assert_eq!(
            stmt.eval(&mut fcontext, vcontext.clone()).unwrap(),
            Value::from(42.0)
        );
        assert_eq!(
            vcontext.borrow().get_variable("x").unwrap().get(),
            Value::from(42.0)
        );
    }

    #[test]
    fn test_def_fun_eval() {
        let (mut fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let stmt = Stmt::DefFun {
            name: "add".to_string(),
            arg_names: vec!["a".to_string(), "b".to_string()],
            body: Expr::InfixOp {
                op: crate::models::operators::InfixOp::Add,
                lhs: Box::new(Expr::Variable("a".to_string())),
                rhs: Box::new(Expr::Variable("b".to_string())),
            },
        };
        assert!(stmt.eval(&mut fcontext, vcontext).is_ok());
        assert!(fcontext.get_function("add").is_some());
    }

    #[test]
    fn test_expr_eval() {
        let (mut fcontext, vcontext) = create_context(&RADIAN_ARGS);
        let stmt = Stmt::Expr(Expr::Number(42.0));
        assert_eq!(
            stmt.eval(&mut fcontext, vcontext).unwrap(),
            Value::from(42.0)
        );
    }
}
