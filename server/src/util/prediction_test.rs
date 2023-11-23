use std::sync::Arc;

use anyhow::{Result, anyhow};
use actix::{Actor, Context, WrapFuture, AsyncContext};
use rand::Rng;
use time::{OffsetDateTime, Duration, format_description::well_known::Rfc3339};
use twitch_sources_rework::common_data::eventsub_msgs::{EventSubMessage, EventSubData, PredictionsOutcome, TopPredictior, ChannelPredictionBegin, ChannelPredictionProgress, ChannelPredictionLock, ChannelPredictionEnd};
use tokio::time::sleep as async_sleep;

use crate::{RunningTests, my_redis::{RedisClient, publisher::MessagePublisher}, websockets::WEBSOCKET_DATA_TYPES};

use super::get_redis_client_pool;

const TEST_TIME_SECONDS: i64 = 13;

async fn execute_test(user_id: &str) -> Result<()> {
    let conn = RedisClient::new(get_redis_client_pool()?);

    let title = "Some decently long title, just to make sure nothing breaks and stuff, and just a bit more".to_string();
    let option1 = "Somewhat a long option".to_string();
    let option2 = "Short option".to_string();
    let option3 = "But there's more!".to_string();
    let option4 = "Another one".to_string();
    
    let mut rng = rand::thread_rng();
    // they are just numeric usually, but this is good enough to not trigger
    // same id tests, so that different tests are recognized as different
    // prediction events
    let id: String = (0..10).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

    let start_time = OffsetDateTime::now_utc();
    let lock_time = OffsetDateTime::now_utc() + Duration::seconds(TEST_TIME_SECONDS - 2);
    let end_time = OffsetDateTime::now_utc() + Duration::seconds(TEST_TIME_SECONDS);

    let begin_msg = EventSubMessage {
        data: EventSubData::ChannelPredictionBegin(ChannelPredictionBegin {
            id: id.clone(),
            broadcaster_user_id: user_id.to_string(),
            broadcaster_user_login: "cool_user".to_string(),
            broadcaster_user_name: "Cool_User".to_string(),
            title: title.clone(),
            outcomes: vec![
                PredictionsOutcome {
                    id: "1243456".to_string(),
                    title: option1.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
                PredictionsOutcome {
                    id: "2243456".to_string(),
                    title: option2.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
                PredictionsOutcome {
                    id: "3243456".to_string(),
                    title: option3.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
                PredictionsOutcome {
                    id: "4243456".to_string(),
                    title: option4.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
            ],
            started_at: start_time.format(&Rfc3339).unwrap(),
            locks_at: lock_time.format(&Rfc3339).unwrap()
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&begin_msg).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[0].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(1)).await;

    let progress_msg1 = EventSubMessage {
        data: EventSubData::ChannelPredictionProgress(ChannelPredictionProgress {
            id: id.clone(),
            broadcaster_user_id: user_id.to_string(),
            broadcaster_user_login: "cool_user".to_string(),
            broadcaster_user_name: "Cool_User".to_string(),
            title: title.clone(),
            outcomes: vec![
                PredictionsOutcome {
                    id: "1243456".to_string(),
                    title: option1.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 1000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234".to_string(),
                            user_login: "cool_user".to_string(),
                            user_name: "Cool_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 1000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "2243456".to_string(),
                    title: option2.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
                PredictionsOutcome {
                    id: "3243456".to_string(),
                    title: option3.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
                PredictionsOutcome {
                    id: "4243456".to_string(),
                    title: option4.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
            ],
            started_at: start_time.format(&Rfc3339).unwrap(),
            locks_at: lock_time.format(&Rfc3339).unwrap()
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_msg1).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[0].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(3)).await;

    let progress_msg2 = EventSubMessage {
        data: EventSubData::ChannelPredictionProgress(ChannelPredictionProgress {
            id: id.clone(),
            broadcaster_user_id: user_id.to_string(),
            broadcaster_user_login: "cool_user".to_string(),
            broadcaster_user_name: "Cool_User".to_string(),
            title: title.clone(),
            outcomes: vec![
                PredictionsOutcome {
                    id: "1243456".to_string(),
                    title: option1.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 1000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234".to_string(),
                            user_login: "cool_user".to_string(),
                            user_name: "Cool_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 1000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "2243456".to_string(),
                    title: option2.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 2000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "12345".to_string(),
                            user_login: "cooler_user".to_string(),
                            user_name: "Cooler_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 2000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "3243456".to_string(),
                    title: option3.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
                PredictionsOutcome {
                    id: "4243456".to_string(),
                    title: option4.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
            ],
            started_at: start_time.format(&Rfc3339).unwrap(),
            locks_at: lock_time.format(&Rfc3339).unwrap()
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_msg2).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[0].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(2)).await;

    let progress_msg3 = EventSubMessage {
        data: EventSubData::ChannelPredictionProgress(ChannelPredictionProgress {
            id: id.clone(),
            broadcaster_user_id: user_id.to_string(),
            broadcaster_user_login: "cool_user".to_string(),
            broadcaster_user_name: "Cool_User".to_string(),
            title: title.clone(),
            outcomes: vec![
                PredictionsOutcome {
                    id: "1243456".to_string(),
                    title: option1.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 1000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234".to_string(),
                            user_login: "cool_user".to_string(),
                            user_name: "Cool_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 1000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "2243456".to_string(),
                    title: option2.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 2000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "12345".to_string(),
                            user_login: "cooler_user".to_string(),
                            user_name: "Cooler_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 2000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "3243456".to_string(),
                    title: option3.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 3000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "123456".to_string(),
                            user_login: "coolest_user".to_string(),
                            user_name: "Coolest_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 3000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "4243456".to_string(),
                    title: option4.clone(),
                    color: "blue".to_string(),
                    users: 0,
                    channel_points: 0,
                    top_predictors: vec![]
                },
            ],
            started_at: start_time.format(&Rfc3339).unwrap(),
            locks_at: lock_time.format(&Rfc3339).unwrap()
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_msg3).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[0].topic, &data).await?;
    async_sleep(std::time::Duration::from_secs(2)).await;

    let progress_msg4 = EventSubMessage {
        data: EventSubData::ChannelPredictionProgress(ChannelPredictionProgress {
            id: id.clone(),
            broadcaster_user_id: user_id.to_string(),
            broadcaster_user_login: "cool_user".to_string(),
            broadcaster_user_name: "Cool_User".to_string(),
            title: title.clone(),
            outcomes: vec![
                PredictionsOutcome {
                    id: "1243456".to_string(),
                    title: option1.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 1000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234".to_string(),
                            user_login: "cool_user".to_string(),
                            user_name: "Cool_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 1000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "2243456".to_string(),
                    title: option2.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 2000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "12345".to_string(),
                            user_login: "cooler_user".to_string(),
                            user_name: "Cooler_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 2000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "3243456".to_string(),
                    title: option3.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 3000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "123456".to_string(),
                            user_login: "coolest_user".to_string(),
                            user_name: "Coolest_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 3000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "4243456".to_string(),
                    title: option4.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 4000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234567".to_string(),
                            user_login: "coolio_user".to_string(),
                            user_name: "Coolio_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 4000
                        }
                    ]
                },
            ],
            started_at: start_time.format(&Rfc3339).unwrap(),
            locks_at: lock_time.format(&Rfc3339).unwrap()
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&progress_msg4).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[0].topic, &data).await?;
    async_sleep((lock_time - OffsetDateTime::now_utc()).try_into().unwrap()).await;

    let lock_msg = EventSubMessage {
        data: EventSubData::ChannelPredictionLock(ChannelPredictionLock {
            id: id.clone(),
            broadcaster_user_id: user_id.to_string(),
            broadcaster_user_login: "cool_user".to_string(),
            broadcaster_user_name: "Cool_User".to_string(),
            title: title.clone(),
            outcomes: vec![
                PredictionsOutcome {
                    id: "1243456".to_string(),
                    title: option1.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 1000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234".to_string(),
                            user_login: "cool_user".to_string(),
                            user_name: "Cool_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 1000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "2243456".to_string(),
                    title: option2.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 2000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "12345".to_string(),
                            user_login: "cooler_user".to_string(),
                            user_name: "Cooler_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 2000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "3243456".to_string(),
                    title: option3.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 3000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "123456".to_string(),
                            user_login: "coolest_user".to_string(),
                            user_name: "Coolest_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 3000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "4243456".to_string(),
                    title: option4.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 4000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234567".to_string(),
                            user_login: "coolio_user".to_string(),
                            user_name: "Coolio_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 4000
                        }
                    ]
                },
            ],
            started_at: start_time.format(&Rfc3339).unwrap(),
            locked_at: lock_time.format(&Rfc3339).unwrap()
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&lock_msg).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[0].topic, &data).await?;
    async_sleep((end_time - OffsetDateTime::now_utc()).try_into().unwrap()).await;

    let end_msg = EventSubMessage {
        data: EventSubData::ChannelPredictionEnd(ChannelPredictionEnd {
            id: id.clone(),
            broadcaster_user_id: user_id.to_string(),
            broadcaster_user_login: "cool_user".to_string(),
            broadcaster_user_name: "Cool_User".to_string(),
            title: title.clone(),
            outcomes: vec![
                PredictionsOutcome {
                    id: "1243456".to_string(),
                    title: option1.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 1000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234".to_string(),
                            user_login: "cool_user".to_string(),
                            user_name: "Cool_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 1000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "2243456".to_string(),
                    title: option2.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 2000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "12345".to_string(),
                            user_login: "cooler_user".to_string(),
                            user_name: "Cooler_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 2000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "3243456".to_string(),
                    title: option3.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 3000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "123456".to_string(),
                            user_login: "coolest_user".to_string(),
                            user_name: "Coolest_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 3000
                        }
                    ]
                },
                PredictionsOutcome {
                    id: "4243456".to_string(),
                    title: option4.clone(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 4000,
                    top_predictors: vec![
                        TopPredictior {
                            user_id: "1234567".to_string(),
                            user_login: "coolio_user".to_string(),
                            user_name: "Coolio_User".to_string(),
                            channel_points_won: None,
                            channel_points_used: 4000
                        }
                    ]
                },
            ],
            started_at: start_time.format(&Rfc3339).unwrap(),
            ended_at: end_time.format(&Rfc3339).unwrap(),
            winning_outcome_id: Some("1243456".to_string()),
            status: "resolved".to_string(),
        }),
        msg_time: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
    };
    let data = serde_json::ser::to_vec(&end_msg).expect("No way we fail serialization");
    conn.publish_message(user_id, WEBSOCKET_DATA_TYPES[0].topic, &data).await?;

    Ok(())
}

pub struct PredictionsTestActor {
    running_tests: Arc<RunningTests>,
    key: String,
    user_id: i64
}

impl PredictionsTestActor {
    pub fn new(running_tests: Arc<RunningTests>, user_id: i64) -> Result<Self> {
        let key = user_id.to_string() + ":tests:predictions";
        if running_tests.contains(&key) {
            return Err(anyhow!("Test is still running"));
        }

        running_tests.insert(key.clone());
        Ok(Self { running_tests, key, user_id })
    }
}

impl Actor for PredictionsTestActor {
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

