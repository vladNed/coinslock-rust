pub mod address;
pub mod crypto;
pub mod secret;
pub mod transactions;
pub mod adapters;
mod types;

const WORD_LIST_PATH: &str = "data/wordlist.txt";
const DEFAULT_SALT: &[u8] = b"mnemonic";
