use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use anyhow::Result;
use futures::future::try_join_all;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use twitch_sources_rework::common_data::SubTypes;

use crate::{twitch_api::subscribe, RedisPool};

use super::db_subscription;


type HmacSha256 = Hmac<Sha256>;

#[derive(Queryable)]
pub struct Subscription {
    id: i64,
    user_id: Option<i64>,
    secret: String,
    sub_id: String,
    #[diesel(deserialize_as = String)]
    type_: SubTypes,
    connected: bool,
    inactive_since: time::PrimitiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = db_subscription)]
struct SubscriptionNew {
    user_id: Option<i64>,
    secret: String,
    sub_id: String,
    type_: String,
}

impl Subscription {
    pub async fn get_or_create_subscriptions(
        sub_types: Vec<SubTypes>,
        user_id: Option<i64>,
        db_conn: &mut AsyncPgConnection,
        redis_pool: &RedisPool,
        http_client: &reqwest::Client
    ) -> Result<Vec<Subscription>> {

        let existing_subs: Vec<Subscription> = match user_id {
            Some(user_id) => db_subscription::dsl::subscription
                .filter(db_subscription::dsl::type_.eq_any(sub_types.iter().map(ToString::to_string)))
                .filter(db_subscription::dsl::user_id.eq(user_id))
                .load::<Subscription>(db_conn)
                .await?
            ,
            None => db_subscription::dsl::subscription
                .filter(db_subscription::dsl::type_.eq_any(sub_types.iter().map(ToString::to_string)))
                .load::<Subscription>(db_conn)
                .await?
            ,
        };
        
        let mut new_subs = Vec::new();
        
        for sub_type in sub_types.iter() {
            if !existing_subs.iter().any(|sub| &sub.type_ == sub_type) {
                new_subs.push(subscribe(sub_type, user_id, redis_pool.get().await?, http_client))
            }
        }

        let new_subs = try_join_all(new_subs).await?;
        let new_subs = new_subs.into_iter().map(|item| SubscriptionNew {
            user_id,
            secret: item.transport.secret.expect("To have the secret"),
            sub_id: item.id,
            type_: item.type_.to_string(),
        }).collect::<Vec<_>>();

        let new_subs: Vec<Subscription> = diesel::insert_into(db_subscription::dsl::subscription)
            .values(&new_subs)
            .get_results::<Subscription>(db_conn).await?;

        Ok(existing_subs.into_iter().chain(new_subs.into_iter()).collect())
    }

    pub async fn get_subscription(sub_id: &str, db_conn: &mut AsyncPgConnection) -> Result<Option<Subscription>> {
        Ok(db_subscription::dsl::subscription
            .filter(db_subscription::dsl::sub_id.eq(sub_id))
            .first::<Subscription>(db_conn).await.optional()?)
    }

    pub async fn remove_subscription(sub_id: &str, db_conn: &mut AsyncPgConnection) -> Result<()> {
        diesel::delete(db_subscription::dsl::subscription
            .filter(db_subscription::dsl::sub_id.eq(sub_id)))
            .execute(db_conn).await?;

        Ok(())
    }

    pub fn verify_msg(&self, msg: &[u8], expected_signature: &[u8]) -> bool {
        let mut hasher = HmacSha256::new_from_slice(self.secret.as_bytes()).expect("HMAC can take key of any size");

        hasher.update(msg);
        let res_hash = [
            b"sha256=",
            hex::encode(&*hasher.finalize().into_bytes()).as_bytes()
        ].concat();

        return res_hash == expected_signature
    }
}