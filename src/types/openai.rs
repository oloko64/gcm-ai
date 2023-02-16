use std::error::Error;

use crate::types::completion::CompletionBody;

pub struct OpenAi {
    api_key: String,
}

impl OpenAi {
    pub fn new(api_key: impl Into<String>) -> Self {
        OpenAi {
            api_key: api_key.into(),
        }
    }

    #[must_use]
    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }

    pub async fn create_completion(
        &self,
        config: impl AsRef<CompletionBody>,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/completions")
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .json(config.as_ref())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Request failed: {:#?}", response.text().await?).into());
        }

        let response = response.json::<serde_json::Value>().await?;

        Ok(response)
    }
}
