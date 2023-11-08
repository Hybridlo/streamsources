use auto_delegate::delegate;
use redis::AsyncCommands;

use super::{ResultRedis, RedisClient, RedisError};

#[async_trait::async_trait]
#[delegate]
pub trait MessagePublisher {
    async fn publish_message(&self, user_id: &str, topic: &str, message: &[u8]) -> ResultRedis<()>;
}

#[async_trait::async_trait]
impl MessagePublisher for RedisClient {
    async fn publish_message(&self, user_id: &str, topic: &str, message: &[u8]) -> ResultRedis<()> {
        let mut redis_conn = self.get_conn().await?;
        
        redis_conn.publish(user_id.to_string() + ":" + topic, message).await
            .map_err(|_| RedisError::Other)?;

        Ok(())
    }
}