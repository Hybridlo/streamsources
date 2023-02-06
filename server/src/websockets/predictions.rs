
use actix_web_actors::ws;
use actix_web::{web::{self, Data}, HttpRequest, HttpResponse};
use twitch_sources_rework::common_data::SubTypes;

use crate::{util::{message_manager::GenericPassthroughWs, session_state::TypedSession}, errors::MyErrors, db::Subscription, DbPool, RedisPool};

const PREDICTIONS_TOPIC: &str = "predictions";

pub async fn predictions_websocket(
    req: HttpRequest,
    session: TypedSession,
    stream: web::Payload,
    db_pool: Data<DbPool>,
    redis_pool: Data<RedisPool>,
    http_client: Data<reqwest::Client>
) -> Result<HttpResponse, MyErrors> {
    let user_id = session.get_user_id()?.ok_or(MyErrors::AccessDenied)?;
    let mut db_conn = db_pool.get().await?;

    Subscription::get_or_create_subscriptions(
        vec![
            SubTypes::ChannelPredictionBegin,
            SubTypes::ChannelPredictionProgress,
            SubTypes::ChannelPredictionLock,
            SubTypes::ChannelPredictionEnd,
        ],
        Some(user_id),
        &mut db_conn,
        &redis_pool,
        &http_client
    )
        .await
        .map_err(|err| MyErrors::InternalServerError(err.to_string()))?;
    
    let resp = ws::start(GenericPassthroughWs::new(user_id, PREDICTIONS_TOPIC), &req, stream)?;
    
    Ok(resp)
}