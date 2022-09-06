#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SourceColor {
    White,
    #[default]
    Black
}

#[derive(Default, Clone, Copy)]
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

    pub fn item_to_params(&self) -> String {
        let mut res = String::new();

        match self.color {
            SourceColor::White => res += "color=white",
            SourceColor::Black => res += "color=black",
        };

        if self.is_expanded { res += "&expanded=yes" }

        return res;
    }

    pub fn params_to_items(params: &str) -> Self {
        Self {
            color: if params.contains("color=white") { SourceColor::White } else { Default::default() },
            is_expanded: if params.contains("expanded=yes") { true } else { Default::default() }
        }
    }
}