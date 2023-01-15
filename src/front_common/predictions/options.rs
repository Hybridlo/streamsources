use serde::{Serialize, Deserialize};
use yew::{Properties, UseStateHandle};

use crate::front_common::SourceColor;

use super::{PredictionState, PreditionStatus};

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PredictionsSourceOptions {
    pub color: SourceColor,
    pub is_expanded: bool
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
            is_expanded,
            ..*self
        }
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