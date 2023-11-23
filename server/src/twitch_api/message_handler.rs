use anyhow::Result;
use thiserror::Error;
use twitch_sources_rework::common_data::eventsub_msgs::{EventSubMessage, EventSubData};

use crate::{db::TwitchUserDb, domain::users::{TwitchUser, DeleteUserError}, my_redis::{RedisError, publisher::MessagePublisher}, websockets};


#[async_trait::async_trait(?Send)]
pub trait EventMessageHandler {
    async fn handle_message(&self, msg: EventSubMessage) -> Result<(), HandleMessageError>;
}

#[async_trait::async_trait(?Send)]
impl<T: TwitchUserDb + MessagePublisher> EventMessageHandler for T {
    async fn handle_message(&self, msg: EventSubMessage) -> Result<(), HandleMessageError> {
        if let EventSubData::UserAuthorizationRevoke(data) = &msg.data {
            TwitchUser::delete_user(self, data.user_id.parse()?).await?;
        }

        // Dispatch to all websockets, that declared this message as it's sub_type
        for websocket_data in websockets::WEBSOCKET_DATA_TYPES {
            if websocket_data.sub_types.contains(&msg.data.sub_type()) {
                let data = serde_json::ser::to_vec(&msg)?;
    
                self.publish_message(msg.data.get_target(), websocket_data.topic, &data).await?;
            }
        }
    
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