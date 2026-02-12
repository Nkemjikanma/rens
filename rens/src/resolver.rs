use crate::errors::{NameResolutionError, RensError, RensResult};

// use super::errors;
use super::name_hash::namehash;
// use alloy::network::Ethereum;
use alloy::primitives::{Address, address};
use alloy::providers::Provider;
use alloy::sol;
// use std::error::Error;
#[derive(Debug)]
pub struct EnsContractAddresses {
    pub ens_registry: Address,
    pub public_resolver: Address,
    pub base_registrar: Address,
    pub registrar_controller: Address,
}

impl EnsContractAddresses {
    pub fn mainnet() -> Self {
        Self {
            ens_registry: address!("0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e"),
            public_resolver: address!("0xF29100983E058B709F3D539b0c765937B804AC15"),
            base_registrar: address!("0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85"),
            registrar_controller: address!("0x253553366Da8546fC250F225fe3d25d0C782303b"),
        }
    }
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract ENSRegistry {
         function resolver(bytes32 node) public view virtual returns (address);
    }
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract PublicResolver {
        function addr(bytes32 node) view returns (address);
    }
}

pub async fn resolve_name(name: &str, provider: &impl Provider) -> RensResult<Address> {
    let registry = ENSRegistry::new(EnsContractAddresses::mainnet().ens_registry, &provider);

    // Hash the name
    let node = namehash(&name)?;

    // Call the ENS Registry to get resolver address
    let resolver_address = registry
        .resolver(node)
        .call()
        .await
        .map_err(|e| RensError::NameResolution(NameResolutionError::RensNameResolution(e)))?;

    // Check if resolver address is Zero address
    if Address::ZERO == resolver_address {
        return Err(RensError::NameResolution(
            NameResolutionError::ZeroAddressResolved,
        ));
    }

    let resolver_contract = PublicResolver::new(resolver_address, &provider);
    let node_address = resolver_contract
        .addr(node)
        .call()
        .await
        .map_err(|e| RensError::NameResolution(NameResolutionError::RensNameResolution(e)))?;

    if node_address == Address::ZERO {
        return Err(RensError::NameResolution(
            NameResolutionError::NoAddressRecord(name.to_string()),
        ));
    }

    Ok(node_address)
}
