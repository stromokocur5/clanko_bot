#[derive(serde::Deserialize)]
pub struct Slack {
    pub token: String,
    pub channel_id: String,
}
