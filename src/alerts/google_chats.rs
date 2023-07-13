use anyhow::anyhow;
use serde_yaml::{Mapping, Value};
use std::convert::TryFrom;
use std::time::Duration;

use crate::alerts::utils;

#[derive(serde::Serialize)]
pub struct GoogleChatsIntegrationConfig {
    pub hook: String,
}

impl TryFrom<&Mapping> for GoogleChatsIntegrationConfig {
    type Error = anyhow::Error;

    fn try_from(value: &Mapping) -> Result<Self, Self::Error> {
        value
            .get("hook")
            .map(|hook| match hook {
                Value::String(hook) => Ok(GoogleChatsIntegrationConfig {
                    hook: hook.to_string(),
                }),
                _ => Err(anyhow!("Field 'hook' has wrong data type.")),
            })
            .unwrap_or_else(|| Err(anyhow!("Field 'hook' must be provided.")))
    }
}

#[derive(serde::Serialize)]
struct Message<'a> {
    text: &'a str,
}

pub async fn send_message(
    message: &str,
    config: &GoogleChatsIntegrationConfig,
) -> Result<(), anyhow::Error> {
    let messages = utils::get_message_chunks(message, 4000)
        .into_iter()
        .map(|text| Message { text });

    let mut has_error = false;

    let client = reqwest::Client::new();

    for message in messages {
        let result = client
            .post(&config.hook)
            .timeout(Duration::from_secs(5))
            .json(&message)
            .send()
            .await;

        has_error = has_error || result.is_err();

        if let Ok(response) = result {
            has_error = has_error || !response.status().is_success()
        }
    }

    if has_error {
        Err(anyhow!("Wrong status code received from Google Chats"))
    } else {
        Ok(())
    }
}
