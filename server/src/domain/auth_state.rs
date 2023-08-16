use rand::Rng;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::db::AuthStateDb;

const STATE_LENGTH: usize = 20;
pub const STATE_TIMEOUT: time::Duration = time::Duration::seconds(3600);

pub struct AuthState {
    id: i64,
    state: String,
    creation: time::PrimitiveDateTime
}

#[derive(Serialize, Deserialize)]
pub struct InfoData {
    pub redirect_uri: String
}

impl AuthState {
    pub async fn check_state_and_get_data<Repo: AuthStateDb>(db: &Repo, check_state: &str) -> Result<InfoData, CheckStateError> {
        if check_state.len() < STATE_LENGTH {
            return Err(CheckStateError::LoginAttemptInvalid)
        }
        let now = {
            let odt = time::OffsetDateTime::now_utc();
            time::PrimitiveDateTime::new(odt.date(), odt.time())
        };

        let (auth_state, info_state) = check_state.split_at(STATE_LENGTH);

        let state = db.get_state(auth_state).await
            .map_err(|_| CheckStateError::Fail)?;

        match state {
            Some(outdated_state) if now - outdated_state.creation() > STATE_TIMEOUT => {
                Err(CheckStateError::LoginAttemptTimedOut)
            }
            Some(_valid_state) => {
                let info_state = base64::decode_config(info_state, base64::URL_SAFE_NO_PAD)
                    .map_err(|_| CheckStateError::Fail)?;
                let info_data = serde_json::de::from_slice::<InfoData>(&*info_state)
                    .map_err(|_| CheckStateError::Fail)?;
        
                Ok(info_data)
            }
            None => Err(CheckStateError::LoginAttemptNotFound),
        }
    }

    pub async fn get_new_state<Repo: AuthStateDb>(db: &Repo, redirect_uri: &str) -> Result<String, GetNewStateError> {
        let mut rng = rand::thread_rng();
        let state_token: String = (0..STATE_LENGTH).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

        db.save_state(&state_token).await.map_err(|_| GetNewStateError::Fail)?;

        let info_data = InfoData { redirect_uri: redirect_uri.to_string() };
        let info_state = serde_json::ser::to_vec(&info_data)
            .map_err(|_| GetNewStateError::Fail)?;
        let info_state = base64::encode_config(info_state, base64::URL_SAFE_NO_PAD);

        return Ok(state_token + &info_state);
    }
}

#[derive(Debug, Error)]
pub enum CheckStateError {
    #[error("Failed checking state and getting data")]
    Fail,
    #[error("Login attempt invalid, use login with twitch button")]
    LoginAttemptInvalid,
    #[error("Login attempt has timed out, please try again")]
    LoginAttemptTimedOut,
    #[error("Login attempt was not found, make sure your login attempt came from this website")]
    LoginAttemptNotFound
}

#[derive(Debug, Error)]
pub enum GetNewStateError {
    #[error("Failed getting new state")]
    Fail
}