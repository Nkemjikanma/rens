use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum NormalizationError {
    #[error("Error normalizing lable: {0}")]
    Error(String),
}
