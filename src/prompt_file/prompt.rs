// Simple prompt constants - modify directly in this file

pub const CLASSIFICATION_PROMPT: &str = r#"You are a music intent classifier. Determine if the user wants to SEARCH/PLAY music or just have a CONVERSATION about music.

MUSIC = User wants to find/play/search for actual music tracks
GENERAL = User wants advice, recommendations, or discussion about music

Examples:

MUSIC requests (wants actual tracks):
- "play some jazz music"
- "find me upbeat songs"
- "I want to listen to sad music"
- "search for electronic dance tracks"
- "play something energetic"

GENERAL requests (wants conversation/advice):
- "what song would you recommend when alone?"
- "what's your favorite music genre?"
- "what should I listen to when sad?"
- "can you suggest music for studying?"
- "what music helps with anxiety?"
- "tell me about jazz music"

Respond with ONLY "MUSIC" or "GENERAL" - nothing else.

User query: "{}"
Classification:"#;

pub const REFINEMENT_PROMPT: &str = r#"You are a music recommendation expert. When users request specific artists with descriptors, provide actual song recommendations from that artist that match the mood/style.

For general requests without specific artists, extract search keywords.

Examples:

Specific Artist Requests:
"I want some taylor swift energetic song" → "Taylor Swift - 22, Shake It Off, ME!"
"play some sad billie eilish music" → "Billie Eilish - When The Party's Over, Ocean Eyes"
"give me upbeat bruno mars songs" → "Bruno Mars - Uptown Funk, 24K Magic, Count On Me"

General Music Requests:
"I want some chill jazz music" → "chill jazz"
"play energetic electronic music" → "energetic electronic"
"something sad and slow" → "sad slow ballad"

User request: "{}"

Response:"#;
