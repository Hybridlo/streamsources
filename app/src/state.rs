use twitch_sources_client::apis::configuration::Configuration;
use yewdux::prelude::*;

#[derive(Default, Store, Clone)]
pub struct ClientConfig {
    pub config: Configuration
}

impl PartialEq for ClientConfig {
    fn eq(&self, other: &Self) -> bool {
        self.config.base_path == other.config.base_path
    }
}

#[derive(Default, Store, Clone, PartialEq)]
pub struct ErrorState {
    pub show_error: bool,
    pub error_message: String
}

#[derive(Clone, PartialEq)]
pub struct LoginInfo {
    pub username: String
}

#[derive(Default, Store, Clone, PartialEq)]
pub struct LoginState {
    pub info: Option<LoginInfo>,
    pub last_check: chrono::NaiveDateTime
}