use bitcoin::Network;
use std::{env, sync::{Mutex, MutexGuard}};
use once_cell::sync::Lazy;

pub struct Settings {

    // Default settings
    pub environment: String,

    // Network settings
    pub network: Network,
    pub rpc_hostname: String,
    pub rpc_username: String,
    pub rpc_password: String,

    // Cache settings
    pub url: String,
    pub port: String,
    pub ttl: u64,
}

impl Settings {
    pub fn new() -> Self {
        let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());
        let network = env::var("NETWORK").unwrap_or_else(|_| "regtest".into());
        let rpc_hostname = env::var("RPC_HOSTNAME").unwrap_or_else(|_| "localhost".into());
        let rpc_username = env::var("RPC_USERNAME").unwrap_or_else(|_| "user".into());
        let rpc_password = env::var("RPC_PASSWORD").unwrap_or_else(|_| "password".into());
        let url = env::var("REDIS_URL").unwrap_or_else(|_| "localhost".into());
        let port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".into());
        let ttl = env::var("REDIS_TTL").unwrap_or_else(|_| "60".into()).parse().unwrap();

        return Settings {
            environment: env,
            network: match network.as_str() {
                "mainnet" => Network::Bitcoin,
                "testnet" => Network::Testnet,
                "regtest" => Network::Regtest,
                _ => Network::Regtest,
            },
            rpc_hostname,
            rpc_username,
            rpc_password,
            url,
            port,
            ttl,
        }
    }
}

static SETTINGS: Lazy<Mutex<Settings>> = Lazy::new(|| Mutex::new(Settings::new()));

pub fn get_settings() -> MutexGuard<'static, Settings> {
    SETTINGS.lock().unwrap()
}