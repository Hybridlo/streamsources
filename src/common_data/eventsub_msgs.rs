use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize)]
pub struct UserAuthorizationRevoke {
    pub client_id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopPredictior {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub channel_points_won: Option<i64>,
    pub channel_points_used: i64
}

#[derive(Serialize, Deserialize)]
pub struct PredictionsOutcome {
    pub id: String,
    pub title: String,
    pub color: String,
    #[serde(default)]
    pub users: i64,
    #[serde(default)]
    pub channel_points: i64,
    #[serde(default)]
    pub top_predictors: Vec<TopPredictior>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum PredictionEndStatus {
    Resolved,
    Canceled
}

#[derive(Serialize, Deserialize)]
pub struct ChannelPredictionBegin {
    pub id: String,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub outcomes: Vec<PredictionsOutcome>,
    pub started_at: String,
    pub locks_at: String
}

#[derive(Serialize, Deserialize)]
pub struct ChannelPredictionProgress {
    pub id: String,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub outcomes: Vec<PredictionsOutcome>,
    pub started_at: String,
    pub locks_at: String
}

#[derive(Serialize, Deserialize)]
pub struct ChannelPredictionLock {
    pub id: String,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub outcomes: Vec<PredictionsOutcome>,
    pub started_at: String,
    pub locked_at: String
}

#[derive(Serialize, Deserialize)]
pub struct ChannelPredictionEnd {
    pub id: String,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub title: String,
    pub winning_outcome_id: Option<String>,
    pub outcomes: Vec<PredictionsOutcome>,
    pub status: String,
    pub started_at: String,
    pub ended_at: String
}

#[derive(Serialize, Deserialize)]
pub enum EventSubData {
    UserAuthorizationRevoke(UserAuthorizationRevoke),
    ChannelPredictionBegin(ChannelPredictionBegin),
    ChannelPredictionProgress(ChannelPredictionProgress),
    ChannelPredictionLock(ChannelPredictionLock),
    ChannelPredictionEnd(ChannelPredictionEnd),
}

#[derive(Serialize, Deserialize)]
pub struct EventSubMessage {
    pub data: EventSubData,
    pub msg_time: String
}

impl EventSubMessage {
    pub fn new(msg_type: &str, msg_time: &str, data: serde_json::Value) -> Result<EventSubMessage> {
        let res = match msg_type {
            "user.authorization.revoke" => EventSubMessage {
                data: EventSubData::UserAuthorizationRevoke(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            "channel.prediction.begin" => EventSubMessage {
                data: EventSubData::ChannelPredictionBegin(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            "channel.prediction.progress" => EventSubMessage {
                data: EventSubData::ChannelPredictionProgress(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            "channel.prediction.lock" => EventSubMessage {
                data: EventSubData::ChannelPredictionLock(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            "channel.prediction.end" => EventSubMessage {
                data: EventSubData::ChannelPredictionEnd(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            _ => return Err(anyhow!("Invalid eventsub type"))
        };

        Ok(res)
    }
}