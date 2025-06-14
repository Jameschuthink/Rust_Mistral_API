use serde::Deserialize;

// ════════════════════════════════════════════════════════════════════════════════
// JAMENDO MUSIC API TYPES - For music search functionality
// ════════════════════════════════════════════════════════════════════════════════

// Jamendo Response return type
#[derive(Deserialize, Debug)]
pub struct JamendoResponse {
    pub results: Vec<Track>,
}

// Jamendo Json response -> Struct
#[derive(Deserialize, Debug)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artist_name: String,
    pub audio: String, // The streaming URL
}
