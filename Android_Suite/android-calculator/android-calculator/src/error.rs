// src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalcError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid expression: {0}")]
    InvalidExpression(String),

    #[error("Math domain error: {0}")]
    MathDomain(String),

    #[error("Overflow")]
    Overflow,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type CalcResult<T> = Result<T, CalcError>;
