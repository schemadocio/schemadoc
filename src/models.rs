use std::fmt;
use serde_yaml::Value;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::storage::Storer;
use crate::versions::statistics::DiffStatistics;
use crate::persistence::{load_data_file, persist_data_file, PersistentDataFile, Versioned};

#[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slug(String);

impl Slug {
    pub fn new(slug: String) -> Self {
        Self(slug)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectSlug(Slug);

impl fmt::Display for ProjectSlug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl ProjectSlug {
    pub fn new(slug: String) -> Self {
        Self(Slug::new(slug))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<Slug> for ProjectSlug {
    fn from(value: Slug) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub slug: ProjectSlug,
    pub name: String,
    pub kind: ProjectKind,
    pub description: Option<String>,

    pub default_branch: String,

    pub links: Option<Vec<Link>>,

    pub alerts: Vec<Alert>,
    pub branches: Vec<Branch>,
    pub data_source: Option<DataSource>,
    pub dependencies: Vec<Dependency>,
}


impl Project {
    pub async fn load_persistent_data<S>(&mut self, storage: &S) -> anyhow::Result<()>
        where
            S: Storer,
    {
        let branches_file_path = format!("projects/{}/branches.yaml", self.slug);
        self.branches = load_data_file::<Vec<Branch>, _, _, PersistentDataFile<_>>(storage, branches_file_path)
            .await
            .unwrap_or_default();

        if self.branches.is_empty() {
            println!("{} - Add default branch: {} ", self.slug, self.default_branch);
            self.branches.push(Branch {
                name: self.default_branch.clone(),
                versions: vec![],
                base: None,
            });

            self.persist_branches(storage).await?;
        }

        if let Some(data_source) = &mut self.data_source {
            let data_source_status_file_path = format!("projects/{}/datasource.yaml", self.slug);
            let data_source_status =
                load_data_file::<DataSourceStatus, _, _, PersistentDataFile<_>>(storage, data_source_status_file_path)
                    .await
                    .unwrap_or_default();

            data_source.status = Some(data_source_status);
        }

        Ok(())
    }

    pub async fn persist_branches<S>(&self, storage: &S) -> anyhow::Result<()>
        where
            S: Storer,
    {
        let path = format!("projects/{}/branches.yaml", self.slug);
        persist_data_file::<Vec<Branch>, _, _, PersistentDataFile<_>>(storage, path, &self.branches).await?;

        Ok(())
    }

    pub async fn persist_datasource<S>(&self, storage: &S) -> anyhow::Result<()>
        where
            S: Storer,
    {
        let Some(data_source) = &self.data_source else {
            return Ok(());
        };

        let Some(data_source_status) = &data_source.status else {
            return Ok(());
        };

        let path = format!("projects/{}/datasource.yaml", self.slug);
        persist_data_file::<DataSourceStatus, _, _, PersistentDataFile<_>>(storage, path, data_source_status).await?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectKind {
    Server,
    Client,
}

impl ProjectKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Server => "server",
            Self::Client => "client",
        }
    }
}

impl ProjectKind {
    pub fn is_server(&self) -> bool {
        matches!(self, Self::Server)
    }
    pub fn is_client(&self) -> bool {
        matches!(self, Self::Client)
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub versions: Vec<Version>,
    pub base: Option<BranchBase>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BranchBase {
    pub name: String,
    pub version_id: u32,
}


impl Versioned for Vec<Branch> {
    fn latest() -> &'static str {
        "0.1"
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    pub project: ProjectSlug,
    pub branch: String,

    pub version: u32,

    // calculated fields
    pub breaking: Option<bool>,
    pub outdated: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub name: String,
    pub kind: AlertKind,
    pub source: AlertSource,

    pub branches: Vec<String>,

    pub is_active: bool,

    pub service: String,
    pub service_config: Value,
}

impl Alert {
    pub fn includes_branch(&self, branch: &str, default_branch: &str) -> bool {
        if self.branches.is_empty() && branch == default_branch {
            true
        } else if self.branches.iter().any(|b| b.as_str() == "*" || b.as_str() == branch) {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum AlertKind {
    #[serde(rename = "all")]
    All,
    #[default]
    #[serde(rename = "breaking")]
    Breaking,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum AlertSource {
    #[default]
    #[serde(rename = "own")]
    Own,
    #[serde(rename = "deps")]
    Dependencies,
}

impl AlertSource {
    pub fn is_own(&self) -> bool {
        matches!(self, AlertSource::Own)
    }
    pub fn is_deps(&self) -> bool {
        matches!(self, AlertSource::Dependencies)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub id: u32,
    pub version: Option<String>,
    pub message: Option<String>,

    pub file_path: String,

    pub diff_file_path: String,
    pub diff_file_version: String,

    pub statistics: DiffStatistics,

    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    pub name: String,
    pub branch: String,
    pub source: DataSourceSource,
    // persisted field
    pub status: Option<DataSourceStatus>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataSourceSource {
    Url { url: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSourceStatus {
    pub pull_enabled: bool,
    pub pull_attempt: u32,

    pub pull_interval_minutes: u32,
    pub pull_last_at: Option<DateTime<Utc>>,
    pub pull_error: bool,
    pub pull_error_message: Option<String>,
}

impl Default for DataSourceStatus {
    fn default() -> Self {
        Self {
            pull_enabled: true,
            pull_attempt: 0,
            pull_interval_minutes: 15,
            pull_last_at: None,
            pull_error: false,
            pull_error_message: None,
        }
    }
}

impl Versioned for DataSourceStatus {
    fn latest() -> &'static str {
        "0.1"
    }
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub url: String,
}