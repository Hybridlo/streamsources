use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web;
use actix_web_actors::ws;
use twitch_sources_rework::common_data::eventsub_msgs::SubType;

use crate::domain::subscription::Subscription;
use crate::domain::users::TwitchUser;
use crate::errors::MyErrors;
use crate::http_client::twitch_client::SubCondition;
use crate::util::Context;
use crate::util::message_manager::GenericPassthroughWs;
use crate::util::session_state::TypedSession;

pub struct WebsocketData {
    pub topic: &'static str,
    pub sub_types: &'static [SubType],
    pub scopes: &'static [&'static str]
}

pub const WEBSOCKET_DATA_TYPES: &[WebsocketData] = &[
    WebsocketData {
        topic: "predictions",
        sub_types: &[
            SubType::ChannelPredictionBegin,
            SubType::ChannelPredictionProgress,
            SubType::ChannelPredictionLock,
            SubType::ChannelPredictionEnd,
        ],
        scopes: &["channel:read:predictions"]
    },

    WebsocketData {
        topic: "hype_train",
        sub_types: &[
            SubType::HypeTrainBegin,
            SubType::HypeTrainProgress,
            SubType::HypeTrainEnd,
        ],
        scopes: &["channel:read:hype_train"]
    }
];

pub async fn websocket_starter(
    data: &WebsocketData,
    req: HttpRequest,
    session: TypedSession,
    stream: web::Payload,
    ctx: Context
) -> Result<HttpResponse, MyErrors> {
    let user_id = session.get_user_id()?.ok_or(MyErrors::AccessDenied)?;

    let user = TwitchUser::get_user(&ctx, user_id).await?.ok_or(MyErrors::AccessDenied)?;
    if !data.scopes.iter().all(|scope| user.scopes.contains(&scope.to_string())) {
        return Err(MyErrors::AccessDenied);
    }

    let subs = Subscription::get_or_create_subscriptions(
        &ctx,
        data.sub_types,
        SubCondition::BroadcasterUserId(user_id.to_string()),
    )
        .await
        .map_err(|err| MyErrors::InternalServerError(err.to_string()))?;

    let sub_ids = subs.into_iter().map(|sub| sub.sub_id().to_string()).collect();
    
    let resp = ws::start(GenericPassthroughWs::new(user_id, data.topic, sub_ids, ctx.repository.clone()), &req, stream)?;
    
    Ok(resp)
}