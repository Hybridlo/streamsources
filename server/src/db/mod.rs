mod schema;
mod auth_state;
mod users;
mod login_token;
mod subscription;

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Object;
use schema::twitch_users;
use schema::auth_state as db_auth_state;
use schema::quick_login_token;
use schema::subscription as db_subscription;

pub use auth_state::AuthStateDb;
pub use users::TwitchUser;
pub use users::NewTwitchUser;
pub use users::TwitchUserDb;
pub use login_token::LoginTokenDb;
pub use subscription::Subscription;
pub use subscription::SubscriptionDb;

use thiserror::Error;

use crate::DbPool;
//use crate::errors::{MyErrors, IntoResultMyErr};

#[derive(Clone)]
pub struct Repository {
    pool: DbPool
}

impl Repository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_conn(&self) -> ResultDb<Object<AsyncPgConnection>> {
        self.pool.get().await.map_err(|_| DbError::Other)
    }
}

impl From<DbPool> for Repository {
    fn from(pool: DbPool) -> Self {
        Self::new(pool)
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Other DB error")]
    Other
}

pub type ResultDb<T> = Result<T, DbError>;

/* impl<T> IntoResultMyErr<T> for DbError {
    fn into_my(res: Result<T, Self>) -> Result<T, MyErrors> {
        res.map_err(|db_err| {
            match db_err {
                DbError::Other => MyErrors::InternalServerError("Other DB error".to_string()),
            }
        })
    }
} */