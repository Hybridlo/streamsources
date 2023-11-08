use std::future::{Ready, ready};

use actix_web::FromRequest;
use auto_delegate::Delegate;
use paperclip::{actix::OperationModifier, v2::schema::TypedData};

use crate::{errors::MyErrors, db::Repository, http_client::HttpClient, my_redis::RedisClient};

#[derive(Clone, Delegate)]
pub struct Context {
    #[to(AuthStateDb, LoginTokenDb, SubscriptionDb, TwitchUserDb)]
    pub repository: Repository,
    #[to(TokenCache, MessagePublisher)]
    pub redis: RedisClient,
    #[to(TwitchHttpClient)]
    pub http_client: HttpClient,
}

impl Context {
    pub fn new() -> Self {
        Self {
            repository: super::create_connection_pool().expect("Unable to create DB pool").into(),
            redis: super::get_redis_client_pool().expect("Unable to connect to Redis").into(),
            http_client: HttpClient(reqwest::Client::new()),
        }
    }
}

impl FromRequest for Context {
    type Error = MyErrors;
    type Future = Ready<Result<Self, MyErrors>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(ctx) = req.app_data::<Self>() {
            return ready(Ok(ctx.clone()));
        }

        ready(Err(MyErrors::InternalServerError("app_data::<Context>() failed".to_string())))
    }
}

impl TypedData for Context {
    fn data_type() -> paperclip::v2::models::DataType {
        paperclip::v2::models::DataType::Object
    }

    fn format() -> Option<paperclip::v2::models::DataTypeFormat> {
        None
    }
}

impl OperationModifier for Context {
    // everything is no-op in openapi gen for context
    fn update_parameter(_op: &mut paperclip::v2::models::DefaultOperationRaw) {}

    fn update_response(_op: &mut paperclip::v2::models::DefaultOperationRaw) {}

    fn update_definitions(_map: &mut std::collections::BTreeMap<String, paperclip::v2::models::DefaultSchemaRaw>) {}

    fn update_security(_op: &mut paperclip::v2::models::DefaultOperationRaw) {}

    fn update_security_definitions(_map: &mut std::collections::BTreeMap<String, paperclip::v2::models::SecurityScheme>) {}
}