pub mod errors;
pub mod name_hash;
pub mod resolver;
pub mod types;

use alloy::primitives::B256;
use name_hash::namehash;

// use resolver;
pub fn add(value: &str) -> B256 {
    let hash = namehash(value).expect("Expecting non-empty string");

    println!("Vitalike.eth: {:?}", hash);

    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::b256;
    #[test]
    fn test_ens_hashing() {
        let result = add("vitalik.eth");
        println!("{}", result);
        assert_eq!(
            result,
            b256!("0xee6c4522aab0003e8d14cd40a6af439055fd2577951148c14b6cea9a53475835")
        );
    }

    #[test]
    fn test_subdomain_hasing() {
        let result = add("sub.vitalik.eth");
        println!("{}", result);
        assert_eq!(
            result,
            b256!("0x02db957db5283c30c2859ec435b7e24e687166eddf333b9615ed3b91bd063359")
        );
    }

    #[test]
    fn test_eth_string_hashing() {
        let result = add("eth");
        assert_eq!(
            result,
            b256!("0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae")
        )
    }
}
