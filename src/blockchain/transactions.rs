use bitcoin::{
    opcodes::{all::*, OP_0},
    script::Builder,
    Address, Script,
};

use crate::settings::get_settings;

use super::{
    crypto,
    types::{HashValue, RecipientKey},
};

/// Create a hash lock contract that locks the funds until the secret is revealed.
fn hash_lock_contract(secret_hash: HashValue, recipient: RecipientKey) -> Box<Script> {
    // Transform bytes to Push bytes to be added to script
    let script = Builder::new()
        .push_opcode(OP_HASH256)
        .push_slice(&secret_hash)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_DUP)
        .push_opcode(OP_HASH160)
        .push_slice(&recipient)
        .push_opcode(OP_EQUALVERIFY)
        .push_opcode(OP_CHECKSIG)
        .into_script();

    script.into_boxed_script()
}

fn pub_key_contract(script_hash: HashValue) -> Box<Script> {
    let script = Builder::new()
        .push_opcode(OP_0)
        .push_slice(&script_hash)
        .into_script();

    script.into_boxed_script()
}

/// Generate a pay-to-witness-script-hash address.
pub fn generate_p2wsh_address(secret: &[u8], recipient: RecipientKey) -> Address {
    let settings = get_settings();
    let secret_hash = crypto::sha256(secret);
    let script = hash_lock_contract(secret_hash, recipient);

    let script_hash = crypto::sha256(&script.as_bytes());
    let script_pub_key = pub_key_contract(script_hash);

    Address::from_script(&script_pub_key, settings.network).unwrap()
}
