use confy::ConfyError;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
}

impl AppConfig {
    pub fn try_set(api_key: impl Into<String>) -> Result<Self, ConfyError> {
        let config: Self = AppConfig {
            api_key: api_key.into(),
        };

        confy::store(APP_NAME, None, &config)?;

        println!(
            "Successfully set the API key at {}\n",
            confy::get_configuration_file_path(APP_NAME, None)?.to_string_lossy()
        );

        Ok(config)
    }

    pub fn try_get() -> Result<Self, ConfyError> {
        let config: Self = confy::load(APP_NAME, None)?;
        Ok(config)
    }
}
