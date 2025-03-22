use rustyline::error::ReadlineError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("division by zero")]
    DivisionByZero,

    #[error("domain error: {0}")]
    MathDomain(String),

    #[error("overflow")]
    Overflow,

    #[error("convertion from {0} to {1} is not supported")]
    InvalidConversion(String, String),

    #[error("type error (expected: {0}, found: {1})")]
    TypeError(String, String),

    #[error("invalid number of arguments (expected: {0}, found: {1})")]
    InvalidNumberOfArguments(usize, usize),

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
