use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum SourceColor {
    #[allow(non_camel_case_types)]
    white,
    #[default]
    #[allow(non_camel_case_types)]
    black
}