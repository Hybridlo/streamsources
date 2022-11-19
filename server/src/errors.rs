use paperclip::actix::api_v2_errors;
use actix_web::error::ResponseError;
use serde::Serialize;

pub fn e500<T>(e: T) -> actix_web::Error 
where
    T: std::fmt::Debug + std::fmt::Display + 'static
{
    actix_web::error::ErrorInternalServerError(e)
}

#[derive(Serialize)]
struct ErrorResponse {
    status: u16
}

#[api_v2_errors(
    code=500,
    code=403
)]
// i don't think paperclip allows to type the errors
// so the api gen will just give empty structs of
// Status403(), Status500(), etc, nothing can be done
// about that right now, not that big of a deal anyway tho
pub enum MyErrors {
    InternalServerError(String),
    AccessDenied
}

impl std::fmt::Debug for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalServerError(msg) => write!(f, "MyErrors::InternalServerError: {}", msg),
            Self::AccessDenied => write!(f, "MyErrors::AccessDenied"),
        }
    }
}

impl std::fmt::Display for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalServerError(_) => write!(f, "Internal server error"),
            Self::AccessDenied => write!(f, "Access denied"),
        }
    }
}

impl std::error::Error for MyErrors {}

impl ResponseError for MyErrors {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            MyErrors::InternalServerError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            MyErrors::AccessDenied => reqwest::StatusCode::FORBIDDEN,
        }
    }
}