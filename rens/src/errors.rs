use alloy::contract::Error as ContractCallError;
// use alloy::providers::EthCall;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NameHashingError {
    #[error("Please provide a name to resolve, name can't be empty")]
    EmptyName,

    #[error("Error encountered during normalization: {0}")]
    NormalizationError(String),
}
#[derive(Error, Debug)]
pub enum ReverseNameResolutionError {
    #[error("Error processing provided address")]
    AddressError,
}

#[derive(Error, Debug)]
pub enum ForwardNameResolutionError {
    #[error("No address records exists for: {0}")]
    NoAddressRecord(String),
}

#[derive(Error, Debug)]
pub enum NameResolutionError {
    #[error(transparent)]
    Forward(#[from] ForwardNameResolutionError),

    #[error(transparent)]
    Reverse(#[from] ReverseNameResolutionError),

    #[error("There was an error during the name resolution proces: {0}")]
    RensNameResolution(#[source] ContractCallError),

    #[error("Zero address found")]
    ZeroAddressResolved,

    #[error("No resolver found")]
    NoResolverFound,

    #[error("Primary name not set")]
    PrimaryNameNotSet,
}

#[derive(Error, Debug)]
pub enum RensError {
    // #[error("Contract call failed: {contract_error}")]
    // #[error(transparent)]
    // ContractError(#[from] provider::CallError),
    #[error(transparent)]
    NameHashing(#[from] NameHashingError),

    #[error(transparent)]
    NameResolution(#[from] NameResolutionError),
}

pub type RensResult<T> = Result<T, RensError>;
