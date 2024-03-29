pub mod front_common;
pub mod common_data;
pub mod util;

pub const GLOBAL_DELAY_VALUE_SECONDS: u32 = 1;
pub const GLOBAL_DELAY_VALUE: u32 = GLOBAL_DELAY_VALUE_SECONDS * 1_000;

pub const FPS: u32 = 60;

pub const TWITCH_AUTH_URL: &str = "https://id.twitch.tv/oauth2/authorize";