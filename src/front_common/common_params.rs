use serde::{Serialize, Deserialize};
use strum::{EnumIter, Display};

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize, EnumIter, Display)]
#[serde(rename_all="snake_case")]
pub enum SourceColor {
    White,
    #[default]
    Black
}