use std::str::from_utf8;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Bytes;
use serde::Deserialize;
use serde_json::Value;
use twitch_sources_rework::common_data::EventSubMessage;

use crate::domain::subscription::GetSub;
use crate::domain::subscription::Subscription;
use crate::errors::IntoResultMyErr;
use crate::errors::MyErrors;
use crate::http_client::twitch_client::SubData;
use crate::util::Context;
use crate::twitch_api::message_handler::EventMessageHandler;

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

pub async fn webhook(request: HttpRequest, body: Bytes, ctx: Context) -> Result<HttpResponse, MyErrors> {
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
            
            ctx.handle_message(message).await.into_my()?;
            
            return Ok(HttpResponse::Accepted().body(""));
        },
    }
}