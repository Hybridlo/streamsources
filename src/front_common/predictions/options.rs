use serde::{Serialize, Deserialize};
use yew::{Properties, UseStateHandle};

use crate::front_common::{SourceColor, options_util::IntoWithLogin};

use super::{PredictionState, PreditionStatus};

#[derive(Serialize)]
pub struct PredictionsSourceOptionsLogin {
    #[serde(flatten)]
    pub data: PredictionsSourceOptions,
    pub login_token: String
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct PredictionsSourceOptions {
    pub color: SourceColor,
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

    pub fn with_color(&self, color: SourceColor) -> Self {
        Self {
            color,
            ..*self
        }
    }

    pub fn with_is_expanded(&self, is_expanded: bool) -> Self {
        Self {
            is_maximized: is_expanded,
            ..*self
        }
    }

    pub fn with_login_token(&self, token: &str) -> PredictionsSourceOptionsLogin {
        PredictionsSourceOptionsLogin { data: self.clone(), login_token: token.to_string() }
    }
}

#[derive(PartialEq, Properties)]
pub struct PredictionsProps {
    pub is_white: bool,
    pub is_maximized: bool,
    pub state: UseStateHandle<PredictionState>,
    pub show_element_state: UseStateHandle<bool>,
    pub show_status_state: UseStateHandle<bool>,
    pub status_state: UseStateHandle<PreditionStatus>,
}