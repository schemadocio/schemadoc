use std::collections::HashMap;
use anyhow::bail;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::constants;

use crate::settings::Settings;
use crate::dependencies::setup_project_dependencies;
use crate::models::{Alert, AlertKind, AlertSource, DataSource, DataSourceSource, Dependency, Link, Project, ProjectKind, ProjectSlug};
use crate::persistence::{load_data_file, PersistentData, Versioned};
use crate::storage::{LocalStorage, Storage};

#[derive(Debug)]
pub struct AppState {
    pub storage: Storage,
    pub config_storage: Option<Storage>,
    pub projects: IndexMap<ProjectSlug, Project>,
}

impl AppState {
    pub async fn from_settings(settings: &Settings) -> anyhow::Result<Self> {
        let storage = if settings.persistence.is_local() {
            Storage::Local(LocalStorage::new(&settings.persistence_path))
        } else {
            bail!("Persistence {:?} not supported", settings.persistence)
        };

        let config_storage = if settings.persistence == settings.persistence_config
            && settings.persistence_path == settings.persistence_config_path {
            None
        } else {
            let storage = if settings.persistence_config.is_local() {
                Storage::Local(LocalStorage::new(&settings.persistence_config_path))
            } else {
                bail!("Persistence {:?} not supported", settings.persistence)
            };
            Some(storage)
        };

        AppState::read(storage, config_storage).await
    }

    pub async fn read(storage: Storage, config_storage: Option<Storage>) -> anyhow::Result<Self> {
        let state_data =
            load_data_file::<AppStatePersistentData, _, _, PersistentProjectsFile<_>>(
                config_storage.as_ref().unwrap_or(&storage), "schemadoc.yaml",
            ).await?;

        let default_branches: HashMap<_, _> = state_data.0.iter()
            .map(|(slug, config)|
                (
                    slug.clone(),
                    config.default_branch
                        .as_ref()
                        .map(|v| v.as_str())
                        .unwrap_or(constants::BRANCH_DEFAULT_NAME)
                        .to_owned()
                )
            ).collect();

        let mut projects: IndexMap<_, _> = state_data.0
            .into_iter()
            .map(|(slug, config)| {
                let default_branch = config.default_branch
                    .unwrap_or_else(|| constants::BRANCH_DEFAULT_NAME.to_owned());

                let dependencies = config.dependencies
                    .map(
                        |deps|
                            deps.into_iter().filter_map(
                                |(project, def)|
                                    {
                                        let Some(branch) = default_branches.get(&project).map(|x| x.to_owned()) else {
                                            return None;
                                        };

                                        let dependency = if let Ok(version) = serde_yaml::from_value::<u32>(def.clone()) {
                                            Dependency {
                                                branch,
                                                project,
                                                version,
                                                breaking: None,
                                                outdated: None,
                                            }
                                        } else if let Ok(version) = serde_yaml::from_value::<String>(def.clone()) {
                                            Dependency {
                                                branch,
                                                project,
                                                version: version.parse().unwrap_or(0),
                                                breaking: None,
                                                outdated: None,
                                            }
                                        } else if let Ok(config) = serde_yaml::from_value::<ProjectDependencyConfig>(def.clone()) {
                                            Dependency {
                                                branch,
                                                project,
                                                version: config.version
                                                    .parse()
                                                    .unwrap_or(0),
                                                breaking: None,
                                                outdated: None,
                                            }
                                        } else {
                                            panic!("In {} dependency {} has unknown structure", slug, project)
                                        };

                                        Some(dependency)
                                    }
                            ).collect()
                    );

                let kind = config.kind.unwrap_or(ProjectKind::Server);

                let alerts = config.alerts
                    .map(|alerts|
                        alerts.into_iter().map(
                            |alert|
                                Alert {
                                    name: alert.name,
                                    kind: alert.kind,
                                    source: alert.source,
                                    branches: alert.branches.unwrap_or_default(),
                                    is_active: alert.is_active,
                                    service: alert.service,
                                    service_config: alert.service_config,
                                }
                        ).collect()
                    )
                    .unwrap_or_default();


                let data_source = config.data_source
                    .map(|ds| DataSource {
                        status: None,
                        name: ds.name,
                        source: ds.source,
                        branch: ds.branch.unwrap_or(default_branch.clone()),
                    });

                let project = Project {
                    kind,
                    alerts,
                    data_source,
                    default_branch,
                    branches: vec![],
                    slug: slug.clone(),
                    name: config.name,
                    links: config.links,
                    description: config.description,
                    dependencies: dependencies.unwrap_or_default(),
                };

                (slug, project)
            }).collect();


        for project in projects.values_mut() {
            project.load_persistent_data(&storage).await?;
        }

        let mut state = Self {
            projects,
            storage,
            config_storage,
        };

        setup_project_dependencies(&mut state).await?;

        Ok(state)
    }
}


#[derive(Serialize, Deserialize, Default)]
struct AppStatePersistentData(pub IndexMap<ProjectSlug, ProjectConfig>);

impl Versioned for AppStatePersistentData {
    fn latest() -> &'static str {
        "0.1"
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub kind: Option<ProjectKind>,
    pub description: Option<String>,

    pub links: Option<Vec<Link>>,

    pub alerts: Option<Vec<AlertConfig>>,
    pub data_source: Option<DataSourceConfig>,
    pub dependencies: Option<IndexMap<ProjectSlug, serde_yaml::Value>>,

    pub default_branch: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertConfig {
    pub name: String,
    pub kind: AlertKind,
    pub source: AlertSource,

    pub branches: Option<Vec<String>>,

    pub is_active: bool,

    pub service: String,
    pub service_config: serde_yaml::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSourceConfig {
    pub name: String,
    pub branch: Option<String>,
    pub source: DataSourceSource,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDependencyConfig {
    pub version: String,
    pub branch: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PersistentProjectsFile<T> {
    pub version: String,
    pub projects: T,
}

impl<T> PersistentData<T> for PersistentProjectsFile<T> {
    fn version(&self) -> &str {
        &self.version
    }

    fn data(self) -> T {
        self.projects
    }

    fn new(version: impl Into<String>, data: T) -> Self {
        PersistentProjectsFile {
            version: version.into(),
            projects: data,
        }
    }
}