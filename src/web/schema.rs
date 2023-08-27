use crate::models::{
    Alert, AlertKind, AlertSource, DataSource, DataSourceSource, DataSourceStatus, Dependency,
    Link, Project, ProjectSlug, Version,
};
use crate::versions::statistics::DiffStatistics;
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

    pub statistics: &'s DiffStatistics,

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
            statistics: &value.statistics,
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
pub struct DependencyOut<'s> {
    pub project: &'s str,
    pub branch: &'s str,
    pub version: u32,
    pub outdated: Option<bool>,
    pub breaking: Option<bool>,
}

impl<'s> From<&'s Dependency> for DependencyOut<'s> {
    fn from(dependency: &'s Dependency) -> DependencyOut<'s> {
        Self {
            version: dependency.version,
            outdated: dependency.outdated,
            breaking: dependency.breaking,
            branch: dependency.branch.as_str(),
            project: dependency.project.as_str(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOut<'s> {
    pub slug: &'s ProjectSlug,
    pub name: &'s str,
    pub kind: &'static str,
    pub description: Option<&'s str>,

    pub links: Option<&'s Vec<Link>>,

    pub branches: Vec<String>,
    pub alerts: Vec<AlertOut<'s>>,
    pub data_source: Option<DataSourceOut<'s>>,
    pub dependencies: Vec<DependencyOut<'s>>,
}

impl<'s> From<&'s Project> for ProjectOut<'s> {
    fn from(project: &'s Project) -> ProjectOut<'s> {
        let alerts = project.alerts.iter().map(AlertOut::from).collect();

        let branches = project.branches.iter().map(|b| b.name.clone()).collect();

        Self {
            alerts,
            branches,
            slug: &project.slug,
            name: &project.name,
            kind: project.kind.as_str(),
            links: project.links.as_ref(),
            description: project.description.as_deref(),
            data_source: project.data_source.as_ref().map(DataSourceOut::from),
            dependencies: project
                .dependencies
                .iter()
                .map(DependencyOut::from)
                .collect(),
        }
    }
}
