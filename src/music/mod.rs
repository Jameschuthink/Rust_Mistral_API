pub mod types;

use crate::config::load_jamendo_client_id;
use miette::{IntoDiagnostic, Result};
use types::{JamendoResponse, Track};

pub async fn jamendo_search(query: &str) -> Result<Vec<Track>> {
    let client_id = crate::config::load_jamendo_client_id()?;

    let url = format!(
        "https://api.jamendo.com/v3.0/tracks/?client_id={}&format=json&search={}&limit=2",
        client_id, query
    );

    reqwest::get(&url)
        .await
        .into_diagnostic()?
        .json::<JamendoResponse>()
        .await
        .into_diagnostic()
        .and_then(|response| {
            if response.results.is_empty() {
                Err(miette::miette!("No music found for: '{}'", query))
            } else {
                Ok(response.results)
            }
        })
}
