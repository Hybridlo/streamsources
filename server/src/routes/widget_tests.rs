use actix::Actor;
use actix_web::web::{Query, Json, Data};
use paperclip::actix::{Apiv2Schema, api_v2_operation};
use serde::Deserialize;
use crate::{RedisPool, RunningTests};
use crate::util::{session_state::TypedSession, PredictionsTestActor};
use crate::errors::MyErrors;

#[api_v2_operation]
pub async fn execute_test(
    session: TypedSession,
    tests_set: Data<RunningTests>,
    query: Query<TestWrap>
) -> Result<Json<()>, MyErrors> {
    match query.test {
        AvaliableTests::Predictions => {
            PredictionsTestActor::new(
                (*tests_set).clone(),
                session.get_user_id()?.ok_or(MyErrors::AccessDenied)?
            )
            .map_err(|_| MyErrors::AccessDenied)?
            .start();
        },
    }

    Ok(Json(()))
}

#[derive(Deserialize, Apiv2Schema)]
#[serde(rename_all="snake_case")]
pub enum AvaliableTests {
    Predictions
}

#[derive(Deserialize, Apiv2Schema)]
pub struct TestWrap {
    test: AvaliableTests
}