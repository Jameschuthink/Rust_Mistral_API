use std::env;

pub fn load_api_key() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("MISTRAL_API_KEY")?;
    Ok(api_key.to_string())
}
