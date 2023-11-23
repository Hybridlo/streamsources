use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::http::{header, StatusCode};
use actix_web::web::{Json, Query};
use paperclip::actix::{Apiv2Schema, api_v2_operation};
use serde::{Serialize, Deserialize};
use itertools::Itertools as _;

use crate::domain::users::TwitchUser;
use crate::errors::IntoResultMyErr;
use crate::errors::{e500, MyErrors};
use crate::REDIRECT_URL;
use crate::domain::auth_state::AuthState;
use crate::domain::login_token::LoginToken;
use crate::util::Context;
use crate::util::get_twitch_key;
use crate::util::session_state::TypedSession;
use crate::websockets::WEBSOCKET_DATA_TYPES;


#[derive(Serialize, Apiv2Schema)]
pub struct LoginUrlResponse {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    state: String
}

impl LoginUrlResponse {
    pub fn new(host: &str, state: &str) -> Self {
        let twitch_key = get_twitch_key();

        Self {
            client_id: twitch_key,
            redirect_uri: host.to_string() + REDIRECT_URL,
            response_type: "code".to_string(),
            scope: WEBSOCKET_DATA_TYPES
                .into_iter()
                .flat_map(|type_| type_.scopes)
                .map(|a| *a)
                .intersperse(" ")
                .collect(),
            state: state.to_string()
        }
    }
}

#[derive(Deserialize, Apiv2Schema)]
pub struct LoginUrlRequest {
    callback_url: String
}
#[api_v2_operation]
pub async fn login_url(request: HttpRequest, query: Query<LoginUrlRequest>, ctx: Context) -> Result<Json<LoginUrlResponse>, actix_web::Error> {
    let full_uri = &query.callback_url;
    let new_state = AuthState::get_new_state(&ctx.repository, full_uri).await.map_err(e500)?;
    
    let host = request.connection_info().scheme().to_string() + "://" + request.connection_info().host();
    Ok(Json(LoginUrlResponse::new(&host, &new_state)))
}


#[derive(Deserialize)]
pub struct LoginEndQuery {
    state: String,
    code: String,
    //scopes: String
}
#[api_v2_operation(skip)]
pub async fn twitch_login_end(
    request: HttpRequest,
    query: Query<LoginEndQuery>,
    session: TypedSession,
    ctx: Context,
) -> Result<HttpResponse, actix_web::Error> {
    let data = AuthState::check_state_and_get_data(&ctx.repository, &query.state).await;

    let host = request.connection_info().scheme().to_string() + "://" + request.connection_info().host();
    let mut response = HttpResponse::Ok();

    match data {
        Ok(data) => {
            let user = TwitchUser::update_or_create_and_get_user(&ctx, &query.code, &host)
                .await.map_err(e500)?;

            session.renew();
            // TODO: this isn't an e500, placeholder for now
            session.insert_user_id(user.id).map_err(e500)?;

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

#[derive(Serialize, Apiv2Schema)]
pub struct UserInfo {
    username: String
}
#[api_v2_operation]
pub async fn login_check(session: TypedSession, ctx: Context) -> Result<Json<UserInfo>, MyErrors> {
    match session.get_user_id()? {
        Some(user_id) => {
            let username = TwitchUser::get_user(&ctx, user_id).await.into_my()?
                .ok_or(MyErrors::AccessDenied)?.username;
            Ok(Json(UserInfo { username }))
        },
        None => Err(MyErrors::AccessDenied),
    }
}

#[derive(Serialize, Apiv2Schema)]
pub struct LoginTokenResponse {
    token: String
}
#[api_v2_operation]
pub async fn generate_login_token(session: TypedSession, ctx: Context) -> Result<Json<LoginTokenResponse>, MyErrors> {
    let user_id = session.get_user_id()?.ok_or(MyErrors::AccessDenied)?;

    let token = LoginToken::create_or_get_login_token(&ctx.repository, user_id).await.into_my()?;

    Ok(Json(LoginTokenResponse { token }))
}