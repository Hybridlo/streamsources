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