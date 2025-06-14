use miette::Result;
use serde::{Deserialize, Serialize};
use std::clone::Clone;

// ════════════════════════════════════════════════════════════════════════════════
// MISTRAL AI API TYPES - For chat/question functionality
// ════════════════════════════════════════════════════════════════════════════════

// Request structs (for sending TO API)
#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ApiRequest {
    pub model: ModelType,
    pub messages: Vec<Message>,
}

// Response structs (for receiving FROM API)
#[derive(Deserialize, Clone)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Clone)]
pub struct Choice {
    pub message: JsonField,
    pub index: u32,
    pub finish_reason: String,
}

#[derive(Deserialize, Clone)]
pub struct JsonField {
    pub role: String,
    pub content: String,
    pub tool_calls: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub enum ModelType {
    #[serde(rename = "mistral-large-latest")]
    MistralLarge,
    #[serde(rename = "mistral-small-latest")]
    MistralSmall,
    #[serde(rename = "codestral-latest")]
    Codestral,
}
