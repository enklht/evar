use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("")]
    ZeroDivionError,
    #[error("")]
    DomainError,
    #[error("")]
    OutOfBound,
}
