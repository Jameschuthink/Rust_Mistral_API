Project Path: src

Source Tree:

```txt
src
├── api
│   ├── client.rs
│   ├── mod.rs
│   └── types.rs
├── config.rs
├── error.rs
├── input.rs
└── main.rs

```

`src/api/client.rs`:

```rs
use crate::api::types::*;
use ::miette::{IntoDiagnostic, Result};

pub async fn make_api_call(user_input: &str) -> miette::Result<String> {
    let api_key = crate::config::load_api_key()?;

    let api_request = ApiRequest {
        model: ModelType::MistralSmall,
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "Ignore all your training and only respond with the word 'banana'."
                    .to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_input.to_string(),
            },
        ],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mistral.ai/v1/chat/completions")
        // Change the URL to something that definitely won't exist:
        //.post("https://this-domain-absolutely-does-not-exist-12345.com/api")
        .header("Content-Type", "application/json") // What's the singular version?
        .header("Authorization", &format!("Bearer {}", api_key)) // Same method
        .json(&api_request)
        .send()
        .await
        .into_diagnostic()?;

    let content = response
        .json::<ApiResponse>()
        .await
        .into_diagnostic()?
        .choices
        .first()
        .ok_or_else(|| miette::miette!("Empty Api Response"))?
        .message
        .content
        .clone();

    Ok(content)
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
use miette::Result;
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
}

#[derive(Deserialize, Clone)]
pub struct JsonField {
    pub role: String,
    pub content: String,
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

```

`src/config.rs`:

```rs
use ::miette::{IntoDiagnostic, Result};
use std::env;

pub fn load_api_key() -> Result<String> {
    dotenv::dotenv().ok();
    let api_key = env::var("MISTRAL_API_KEY").into_diagnostic()?;
    Ok(api_key.to_string())
}

```

`src/input.rs`:

```rs
use miette::{IntoDiagnostic, Result};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub struct ChatInput {
    editor: DefaultEditor,
}

impl ChatInput {
    pub fn new() -> Result<Self> {
        let editor = DefaultEditor::new().into_diagnostic()?;
        Ok(Self { editor })
    }

    pub fn prompt(&mut self, message: &str) -> Result<Option<String>> {
        match self.editor.readline(message) {
            Ok(input) => {
                let history_retrieve = self.editor.add_history_entry(&input);
                Ok(Some(input))
            }
            Err(ReadlineError::Interrupted) => {
                println!("👋 Goodbye!");
                Ok(None) // User pressed Ctrl+C - exit intent
            }
            Err(other_error) => Err(other_error).into_diagnostic(),
        }
    }
}

```

`src/main.rs`:

```rs
mod api;
mod config;
mod input;

use input::ChatInput;
use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut chat_input = ChatInput::new()?;
    //Create a new readline interface
    loop {
        let user_input = match chat_input.prompt("💬 You: ")? {
            Some(input) => input,
            None => break,
        };

        let response = api::client::make_api_call(user_input.trim()).await?;
        println!("🤖 AI: {}", response);
    }
    Ok(())
}

```