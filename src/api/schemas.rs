use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub(super) struct SaveLockerRequest {
    tx_id: String,
}