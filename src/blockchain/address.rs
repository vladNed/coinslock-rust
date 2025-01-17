use std::io::{Error, ErrorKind};

use hmac::Hmac;
use pbkdf2::pbkdf2;
use secp256k1::{ecdsa::SerializedSignature, Message, PublicKey, Secp256k1, SecretKey};
use sha2::Sha512;

use super::{secret, types::RecipientKey};

fn mnemonic_to_seed(mnemonic: Vec<String>) -> Result<[u8; 64], Error> {
    let mut seed = [0u8; 64];
    match pbkdf2::<Hmac<Sha512>>(
        mnemonic.join(" ").as_bytes(),
        super::DEFAULT_SALT,
        2048,
        &mut seed,
    ) {
        Ok(_) => {}
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    }

    Ok(seed)
}

#[derive(Debug)]
pub struct GuardianWallet {
    sk: SecretKey,
    pk: PublicKey,
}

impl GuardianWallet {
    fn public_key_bytes(&self) -> [u8; 33] {
        self.public_key().serialize()
    }

    pub async fn generate_new() -> Result<Self, Error> {
        let entropy = secret::token_bytes::<32>();
        let mnemonic = secret::generate_secret(&entropy).await?;
        let seed = mnemonic_to_seed(mnemonic).unwrap();

        let secp = Secp256k1::new();
        let sk =
            SecretKey::from_slice(&seed[..32]).expect("Seed did not produce a valid secret key");
        let pk = PublicKey::from_secret_key(&secp, &sk);

        Ok(Self { sk, pk })
    }

    pub fn public_key(&self) -> PublicKey {
        self.pk
    }

    pub fn public_key_commitment(&self) -> RecipientKey {
        super::crypto::hash_160(&self.public_key_bytes())
    }

    pub fn sign(&self, hashed_data: [u8; 32]) -> Result<SerializedSignature, Error> {
        let secp = Secp256k1::new();
        let message = Message::from_digest(hashed_data);
        let signature = secp.sign_ecdsa(&message, &self.sk);

        Ok(signature.serialize_der())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_new() {
        let wallet = GuardianWallet::generate_new().await.unwrap();
        assert_eq!(wallet.sk.secret_bytes().len(), 32);
        assert_eq!(wallet.public_key().serialize().len(), 33);
    }

    #[tokio::test]
    async fn test_public_key_commitment() {
        let wallet = GuardianWallet::generate_new().await.unwrap();
        let commitment = wallet.public_key_commitment();
        assert_eq!(commitment.len(), 20);
    }

    #[tokio::test]
    async fn test_sign() {
        let wallet = GuardianWallet::generate_new().await.unwrap();
        let hashed_data = [0u8; 32];
        let signature = wallet.sign(hashed_data);
        assert!(signature.is_ok());
        assert_eq!(signature.unwrap().len(), 71);
    }
}
