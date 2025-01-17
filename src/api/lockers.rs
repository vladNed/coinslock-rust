use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use tokio::join;

use crate::blockchain::{address, secret, transactions};

/// Generate a new locker with a new guardian wallet
#[get("/lockers/new/")]
async fn new_locker() -> impl Responder {

    // Generate a new locker password which is a mnemonic key
    let entropy = secret::token_bytes::<32>();

    let (mnemonic_result, guardian_wallet_result) = join!(
        secret::generate_secret(&entropy),
        address::GuardianWallet::generate_new()
    );
    let mnemonic = match mnemonic_result {
        Ok(mnemonic) => mnemonic,
        Err(e) => return HttpResponse::InternalServerError().json(json!({
            "error": format!("Error generating locker: {}", e)
        })),
    };

    // Generate a new guardian wallet
    let guardian_wallet = match guardian_wallet_result {
        Ok(wallet) => wallet,
        Err(e) => return HttpResponse::InternalServerError().json(json!({
            "error": format!("Error generating guardian wallet: {}", e)
        })),
    };

    // Generate a new locker address
    let address = transactions::generate_p2wsh_address(&entropy, guardian_wallet.public_key_commitment());

    HttpResponse::Ok().json(json!({
        "mnemonic": mnemonic.join(" "),
        "address": address.to_string()
    }))
}