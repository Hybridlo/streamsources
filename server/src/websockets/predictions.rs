
use actix_web_actors::ws;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::{util::{message_manager::GenericPassthroughWs, session_state::TypedSession}, errors::MyErrors};

const PREDICTIONS_TOPIC: &str = "predictions";

pub async fn predictions_websocket(req: HttpRequest, session: TypedSession, stream: web::Payload) -> Result<HttpResponse, MyErrors> {
    let user_id = session.get_user_id()?.ok_or(MyErrors::AccessDenied)?;
    
    let resp = ws::start(GenericPassthroughWs::new(user_id, PREDICTIONS_TOPIC), &req, stream)?;
    
    Ok(resp)
}