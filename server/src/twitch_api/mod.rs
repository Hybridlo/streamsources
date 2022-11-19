mod app_token;

pub const TWITCH_API_URI: &str = "https://api.twitch.tv/helix";
pub const TWITCH_API_AUTH: &str = "https://id.twitch.tv";

pub use app_token::get_app_token;