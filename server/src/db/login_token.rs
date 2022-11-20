use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use anyhow::{Result, anyhow};
use rand::Rng;

use super::quick_login_token;

const TOKEN_TIMEOUT: i64 = 300;
const TOKEN_LENGTH: u32 = 20;

#[derive(Queryable)]
pub struct LoginToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub creation: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = quick_login_token)]
struct LoginTokenNew {
    user_id: i64,
    token: String
}

impl LoginToken {
    pub async fn create_login_token(user_id: i64, db_conn: &mut AsyncPgConnection) -> Result<String> {
        let mut rng = rand::thread_rng();
        let login_token: String = (0..TOKEN_LENGTH).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

        diesel::delete(quick_login_token::table)
            .filter(quick_login_token::dsl::creation.lt(chrono::offset::Utc::now().naive_utc() - chrono::Duration::seconds(TOKEN_TIMEOUT)))
            .execute(db_conn).await?;

        diesel::insert_into(quick_login_token::table)
            .values(&LoginTokenNew { user_id, token: login_token.clone()  })
            .execute(db_conn).await?;

        Ok(login_token)
    }

    pub async fn validate_token(token: &str, db_conn: &mut AsyncPgConnection) -> Result<i64> {
        let login_token_item: Option<LoginToken> = quick_login_token::dsl::quick_login_token
            .filter(quick_login_token::dsl::token.eq(token))
            .first::<LoginToken>(db_conn).await.optional()?;

        match login_token_item {
            Some(login_token_item_value) => {
                if chrono::offset::Utc::now().naive_utc() - login_token_item_value.creation > chrono::Duration::seconds(TOKEN_TIMEOUT) {
                    Ok(login_token_item_value.user_id)
                } else {
                    Err(anyhow!("Token timed out"))
                }
            },
            None => Err(anyhow!("Token does not exist")),
        }
    }
}