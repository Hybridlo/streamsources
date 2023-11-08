use std::env;

use actix_session::storage::RedisSessionStore;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool;
use deadpool_redis::{Config, Runtime};
use anyhow::Result;
use dotenvy::dotenv;

pub type RedisPool = deadpool_redis::Pool;
pub type DbPool = deadpool::Pool<AsyncPgConnection>;

pub fn init_dotenv() {
    dotenv().expect("Dotenv to succeed");
}

pub fn create_connection_pool() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = deadpool::Pool::builder(config).build()?;

    Ok(pool)
}

pub async fn get_redis_session() -> Result<RedisSessionStore> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let session = RedisSessionStore::new(redis_url).await?;

    Ok(session)
}

pub fn init_debug_log() {
    // set up debug
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

pub fn get_redis_client_pool() -> Result<RedisPool> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    Ok(pool)
}

pub async fn get_redis_connection() -> Result<redis::aio::Connection> {
    let redis_url = env::var("REDIS_URL")?;
    let client = redis::Client::open(redis_url)?;

    Ok(client.get_async_connection().await?)
}

/// Gets a twitch key from the environment variable.
/// 
/// # Panics
/// 
/// If the environment variable is not set
pub fn get_twitch_key() -> String {
    std::env::var("TWITCH_KEY").expect("TWITCH_KEY is not set")
}

/// Gets a twitch secret from the environment variable.
/// 
/// # Panics
/// 
/// If the environment variable is not set
pub fn get_twitch_secret() -> String {
    std::env::var("TWITCH_SECRET").expect("TWITCH_SECRET is not set")
}