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
