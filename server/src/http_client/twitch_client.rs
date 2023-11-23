
use crate::util::get_twitch_key;

use super::HttpClient;

use auto_delegate::delegate;
pub use get_new_token_info::GetTokenError;
pub use get_user_token_info::{UserTokenResponse, GetUserTokenError};
pub use subscribe_info::{SubCondition, SubData, SubscribeRequestError};
pub use get_user_data_info::{UserDataObject, GetUserDataError};
use twitch_sources_rework::common_data::eventsub_msgs::SubType;

pub const TWITCH_API_URI: &str = "https://api.twitch.tv/helix";
pub const TWITCH_API_AUTH: &str = "https://id.twitch.tv";

#[async_trait::async_trait(?Send)]
#[delegate]
pub trait TwitchHttpClient {
    async fn get_new_token(&self) -> Result<String, GetTokenError>;
    async fn create_subscription(
        &self,
        sub_cond: SubCondition,
        sub_type: SubType,
        callback_url: &str,
        secret: &str,
        app_token: &str
    ) -> Result<subscribe_info::SubData, subscribe_info::SubscribeRequestError>;
    async fn get_user_token(&self, code: &str, host: &str) -> Result<UserTokenResponse, GetUserTokenError>;
    async fn get_user_data(&self, user_access_token: &str) -> Result<UserDataObject, GetUserDataError>;
}

#[async_trait::async_trait(?Send)]
impl TwitchHttpClient for HttpClient {
    async fn get_new_token(&self) -> Result<String, GetTokenError> {
        let response = self.0.post(&(TWITCH_API_AUTH.to_string() + "/oauth2/token"))
            .json(&get_new_token_info::NewTokenRequest::default())
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(GetTokenError::HttpError)?
            .error_for_status()
            .map_err(GetTokenError::HttpError)?
            .json::<get_new_token_info::NewTokenResponse>()
            .await
            .map_err(GetTokenError::DeserializeError)?;

        Ok(response.access_token)
    }

    async fn create_subscription(
        &self,
        sub_cond: SubCondition,
        sub_type: SubType,
        callback_url: &str,
        secret: &str,
        app_token: &str
    ) -> Result<SubData, SubscribeRequestError> {
        let response = self.0.post(TWITCH_API_URI.to_string() + "/eventsub/subscriptions")
            .header("Client-ID", get_twitch_key())
            .bearer_auth(app_token)
            .json(&subscribe_info::SubRequest::new(sub_cond, sub_type, callback_url, secret))
            .send()
            .await
            .map_err(SubscribeRequestError::HttpError)?
            .json::<subscribe_info::SubResponse>()
            .await
            .map_err(SubscribeRequestError::DeserializeError)?;

        response.data.into_iter().next().ok_or(SubscribeRequestError::ResponseSubEmpty)
    }

    async fn get_user_token(&self, code: &str, host: &str) -> Result<UserTokenResponse, GetUserTokenError> {
        let mut resp = self.0.post(TWITCH_API_AUTH.to_string() + "/oauth2/token")
            .json(&get_user_token_info::UserTokenRequest::new(code, host))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(GetUserTokenError::HttpError)?
            .json::<UserTokenResponse>()
            .await
            .map_err(GetUserTokenError::DeserializeError)?;

        resp.scope.sort_unstable();

        Ok(resp)
    }

    async fn get_user_data(&self, user_access_token: &str) -> Result<UserDataObject, GetUserDataError> {
        let response = self.0.get(TWITCH_API_URI.to_string() + "/users")
            .header("Authorization", format!("Bearer {}", user_access_token))
            .header("Client-Id", get_twitch_key())
            .send()
            .await
            .map_err(GetUserDataError::HttpError)?
            .error_for_status()
            .map_err(GetUserDataError::HttpError)?
            .json::<get_user_data_info::UserDataResponse>()
            .await
            .map_err(GetUserDataError::DeserializeError)?;

        response.data.into_iter().next().ok_or(GetUserDataError::ResponseUserEmpty)
    }
}


mod get_new_token_info {
    use serde::{Serialize, Deserialize};
    use thiserror::Error;

    use crate::util::{get_twitch_key, get_twitch_secret};

    #[derive(Serialize)]
    pub(crate) struct NewTokenRequest {
        pub client_id: String,
        pub client_secret: String,
        pub grant_type: String
    }

    impl Default for NewTokenRequest {
        fn default() -> Self {
            Self {
                client_id: get_twitch_key(),
                client_secret: get_twitch_secret(),
                grant_type: "client_credentials".to_string()
            }
        }
    }

    #[derive(Deserialize)]
    pub(crate) struct NewTokenResponse {
        pub access_token: String,
        pub expires_in: i32,
        pub token_type: String
    }

