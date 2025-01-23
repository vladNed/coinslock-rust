use bitcoin::hex::DisplayHex;
use once_cell::sync::Lazy;
use rand::RngCore;
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncReadExt, sync::Mutex};

type SecretResult<T> = Result<T, std::io::Error>;

const ALLOWED_SIZE: [usize; 2] = [16, 32];

static WORD_LIST: Lazy<Mutex<Option<Vec<String>>>> = Lazy::new(|| Mutex::new(None));

#[inline]
pub fn token_bytes<const T: usize>() -> [u8; T] {
    let mut rng = rand::thread_rng();
    let mut entropy = [0u8; T];
    rng.fill_bytes(&mut entropy);

    entropy
}

#[inline(always)]
fn bytes_to_bits(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut acc, byte| {
        acc.push_str(&format!("{:08b}", byte));
        acc
    })
}

#[inline]
fn get_entropy_size(mnemonic_len: usize) -> usize {
    match mnemonic_len {
        12 => 128,
        24 => 256,
        _ => panic!("Invalid mnemonic length"),
    }
}

fn bits_to_bytes(bits: &str) -> Vec<u8> {
    bits.chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| {
            let byte = chunk.iter().collect::<String>();
            u8::from_str_radix(byte.as_str(), 2).unwrap()
        })
        .collect()
}

async fn read_word_list(word_list_path: &str) -> SecretResult<Vec<String>> {
    let mut word_list = WORD_LIST.lock().await;
    if word_list.is_none() {
        let mut file = File::open(word_list_path).await?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).await?;
        *word_list = Some(buffer.lines().map(|s| s.to_string()).collect());
    }

    Ok(word_list.clone().unwrap())
}

/// Generates a mnemonic key based on the BIP39 algorithm.
/// The default word list is the one provided by the BIP39 standard and the
/// default entropy is 16 bytes (128 bits).
///
/// ### Returns
/// A vector of strings containing the mnemonic words.
///
/// ### Errors
/// If the word list file cannot be read or if there is an error generating
pub async fn generate_secret(entropy: &[u8]) -> SecretResult<Vec<String>> {
    let entropy_size = entropy.len();
    if !ALLOWED_SIZE.contains(&entropy_size) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid entropy size",
        ));
    }
    let words = read_word_list(super::WORD_LIST_PATH).await?;
    let entropy_bits = bytes_to_bits(entropy);
    let checksum = Sha256::digest(entropy);
    let checksum_size = (entropy_size * 8) / 32;
    let checksum_bits = bytes_to_bits(&checksum)[..checksum_size].to_string();
    let total_bits = entropy_bits + &checksum_bits;

    let mnemonic_words = total_bits
        .chars()
        .collect::<Vec<char>>()
        .chunks(11)
        .map(|chunk| {
            let bit_group = chunk.iter().collect::<String>();
            let word_index = usize::from_str_radix(bit_group.as_str(), 2).unwrap();
            words[word_index].clone()
        })
        .collect::<Vec<String>>();

    Ok(mnemonic_words)
}

/// Converts a mnemonic key to its corresponding entropy.
/// The default word list is the one provided by the BIP39 standard and the
/// default entropy is 16 bytes (128 bits).
///
/// ### Arguments
/// * `mnemonic` - A vector of strings containing the mnemonic words.
///
/// ### Returns
/// A 16-byte array containing the entropy.
pub async fn mnemonic_to_entropy(mnemonic: Vec<String>) -> SecretResult<Vec<u8>> {
    let words = read_word_list(super::WORD_LIST_PATH).await?;

    let mnemonic_indices = mnemonic
        .iter()
        .map(|word| words.iter().position(|w| w == word).unwrap())
        .collect::<Vec<usize>>();

    let total_bits = mnemonic_indices.iter().fold(String::new(), |acc, index| {
        let bit_group = format!("{:011b}", index);
        acc + &bit_group
    });
    let entropy_size = get_entropy_size(mnemonic.len());
    let (entropy_bits, checksum_bits_split) = &total_bits.split_at(entropy_size);
    let entropy_bytes = bits_to_bytes(entropy_bits);
    let checksum = Sha256::digest(&entropy_bytes);
    let checksum_size = entropy_size / 32;
    let checksum_bits = &bytes_to_bits(&checksum)[..checksum_size];
    if checksum_bits != *checksum_bits_split {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid checksum",
        ));
    }

    Ok(entropy_bytes)
}

pub fn hash_id(data: String) -> String {
    let d = super::crypto::sha256(data.as_bytes());
    let mut id = [0u8; 8];
    id.copy_from_slice(&d[0..8]);

    id.as_hex().to_string()
}

#[cfg(test)]
mod tests {
    use secret::{bytes_to_bits, token_bytes};

    use crate::blockchain::*;

    #[test]
    fn test_token_bytes() {
        let bytes = secret::token_bytes::<32>();
        assert_eq!(bytes.len(), 32);

        let bytes_16 = secret::token_bytes::<16>();
        assert_eq!(bytes_16.len(), 16);
    }

    #[test]
    fn test_bytes_to_bits() {
        let bytes = [0x0000, 0x0001, 0x0002, 0x0003];
        let test_bits = bytes_to_bits(&bytes);
        assert_eq!(test_bits.len(), bytes.len() * 8);
    }

    #[tokio::test]
    async fn read_word_list() {
        let words = match secret::read_word_list(WORD_LIST_PATH).await {
            Ok(words) => words,
            Err(e) => panic!("Error reading word list: {}", e),
        };
        assert_eq!(words.len(), 2048);
    }

    // TODO: Revive test when we have a way to mock the file system
    // #[tokio::test]
    // async fn test_read_inexistent_word_list() {
    //     let mut words = secret::WORD_LIST.lock().await;
    //     *words = None;
    //     drop(words);
    //     let words = secret::read_word_list("data/does_not_exist.txt").await;
    //     assert!(words.is_err());
    // }

    #[tokio::test]
    async fn test_generate_secret() {
        let entropy = token_bytes::<16>();
        let mnemonic = secret::generate_secret(&entropy).await;
        assert!(mnemonic.is_ok());
        assert_eq!(mnemonic.unwrap().len(), 12);

        let entropy = token_bytes::<32>();
        let mnemonic = secret::generate_secret(&entropy).await;
        assert!(mnemonic.is_ok());
        assert_eq!(mnemonic.unwrap().len(), 24);
    }

    #[tokio::test]
    async fn test_invalid_entropy_value() {
        let entropy = token_bytes::<20>();
        let mnemonic = secret::generate_secret(&entropy).await;
        assert!(mnemonic.is_err());
    }

    #[tokio::test]
    async fn test_mnemonic_to_entropy() {
        let entropy = token_bytes::<16>();
        let mnemonic = secret::generate_secret(&entropy).await;
        assert!(mnemonic.is_ok());

        let entropy = secret::mnemonic_to_entropy(mnemonic.unwrap()).await;
        assert!(entropy.is_ok());
        assert_eq!(entropy.unwrap().len(), 16);

        let entropy = token_bytes::<32>();
        let mnemonic = secret::generate_secret(&entropy).await;
        assert!(mnemonic.is_ok());

        let entropy = secret::mnemonic_to_entropy(mnemonic.unwrap()).await;
        assert!(entropy.is_ok());
        assert_eq!(entropy.unwrap().len(), 32);
    }
}
