use alloy::primitives::{B256, FixedBytes, b256, fixed_bytes, keccak256};
use ens_normalize::normalize;

pub fn namehashing(s: &str) -> B256 {
    // if empty string, namehash is 32 zero bytes
    if s.is_empty() {
        return B256::ZERO;
    }

    // 64-bytes zeroed array;
    let mut six_four_bytes = [0u8; 64];

    let normalized_string = normalize(s);

    if normalized_string.contains('.') {
        let lowercase_name = normalized_string;
        let split_name = lowercase_name.split_once('.');

        match split_name {
            Some((label, parent)) => {
                six_four_bytes[0..32].copy_from_slice(namehashing(parent).as_slice());

                six_four_bytes[32..64].copy_from_slice(keccak256(label.as_bytes()).as_slice());

                return keccak256(six_four_bytes);
            }

            None => {
                println!("Invalid");
                return b256!("0x0000000000000000000000000000000000000000000000000000000000000000");
            }
        }
    } else {
        six_four_bytes[0..32].copy_from_slice(namehashing("").as_slice());
        six_four_bytes[32..64].copy_from_slice(keccak256(s.as_bytes()).as_slice());
        return keccak256(six_four_bytes);
    }
}
