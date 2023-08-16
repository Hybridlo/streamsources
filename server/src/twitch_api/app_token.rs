use anyhow::Result;
use deadpool_redis::Connection;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description::well_known::Rfc3339, Duration};

use super::TWITCH_API_AUTH;


#[derive(Serialize)]
struct NewTokenRequest {
    client_id: String,
    client_secret: String,
    grant_type: String
}
#[derive(Deserialize)]
struct NewTokenResponse {
    access_token: String,
    expires_in: i32,
    token_type: String
}
async fn new_token(redis_conn: &mut Connection, http_client: &reqwest::Client) -> Result<String> {
    let twitch_key = std::env::var("TWITCH_KEY").expect("TWITCH_KEY is not set");
    let twitch_secret = std::env::var("TWITCH_SECRET").expect("TWITCH_SECRET is not set");

    let body = serde_json::ser::to_vec(
        &NewTokenRequest {
            client_id: twitch_key,
            client_secret: twitch_secret,
            grant_type: "client_credentials".to_string()
        }
    )?;

    let response = http_client.post(&(TWITCH_API_AUTH.to_string() + "/oauth2/token"))
                                       .body(body)
                                       .header("Content-Type", "application/json")
                                       .send()
                                       .await?;

    //panic if we can't get a new token
    let response = response.error_for_status().expect("Got an error request when trying to get a new token");

    let resp_bytes = response.bytes().await?;
    let response = serde_json::de::from_slice::<NewTokenResponse>(&*resp_bytes)?;

    redis_conn.set("twitch_app_access_token", &response.access_token).await?;
    redis_conn.set("twitch_app_access_token_creation", OffsetDateTime::now_utc().format(&Rfc3339).unwrap()).await?;

    Ok(response.access_token)
}

pub async fn get_app_token(redis_conn: &mut Connection, http_client: &reqwest::Client) -> Result<String> {
    if let Ok(val) = redis_conn.get::<_, String>("twitch_app_access_token_creation").await {
        let datetime = OffsetDateTime::parse(&val, &Rfc3339)
            .expect("To parse datetime of app token fetch time");
        
        if OffsetDateTime::now_utc() - datetime < Duration::days(1) {
            if let Ok(token) = redis_conn.get("twitch_app_access_token").await {
                return Ok(token);
            }
        }
    }


    Ok(new_token(redis_conn, http_client).await?)
}