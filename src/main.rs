mod api;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = api::client::make_api_call().await?;
    println!("AI Response: {}", response);
    Ok(())
}
