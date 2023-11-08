use anyhow::Result;
use thiserror::Error;
use twitch_sources_rework::common_data::{EventSubMessage, EventSubData};

use crate::{db::TwitchUserDb, domain::users::{TwitchUser, DeleteUserError}, my_redis::{RedisError, publisher::MessagePublisher}};


#[async_trait::async_trait(?Send)]
pub trait EventMessageHandler {
    async fn handle_message(&self, msg: EventSubMessage) -> Result<(), HandleMessageError>;
}

#[async_trait::async_trait(?Send)]
impl<T: TwitchUserDb + MessagePublisher> EventMessageHandler for T {
    async fn handle_message(&self, msg: EventSubMessage) -> Result<(), HandleMessageError> {
        match &msg.data {
            EventSubData::UserAuthorizationRevoke(data) => TwitchUser::delete_user(self, data.user_id.parse()?).await?,
            EventSubData::ChannelPredictionBegin(_)
          | EventSubData::ChannelPredictionProgress(_)
          | EventSubData::ChannelPredictionLock(_)
          | EventSubData::ChannelPredictionEnd(_) => {
                let data = serde_json::ser::to_vec(&msg)?;
    
                self.publish_message(&msg.get_target(), "predictions", &data).await?;
            },
        };
    
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum HandleMessageError {
    #[error("User id was not a valid i64 number")]
    MalformedUserId(#[from] std::num::ParseIntError),
    #[error("User unauthrization failed: {0}")]
    UnauthorizeFail(#[from] DeleteUserError),
    #[error("Message serialization failed: {0}")]
    MessageSerializationFail(#[from] serde_json::Error),
    #[error("Publishing message failed: {0}")]
    PublishFail(#[from] RedisError),
}