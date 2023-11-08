use auto_delegate::delegate;
use redis::AsyncCommands;
use time::{OffsetDateTime, format_description::well_known::Rfc3339, Duration};

use super::{ResultRedis, RedisClient, RedisError};

#[async_trait::async_trait(?Send)]
#[delegate]
pub trait TokenCache {
    async fn try_get_existing_token(&self) -> ResultRedis<Option<String>>;
    async fn update_token(&self, token: &str) -> ResultRedis<()>;
}

#[async_trait::async_trait(?Send)]
impl TokenCache for RedisClient {
    async fn try_get_existing_token(&self) -> ResultRedis<Option<String>> {
        let mut redis_conn = self.get_conn().await?;

        if let Ok(val) = redis_conn.get::<_, String>("twitch_app_access_token_creation").await {
            let datetime = OffsetDateTime::parse(&val, &Rfc3339)
                .expect("To parse datetime of app token fetch time");
            
            if OffsetDateTime::now_utc() - datetime < Duration::days(1) {
                if let Ok(token) = redis_conn.get("twitch_app_access_token").await {
                    return Ok(Some(token));
                }
            }
        }

        Ok(None)
    }
    
    async fn update_token(&self, token: &str) -> ResultRedis<()> {
        let mut redis_conn = self.get_conn().await?;

        redis_conn.set("twitch_app_access_token", token).await.map_err(|_| RedisError::Other)?;
        redis_conn.set("twitch_app_access_token_creation", OffsetDateTime::now_utc().format(&Rfc3339).unwrap()).await.map_err(|_| RedisError::Other)?;

        Ok(())
    }
}