use crate::errors::NameHashingError;

use super::errors::{RensError, RensResult};
use alloy::primitives::{B256, keccak256};
use ens_normalize::normalize;
// use error_stack::{Report, ResultExt};

pub fn namehash(s: &str) -> RensResult<B256> {
    let normalized_string = normalize(s);

    match normalized_string {
        Ok(string_value) => {
            let node = compute_namehash(string_value.as_str());

            Ok(node)
        }
        Err(error) => Err(RensError::NameHashing(
            NameHashingError::NormalizationError(error.to_string()),
        )),
    }
}

fn compute_namehash(s: &str) -> B256 {
    // if empty string, namehash is 32 zero bytes
    if s.is_empty() {
        return B256::ZERO;
    }
    // 64-bytes zeroed array;
    let mut six_four_bytes = [0u8; 64];

    if s.contains('.') {
        let split_name = s.split_once('.');

        match split_name {
            Some((label, parent)) => {
                six_four_bytes[0..32].copy_from_slice(compute_namehash(parent).as_slice());

                six_four_bytes[32..64].copy_from_slice(keccak256(label.as_bytes()).as_slice());

                keccak256(six_four_bytes)
            }

            None => {
                println!("Invalid");
                B256::ZERO
            }
        }
    } else {
        six_four_bytes[0..32].copy_from_slice(compute_namehash("").as_slice());
        six_four_bytes[32..64].copy_from_slice(keccak256(s.as_bytes()).as_slice());
        keccak256(six_four_bytes)
    }
}
