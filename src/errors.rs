use rustyline::error::ReadlineError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("division by zero")]
    DivisionByZero,

    #[error("domain error")]
    MathDomain,

    #[error("overflow")]
    Overflow,

    #[error("invalid number of arguments (expected: {expected}, found: {found})")]
    InvalidNumberOfArguments { expected: usize, found: usize },

    #[error("function not found: {0}")]
    FunctionNotFound(String),

    #[error("variable not found: {0}")]
    VariableNotFound(String),

    #[error("failed to define a variable: {0}")]
    InvalidVariableDefinition(String),
}

#[derive(Debug, Error)]
pub enum SevaError {
    #[error(transparent)]
    ReadlineError(#[from] ReadlineError),
}
