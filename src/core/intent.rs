use crate::api::client::make_api_call;
use crate::prompt_file::prompt;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum UserIntent<'a> {
    GeneralQuery(Cow<'a, str>),
    MusicRequest(Cow<'a, str>),
}

pub struct IntentClassifier;

impl IntentClassifier {
    pub fn new() -> Self {
        Self
    }

    pub async fn classify<'a>(&self, input: &'a str) -> miette::Result<UserIntent<'a>> {
        let trimmed = input.trim();
        let is_music_intent = self.classify_with_ai(trimmed).await?;

        if is_music_intent {
            let refined_query = self.refine_music(trimmed).await?;
            Ok(UserIntent::MusicRequest(Cow::Owned(refined_query)))
        } else {
            Ok(UserIntent::GeneralQuery(Cow::Borrowed(trimmed)))
        }
    }
    async fn classify_with_ai(&self, input: &str) -> miette::Result<bool> {
        let classification_prompt = format!(
            r#"Prompt {},User query: "{}"#,
            prompt::CLASSIFICATION_PROMPT,
            input
        );
        let response = crate::api::client::make_api_call(&classification_prompt).await?;
        let is_music = response.trim().to_uppercase() == "MUSIC";
        Ok(is_music)
    }

    async fn refine_music(&self, input: &str) -> miette::Result<String> {
        let refine_prompt = format!(r#"{}, User input{}"#, prompt::REFINEMENT_PROMPT, input);
        let refine_response = crate::api::client::make_api_call(&refine_prompt).await?;
        let trimmed_reponse = refine_response.trim().to_lowercase();
        Ok(trimmed_reponse)
    }
}
