use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct UserAuthorizationRevoke {
    pub client_id: String,
    pub user_id: String,
    pub user_login: Option<String>,
    pub user_name: Option<String>,
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

impl ChannelPredictionEnd {
    pub fn get_winning_outcome_index(&self) -> Option<usize> {
        if let Some(win_id) = &self.winning_outcome_id {
            return self.outcomes.iter().position(|item| &item.id == win_id);
        }

        None
    }
}

// the wrapper for the actual data
#[derive(Serialize, Deserialize)]
pub enum EventSubData {
    UserAuthorizationRevoke(UserAuthorizationRevoke),
    ChannelPredictionBegin(ChannelPredictionBegin),
    ChannelPredictionProgress(ChannelPredictionProgress),
    ChannelPredictionLock(ChannelPredictionLock),
    ChannelPredictionEnd(ChannelPredictionEnd),
}

// discriminator for supported types
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SubTypes {
    #[serde(rename="user.authorization.revoke")]
    UserAuthorizationRevoke,
    #[serde(rename="channel.prediction.begin")]
    ChannelPredictionBegin,
    #[serde(rename="channel.prediction.progress")]
    ChannelPredictionProgress,
    #[serde(rename="channel.prediction.lock")]
    ChannelPredictionLock,
    #[serde(rename="channel.prediction.end")]
    ChannelPredictionEnd
}

impl ToString for SubTypes {
    fn to_string(&self) -> String {
        serde_json::ser::to_string(self).expect("Serialization shouldn't fail")
    }
}

impl TryFrom<String> for SubTypes {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Ok(serde_json::de::from_str(&value)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct EventSubMessage {
    pub data: EventSubData,
    pub msg_time: String
}

impl EventSubMessage {
    pub fn new(msg_type: &SubTypes, msg_time: &str, data: serde_json::Value) -> Result<EventSubMessage> {
        let res = match msg_type {
            SubTypes::UserAuthorizationRevoke => EventSubMessage {
                data: EventSubData::UserAuthorizationRevoke(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            SubTypes::ChannelPredictionBegin => EventSubMessage {
                data: EventSubData::ChannelPredictionBegin(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            SubTypes::ChannelPredictionProgress => EventSubMessage {
                data: EventSubData::ChannelPredictionProgress(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            SubTypes::ChannelPredictionLock => EventSubMessage {
                data: EventSubData::ChannelPredictionLock(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
            SubTypes::ChannelPredictionEnd => EventSubMessage {
                data: EventSubData::ChannelPredictionEnd(serde_json::de::from_str(&data.to_string())?),
                msg_time: msg_time.to_string()
            },
        };

        Ok(res)
    }
}

// making sure we can parse this stuff correctly, it will be big... maybe should be in a seperate file(s)?
#[cfg(test)]
mod tests {
    use super::{EventSubMessage, SubTypes, EventSubData};

    #[test]
    fn test_message_serialization_user_authorization_revoke() {
        let json_val = serde_json::json!({
            "client_id": "crq72vsaoijkc83xx42hz6i37",
            "user_id": "1337",
            "user_login": "cool_user",
            "user_name": "Cool_User"
        });

        EventSubMessage::new(&SubTypes::UserAuthorizationRevoke, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_begin() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {"id": "1243456", "title": "Yeah!", "color": "blue"},
                {"id": "2243456", "title": "No!", "color": "pink"}
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locks_at": "2020-07-15T17:21:03.17106713Z"
        });

        EventSubMessage::new(&SubTypes::ChannelPredictionBegin, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_progress_normal() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {
                    "id": "1243456",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 10,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 200
                        }
                    ]
                },
                {
                    "id": "2243456",
                    "title": "No!",
                    "color": "pink",
                    "users": 2,
                    "channel_points": 10000,
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 5000
                        }
                    ]
                }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locks_at": "2020-07-15T17:21:03.17106713Z"
        });

        EventSubMessage::new(&SubTypes::ChannelPredictionProgress, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_progress_empty_outcome1() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {
                    "id": "1243456",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 10,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 200
                        }
                    ]
                },
                {
                    "id": "2243456",
                    "title": "No!",
                    "color": "pink",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locks_at": "2020-07-15T17:21:03.17106713Z"
        });

        EventSubMessage::new(&SubTypes::ChannelPredictionProgress, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_progress_empty_outcome2() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {
                    "id": "1243456",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                },
                {
                    "id": "2243456",
                    "title": "No!",
                    "color": "pink",
                    "users": 2,
                    "channel_points": 10000,
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 5000
                        }
                    ]
                }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locks_at": "2020-07-15T17:21:03.17106713Z"
        });

        EventSubMessage::new(&SubTypes::ChannelPredictionProgress, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_lock_normal() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {
                    "id": "1243456",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 10,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 200
                        }
                    ]
                },
                {
                    "id": "2243456",
                    "title": "No!",
                    "color": "pink",
                    "users": 2,
                    "channel_points": 10000,
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 5000
                        }
                    ]
                }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locked_at": "2020-07-15T17:21:03.17106713Z"
        });

        EventSubMessage::new(&SubTypes::ChannelPredictionLock, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_lock_empty_outcome1() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {
                    "id": "1243456",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 10,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 200
                        }
                    ]
                },
                {
                    "id": "2243456",
                    "title": "No!",
                    "color": "pink",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locked_at": "2020-07-15T17:21:03.17106713Z"
        });

        EventSubMessage::new(&SubTypes::ChannelPredictionLock, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_lock_empty_outcome2() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "outcomes": [
                {
                    "id": "1243456",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                },
                {
                    "id": "2243456",
                    "title": "No!",
                    "color": "pink",
                    "users": 2,
                    "channel_points": 10000,
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 5000
                        }
                    ]
                }
            ],
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locked_at": "2020-07-15T17:21:03.17106713Z"
        });

        EventSubMessage::new(&SubTypes::ChannelPredictionLock, "", json_val).unwrap();
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_normal_win1() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "12345",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 2,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": 10000,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": 5000,
                            "channel_points_used": 100
                        }
                    ]
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 2,
                    "channel_points": 200,
                    "color": "pink",
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        },
                        {
                            "user_name": "Elite_User",
                            "user_login": "elite_user",
                            "user_id": "1337",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        }
                    ]
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), Some(0));
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_empty_outcome1_win1() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "12345",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 2,
                    "channel_points": 200,
                    "color": "pink",
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        },
                        {
                            "user_name": "Elite_User",
                            "user_login": "elite_user",
                            "user_id": "1337",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        }
                    ]
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), Some(0));
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_empty_outcome2_win1() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "12345",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 2,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": 10000,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": 5000,
                            "channel_points_used": 100
                        }
                    ]
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 0,
                    "channel_points": 0,
                    "color": "pink",
                    "top_predictors": []
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), Some(0));
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_normal_win2() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "22435",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 2,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        }
                    ]
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 2,
                    "channel_points": 200,
                    "color": "pink",
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": 7500,
                            "channel_points_used": 100
                        },
                        {
                            "user_name": "Elite_User",
                            "user_login": "elite_user",
                            "user_id": "1337",
                            "channel_points_won": 7500,
                            "channel_points_used": 100
                        }
                    ]
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), Some(1));
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_empty_outcome1_win2() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "22435",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 2,
                    "channel_points": 200,
                    "color": "pink",
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": 7500,
                            "channel_points_used": 100
                        },
                        {
                            "user_name": "Elite_User",
                            "user_login": "elite_user",
                            "user_id": "1337",
                            "channel_points_won": 7500,
                            "channel_points_used": 100
                        }
                    ]
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), Some(1));
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_empty_outcome2_win2() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "22435",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 2,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        }
                    ]
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 0,
                    "channel_points": 0,
                    "color": "pink",
                    "top_predictors": []
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), Some(1));
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_empty_outcome_both_win1() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "12345",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 0,
                    "channel_points": 0,
                    "color": "pink",
                    "top_predictors": []
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), Some(0));
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_empty_outcome_both_refund() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": null,
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 0,
                    "channel_points": 0,
                    "top_predictors": []
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 0,
                    "channel_points": 0,
                    "color": "pink",
                    "top_predictors": []
                }
            ],
            "status": "canceled",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), None);
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }

    #[test]
    fn test_message_serialization_channel_prediction_end_normal_refund() {
        let json_val = serde_json::json!({
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": null,
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!",
                    "color": "blue",
                    "users": 2,
                    "channel_points": 15000,
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": null,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        }
                    ]
                },
                {
                    "id": "22435",
                    "title": "No!",
                    "users": 2,
                    "channel_points": 200,
                    "color": "pink",
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        },
                        {
                            "user_name": "Elite_User",
                            "user_login": "elite_user",
                            "user_id": "1337",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        }
                    ]
                }
            ],
            "status": "canceled",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        });

        let res = EventSubMessage::new(&SubTypes::ChannelPredictionEnd, "", json_val).unwrap();
        if let EventSubData::ChannelPredictionEnd(data) = res.data {
            assert_eq!(data.get_winning_outcome_index(), None);
        } else {
            panic!("Somehow successfully parsed wrong type");
        }
    }
}