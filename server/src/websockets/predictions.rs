
use actix_web_actors::ws;
use actix_web::{web, HttpRequest, HttpResponse};
use twitch_sources_rework::common_data::SubType;

use crate::{util::{message_manager::GenericPassthroughWs, session_state::TypedSession, Context}, errors::MyErrors, domain::subscription::Subscription, http_client::twitch_client::SubCondition};

const PREDICTIONS_TOPIC: &str = "predictions";

pub async fn predictions_websocket(
    req: HttpRequest,
    session: TypedSession,
    stream: web::Payload,
    ctx: Context,
) -> Result<HttpResponse, MyErrors> {
    let user_id = session.get_user_id()?.ok_or(MyErrors::AccessDenied)?;

    Subscription::get_or_create_subscriptions(
        &ctx,
        vec![
            SubType::ChannelPredictionBegin,
            SubType::ChannelPredictionProgress,
            SubType::ChannelPredictionLock,
            SubType::ChannelPredictionEnd,
        ],
        SubCondition::BroadcasterUserId(user_id.to_string()),
    )
        .await
        .map_err(|err| MyErrors::InternalServerError(err.to_string()))?;
    
    let resp = ws::start(GenericPassthroughWs::new(user_id, PREDICTIONS_TOPIC), &req, stream)?;
    
    Ok(resp)
}