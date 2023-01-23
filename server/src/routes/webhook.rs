use std::str::from_utf8;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Bytes;
use actix_web::web::{Data, Json};
use serde::Deserialize;
use serde_json::Value;
use twitch_sources_rework::common_data::EventSubMessage;

use crate::db::Subscription;
use crate::errors::IntoResultMyErr;
use crate::errors::MyErrors;
use crate::DbPool;
use crate::twitch_api::SubData;
use crate::twitch_api::handle_message;

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

pub async fn webhook(request: HttpRequest, body: Bytes, db_pool: Data<DbPool>) -> Result<HttpResponse, MyErrors> {    
    let mut db_conn = db_pool.get().await?;
    // can't have both Json<> and Bytes parameters, only first parameter will be populated
    let post = serde_json::de::from_slice::<WebhookRequestData>(&*body)?;

    let sub = Subscription::get_subscription(&post.subscription.id, &mut db_conn).await.into_my()?;

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
        Some(sub) => {
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
        None => if msg_type == b"revocation" {
            return Ok(HttpResponse::Accepted().body(""))
        } else {
            return Err(MyErrors::InternalServerError("Something went wrong!".to_string()));
        },
    };

    if msg_type == b"revocation" {
        Subscription::remove_subscription(&post.subscription.id, &mut db_conn).await.into_my()?;
        return Ok(HttpResponse::Accepted().body(""));
    }
    
    match &post.type_ {
        WebhookRequestType::Challenge(challenge_string) => {
            return Ok(HttpResponse::Ok().body(challenge_string.clone()));
        },
        WebhookRequestType::Event(event) => {
            let message = EventSubMessage::new(
                &post.subscription.type_,
                from_utf8(timestamp).expect("This can never be not valid ascii/utf8"),
                event.clone()
            ).into_my()?;
            
            handle_message(message, &mut db_conn).await.into_my()?;
            
            return Ok(HttpResponse::Accepted().body(""));
        },
    }
}