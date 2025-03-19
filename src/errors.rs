use rustyline::error::ReadlineError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("division by zero")]
    ZeroDivionError,

    #[error("domain error")]
    DomainError,

    #[error("overflow")]
    OverFlowError,

    #[error("invalid number of arguments (expected: {expected}, found: {found})")]
    InvalidNumberOfArgumentsError { expected: usize, found: usize },

    #[error("function not found: {0}")]
    FunctionNotFoundError(String),
}

#[derive(Debug, Error)]
pub enum SevaError {
    #[error(transparent)]
    ReadlineError(#[from] ReadlineError),
}
