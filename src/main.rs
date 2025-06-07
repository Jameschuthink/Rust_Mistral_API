mod api;
mod config;

use miette::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let response = api::client::make_api_call().await?;
    println!("AI Response: {}", response);
    Ok(())
}
