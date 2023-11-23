use serde::{Serialize, Deserialize};
use yew::{Properties, UseStateHandle};

use crate::{front_common::{SourceColor, options_util::IntoWithLogin}, util::is_default};

use super::{PredictionState, PredictionStatus};

#[derive(Serialize)]
pub struct PredictionsSourceOptionsLogin {
    #[serde(flatten)]
    pub data: PredictionsSourceOptions,
    pub login_token: String
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct PredictionsSourceOptions {
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub color: SourceColor,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub is_maximized: bool
}

impl IntoWithLogin for PredictionsSourceOptions {
    type WithLogin = PredictionsSourceOptionsLogin;

    fn with_login(&self, token: &str) -> Self::WithLogin {
        Self::WithLogin { data: self.clone(), login_token: token.to_string() }
    }
}

impl PredictionsSourceOptions {
    pub fn new() -> Self {
        return Default::default();
    }
}

#[derive(PartialEq, Properties)]
pub struct PredictionsProps {
    pub color: SourceColor,
    pub is_maximized: bool,
    pub state: UseStateHandle<PredictionState>,
    pub show_element_state: UseStateHandle<bool>,
    pub show_status_state: UseStateHandle<bool>,
    pub status_state: UseStateHandle<PredictionStatus>,
}