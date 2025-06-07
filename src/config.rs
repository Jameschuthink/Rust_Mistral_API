use ::miette::{IntoDiagnostic, Result};
use std::env;

pub fn load_api_key() -> Result<String> {
    dotenv::dotenv().ok();
    let api_key = env::var("MISTRAL_API_KEY").into_diagnostic()?;
    Ok(api_key.to_string())
}
