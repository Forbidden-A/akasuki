use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AkasukiConfig {
    #[serde(alias = "bot")]
    pub discord: DiscordConfig,
}
#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub token: String,
}
