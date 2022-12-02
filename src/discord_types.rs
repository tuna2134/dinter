use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub type_name: i32,
    pub data: Option<InteractionResponseData>,
}

#[derive(Serialize, Deserialize)]
pub struct InteractionResponseData {
    pub tts: Option<bool>,
    pub content: Option<String>,
    pub embeds: Option<serde_json::Value>,
    pub flags: Option<i32>,
}