use thiserror::Error;

use crate::db::LoginTokenDb;

pub struct LoginToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String
}

impl LoginToken {
    pub async fn create_or_get_login_token<Repo: LoginTokenDb>(db: &Repo, user_id: i64) -> Result<String, CreateGetTokenError> {
        if let Ok(res) = db.get_login_token(user_id).await {
            return Ok(res);
        }

        db.create_login_token(user_id).await.map_err(|_| CreateGetTokenError::Fail)
    }

    pub async fn validate_user_token<Repo: LoginTokenDb>(db: &Repo, token: &str) -> Result<i64, ValidateTokenError> {
        db.find_token(token).await.map_err(|_| ValidateTokenError::Fail)
    }
}

#[derive(Debug, Error)]
pub enum CreateGetTokenError {
    #[error("Failed to create or get a login token")]
    Fail
}

#[derive(Debug, Error)]
pub enum ValidateTokenError {
    #[error("Failed to validate login token")]
    Fail
}