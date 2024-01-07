#[derive(serde::Deserialize, Clone)]
pub struct Slack {
    pub token: String,
    pub channel_id: String,
}
