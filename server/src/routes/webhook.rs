use std::str::from_utf8;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Bytes;
use actix_web::web::Data;
use serde::Deserialize;
use serde_json::Value;
use twitch_sources_rework::common_data::EventSubMessage;

use crate::RedisPool;
use crate::domain::subscription::GetSub;
use crate::domain::subscription::Subscription;
use crate::errors::IntoResultMyErr;
use crate::errors::MyErrors;
use crate::DbPool;
use crate::twitch_api::SubData;
use crate::twitch_api::handle_message;
use crate::util::Context;

#[derive(Deserialize)]
#[serde(rename_all="snake_case")]
pub enum WebhookRequestType {
    Challenge(String),
    Event(Value)
}

#[derive(Deserialize)]
pub struct WebhookRequestData {
    subscription: SubData,
    #[serde(flatten)]
    type_: WebhookRequestType
}

pub async fn webhook(request: HttpRequest, body: Bytes, ctx: Context, db_pool: Data<DbPool>, redis_pool: Data<RedisPool>) -> Result<HttpResponse, MyErrors> {
    let mut db_conn = db_pool.get().await?;
    let mut redis_conn = redis_pool.get().await?;
    // can't have both Json<> and Bytes parameters, only first parameter will be populated
    let post = serde_json::de::from_slice::<WebhookRequestData>(&*body)?;

    let sub = Subscription::get_subscription(&ctx.repository, &post.subscription.id).await;

    let msg_type = request
        .headers()
        .get("Twitch-Eventsub-Message-Type")
        .ok_or(MyErrors::InternalServerError("Missing twitch header".to_string()))?
        .as_bytes();

    let timestamp = request
        .headers()
        .get("Twitch-Eventsub-Message-Timestamp")
        .ok_or(MyErrors::InternalServerError("Missing twitch header".to_string()))?
        .as_bytes();

    // this part checks for all unhappy paths and ends the function
    match sub {
        Ok(sub) => {
            let msg_id = request
                .headers()
                .get("Twitch-Eventsub-Message-Id")
                .ok_or(MyErrors::InternalServerError("Missing twitch header".to_string()))?
                .as_bytes();

            let twitch_signature = request
                .headers()
                .get("Twitch-Eventsub-Message-Signature")
                .ok_or(MyErrors::InternalServerError("Missing twitch header".to_string()))?
                .as_bytes();

            if !sub.verify_msg(&[msg_id, timestamp, &*body].concat(), twitch_signature) {
                return Err(MyErrors::AccessDenied)
            }
        },
        Err(GetSub::NotFound) if msg_type == b"revocation" => {
            return Ok(HttpResponse::Accepted().body(""))
        },
        _ => {
            return Err(MyErrors::InternalServerError("Something went wrong!".to_string()));
        },
    };

    if msg_type == b"revocation" {
        Subscription::remove_subscription(&ctx.repository, &post.subscription.id).await.into_my()?;
        return Ok(HttpResponse::Accepted().body(""));
    }
    
    match post.type_ {
        WebhookRequestType::Challenge(challenge_string) => {
            return Ok(HttpResponse::Ok().body(challenge_string));
        },
        WebhookRequestType::Event(event) => {
            let message = EventSubMessage::new(
                &post.subscription.type_,
                from_utf8(timestamp).expect("This can never be not valid ascii/utf8"),
                event
            ).into_my()?;
            
            handle_message(message, &mut db_conn, &mut redis_conn).await.into_my()?;
            
            return Ok(HttpResponse::Accepted().body(""));
        },
    }
}