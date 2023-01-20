use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use anyhow::Result;
use rand::Rng;

use super::quick_login_token;

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

impl LoginToken {
    pub async fn create_or_get_login_token(user_id: i64, db_conn: &mut AsyncPgConnection) -> Result<String> {
        // let's try to find existing one first
        if let Ok(login_token) = quick_login_token::dsl::quick_login_token
            .filter(quick_login_token::dsl::user_id.eq(user_id))
            .first::<LoginToken>(db_conn).await {

            return Ok(login_token.token)
        };

        let mut rng = rand::thread_rng();
        let login_token: String = (0..TOKEN_LENGTH).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

        diesel::insert_into(quick_login_token::table)
            .values(&LoginTokenNew { user_id, token: login_token.clone()  })
            .execute(db_conn).await?;

        Ok(login_token)
    }

    pub async fn validate_token(token: &str, db_conn: &mut AsyncPgConnection) -> Result<i64> {
        let login_token_item: LoginToken = quick_login_token::dsl::quick_login_token
            .filter(quick_login_token::dsl::token.eq(token))
            .first::<LoginToken>(db_conn).await?;

        Ok(login_token_item.user_id)
    }
}