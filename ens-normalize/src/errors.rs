use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum NormalizationError {
    #[error("Error normalizing label: {0}")]
    Error(String),
}
