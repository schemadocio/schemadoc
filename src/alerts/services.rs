use anyhow::anyhow;
use serde_yaml::{Mapping, Value};

use schemadoc_diff::checker::ValidationIssue;
use schemadoc_diff::schema_diff::HttpSchemaDiff;
use schemadoc_diff::exporters::{Exporter, Markdown};

use crate::alerts::{google_chats, slack};
use crate::settings::Settings;
use crate::models::{AlertKind, Project};

pub struct AlertInfo<'s> {
    pub markdown: Markdown,
    pub service: &'s String,
    pub service_config: &'s Mapping,
}

pub async fn get_own_alerts_info<'s>(
    settings: &Settings,
    project: &'s Project,
    branch_name: &str,
    tgt_version_id: u32,
    diff: &HttpSchemaDiff,
    validations: &[ValidationIssue],
) -> Result<Vec<AlertInfo<'s>>, anyhow::Error> {
    if project.alerts.is_empty() {
        return Ok(vec![]);
    };

    let version_url = settings.url_to_version(
        &project.slug, branch_name, tgt_version_id,
    );

    let mut info = Vec::new();

    for alert in &project.alerts {
        if !alert.is_active {
            println!("Alert is not active");
            continue;
        }

        if !alert.includes_branch(branch_name, &project.default_branch) {
            continue;
        }

        let fields = [
            ("Project", project.name.as_str()),
            ("Kind", project.kind.as_str()),
            ("Branch", branch_name),
        ]
            .into();

        let breaking_only = matches!(alert.kind, AlertKind::Breaking);
        let markdown = diff.export(fields, &version_url, breaking_only, None, Some(validations));

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

pub async fn get_deps_alerts_info<'s>(
    settings: &Settings,
    project: &'s Project,
    src_branch_name: &str,
    src_version_id: u32,
    tgt_branch_name: &str,
    tgt_version_id: u32,
    dep_projects: Vec<&'s Project>,
    diff: &HttpSchemaDiff,
    validations: &[ValidationIssue],
) -> Result<Vec<AlertInfo<'s>>, anyhow::Error> {
    let mut info = Vec::new();

    for dep in dep_projects {
        let version_url =
            settings.url_to_dependency_compare(
                &dep.slug,
                &project.slug,
                src_branch_name,
                src_version_id,
                tgt_branch_name,
                tgt_version_id,
            );

        for alert in &dep.alerts {
            if !alert.source.is_deps() {
                continue;
            }

            if !alert.is_active {
                println!("Alert is not active");
                continue;
            }

            if !alert.includes_branch(tgt_branch_name, &project.default_branch) {
                continue;
            }

            let fields = [
                ("Project", dep.name.as_str()),
                ("Kind", dep.kind.as_str()),
                ("Dependency", project.name.as_str()),
                ("Branch", tgt_branch_name),
            ]
                .into();

            let breaking_only = matches!(alert.kind, AlertKind::Breaking);
            let markdown =
                diff.export(fields, &version_url, breaking_only, None, Some(validations));

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
                // TODO: handler errors during message sending
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
