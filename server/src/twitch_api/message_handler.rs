use anyhow::Result;
use diesel_async::AsyncPgConnection;
use twitch_sources_rework::common_data::{EventSubMessage, EventSubData};

use crate::db::TwitchUser;

async fn unauthorize_user(user_id: &str, db_conn: &mut AsyncPgConnection) -> Result<()> {
    TwitchUser::delete_user(user_id.parse()?, db_conn).await?;

    Ok(())
}

pub async fn handle_message(msg: EventSubMessage, db_conn: &mut AsyncPgConnection) -> Result<()> {
    match msg.data {
        EventSubData::UserAuthorizationRevoke(data) => unauthorize_user(&data.user_id, db_conn).await?,
        EventSubData::ChannelPredictionBegin(data) => todo!(),
        EventSubData::ChannelPredictionProgress(data) => todo!(),
        EventSubData::ChannelPredictionLock(data) => todo!(),
        EventSubData::ChannelPredictionEnd(data) => todo!(),
    };

    Ok(())
}