use actix_web::{web::{Data, Json}, Responder};
use paperclip::actix::{Apiv2Schema, api_v2_operation};
use serde::Serialize;

use crate::e500;
use crate::DbPool;
use crate::SCOPES;
use crate::db::AuthState;


#[derive(Serialize, Apiv2Schema)]
pub struct LoginUrlResponse {
    client_id: String,
    redirect_url: String,
    response_type: String,
    scopes: String,
    state: String
}

impl LoginUrlResponse {
    fn new(redirect_uri: &str, state: &str) -> Self {
        let twitch_key = std::env::var("TWITCH_KEY").expect("TWITCH_KEY must be set");

        Self {
            client_id: twitch_key,
            redirect_url: redirect_uri.to_string(),
            response_type: "code".to_string(),
            scopes: SCOPES.join("%20"),
            state: state.to_string()
        }
    }
}

#[api_v2_operation]
pub async fn login_url(req: actix_web::HttpRequest, db_pool: Data<DbPool>) -> Result<Json<LoginUrlResponse>, actix_web::Error> {
    let mut db_conn = db_pool.get().await.map_err(e500)?;

    let full_uri = req.connection_info().scheme().to_string() + "://" + req.connection_info().host() + &req.uri().to_string();
    let new_state = AuthState::get_new_state(&full_uri, &mut db_conn).await.map_err(e500)?;

    Ok(Json(LoginUrlResponse::new(&full_uri, &new_state)))
}