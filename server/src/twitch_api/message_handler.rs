use anyhow::Result;
use deadpool_redis::Connection;
use diesel_async::AsyncPgConnection;
use twitch_sources_rework::common_data::{EventSubMessage, EventSubData};

use crate::{db::TwitchUser, util::message_manager::send_message};

async fn unauthorize_user(user_id: &str, db_conn: &mut AsyncPgConnection) -> Result<()> {
    TwitchUser::delete_user(user_id.parse()?, db_conn).await?;

    Ok(())
}

pub async fn handle_message(msg: EventSubMessage, db_conn: &mut AsyncPgConnection, redis_conn: &mut Connection) -> Result<()> {
    match &msg.data {
        EventSubData::UserAuthorizationRevoke(data) => unauthorize_user(&data.user_id, db_conn).await?,
        EventSubData::ChannelPredictionBegin(_)
      | EventSubData::ChannelPredictionProgress(_)
      | EventSubData::ChannelPredictionLock(_)
      | EventSubData::ChannelPredictionEnd(_) => {
            let data = serde_json::ser::to_vec(&msg)?;

            send_message(redis_conn, &msg.get_target(), "predictions", &data).await?;
        },
    };

    Ok(())
}