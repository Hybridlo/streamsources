use anyhow::Result;
use thiserror::Error;

use crate::{http_client::twitch_client::{GetTokenError, TwitchHttpClient}, my_redis::{token_cache::TokenCache, RedisError}};

#[async_trait::async_trait(?Send)]
pub trait TwitchTokenManager {
    async fn get_app_token(&self) -> Result<String, TwitchTokenError>;
}

#[derive(Debug, Error)]
pub enum TwitchTokenError {
    #[error("Token request from Twitch failed: {0}")]
    RequestTokenError(#[from] GetTokenError),
    #[error("Redis operation failed: {0}")]
    RedisFail(#[from] RedisError)
}

#[async_trait::async_trait(?Send)]
impl<T: TokenCache + TwitchHttpClient> TwitchTokenManager for T {
    async fn get_app_token(&self) -> Result<String, TwitchTokenError> {
        if let Some(token) = self.try_get_existing_token().await? {
            return Ok(token);
        }

        let token = self.get_new_token().await?;
        self.update_token(&token).await?;

        Ok(token)
    }
}