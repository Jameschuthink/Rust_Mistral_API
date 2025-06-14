Project Path: src

Source Tree:

```txt
src
├── api
│   ├── client.rs
│   ├── mod.rs
│   └── types.rs
├── config.rs
├── core
│   ├── intent.rs
│   └── mod.rs
├── error.rs
├── input.rs
├── main.rs
├── music
│   ├── mod.rs
│   └── types.rs
└── prompt_file
    ├── mod.rs
    └── prompt.rs

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
                content: "You are a helpful assistant who speak directly".to_string(),
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
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", api_key))
        .json(&api_request)
        .send()
        .await
        .into_diagnostic()?;

    let raw_response = response.text().await.into_diagnostic()?;

    let api_response: ApiResponse = serde_json::from_str(&raw_response)
        .into_diagnostic()
        .map_err(|e| miette::miette!("JSON parse error: {}\nRaw response: {}", e, raw_response))?;

    let content = api_response
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

```

`src/config.rs`:

```rs
use ::miette::{IntoDiagnostic, Result};
use std::env;

//Mistral API
pub fn load_api_key() -> Result<String> {
    dotenv::dotenv().ok();
    let api_key = env::var("MISTRAL_API_KEY").into_diagnostic()?;
    Ok(api_key.to_string())
}
//Jamendo API
pub fn load_jamendo_client_id() -> Result<String> {
    dotenv::dotenv().ok();
    let client_id = env::var("JAMENDO_CLIENT_ID").into_diagnostic()?;
    Ok(client_id)
}

```

`src/core/intent.rs`:

```rs
use crate::api::client::make_api_call;
use crate::prompt_file::prompt;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum UserIntent<'a> {
    GeneralQuery(Cow<'a, str>),
    MusicRequest(Cow<'a, str>),
}

pub struct IntentClassifier;

impl IntentClassifier {
    pub fn new() -> Self {
        Self
    }

    pub async fn classify<'a>(&self, input: &'a str) -> miette::Result<UserIntent<'a>> {
        let trimmed = input.trim();
        let is_music_intent = self.classify_with_ai(trimmed).await?;

        if is_music_intent {
            let refined_query = self.refine_music(trimmed).await?;
            Ok(UserIntent::MusicRequest(Cow::Owned(refined_query)))
        } else {
            Ok(UserIntent::GeneralQuery(Cow::Borrowed(trimmed)))
        }
    }
    async fn classify_with_ai(&self, input: &str) -> miette::Result<bool> {
        let classification_prompt = format!(
            r#"Prompt {},User query: "{}"#,
            prompt::CLASSIFICATION_PROMPT,
            input
        );
        let response = crate::api::client::make_api_call(&classification_prompt).await?;
        let is_music = response.trim().to_uppercase() == "MUSIC";
        Ok(is_music)
    }

    async fn refine_music(&self, input: &str) -> miette::Result<String> {
        let refine_prompt = format!(r#"{}, User input{}"#, prompt::REFINEMENT_PROMPT, input);
        let refine_response = crate::api::client::make_api_call(&refine_prompt).await?;
        let trimmed_reponse = refine_response.trim().to_lowercase();
        Ok(trimmed_reponse)
    }
}

```

`src/core/mod.rs`:

```rs
pub mod intent;

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
mod core;
mod input;
mod music;
mod prompt_file;

use crate::core::intent::{IntentClassifier, UserIntent};
use input::ChatInput;
use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut chat_input = ChatInput::new()?;
    let classifier = IntentClassifier::new();
    // Create a new readline interface
    loop {
        let user_input = match chat_input.prompt("💬 You: ")? {
            Some(input) => input,
            None => break,
        };

        let intent = classifier.classify(&user_input).await?;
        println!("🔍 Classified as: {:?}", intent);

        match intent {
            UserIntent::GeneralQuery(input) => {
                let response = api::client::make_api_call(&*input).await?;
                println!("🤖 AI: {}", response);
            }
            UserIntent::MusicRequest(refined_query) => {
                println!("🎯 Refined search: {}", refined_query);

                let tracks = music::jamendo_search(&*refined_query).await?;

                if tracks.is_empty() {
                    println!("🎵 No tracks found for: {}", refined_query);
                } else {
                    println!("🎵 Found tracks:");
                    for song in tracks {
                        println!(
                            "  - {} by {} (Stream: {})",
                            song.name, song.artist_name, song.audio
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

```

`src/music/mod.rs`:

```rs
pub mod types;

use crate::config::load_jamendo_client_id;
use miette::{IntoDiagnostic, Result};
use types::{JamendoResponse, Track};

pub async fn jamendo_search(query: &str) -> Result<Vec<Track>> {
    let client_id = crate::config::load_jamendo_client_id()?;

    let url = format!(
        "https://api.jamendo.com/v3.0/tracks/?client_id={}&format=json&search={}&limit=2",
        client_id, query
    );

    reqwest::get(&url)
        .await
        .into_diagnostic()?
        .json::<JamendoResponse>()
        .await
        .into_diagnostic()
        .and_then(|response| {
            if response.results.is_empty() {
                Err(miette::miette!("No music found for: '{}'", query))
            } else {
                Ok(response.results)
            }
        })
}

```

`src/music/types.rs`:

```rs
use serde::Deserialize;

// ════════════════════════════════════════════════════════════════════════════════
// JAMENDO MUSIC API TYPES - For music search functionality
// ════════════════════════════════════════════════════════════════════════════════

// Jamendo Response return type
#[derive(Deserialize, Debug)]
pub struct JamendoResponse {
    pub results: Vec<Track>,
}

// Jamendo Json response -> Struct
#[derive(Deserialize, Debug)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artist_name: String,
    pub audio: String, // The streaming URL
}

```

`src/prompt_file/mod.rs`:

```rs
pub mod prompt;

```

`src/prompt_file/prompt.rs`:

```rs
// Simple prompt constants - modify directly in this file

pub const CLASSIFICATION_PROMPT: &str = r#"You are a music intent classifier. Determine if the user wants to SEARCH/PLAY music or just have a CONVERSATION about music.

MUSIC = User wants to find/play/search for actual music tracks
GENERAL = User wants advice, recommendations, or discussion about music

Examples:

MUSIC requests (wants actual tracks):
- "play some jazz music"
- "find me upbeat songs"
- "I want to listen to sad music"
- "search for electronic dance tracks"
- "play something energetic"

GENERAL requests (wants conversation/advice):
- "what song would you recommend when alone?"
- "what's your favorite music genre?"
- "what should I listen to when sad?"
- "can you suggest music for studying?"
- "what music helps with anxiety?"
- "tell me about jazz music"

Respond with ONLY "MUSIC" or "GENERAL" - nothing else.

User query: "{}"
Classification:"#;

pub const REFINEMENT_PROMPT: &str = r#"You are a music recommendation expert. When users request specific artists with descriptors, provide actual song recommendations from that artist that match the mood/style.

For general requests without specific artists, extract search keywords.

Examples:

Specific Artist Requests:
"I want some taylor swift energetic song" → "Taylor Swift - 22, Shake It Off, ME!"
"play some sad billie eilish music" → "Billie Eilish - When The Party's Over, Ocean Eyes"
"give me upbeat bruno mars songs" → "Bruno Mars - Uptown Funk, 24K Magic, Count On Me"

General Music Requests:
"I want some chill jazz music" → "chill jazz"
"play energetic electronic music" → "energetic electronic"
"something sad and slow" → "sad slow ballad"

User request: "{}"

Response:"#;

```
