use serde::{Serialize, Deserialize};
use yew::{Properties, UseStateHandle};

use crate::{front_common::{IntoWithLogin, SourceColor}, util::is_default};

use super::HypetrainState;

#[derive(Serialize)]
pub struct HypetrainSourceOptionsLogin {
    #[serde(flatten)]
    pub data: HypetrainSourceOptions,
    pub login_token: String,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct HypetrainSourceOptions {
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub color: SourceColor,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub last_events_shown_count: usize,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub show_cooldown: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub show_ready: bool,
}

impl IntoWithLogin for HypetrainSourceOptions {
    type WithLogin = HypetrainSourceOptionsLogin;

    fn with_login(&self, token: &str) -> Self::WithLogin {
        Self::WithLogin{ data: self.clone(), login_token: token.to_string() }
    }
}

impl HypetrainSourceOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(PartialEq, Properties)]
pub struct HypetrainProps {
    pub settings: HypetrainSourceOptions,
    pub last_message: UseStateHandle<HypetrainState>,
}
