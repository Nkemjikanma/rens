use alloy::contract::Error as ContractCallError;
// use alloy::providers::EthCall;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NameHashingError {
    #[error("Please provide a name to resolve, name can't be empty")]
    EmptyName,
    // EmptyName(&'static str),
}
// #[derive(Error, Debug)]
// pub enum ProviderError {
//     // Provider initializations error
// }
#[derive(Error, Debug)]
pub enum NameResolutionError {
    #[error("There was an error during the name resolution proces: {0}")]
    RensNameResolution(#[source] ContractCallError),

    #[error("Resolved address is Zero address")]
    ZeroAddressResolved,

    #[error("No address records exists for: {0}")]
    NoAddressRecord(String),
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
