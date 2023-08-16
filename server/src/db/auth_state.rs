use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::domain::auth_state::STATE_TIMEOUT;

use super::{db_auth_state, DbError, Repository};

#[derive(Queryable)]
pub struct AuthState {
    id: i64,
    state: String,
    creation: time::PrimitiveDateTime
}

impl AuthState {
    pub fn creation(&self) -> time::PrimitiveDateTime {
        self.creation
    }
}

#[derive(Insertable)]
#[diesel(table_name = db_auth_state)]
struct AuthStateNew {
    state: String
}

#[async_trait::async_trait(?Send)]
pub trait AuthStateDb {
    async fn get_state(&self, auth_state: &str) -> Result<Option<AuthState>, DbError>;
    async fn save_state(&self, state_token: &str) -> Result<(), DbError>;
}

#[async_trait::async_trait(?Send)]
impl AuthStateDb for Repository {
    async fn get_state(&self, auth_state: &str) -> Result<Option<AuthState>, DbError> {
        let mut db_conn = self.get_conn().await?;

        db_auth_state::dsl::auth_state
            .filter(db_auth_state::dsl::state.eq(auth_state))
            .first::<AuthState>(&mut db_conn).await.optional()
            .map_err(|_| DbError::Other)
    }

    async fn save_state(&self, state_token: &str) -> Result<(), DbError> {
        let mut db_conn = self.get_conn().await?;
        let now = {
            let odt = time::OffsetDateTime::now_utc();
            time::PrimitiveDateTime::new(odt.date(), odt.time())
        };

        diesel::delete(db_auth_state::table)
            .filter(db_auth_state::dsl::creation.lt(now - STATE_TIMEOUT))
            .execute(&mut db_conn).await
            .map_err(|_| DbError::Other)?;

        diesel::insert_into(db_auth_state::table)
            .values(&AuthStateNew { state: state_token.to_string() })
            .execute(&mut db_conn).await
            .map_err(|_| DbError::Other)?;

        Ok(())
    }
}
