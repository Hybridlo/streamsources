pub mod token_cache;
pub mod publisher;

use deadpool_redis::Connection;
use thiserror::Error;

use crate::RedisPool;

#[derive(Clone)]
pub struct RedisClient {
    pool: RedisPool
}

impl RedisClient {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    pub async fn get_conn(&self) -> ResultRedis<Connection> {
        self.pool.get().await.map_err(|_| RedisError::Other)
    }
}

impl From<RedisPool> for RedisClient {
    fn from(pool: RedisPool) -> Self {
        Self::new(pool)
    }
}

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("Other DB error")]
    Other
}

type ResultRedis<T> = Result<T, RedisError>;