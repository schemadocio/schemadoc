use crate::branches;
use chrono::{DateTime, Utc};
use schemadoc_diff::schema_diff::HttpSchemaDiff;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use sha2::{Digest, Sha256};
use std::fmt;

use crate::persistence::{load_data_file, persist_data_file, PersistentDataFile, Versioned};
use crate::storage::Storer;
use crate::versions::statistics::DiffStatistics;

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
    pub data_sources: Vec<DataSource>,
    pub dependencies: Vec<Dependency>,
}

impl Project {
    pub async fn load_persistent_data<S>(&mut self, storage: &S) -> anyhow::Result<()>
    where
        S: Storer,
    {
        let branches_file_path = format!("projects/{}/branches.yaml", self.slug);
        self.branches =
            load_data_file::<Vec<Branch>, _, _, PersistentDataFile<_>>(storage, branches_file_path)
                .await
                .unwrap_or_default();

        if self.branches.is_empty() {
            println!(
                "{} - Add default branch: {} ",
                self.slug, self.default_branch
            );
            self.branches.push(Branch {
                name: self.default_branch.clone(),
                versions: vec![],
                base: None,
            });

            self.persist_branches(storage).await?;
        }

        for data_source in &mut self.data_sources {
            let branch_name = branches::sanitise_branch_name(&data_source.branch);

            let data_source_status_file_path = format!(
                "projects/{}/branches/{}/datasource.yaml",
                self.slug, branch_name,
            );

            let data_source_status =
                load_data_file::<DataSourceStatus, _, _, PersistentDataFile<_>>(
                    storage,
                    data_source_status_file_path,
                )
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
        persist_data_file::<Vec<Branch>, _, _, PersistentDataFile<_>>(
            storage,
            path,
            &self.branches,
        )
        .await?;

        Ok(())
    }

    pub async fn persist_datasource<S>(&self, storage: &S, branch_name: &str) -> anyhow::Result<()>
    where
        S: Storer,
    {
        let Some(ds) = &self.data_sources.iter().find(|ds| ds.branch == branch_name) else {
            return Ok(());
        };

        let Some(data_source_status) = &ds.status else {
            return Ok(());
        };

        let branch_name = branches::sanitise_branch_name(branch_name);

        let path = format!(
            "projects/{}/branches/{branch_name}/datasource.yaml",
            self.slug,
        );

        persist_data_file::<DataSourceStatus, _, _, PersistentDataFile<_>>(
            storage,
            path,
            data_source_status,
        )
        .await?;

        Ok(())
    }

    pub async fn persist_version<S: Storer>(
        &self,
        storage: &S,
        content: &str,
    ) -> anyhow::Result<String> {
        let hash = Sha256::digest(content);
        // Write versions to shared folder, in that way we are caching them,
        //  but someday we need to add check for collisions here
        let file_path = format!("projects/{}/versions/{hash:x}.json", self.slug);
        if !storage.exists(&file_path).await? {
            storage.put_file(&file_path, content.as_bytes()).await?;
        }

        Ok(file_path)
    }

    pub async fn persist_version_diff<S: Storer>(
        &self,
        storage: &S,
        branch_name: &str,
        version_id: u32,
        diff: &HttpSchemaDiff,
    ) -> anyhow::Result<String> {
        let branch_name = branches::sanitise_branch_name(branch_name);

        let diff_file_path = format!(
            "projects/{}/branches/{branch_name}/diffs/{version_id}.json",
            self.slug,
        );

        storage
            .put_file(&diff_file_path, &serde_json::to_vec(diff)?)
            .await?;

        Ok(diff_file_path)
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
        } else {
            self.branches
                .iter()
                .any(|b| b.as_str() == "*" || b.as_str() == branch)
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
