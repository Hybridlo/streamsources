use rand::Rng;
use thiserror::Error;
use twitch_sources_rework::common_data::SubType;

#[cfg(not(debug_assertions))]
use crate::PROD_BASE_URL;
use crate::http_client::twitch_client::{SubCondition, SubData, SubscribeRequestError, TwitchHttpClient};

use super::app_token::{TwitchTokenError, TwitchTokenManager};


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

#[derive(Debug, Error)]
pub enum TwitchSubscriptionError {
    #[error("Token fetching/request failed: {0}")]
    TokenError(#[from] TwitchTokenError),
    #[error("Subscription request failed: {0}")]
    RequestError(#[from] SubscribeRequestError)
}

#[async_trait::async_trait(?Send)]
pub trait TwitchSubscriptionManager {
    async fn subscribe(&self, sub_cond: SubCondition, sub_type: SubType) -> Result<SubData, TwitchSubscriptionError>;
}

#[async_trait::async_trait(?Send)]
impl<T: TwitchTokenManager + TwitchHttpClient> TwitchSubscriptionManager for T {
    async fn subscribe(&self, sub_cond: SubCondition, sub_type: SubType) -> Result<SubData, TwitchSubscriptionError> {
        #[cfg(debug_assertions)]
        let callback_url = ngrok::get_https_link().await + "/webhook/";
        #[cfg(not(debug_assertions))]
        let callback_url = PROD_BASE_URL.to_string() + "/webhook/";

        let mut rng = rand::thread_rng();
        let secret: String = (0..SECRET_LENGTH).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

        let token = self.get_app_token().await?;
        let mut sub_data = self.create_subscription(sub_cond, sub_type, &callback_url, &secret, &token).await?;
        sub_data.transport.secret = Some(secret);

        Ok(sub_data)
    }
}
