use hmac::Hmac;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::Sha512;
use pbkdf2::pbkdf2;

use super::secret;

const DEFAULT_SALT: &[u8] = b"mnemonic";

fn mnemonic_to_seed(mnemonic: Vec<String>) -> Result<[u8; 64], std::io::Error> {
    let mut seed = [0u8; 64];
    match pbkdf2::<Hmac<Sha512>>(
        mnemonic.join(" ").as_bytes(),
        DEFAULT_SALT,
        2048,
        &mut seed,
    ) {
        Ok(_) => {}
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
    }

    Ok(seed)
}

#[derive(Debug)]
pub struct GuardianWallet {
    sk: SecretKey,
    pk: PublicKey,
}

impl GuardianWallet {
    pub async fn generate_new<const T: usize>() -> Result<Self, std::io::Error> {
        let mnemonic = secret::generate_secret::<T>().await?;
        let seed = mnemonic_to_seed(mnemonic).unwrap();

        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&seed[..32]).expect("Seed did not produce a valid secret key");
        let pk = PublicKey::from_secret_key(&secp, &sk);

        Ok(Self { sk, pk })
    }

    pub fn public_key(&self) -> PublicKey {
        self.pk
    }

    pub fn public_key_commitment() -> PublicKey {
        unimplemented!()
    }

}
