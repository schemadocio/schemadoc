use anyhow::anyhow;
use serde_yaml::{Mapping, Value};

use schemadoc_diff::checker::ValidationIssue;
use schemadoc_diff::exporters::{Exporter, Markdown};
use schemadoc_diff::schema_diff::HttpSchemaDiff;

use crate::alerts::{google_chats, slack};
use crate::config::Config;
use crate::models::{AlertKind, Project};

pub struct AlertInfo<'s> {
    pub markdown: Markdown,
    pub service: &'s String,
    pub service_config: &'s Mapping,
}

pub async fn get_alerts_info<'s>(
    config: &Config,
    project: &'s Project,
    tgt_version_id: u32,
    diff: &HttpSchemaDiff,
    validations: &Vec<ValidationIssue>,
) -> Result<Vec<AlertInfo<'s>>, anyhow::Error> {
    let Some(alerts) = &project.alerts else {
        return Ok(vec![]);
    };

    let version_url = config.url_to_version(&project.slug, tgt_version_id);

    let mut info = Vec::new();

    for alert in alerts {
        if !alert.is_active || !alert.source.is_own() {
            println!("Alert is not active");
            continue;
        }

        let invalid_only = matches!(alert.kind, AlertKind::AllBreaking);

        let fields = [("Project", project.name.as_str())].into();

        let markdown = diff.export(fields, &version_url, invalid_only, None, Some(validations));
        if markdown.is_empty() {
            println!("Alert markdown is empty");
            continue;
        }

        let service_config = match &alert.service_config {
            Value::Mapping(m) => m,
            _ => {
                println!("Alert was not properly configured");
                continue;
            }
        };

        info.push(AlertInfo {
            markdown,
            service_config,
            service: &alert.service,
        });
    }

    Ok(info)
}

pub async fn send_alert(alert_info: AlertInfo<'_>) -> Result<(), anyhow::Error> {
    match alert_info.service.as_ref() {
        "GoogleChats" => {
            let config =
                google_chats::GoogleChatsIntegrationConfig::try_from(alert_info.service_config)?;
            let message = alert_info.markdown.as_str().to_owned();
            tokio::spawn(async move {
                // TODO: handler errors during message send
                let _r = google_chats::send_message(&message, &config).await;
            });

            Ok(())
        }
        "Slack" => {
            let config = slack::SlackIntegrationConfig::try_from(alert_info.service_config)?;
            let message = alert_info.markdown.as_str().to_owned();
            tokio::spawn(async move {
                let _r = slack::send_message(&message, &config).await;
            });

            Ok(())
        }
        _ => Err(anyhow!("Invalid alert service provided")),
    }
}
