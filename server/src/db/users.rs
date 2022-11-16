use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

use super::twitch_users;
use crate::errors::e500;
use crate::twitch_api::{TWITCH_API_AUTH, TWITCH_API_URI};
use crate::REDIRECT_URL;

#[derive(Insertable)]
#[diesel(table_name = twitch_users)]
struct NewTwitchUser {
    id: i64,
    username: String,
    access_token: String,
    refresh_token: String,
    expires_in: i32,
    scopes: Vec<String>,
    broadcaster_type: String
}

#[derive(Queryable, Identifiable, Debug)]
pub struct TwitchUser {
    id: i64,
    username: String,
    access_token: String,
    refresh_token: String,
    creation: chrono::NaiveDateTime,
    last_login: chrono::NaiveDateTime,
    expires_in: i32,
    scopes: Vec<Option<String>>,
    broadcaster_type: String
}

#[derive(Serialize)]
struct AuthParams {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String
}

impl AuthParams {
    pub fn new(code: &str, host: &str, twitch_key: &str, twitch_secret: &str) -> Self {

        AuthParams {
            client_id: twitch_key.to_string(),
            client_secret: twitch_secret.to_string(),
            code: code.to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: host.to_string() + REDIRECT_URL
        }
    }
}

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    expires_in: i32,
    refresh_token: String,
    scope: Vec<String>,
    token_type: String
}

#[derive(Deserialize)]
struct UserObject {
    id: String,
    login: String,
    broadcaster_type: String
}
#[derive(Deserialize)]
struct UserResponse {
    data: Vec<UserObject>
}
pub async fn update_or_create_and_get_user(
    code: &str,
    host: &str,
    http_client: &reqwest::Client,
    db_conn: &mut AsyncPgConnection
) -> Result<TwitchUser> {
    let twitch_key = std::env::var("TWITCH_KEY").expect("TWITCH_KEY must be set");
    let twitch_secret = std::env::var("TWITCH_SECRET").expect("TWITCH_SECRET must be set");

    let req_body = serde_json::ser::to_vec(&AuthParams::new(code, host, &twitch_key, &twitch_secret))?;

    let response = http_client.post(TWITCH_API_AUTH.to_string() + "/oauth2/token")
        .body(req_body)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    
    let resp_bytes = response.bytes().await?;
    println!("{:?}", resp_bytes);
    let mut auth_response = serde_json::de::from_slice::<AuthResponse>(&resp_bytes)?;
    auth_response.scope.sort_unstable();

    let response = http_client.get(TWITCH_API_URI.to_string() + "/users")
        .header("Authorization", format!("Bearer {}", auth_response.access_token))
        .header("Client-Id", twitch_key)
        .send()
        .await?.error_for_status()?;

    let resp_bytes = response.bytes().await?;
    let user_response = serde_json::de::from_slice::<UserResponse>(&resp_bytes)?;

    let user_id = user_response.data[0].id.parse::<i64>()?;

    let existing_user: Result<TwitchUser, diesel::result::Error> = twitch_users::dsl::twitch_users
        .filter(twitch_users::dsl::id.eq(user_id))
        .first::<TwitchUser>(db_conn)
        .await;

    let mut user: TwitchUser = match existing_user {
        Ok(user) => {
            // user exists
            diesel::update(&user)
                .set((
                    twitch_users::dsl::access_token.eq(auth_response.access_token),
                    twitch_users::dsl::refresh_token.eq(auth_response.refresh_token),
                ))
                .get_result::<TwitchUser>(db_conn)
                .await?
        },
        Err(_) => {
            // create a new user
            let new_user = NewTwitchUser {
                id: user_id,
                username: user_response.data[0].login.clone(),
                access_token: auth_response.access_token,
                refresh_token: auth_response.refresh_token,
                expires_in: auth_response.expires_in,
                scopes: auth_response.scope.clone(),
                broadcaster_type: user_response.data[0].broadcaster_type.clone(),
            };

            diesel::insert_into(twitch_users::table)
                .values(&new_user)
                .get_result::<TwitchUser>(db_conn).await?
        },
    };

    if user.username != user_response.data[0].login {
        // username update
        user = diesel::update(&user)
            .set(twitch_users::dsl::username.eq(&user_response.data[0].login))
            .get_result::<TwitchUser>(db_conn)
            .await?;
    }

    if
        user.scopes.iter().all(|el| el.is_some())
        // there's probably a better way to do this
        && user.scopes.iter().map(|el| el.as_ref().unwrap()).collect::<Vec<_>>() != auth_response.scope.iter().collect::<Vec<_>>() {
            // scope update
            user = diesel::update(&user)
                .set(twitch_users::dsl::scopes.eq(&auth_response.scope))
                .get_result::<TwitchUser>(db_conn)
                .await?;
    }

    Ok(user)
}