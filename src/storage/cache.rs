use redis::{AsyncCommands, RedisResult};

use crate::settings::get_settings;

pub struct CacheClient {
    client: redis::Client,
    ttl: u64
}

impl CacheClient {
    pub fn new() -> Result<Self, redis::RedisError> {
        let settings = get_settings();
        let client = redis::Client::open(format!("redis://{}:{}", settings.url, settings.port))?;
        Ok(Self { client, ttl: settings.ttl })
    }

    pub async fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        con.set_ex(key, value, self.ttl).await
    }

    pub async fn get(&self, key: &str) -> RedisResult<Vec<u8>> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        con.get(key).await
    }

    pub async fn health(&self) -> RedisResult<String> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let _: () = redis::cmd("PING").query_async(&mut con).await?;
        Ok("ok".to_string())
    }
}
