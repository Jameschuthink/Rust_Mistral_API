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
