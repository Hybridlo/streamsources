pub mod twitch_client;

#[derive(Clone)]
pub struct HttpClient(pub reqwest::Client);