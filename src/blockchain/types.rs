
/// Represents the key of a recipient in the blockchain which is the public key
/// commitment that comes out of the RIPEMD160 hash of the public key bytes.
pub(super) type RecipientKey = [u8; 20];

/// The output for a hash
pub(super) type HashValue = [u8; 32];
