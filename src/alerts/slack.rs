use crate::alerts::utils;
use anyhow::anyhow;
use serde_yaml::{Mapping, Value};
use std::convert::TryFrom;
use std::time::Duration;

#[derive(serde::Serialize)]
pub struct SlackIntegrationConfig {
    pub hook: String,
}

impl TryFrom<&Mapping> for SlackIntegrationConfig {
    type Error = anyhow::Error;

    fn try_from(value: &Mapping) -> Result<Self, Self::Error> {
        value
            .get("hook")
            .map(|hook| match hook {
                Value::String(hook) => Ok(SlackIntegrationConfig {
                    hook: hook.to_string(),
                }),
                _ => Err(anyhow!("Field 'hook' has wrong data type.")),
            })
            .unwrap_or_else(|| Err(anyhow!("Field 'hook' must be provided.")))
    }
}

#[derive(serde::Serialize, Debug)]
struct Text<'s> {
    r#type: &'static str,
    text: &'s str,
}

#[derive(serde::Serialize, Debug)]
struct Block<'s> {
    r#type: &'static str,
    text: Text<'s>,
}

#[derive(serde::Serialize, Debug)]
struct Message<'s> {
    text: &'s str,
    blocks: Vec<Block<'s>>,
}

pub async fn send_message(
    message: &str,
    config: &SlackIntegrationConfig,
) -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();

    let blocks: Vec<Block> = utils::get_message_chunks(message, 3000)
        .into_iter()
        .map(|text| Block {
            r#type: "section",
            text: Text {
                r#type: "mrkdwn",
                text,
            },
        })
        .collect();

    let message = Message {
        blocks,
        text: "New api changes",
    };

    let result = client
        .post(&config.hook)
        .timeout(Duration::from_secs(5))
        .json(&message)
        .send()
        .await?;

    if result.status().is_success() {
        Ok(())
    } else {
        let status = result.status();
        Err(anyhow!(
            "Wrong status code received from Slack Web Hook: {status}"
        ))
    }
}
