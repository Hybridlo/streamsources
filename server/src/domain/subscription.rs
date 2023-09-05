use futures_util::future::try_join_all;
use hmac::{Hmac, Mac as _};
use sha2::Sha256;
use thiserror::Error;
use twitch_sources_rework::common_data::SubTypes;

use crate::{db::SubscriptionDb, twitch_api::subscribe, RedisPool};

pub struct Subscription {
    id: i64,
    user_id: Option<i64>,
    secret: String,
    sub_id: String,
    type_: SubTypes,
    connected: bool,
    inactive_since: time::PrimitiveDateTime
}

type HmacSha256 = Hmac<Sha256>;

impl Subscription {
    pub async fn get_or_create_subscriptions<Repo: SubscriptionDb>(
        db: &Repo,
        sub_types: Vec<SubTypes>,
        user_id: Option<i64>,
        redis_pool: &RedisPool,
        http_client: &reqwest::Client
    ) -> Result<Vec<Self>, GetOrCreateSubs> {
        let existing_subs = db.get_subscriptions(&sub_types, user_id).await
            .map_err(|_| GetOrCreateSubs::Fail)?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Self>>();
        
        let mut new_subs = Vec::new();
        
        for sub_type in sub_types.iter() {
            if !existing_subs.iter().any(|sub| &sub.type_ == sub_type) {
                new_subs.push(subscribe(
                    sub_type,
                    user_id,
                    // TODO: abstract redis actions, maybe even create/delegate traits for context
                    redis_pool.get().await
                        .map_err(|_| GetOrCreateSubs::Fail)?,
                    http_client
                ))
            }
        }

        let new_subs = try_join_all(new_subs).await
            .map_err(|_| GetOrCreateSubs::Fail)?;
        let new_subs = db.create_subscriptions(new_subs, user_id).await
            .map_err(|_| GetOrCreateSubs::Fail)?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Self>>();

        Ok(existing_subs.into_iter().chain(new_subs.into_iter()).collect())
    }

    pub async fn get_subscription<Repo: SubscriptionDb>(db: &Repo, sub_id: &str) -> Result<Self, GetSub> {
        Ok(
            db
                .get_subscription(sub_id)
                .await
                .map_err(|_| GetSub::Fail)?
                .ok_or(GetSub::NotFound)?
                .into()
        )
    }

    pub async fn remove_subscription<Repo: SubscriptionDb>(db: &Repo, sub_id: &str) -> Result<(), RemoveSub> {
        db.remove_subscription(sub_id).await.map_err(|_| RemoveSub::Fail)?;
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

#[derive(Debug, Error)]
pub enum GetOrCreateSubs {
    #[error("Failed to get or create subscriptions")]
    Fail
}

#[derive(Debug, Error)]
pub enum GetSub {
    #[error("Failed to get a subscription")]
    Fail,
    #[error("Subscription does not exist")]
    NotFound
}

#[derive(Debug, Error)]
pub enum RemoveSub {
    #[error("Failed to remove a subscription")]
    Fail
}

mod db_conv {
    use super::Subscription;
    use crate::db::Subscription as DbSubscription;

    impl From<DbSubscription> for Subscription {
        fn from(sub: DbSubscription) -> Self {
            Self {
                id: sub.id,
                user_id: sub.user_id,
                secret: sub.secret,
                sub_id: sub.sub_id,
                type_: sub.type_,
                connected: sub.connected,
                inactive_since: sub.inactive_since,
            }
        }
    }
}