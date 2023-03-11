use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionBody {
    model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    echo: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    best_of: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

pub struct CompletionBodyBuilder {
    model: String,
    prompt: Option<Vec<String>>,
    suffix: Option<String>,
    max_tokens: Option<u16>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<u16>,
    stream: Option<bool>,
    logprobs: Option<u16>,
    echo: Option<bool>,
    stop: Option<Vec<String>>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    best_of: Option<u16>,
    logit_bias: Option<serde_json::Value>,
    user: Option<String>,
}

impl Default for CompletionBodyBuilder {
    fn default() -> Self {
        Self {
            model: String::from("text-davinci-003"),
            prompt: None,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }
}

impl CompletionBodyBuilder {
    pub fn new(model: String) -> Self {
        Self {
            model,
            ..Default::default()
        }
    }

    pub fn prompt(mut self, prompt: impl Into<Vec<String>>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn max_tokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn n(mut self, n: u16) -> Self {
        self.n = Some(n);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn logprobs(mut self, logprobs: u16) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    pub fn echo(mut self, echo: bool) -> Self {
        self.echo = Some(echo);
        self
    }

    pub fn stop(mut self, stop: impl Into<Vec<String>>) -> Self {
        self.stop = Some(stop.into());
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn best_of(mut self, best_of: u16) -> Self {
        self.best_of = Some(best_of);
        self
    }

    pub fn logit_bias(mut self, logit_bias: impl Into<serde_json::Value>) -> Self {
        self.logit_bias = Some(logit_bias.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn build(self) -> CompletionBody {
        CompletionBody {
            model: self.model,
            prompt: self.prompt,
            suffix: self.suffix,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            n: self.n,
            stream: self.stream,
            logprobs: self.logprobs,
            echo: self.echo,
            stop: self.stop,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
            best_of: self.best_of,
            logit_bias: self.logit_bias,
            user: self.user,
        }
    }
}

impl CompletionBody {
    pub fn builder(model: impl Into<String>) -> CompletionBodyBuilder {
        CompletionBodyBuilder {
            model: model.into(),
            ..CompletionBodyBuilder::default()
        }
    }
}

impl AsRef<CompletionBody> for CompletionBody {
    fn as_ref(&self) -> &CompletionBody {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_completion_body_builder() {
        let body = CompletionBody::builder("text-davinci-003")
            .prompt(vec!["Hello, my name is".to_string()])
            .max_tokens(10)
            .temperature(0.5)
            .top_p(1.0)
            .n(1)
            .stream(false)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build();

        let expected = json!({
            "model": "text-davinci-003",
            "prompt": ["Hello, my name is"],
            "max_tokens": 10,
            "temperature": 0.5,
            "top_p": 1.0,
            "n": 1,
            "stream": false,
            "presence_penalty": 0.0,
            "frequency_penalty": 0.0
        });

        assert_eq!(serde_json::to_value(body).unwrap(), expected);
    }

    #[test]
    fn test_completion_body_builder_default() {
        let body = CompletionBody::builder("text-davinci-003").build();

        let expected = json!({
            "model": "text-davinci-003"
        });

        assert_eq!(serde_json::to_value(body).unwrap(), expected);
    }
}
