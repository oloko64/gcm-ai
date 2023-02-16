pub mod configs;

use std::error::Error;

use configs::completion::CompletionBody;

pub struct OpenAi {
    api_key: String,
}

impl OpenAi {
    pub fn new(api_key: impl Into<String>) -> Self {
        OpenAi {
            api_key: api_key.into(),
        }
    }

    pub async fn create_completion(
        &self,
        config: impl AsRef<CompletionBody>,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.openai.com/v1/completions")
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .json(config.as_ref())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("Request failed".into());
        }

        let response = response.json::<serde_json::Value>().await?;

        Ok(response)
    }
}
