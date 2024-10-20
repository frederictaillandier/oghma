#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub global_chat_id: String,
    pub flatmates: Vec<String>,
}