    #[derive(Debug, Error)]
    pub enum GetTokenError {
        #[error("Error while executing an HTTP request: {0}")]
        HttpError(reqwest::Error),
        #[error("Error while deserializing an HTTP response: {0}")]
        DeserializeError(reqwest::Error),
    }
}

mod subscribe_info {
    use serde::{Serialize, Deserialize};
    use thiserror::Error;
    use twitch_sources_rework::common_data::eventsub_msgs::SubType;

    use crate::util::get_twitch_key;

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all="snake_case")]
    pub enum SubCondition {
        BroadcasterUserId(String),
        ClientId(String)
    }

    impl SubCondition {
        pub fn into_user_id(self) -> Option<i64> {
            if let SubCondition::BroadcasterUserId(user_id) = self {
                return Some(user_id.parse().expect("Must be a valid i64 number"))
            }

            None
        }

        pub fn client_id() -> Self {
            Self::ClientId(get_twitch_key())
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SubTransport {
        pub method: String,
        pub callback: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub secret: Option<String>
    }

    #[derive(Serialize)]
    pub(crate) struct SubRequest {
        #[serde(rename = "type")]
        pub type_: SubType,
        pub version: String,
        pub condition: SubCondition,
        pub transport: SubTransport
    }

    impl SubRequest {
        pub(crate) fn new(sub_cond: SubCondition, sub_type: SubType, callback_url: &str, secret: &str) -> Self {
            Self {
                type_: sub_type,
                version: "1".to_string(),
                condition: sub_cond,
                transport: SubTransport {
                    method: "webhook".to_string(),
                    callback: callback_url.to_string(),
                    secret: Some(secret.to_string())
                },
            }
        }
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

    #[derive(Deserialize)]
    pub struct SubData {
        pub id: String,
        pub status: SubStatus,
        #[serde(rename="type")]
        pub type_: SubType,
        pub version: String,
        pub cost: i64,
        pub condition: SubCondition,
        pub transport: SubTransport,
        #[serde(with = "time::serde::rfc3339")]
        pub created_at: time::OffsetDateTime,
    }
    
    #[derive(Deserialize)]
    pub(crate) struct SubResponse {
        pub data: Vec<SubData>,
        pub total: i64,
        pub total_cost: i64,
        pub max_total_cost: i64
    }

    #[derive(Debug, Error)]
    pub enum SubscribeRequestError {
        #[error("Error while executing an HTTP request: {0}")]
        HttpError(reqwest::Error),
        #[error("Error while deserializing an HTTP response: {0}")]
        DeserializeError(reqwest::Error),
        #[error("Response is valid, but had subscription data empty")]
        ResponseSubEmpty
    }
}

mod get_user_token_info {
    use serde::{Serialize, Deserialize};
    use thiserror::Error;

    use crate::{REDIRECT_URL, util::{get_twitch_key, get_twitch_secret}};

    #[derive(Serialize)]
    pub(crate) struct UserTokenRequest {
        client_id: String,
        client_secret: String,
        code: String,
        grant_type: String,
        redirect_uri: String
    }

    impl UserTokenRequest {
        pub fn new(code: &str, host: &str) -> Self {
            UserTokenRequest {
                client_id: get_twitch_key(),
                client_secret: get_twitch_secret(),
                code: code.to_string(),
                grant_type: "authorization_code".to_string(),
                redirect_uri: host.to_string() + REDIRECT_URL
            }
        }
    }

    #[derive(Deserialize)]
    pub struct UserTokenResponse {
        pub access_token: String,
        pub expires_in: i32,
        pub refresh_token: String,
        pub scope: Vec<String>
    }

    #[derive(Debug, Error)]
    pub enum GetUserTokenError {
        #[error("Error while executing an HTTP request: {0}")]
        HttpError(reqwest::Error),
        #[error("Error while deserializing an HTTP response: {0}")]
        DeserializeError(reqwest::Error),
    }
}

mod get_user_data_info {
    use serde::Deserialize;
    use thiserror::Error;
    
    #[derive(Deserialize)]
    pub struct UserDataObject {
        pub id: String,
        pub login: String,
        pub broadcaster_type: String
    }
    #[derive(Deserialize)]
    pub(crate) struct UserDataResponse {
        pub data: Vec<UserDataObject>
    }

    #[derive(Debug, Error)]
    pub enum GetUserDataError {
        #[error("Error while executing an HTTP request: {0}")]
        HttpError(reqwest::Error),
        #[error("Error while deserializing an HTTP response: {0}")]
        DeserializeError(reqwest::Error),
        #[error("Response is valid, but had user data empty")]
        ResponseUserEmpty
    }
}