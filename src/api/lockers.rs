use std::sync::Arc;

use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use serde_json::json;
use tokio::{join, sync::Mutex};

use crate::{
    blockchain::{address, secret, transactions},
    storage::cache::CacheClient,
};

use super::schemas::SaveLockerRequest;

/// Generate a new locker with a new guardian wallet
#[get("/lockers/new/")]
async fn new_locker(cache: web::Data<Arc<Mutex<CacheClient>>>) -> impl Responder {
    // Generate a new locker password which is a mnemonic key
    let entropy = secret::token_bytes::<32>();

    let (mnemonic_result, guardian_wallet_result) = join!(
        secret::generate_secret(&entropy),
        address::GuardianWallet::generate_new()
    );
    let mnemonic = match mnemonic_result {
        Ok(mnemonic) => mnemonic,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Error generating locker: {}", e)
            }))
        }
    };

    // Generate a new guardian wallet
    let guardian_wallet = match guardian_wallet_result {
        Ok(wallet) => wallet,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Error generating guardian wallet: {}", e)
            }))
        }
    };

    // Generate a new locker address
    let address =
        transactions::generate_p2wsh_address(&entropy, guardian_wallet.public_key_commitment());

    let locker_id = secret::hash_id(address.to_string());
    let locker_data = json!({
        "address": address.to_string(),
        "locker_id": locker_id,
    });

    let cache_val = cache.lock().await;
    let locker_data_serialized = locker_data.to_string();
    let locker_data_key = format!("locker:{}", locker_id);
    cache_val.set(&locker_data_key, locker_data_serialized.as_str()).await.unwrap();
    info!("Locker saved in cache: {}", locker_id);

    HttpResponse::Ok().json(json!({
        "mnemonic": mnemonic.join(" "),
        "address": address.to_string(),
        "locker_id": secret::hash_id(address.to_string()),
    }))
}

#[post("/lockers/save/")]
async fn save_locker(request: web::Json<SaveLockerRequest>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Locker saved successfully"
    }))
}
