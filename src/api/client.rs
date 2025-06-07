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
