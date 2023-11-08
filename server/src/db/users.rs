use auto_delegate::delegate;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, SaveChangesDsl};

use super::{twitch_users, ResultDb, Repository, DbError};

#[derive(Insertable)]
#[diesel(table_name = twitch_users)]
pub struct NewTwitchUser {
    pub id: i64,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
    pub scopes: Vec<String>,
    pub broadcaster_type: String
}

#[derive(Clone, Queryable, Identifiable, AsChangeset, Debug)]
pub struct TwitchUser {
    pub id: i64,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub creation: time::PrimitiveDateTime,
    pub last_login: time::PrimitiveDateTime,
    pub expires_in: i32,
    pub scopes: Vec<Option<String>>,
    pub broadcaster_type: String
}

#[async_trait::async_trait]
#[delegate]
pub trait TwitchUserDb {
    async fn get_user(&self, user_id: i64) -> ResultDb<Option<TwitchUser>>;
    async fn save_user(&self, new_user: &TwitchUser) -> ResultDb<()>;
    async fn insert_user(&self, user: NewTwitchUser) -> ResultDb<TwitchUser>;
    async fn delete_user(&self, user_id: i64) -> ResultDb<()>;
}

#[async_trait::async_trait]
impl TwitchUserDb for Repository {
    async fn get_user(&self, user_id: i64) -> ResultDb<Option<TwitchUser>> {
        let mut db_conn = self.get_conn().await?;

        twitch_users::dsl::twitch_users
            .filter(twitch_users::dsl::id.eq(user_id))
            .first::<TwitchUser>(&mut db_conn)
            .await
            .optional()
            .map_err(|_| DbError::Other)
    }

    async fn save_user(&self, user: &TwitchUser) -> ResultDb<()> {
        let mut db_conn = self.get_conn().await?;

        let _: TwitchUser = user.save_changes(&mut db_conn).await.map_err(|_| DbError::Other)?;

        Ok(())
    }

    async fn insert_user(&self, new_user: NewTwitchUser) -> ResultDb<TwitchUser> {
        let mut db_conn = self.get_conn().await?;

        diesel::insert_into(twitch_users::table)
            .values(&new_user)
            .get_result::<TwitchUser>(&mut db_conn)
            .await
            .map_err(|_| DbError::Other)
    }

    async fn delete_user(&self, user_id: i64) -> ResultDb<()> {
        let mut db_conn = self.get_conn().await?;
        
        diesel::delete(twitch_users::table.filter(twitch_users::dsl::id.eq(user_id)))
            .execute(&mut db_conn)
            .await
            .map_err(|_| DbError::Other)?;

        Ok(())
    }
}
