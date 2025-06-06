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
