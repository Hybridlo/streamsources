use std::env;

use actix_session::storage::RedisSessionStore;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool;
use r2d2_redis::{r2d2, RedisConnectionManager};
use anyhow::Result;
use dotenvy::dotenv;

pub fn init_dotenv() {
    dotenv().ok();
}

pub fn create_connection_pool() -> Result<deadpool::Pool<AsyncPgConnection>> {
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

pub fn get_redis_client_pool() -> Result<r2d2::Pool<RedisConnectionManager>> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url)?;
    let pool = r2d2::Pool::builder()
        .build(manager)?;

    Ok(pool)
}