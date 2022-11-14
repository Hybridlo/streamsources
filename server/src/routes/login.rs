use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::http::{header, StatusCode};
use actix_web::web::{Data, Json, Query};
use paperclip::actix::{Apiv2Schema, api_v2_operation};
use serde::{Serialize, Deserialize};

use crate::e500;
use crate::DbPool;
use crate::SCOPES;
use crate::REDIRECT_URL;
use crate::db::AuthState;
use crate::db::update_or_create_and_get_user;


#[derive(Serialize, Apiv2Schema)]
pub struct LoginUrlResponse {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    state: String
}

impl LoginUrlResponse {
    fn new(host: &str, state: &str) -> Self {
        let twitch_key = std::env::var("TWITCH_KEY").expect("TWITCH_KEY must be set");

        Self {
            client_id: twitch_key,
            redirect_uri: host.to_string() + REDIRECT_URL,
            response_type: "code".to_string(),
            scope: SCOPES.join("%20"),
            state: state.to_string()
        }
    }
}

#[derive(Deserialize, Apiv2Schema)]
pub struct LoginUrlRequest {
    callback_url: String
}
#[api_v2_operation]
pub async fn login_url(request: HttpRequest, query: Query<LoginUrlRequest>, db_pool: Data<DbPool>) -> Result<Json<LoginUrlResponse>, actix_web::Error> {
    let mut db_conn = db_pool.get().await.map_err(e500)?;

    let full_uri = &query.callback_url;
    let new_state = AuthState::get_new_state(full_uri, &mut db_conn).await.map_err(e500)?;
    
    let host = request.connection_info().host().to_string();
    Ok(Json(LoginUrlResponse::new(&host, &new_state)))
}


#[derive(Deserialize)]
pub struct LoginEndQuery {
    state: String,
    code: String,
    //scopes: String
}
pub async fn twitch_login_end(
    request: HttpRequest,
    query: Query<LoginEndQuery>,
    db_pool: Data<DbPool>,
    http_client: Data<reqwest::Client>
) -> Result<HttpResponse, actix_web::Error> {
    let mut db_conn = db_pool.get().await.map_err(e500)?;
    
    let data = AuthState::check_state_and_get_data(&query.state, &mut db_conn).await;

    let host = request.connection_info().scheme().to_string() + "://" + request.connection_info().host();
    let mut response = HttpResponse::Ok();

    match data {
        Ok(data) => {
            let user = update_or_create_and_get_user(&query.code, &host, &http_client, &mut db_conn)
                .await.map_err(e500)?;

            println!("{:?}", user);

            response.status(StatusCode::SEE_OTHER);
            response.append_header((
                header::LOCATION,
                header::HeaderValue::from_str(&data.redirect_uri).map_err(e500)?
            ));
            Ok(response.body("Login success!".to_string()))
        },
        Err(err) => {
            // i'm not sure this works to notify the user, might make this redirect to frontend app
            // with an error message
            response.status(StatusCode::SEE_OTHER);
            response.append_header((
                header::LOCATION,
                header::HeaderValue::from_str(&host).map_err(e500)?
            ));
            Ok(response.body(err.to_string()))
        },
    }
}