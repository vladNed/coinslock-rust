use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

use super::types::RecipientKey;

/// Returns the RIPEMD160 hash of the input.
pub(super) fn ripemd160(input: &[u8]) -> RecipientKey {

    let mut hasher = Ripemd160::new();
    hasher.update(input);

    hasher.finalize().into()
}

/// Returns the public key hash as stated in the bitcoin wiki:
/// https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses
///
/// This is fairly named hash160 because it is a result of the hash functions
pub(super) fn hash_160(input: &[u8]) -> RecipientKey {
    let mut sha_hasher = Sha256::new();
    sha_hasher.update(input);

    let hash_result = sha_hasher.finalize();
    ripemd160(&hash_result)
}

pub(super) fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);

    let result = hasher.finalize();
    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ripemd160() {
        let input = [0x00, 0x01, 0x02, 0x03];
        let output = ripemd160(&input);
        assert_eq!(output.len(), 20);
    }

    #[test]
    fn test_ripemd160_empty() {
        let input = [];
        let output = ripemd160(&input);
        assert_eq!(output.len(), 20);
    }

    #[test]
    fn test_hash_160() {
        let input = [0x00, 0x01, 0x02, 0x03];
        let output = hash_160(&input);
        assert_eq!(output.len(), 20);
    }
}
