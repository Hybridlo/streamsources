mod app_token;

const TWITCH_REDIRECT_URI: &str = "/twitch_login/";
const TWITCH_API_URI: &str = "https://api.twitch.tv/helix";
const TWITCH_API_AUTH: &str = "https://id.twitch.tv";

pub use app_token::get_app_token;