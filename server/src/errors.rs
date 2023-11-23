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
    AccessDenied,
    NotFound,
}

impl std::fmt::Debug for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalServerError(msg) => write!(f, "MyErrors::InternalServerError: {}", msg),
            Self::AccessDenied => write!(f, "MyErrors::AccessDenied"),
            Self::NotFound => write!(f, "MyErrors::NotFound"),
        }
    }
}

impl std::fmt::Display for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalServerError(_) => write!(f, "Internal server error"),
            Self::AccessDenied => write!(f, "Access denied"),
            Self::NotFound => write!(f, "Not found"),
        }
    }
}

// impl std::error::Error for MyErrors {}

impl ResponseError for MyErrors {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            MyErrors::InternalServerError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            MyErrors::AccessDenied => reqwest::StatusCode::FORBIDDEN,
            MyErrors::NotFound => reqwest::StatusCode::NOT_FOUND,
        }
    }
}

impl<T> From<T> for MyErrors
where
    T: std::error::Error
{
    fn from(err: T) -> Self {
        Self::InternalServerError(err.to_string())
    }
}

// i don't like this, but this shortens error transformation
pub trait IntoResultMyErr<T, E> {
    fn into_my(self) -> Result<T, MyErrors>;
}

impl<T, E> IntoResultMyErr<T, E> for Result<T, E>
where
    E: ToString
{
    fn into_my(self) -> Result<T, MyErrors> {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(MyErrors::InternalServerError(err.to_string())),
        }
    }
}