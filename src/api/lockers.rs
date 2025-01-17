use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

use crate::blockchain::{address, secret, transactions};

#[get("/lockers/new/")]
async fn new_locker() -> impl Responder {
    let entropy = secret::token_bytes::<32>();
    let mnemonic = secret::generate_secret(&entropy).await.unwrap();
    let guardian_wallet = address::GuardianWallet::generate_new().await.unwrap();

    let address = transactions::generate_p2wsh_address(&entropy, guardian_wallet.public_key_commitment());

    HttpResponse::Ok().json(json!({
        "mnemonic": mnemonic.join(" "),
        "address": address.to_string()
    }))
}