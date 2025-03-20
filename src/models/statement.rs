use super::EvalError;

use super::{Context, Expr};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    DefVar {
        name: String,
        expr: Expr,
    },
    DefFun {
        name: String,
        args: Vec<String>,
        body: Expr,
    },
    Expr(Expr),
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::DefVar { name, expr } => write!(f, "let {} := {}", name, expr),
            Stmt::DefFun { name, args, body } => write!(f, "let {}({:?}) := {}", name, args, body),
            Stmt::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

impl Stmt {
    pub fn eval(self, context: &mut Context) -> Result<f64, EvalError> {
        match self {
            Stmt::DefVar { name, expr } => {
                let val = expr.eval(context)?;
                let variable = context
                    .set_variable(&name, val)
                    .ok_or(EvalError::InvalidVariableDefinition(name))?;
                Ok(variable)
            }
            Stmt::DefFun { name, args, body } => {
                context.set_function(&name, args, body);
                Ok(f64::NAN)
            }
            Stmt::Expr(expr) => expr.eval(context),
        }
    }
}
