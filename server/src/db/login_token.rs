use auto_delegate::delegate;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rand::Rng;

use super::{quick_login_token, Repository, DbError, ResultDb};

const TOKEN_LENGTH: u32 = 20;

#[derive(Queryable)]
pub struct LoginToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String
}

#[derive(Insertable)]
#[diesel(table_name = quick_login_token)]
struct LoginTokenNew {
    user_id: i64,
    token: String
}

#[async_trait::async_trait(?Send)]
#[delegate]
pub trait LoginTokenDb {
    async fn get_login_token(&self, user_id: i64) -> ResultDb<String>;
    async fn create_login_token(&self, user_id: i64) -> ResultDb<String>;
    async fn find_token(&self, token: &str) -> ResultDb<i64>;
}

#[async_trait::async_trait(?Send)]
impl LoginTokenDb for Repository {
    async fn get_login_token(&self, user_id: i64) -> ResultDb<String> {
        let mut db_conn = self.get_conn().await?;

        quick_login_token::dsl::quick_login_token
            .filter(quick_login_token::dsl::user_id.eq(user_id))
            .first::<LoginToken>(&mut db_conn)
            .await
            .map(|login_token| login_token.token)
            .map_err(|_| DbError::Other)
    }

    async fn create_login_token(&self, user_id: i64) -> ResultDb<String> {
        let mut db_conn = self.get_conn().await?;

        let mut rng = rand::thread_rng();
        let login_token: String = (0..TOKEN_LENGTH).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

        diesel::insert_into(quick_login_token::table)
            .values(&LoginTokenNew { user_id, token: login_token.clone()  })
            .execute(&mut db_conn).await
            .map_err(|_| DbError::Other)?;

        Ok(login_token)
    }

    async fn find_token(&self, token: &str) -> ResultDb<i64> {
        let mut db_conn = self.get_conn().await?;

        let login_token_item: LoginToken = quick_login_token::dsl::quick_login_token
            .filter(quick_login_token::dsl::token.eq(token))
            .first::<LoginToken>(&mut db_conn).await
            .map_err(|_| DbError::Other)?;

        Ok(login_token_item.user_id)
    }
}