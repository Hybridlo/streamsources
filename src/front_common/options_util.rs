use serde::Serialize;

pub trait IntoWithLogin {
    type WithLogin: Serialize;
    fn with_login(&self, token: &str) -> Self::WithLogin;
}