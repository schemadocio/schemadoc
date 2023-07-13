use crate::models::{
    Alert, AlertKind, AlertSource, DataSource, DataSourceSource, DataSourceStatus, Project,
    ProjectSlug, Version,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionOut<'s> {
    pub id: u32,
    pub version: Option<&'s String>,
    pub message: Option<&'s String>,

    pub file_path: &'s String,
    pub diff_file_path: &'s String,

    pub created_at: &'s DateTime<Utc>,
}

impl<'s> From<&'s Version> for VersionOut<'s> {
    fn from(value: &'s Version) -> Self {
        Self {
            id: value.id,
            version: value.version.as_ref(),
            message: value.message.as_ref(),
            file_path: &value.file_path,
            diff_file_path: &value.diff_file_path,
            created_at: &value.created_at,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertOut<'s> {
    pub name: &'s str,
    pub kind: AlertKind,
    pub source: &'s AlertSource,

    pub is_active: bool,

    pub service: &'s str,
}

impl<'s> From<&'s Alert> for AlertOut<'s> {
    fn from(alert: &'s Alert) -> AlertOut<'s> {
        Self {
            name: &alert.name,
            service: &alert.service,
            kind: alert.kind,
            source: &alert.source,
            is_active: alert.is_active,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceStatusOut {
    pub pull_enabled: bool,
    pub pull_attempt: u32,

    pub pull_interval_minutes: u32,
    pub pull_last_at: Option<DateTime<Utc>>,
    pub pull_error: bool,
}

impl From<&DataSourceStatus> for DataSourceStatusOut {
    fn from(status: &DataSourceStatus) -> DataSourceStatusOut {
        Self {
            pull_enabled: status.pull_enabled,
            pull_attempt: status.pull_attempt,
            pull_interval_minutes: status.pull_interval_minutes,
            pull_last_at: status.pull_last_at,
            pull_error: status.pull_error,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceOut<'s> {
    pub name: &'s str,
    pub source: &'s DataSourceSource,
    pub status: Option<DataSourceStatusOut>,
}

impl<'s> From<&'s DataSource> for DataSourceOut<'s> {
    fn from(data_source: &'s DataSource) -> DataSourceOut<'s> {
        Self {
            name: &data_source.name,
            source: &data_source.source,
            status: data_source.status.as_ref().map(DataSourceStatusOut::from),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOut<'s> {
    pub slug: &'s ProjectSlug,
    pub name: &'s str,
    pub description: &'s str,
    pub alerts: Option<Vec<AlertOut<'s>>>,
    pub data_source: Option<DataSourceOut<'s>>,
}

impl<'s> From<&'s Project> for ProjectOut<'s> {
    fn from(project: &'s Project) -> ProjectOut<'s> {
        Self {
            slug: &project.slug,
            name: &project.name,
            description: &project.description,
            alerts: project
                .alerts
                .as_ref()
                .map(|v| v.iter().map(AlertOut::from).collect()),
            data_source: project.data_source.as_ref().map(DataSourceOut::from),
        }
    }
}