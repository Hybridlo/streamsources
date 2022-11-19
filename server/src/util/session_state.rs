
use actix_session::{Session, SessionInsertError, SessionGetError};
use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use paperclip::actix::OperationModifier;
use paperclip::v2::schema::TypedData;
use std::future::{Ready, ready};
pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: i64) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<i64>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}

impl TypedData for TypedSession {
    fn data_type() -> paperclip::v2::models::DataType {
        paperclip::v2::models::DataType::Object
    }

    fn format() -> Option<paperclip::v2::models::DataTypeFormat> {
        None
    }
}

impl OperationModifier for TypedSession {
    // everything is no-op for a session
    fn update_parameter(_op: &mut paperclip::v2::models::DefaultOperationRaw) {}

    fn update_response(_op: &mut paperclip::v2::models::DefaultOperationRaw) {}

    fn update_definitions(_map: &mut std::collections::BTreeMap<String, paperclip::v2::models::DefaultSchemaRaw>) {}

    fn update_security(_op: &mut paperclip::v2::models::DefaultOperationRaw) {}

    fn update_security_definitions(_map: &mut std::collections::BTreeMap<String, paperclip::v2::models::SecurityScheme>) {}
}