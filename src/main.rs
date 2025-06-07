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
