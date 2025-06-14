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
