use anyhow::{Result, anyhow};
use deadpool_redis::Connection;
use rand::Rng;
use serde::{Serialize, Deserialize};
use twitch_sources_rework::common_data::SubTypes;

#[cfg(not(debug_assertions))]
use crate::PROD_BASE_URL;

use super::{TWITCH_API_URI, get_app_token};


#[cfg(debug_assertions)]
mod ngrok {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TunnelItem {
        public_url: String
    }
    #[derive(Deserialize)]
    struct TunnelsResponse {
        tunnels: Vec<TunnelItem>
    }

    pub async fn get_https_link() -> String {
        let data = reqwest::get("http://localhost:4040/api/tunnels")
            .await
            .expect("Use ngrok in debug")
            .json::<TunnelsResponse>()
            .await
            .expect("To be able to parse ngrok response");

        data.tunnels
            .into_iter()
            .find(|item| item.public_url.starts_with("https"))
            .expect("To have a tunnel with https")
            .public_url
    }
}

const SECRET_LENGTH: usize = 50;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum SubCondition {
    BroadcasterUserId(String),
    ClientId(String)
}
#[derive(Serialize, Deserialize)]
pub struct SubTransport {
    pub method: String,
    pub callback: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum SubStatus {
    Enabled,
    WebhookCallbackVerificationPending,
    WebhookCallbackVerificationFailed,
    NotificationFailuresExceeded,
    AuthorizationRevoked,
    ModeratorRemoved,
    UserRemoved,
    VersionRemoved
}

#[derive(Serialize)]
struct SubRequest {
    #[serde(rename = "type")]
    type_: SubTypes,
    version: String,
    condition: SubCondition,
    transport: SubTransport
}

impl SubRequest {
    fn new(user_id: Option<i64>, sub_type: &SubTypes, callback_url: &str, secret: &str) -> Self {
        Self {
            type_: sub_type.clone(),
            version: "1".to_string(),
            condition: match user_id {
                Some(user_id) => SubCondition::BroadcasterUserId(user_id.to_string()),
                None => SubCondition::ClientId(std::env::var("TWITCH_KEY").expect("TWITCH_KEY is not set")),
            },
            transport: SubTransport {
                method: "webhook".to_string(),
                callback: callback_url.to_string(),
                secret: Some(secret.to_string())
            },
        }
    }
}


#[derive(Deserialize)]
pub struct SubData {
    pub id: String,
    pub status: SubStatus,
    #[serde(rename="type")]
    pub type_: SubTypes,
    pub version: String,
    pub cost: i64,
    pub condition: SubCondition,
    pub transport: SubTransport,
    pub created_at: chrono::DateTime<chrono::Utc>
}
#[derive(Deserialize)]
struct SubResponse {
    data: Vec<SubData>,
    total: i64,
    total_cost: i64,
    max_total_cost: i64
}

pub async fn subscribe(sub_type: &SubTypes, user_id: Option<i64>, mut redis_conn: Connection, http_client: &reqwest::Client) -> Result<SubData> {
    let twitch_key = std::env::var("TWITCH_KEY").expect("TWITCH_KEY is not set");
    
    #[cfg(debug_assertions)]
    let callback_url = ngrok::get_https_link().await + "/webhook/";
    #[cfg(not(debug_assertions))]
    let callback_url = PROD_BASE_URL.to_string() + "/webhook/";

    let mut rng = rand::thread_rng();
    let secret: String = (0..SECRET_LENGTH).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

    let response = match user_id {
        Some(user_id) => 
            http_client.post(TWITCH_API_URI.to_string() + "/eventsub/subscriptions")
                .header("Client-ID", twitch_key)
                .bearer_auth(get_app_token(&mut redis_conn, http_client).await?)
                .json(&SubRequest::new(Some(user_id), sub_type, &callback_url, &secret))
                .send()
                .await?
                .json::<SubResponse>()
                .await?
        ,
        None => {
            http_client.post(TWITCH_API_URI.to_string() + "/eventsub/subscriptions")
                .header("Client-ID", twitch_key)
                .bearer_auth(get_app_token(&mut redis_conn, http_client).await?)
                .json(&SubRequest::new(None, sub_type, &callback_url, &secret))
                .send()
                .await?
                .json::<SubResponse>()
                .await?
        }
        ,
    };

    let mut resp_data = response.data.into_iter().next().ok_or(anyhow!("Response data was empty"))?;

    resp_data.transport.secret = Some(secret);
    Ok(resp_data)
}