Project Path: src

Source Tree:

```txt
src
├── api
│   ├── client.rs
│   ├── mod.rs
│   └── types.rs
├── config.rs
└── main.rs

```

`src/api/client.rs`:

```rs
use crate::api::types::*;

pub async fn make_api_call() -> Result<String, Box<dyn std::error::Error>> {
    let api_key = crate::config::load_api_key()?;

    let api_request = ApiRequest {
        model: "mistral-large-latest".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "What are you good at? One only, one sentence".to_string(),
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mistral.ai/v1/chat/completions")
        .header("Content-Type", "application/json") // What's the singular version?
        .header("Authorization", &format!("Bearer {}", api_key)) // Same method
        .json(&api_request)
        .send()
        .await?;

    let api_response: ApiResponse = response.json().await?;
    let messages = api_response.choices[0].message.clone();
    let role = messages.role;
    let content = messages.content;

    Ok(format!("role: {}, content:{}", role, content))
}

```

`src/api/mod.rs`:

```rs
pub mod client;
pub mod types;

```

`src/api/types.rs`:

```rs
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

```

`src/config.rs`:

```rs
use std::env;

pub fn load_api_key() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("MISTRAL_API_KEY")?;
    Ok(api_key.to_string())
}

```

`src/main.rs`:

```rs
mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = api::client::make_api_call().await?;
    println!("AI Response: {}", response);
    Ok(())
}

```