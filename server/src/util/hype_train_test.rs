use std::sync::Arc;

use actix::{Actor, Context, AsyncContext, WrapFuture};
use anyhow::{anyhow, Result};
use rand::Rng;
use time::{OffsetDateTime, Duration, format_description::well_known::Rfc3339};
use twitch_sources_rework::common_data::eventsub_msgs::{EventSubMessage, EventSubData, HypeTrainBegin, HypeTrainData, Contribution, ContributionType, HypeTrainProgress, HypeTrainEnd};
use tokio::time::sleep as async_sleep;

use crate::{my_redis::{RedisClient, publisher::MessagePublisher}, websockets::WEBSOCKET_DATA_TYPES, RunningTests};

use super::get_redis_client_pool;

const TEST_TIME_SECONDS: i64 = 9;

async fn execute_test(user_id: &str) -> Result<()> {
    let conn = RedisClient::new(get_redis_client_pool()?);

    let mut rng = rand::thread_rng();
    // random id to make different tests count as different events
    let id: String = (0..10).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

    let start_time = OffsetDateTime::now_utc();
    let end_time = OffsetDateTime::now_utc() + Duration::seconds(TEST_TIME_SECONDS);

    let begin_hypetrain_message = EventSubMessage {
        data: EventSubData::HypeTrainBegin(HypeTrainBegin {
            data: HypeTrainData {
                id: id.clone(),
                broadcaster_user_id: user_id.to_string(),
                broadcaster_user_login: "cool_user".to_string(),
                broadcaster_user_name: "Cool_User".to_string(),
                total: 500,
                progress: 500,
                goal: 2000,
                top_contributions: vec![
                    Contribution {
                        user_id: "1234".to_string(),
                        user_login: "cool_user".to_string(),
                        user_name: "Cool_User".to_string(),
                        type_: ContributionType::Subscription,
                        total: 500,
                    }
                ],
                last_contribution: Contribution {
                    user_id: "1234".to_string(),
                    user_login: "cool_user".to_string(),
                    user_name: "Cool_User".to_string(),
                    type_: ContributionType::Subscription,
                    total: 500,
                },
                level: 1,
                started_at: start_time.format(&Rfc3339).unwrap(),
            },
            expires_at: end_time.format(&Rfc3339).unwrap(),
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&begin_hypetrain_message).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[1].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(2)).await;

    let progress_hypetrain_message1 = EventSubMessage {
        data: EventSubData::HypeTrainProgress(HypeTrainProgress {
            data: HypeTrainData {
                id: id.clone(),
                broadcaster_user_id: user_id.to_string(),
                broadcaster_user_login: "cool_user".to_string(),
                broadcaster_user_name: "Cool_User".to_string(),
                total: 1500,
                progress: 1500,
                goal: 2000,
                top_contributions: vec![
                    Contribution {
                        user_id: "1234".to_string(),
                        user_login: "cool_user".to_string(),
                        user_name: "Cool_User".to_string(),
                        type_: ContributionType::Subscription,
                        total: 1000,
                    }
                ],
                last_contribution: Contribution {
                    user_id: "1234".to_string(),
                    user_login: "cool_user".to_string(),
                    user_name: "Cool_User".to_string(),
                    type_: ContributionType::Subscription,
                    total: 1000,
                },
                level: 1,
                started_at: start_time.format(&Rfc3339).unwrap(),
            },
            expires_at: end_time.format(&Rfc3339).unwrap(),
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_hypetrain_message1).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[1].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(2)).await;

    let progress_hypetrain_message2 = EventSubMessage {
        data: EventSubData::HypeTrainProgress(HypeTrainProgress {
            data: HypeTrainData {
                id: id.clone(),
                broadcaster_user_id: user_id.to_string(),
                broadcaster_user_login: "cool_user".to_string(),
                broadcaster_user_name: "Cool_User".to_string(),
                total: 2000,
                progress: 0,
                goal: 3000,
                top_contributions: vec![
                    Contribution {
                        user_id: "1234".to_string(),
                        user_login: "cool_user".to_string(),
                        user_name: "Cool_User".to_string(),
                        type_: ContributionType::Subscription,
                        total: 1000,
                    }
                ],
                last_contribution: Contribution {
                    user_id: "1234".to_string(),
                    user_login: "cool_user".to_string(),
                    user_name: "Cool_User".to_string(),
                    type_: ContributionType::Subscription,
                    total: 500,
                },
                level: 2,
                started_at: start_time.format(&Rfc3339).unwrap(),
            },
            expires_at: end_time.format(&Rfc3339).unwrap(),
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_hypetrain_message2).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[1].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(2)).await;

    let progress_hypetrain_message3 = EventSubMessage {
        data: EventSubData::HypeTrainProgress(HypeTrainProgress {
            data: HypeTrainData {
                id: id.clone(),
                broadcaster_user_id: user_id.to_string(),
                broadcaster_user_login: "cool_user".to_string(),
                broadcaster_user_name: "Cool_User".to_string(),
                total: 7000,
                progress: 2000,
                goal: 5000,
                top_contributions: vec![
                    Contribution {
                        user_id: "1234".to_string(),
                        user_login: "cool_user".to_string(),
                        user_name: "Cool_User".to_string(),
                        type_: ContributionType::Subscription,
                        total: 1000,
                    },
                    Contribution {
                        user_id: "1234".to_string(),
                        user_login: "cool_user".to_string(),
                        user_name: "Cool_User".to_string(),
                        type_: ContributionType::Bits,
                        total: 5000,
                    }
                ],
                last_contribution: Contribution {
                    user_id: "1234".to_string(),
                    user_login: "cool_user".to_string(),
                    user_name: "Cool_User".to_string(),
                    type_: ContributionType::Bits,
                    total: 5000,
                },
                level: 3,
                started_at: start_time.format(&Rfc3339).unwrap(),
            },
            expires_at: end_time.format(&Rfc3339).unwrap(),
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_hypetrain_message3).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[1].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(3)).await;
    
    let progress_hypetrain_end = EventSubMessage {
        data: EventSubData::HypeTrainEnd(HypeTrainEnd {
            data: HypeTrainData {
                id: id.clone(),
                broadcaster_user_id: user_id.to_string(),
                broadcaster_user_login: "cool_user".to_string(),
                broadcaster_user_name: "Cool_User".to_string(),
                total: 7000,
                progress: 2000,
                goal: 5000,
                top_contributions: vec![
                    Contribution {
                        user_id: "1234".to_string(),
                        user_login: "cool_user".to_string(),
                        user_name: "Cool_User".to_string(),
                        type_: ContributionType::Subscription,
                        total: 1000,
                    },
                    Contribution {
                        user_id: "1234".to_string(),
                        user_login: "cool_user".to_string(),
                        user_name: "Cool_User".to_string(),
                        type_: ContributionType::Bits,
                        total: 5000,
                    }
                ],
                last_contribution: Contribution {
                    user_id: "1234".to_string(),
                    user_login: "cool_user".to_string(),
                    user_name: "Cool_User".to_string(),
                    type_: ContributionType::Bits,
                    total: 5000,
                },
                level: 3,
                started_at: start_time.format(&Rfc3339).unwrap(),
            },
            ended_at: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
            cooldown_ends_at: (OffsetDateTime::now_utc() + Duration::seconds(30)).format(&Rfc3339).unwrap(),
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_hypetrain_end).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[1].topic, &data).await?;

    Ok(())
}

pub struct HypetrainTestActor {
    running_tests: Arc<RunningTests>,
    key: String,
    user_id: i64
}

impl HypetrainTestActor {
    pub fn new(running_tests: Arc<RunningTests>, user_id: i64) -> Result<Self> {
        let key = user_id.to_string() + ":tests:hype_train";
        if running_tests.contains(&key) {
            return Err(anyhow!("Test is still running"));
        }

        running_tests.insert(key.clone());
        Ok(Self { running_tests, key, user_id })
    }
}

impl Actor for HypetrainTestActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let user_id = self.user_id;
        let key = self.key.clone();
        let running_tests = self.running_tests.clone();

        let fut = Box::pin(

            async move {
                // silently ignore the error
                _ = execute_test(&user_id.to_string()).await;
                running_tests.remove(&key);
            }

        );

        ctx.spawn(fut.into_actor(self));
    }
}

