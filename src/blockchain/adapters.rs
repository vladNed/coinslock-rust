use serde_json::{json, Value};


trait BitcoinAdapter {

    /// Get the current block height
    async fn get_height(&self) -> u64;

    /// Get the transaction details for a given transaction ID
    async fn get_tx(&self, tx_id: &str) -> Option<String>;

    fn call_request(&self, method: &str, params: Vec<Value>) -> serde_json::Value {
        json!({
            "jsonrpc": "1.0",
            "id": "adapter-btc",
            "method": method,
            "params": params
        })
    }
}


pub struct BitcoinRegtestAdapter;