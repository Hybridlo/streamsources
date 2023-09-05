use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use anyhow::Result;
use futures::future::try_join_all;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use twitch_sources_rework::common_data::SubTypes;

use crate::{twitch_api::{subscribe, SubData}, RedisPool};

use super::{db_subscription, DbError, Repository};


type HmacSha256 = Hmac<Sha256>;

#[derive(Queryable)]
pub struct Subscription {
    pub id: i64,
    pub user_id: Option<i64>,
    pub secret: String,
    pub sub_id: String,
    #[diesel(deserialize_as = String)]
    pub type_: SubTypes,
    pub connected: bool,
    pub inactive_since: time::PrimitiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = db_subscription)]
struct SubscriptionNew {
    user_id: Option<i64>,
    secret: String,
    sub_id: String,
    type_: String,
}

#[async_trait::async_trait(?Send)]
pub trait SubscriptionDb {
    async fn get_subscriptions(&self, sub_types: &[SubTypes], user_id: Option<i64>) -> Result<Vec<Subscription>, DbError>;
    async fn create_subscriptions(&self, new_subs: Vec<SubData>, user_id: Option<i64>) -> Result<Vec<Subscription>, DbError>;
    async fn get_subscription(&self, sub_id: &str) -> Result<Option<Subscription>, DbError>;
    async fn remove_subscription(&self, sub_id: &str) -> Result<(), DbError>;
}

#[async_trait::async_trait(?Send)]
impl SubscriptionDb for Repository {
    async fn get_subscriptions(&self, sub_types: &[SubTypes], user_id: Option<i64>) -> Result<Vec<Subscription>, DbError> {
        let mut db_conn = self.get_conn().await?;

        Ok(match user_id {
            Some(user_id) => db_subscription::dsl::subscription
                .filter(db_subscription::dsl::type_.eq_any(sub_types.iter().map(ToString::to_string)))
                .filter(db_subscription::dsl::user_id.eq(user_id))
                .load::<Subscription>(&mut db_conn)
                .await
                .map_err(|_| DbError::Other)?,

            None => db_subscription::dsl::subscription
                .filter(db_subscription::dsl::type_.eq_any(sub_types.iter().map(ToString::to_string)))
                .load::<Subscription>(&mut db_conn)
                .await
                .map_err(|_| DbError::Other)?,
        })
    }

    async fn create_subscriptions(&self, new_subs: Vec<SubData>, user_id: Option<i64>) -> Result<Vec<Subscription>, DbError> {
        let mut db_conn = self.get_conn().await?;

        let new_subs = new_subs.into_iter().map(|item| SubscriptionNew {
            user_id,
            secret: item.transport.secret.expect("To have the secret"),
            sub_id: item.id,
            type_: item.type_.to_string(),
        }).collect::<Vec<_>>();

        diesel::insert_into(db_subscription::dsl::subscription)
            .values(&new_subs)
            .get_results::<Subscription>(&mut db_conn).await
            .map_err(|_| DbError::Other)
    }
    
    async fn get_subscription(&self, sub_id: &str) -> Result<Option<Subscription>, DbError> {
        let mut db_conn = self.get_conn().await?;

        db_subscription::dsl::subscription
            .filter(db_subscription::dsl::sub_id.eq(sub_id))
            .first::<Subscription>(&mut db_conn).await.optional()
            .map_err(|_| DbError::Other)
    }

    async fn remove_subscription(&self, sub_id: &str) -> Result<(), DbError> {
        let mut db_conn = self.get_conn().await?;

        diesel::delete(db_subscription::dsl::subscription
            .filter(db_subscription::dsl::sub_id.eq(sub_id)))
            .execute(&mut db_conn).await
            .map_err(|_| DbError::Other)?;

        Ok(())
    }
}
