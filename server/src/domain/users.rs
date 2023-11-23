use thiserror::Error;

use crate::{db::{TwitchUserDb, DbError, NewTwitchUser}, http_client::twitch_client::{TwitchHttpClient, GetUserTokenError, GetUserDataError}};

#[derive(Debug)]
pub struct TwitchUser {
    pub id: i64,
    pub username: String,
    pub creation: time::PrimitiveDateTime,
    pub broadcaster_type: String,
    pub scopes: Vec<String>
}

impl TwitchUser {
    pub async fn update_or_create_and_get_user<Ctx: TwitchUserDb + TwitchHttpClient>(
        ctx: &Ctx,
        code: &str,
        host: &str
    ) -> Result<Self, UpdateOrCreateAndGetUserError> {
        let user_access_data = ctx.get_user_token(code, host).await?;
        let user_data = ctx.get_user_data(&user_access_data.access_token).await?;

        let user_id = user_data.id.parse().map_err(|_| UpdateOrCreateAndGetUserError::MalformedUserId)?;

        let user = ctx.get_user(user_id).await.map_err(UpdateOrCreateAndGetUserError::UserGetError)?;

        let user = match user {
            Some(mut user) => {
                user.access_token = user_access_data.access_token;
                user.refresh_token = user_access_data.refresh_token;
                user.username = user_data.login;
                user.scopes = user_access_data.scope.into_iter().map(Some).collect();

                ctx.save_user(&user).await
                    .map_err(UpdateOrCreateAndGetUserError::UserSaveError)?;

                user
            },
            None => {
                let new_user = NewTwitchUser {
                    id: user_id,
                    username: user_data.login,
                    access_token: user_access_data.access_token,
                    refresh_token: user_access_data.refresh_token,
                    expires_in: user_access_data.expires_in,
                    scopes: user_access_data.scope,
                    broadcaster_type: user_data.broadcaster_type,
                };

                ctx.insert_user(new_user)
                    .await
                    .map_err(UpdateOrCreateAndGetUserError::UserInsertError)?
            }
        };

        Ok(user.into())
    }

    pub async fn get_user<Ctx: TwitchUserDb>(ctx: &Ctx, user_id: i64) -> Result<Option<Self>, GetUserError> {
        ctx.get_user(user_id).await.map(|opt_user| opt_user.map(Into::into)).map_err(GetUserError::UserGetError)
    }

    pub async fn delete_user<Ctx: TwitchUserDb>(ctx: &Ctx, user_id: i64) -> Result<(), DeleteUserError> {
        ctx.delete_user(user_id).await.map_err(DeleteUserError::UserDeleteError)
    }
}

#[derive(Debug, Error)]
pub enum UpdateOrCreateAndGetUserError {
    #[error("Getting user token failed: {0}")]
    UserTokenError(#[from] GetUserTokenError),
    #[error("Getting user data failed: {0}")]
    UserDataError(#[from] GetUserDataError),
    #[error("User id was not a valid i64 number")]
    MalformedUserId,
    #[error("Getting user data from DB failed: {0}")]
    UserGetError(DbError),
    #[error("Saving user data in DB failed: {0}")]
    UserSaveError(DbError),
    #[error("Inserting new user in DB failed: {0}")]
    UserInsertError(DbError),
}

#[derive(Debug, Error)]
pub enum GetUserError {
    #[error("Getting user data from DB failed: {0}")]
    UserGetError(DbError)
}

#[derive(Debug, Error)]
pub enum DeleteUserError {
    #[error("Deleting user data from DB failed: {0}")]
    UserDeleteError(DbError)
}

mod db_conv {
    use super::TwitchUser;
    use crate::db::TwitchUser as DbTwitchUser;

    impl From<DbTwitchUser> for TwitchUser {
        fn from(user: DbTwitchUser) -> Self {
            Self {
                id: user.id,
                username: user.username,
                creation: user.creation,
                broadcaster_type: user.broadcaster_type,
                // Option, because weird DB shenanigans, can be unwrapped
                scopes: user.scopes.into_iter().map(Option::unwrap).collect(),
            }
        }
    }
}