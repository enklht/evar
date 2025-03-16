use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("division by zero")]
    ZeroDivionError,

    #[error("domain error")]
    DomainError,

    #[error("overflow")]
    OverFlowError,
}
