use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use anyhow::{Result, anyhow};
use rand::Rng;
use serde::{Serialize, Deserialize};

use super::auth_state;

const STATE_LENGTH: usize = 20;
const STATE_TIMEOUT_SECONDS: i64 = 3600;

#[derive(Queryable)]
pub struct AuthState {
    id: i64,
    state: String,
    creation: chrono::NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = auth_state)]
struct AuthStateNew {
    state: String
}

#[derive(Serialize, Deserialize)]
struct InfoData {
    pub redirect_uri: String
}

impl AuthState {
    pub async fn check_state_and_get_data(check_state: &str, db_conn: &mut AsyncPgConnection) -> Result<InfoData> {
        if check_state.len() < STATE_LENGTH {
            return Err(anyhow!("Invalid state"))
        }

        let (auth_state, info_state) = check_state.split_at(STATE_LENGTH);

        let state: Option<AuthState> = auth_state::dsl::auth_state
            .filter(auth_state::dsl::state.eq(auth_state))
            .first::<AuthState>(db_conn).await.optional()?;

        match state {
            Some(state) => if (chrono::offset::Utc::now().naive_utc() - state.creation) > chrono::Duration::seconds(STATE_TIMEOUT_SECONDS) {
                return Err(anyhow!("State has timed out"));
            }
            None => return Err(anyhow!("State was not found")),
        }

        let info_state = base64::decode_config(info_state, base64::URL_SAFE_NO_PAD)?;
        let info_data = serde_json::de::from_slice::<InfoData>(&*info_state)?;

        Ok(info_data)
    }

    pub async fn get_new_state(redirect_uri: &str, db_conn: &mut AsyncPgConnection) -> Result<String> {
        let mut rng = rand::thread_rng();
        let state_token: String = (0..STATE_LENGTH).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

        diesel::insert_into(auth_state::table)
            .values(&AuthStateNew { state: state_token.clone() })
            .execute(db_conn).await?;

        let info_data = InfoData { redirect_uri: redirect_uri.to_string() };
        let info_state = serde_json::ser::to_vec(&info_data)?;
        let info_state = base64::encode_config(info_state, base64::URL_SAFE_NO_PAD);

        return Ok(state_token + &info_state);
    }
}