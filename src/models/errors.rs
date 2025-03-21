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

    #[error("type error (expected: {expected}, found: {found})")]
    TypeError { expected: String, found: String },

    #[error("invalid number of arguments (expected: {expected}, found: {found})")]
    InvalidNumberOfArguments { expected: usize, found: usize },

    #[error("function not found: {0}")]
    FunctionNotFound(String),

    #[error("variable not found: {0}")]
    VariableNotFound(String),

    #[error("failed to define a variable: {0}")]
    InvalidVariableDefinition(String),

    #[error("no previous answer")]
    NoHistory,
}

#[derive(Debug, Error)]
pub enum SevaError {
    #[error(transparent)]
    ReadlineError(#[from] ReadlineError),
}
