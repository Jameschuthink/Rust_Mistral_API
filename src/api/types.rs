use serde::{Deserialize, Serialize};
use std::clone::Clone;

// Request structs (for sending TO API)
#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ApiRequest {
    pub model: String,
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
}

#[derive(Deserialize, Clone)]
pub struct JsonField {
    pub role: String,
    pub content: String,
}
