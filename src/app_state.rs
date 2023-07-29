use anyhow::bail;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::dependencies::update_dependent_projects;
use crate::models::{Alert, DataSource, Dependency, Project, ProjectKind, ProjectSlug};
use crate::persistence::{load_data_file, PersistentData, Versioned};
use crate::settings::Settings;
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
        let app_state =
            load_data_file::<AppStatePersistentData, _, _, PersistentProjectsFile<_>>(
                config_storage.as_ref().unwrap_or(&storage), "schemadoc.yaml",
            ).await?;

        let mut projects: IndexMap<_, _> = app_state.0
            .into_iter()
            .map(|(slug, config)| {
                let dependencies = config.dependencies
                    .map(
                        |deps|
                            deps.into_iter().map(
                                |(project, version)|
                                    Dependency {
                                        project,
                                        version: version.parse().unwrap_or(0),
                                        breaking: None,
                                        outdated: None,
                                    }
                            ).collect()
                    );

                let kind = config.kind.unwrap_or(ProjectKind::Server);

                let project = Project {
                    kind,
                    dependencies,
                    versions: None,
                    slug: slug.clone(),
                    name: config.name,
                    alerts: config.alerts,
                    description: config.description,
                    data_source: config.data_source,
                };

                (slug, project)
            }).collect();

        for project in projects.values_mut() {
            project.load_persistent_data(&storage).await?;
        }

        let mut app_state = Self {
            projects,
            storage,
            config_storage,
        };

        // Update dependent projects dependencies status
        let slugs = app_state
            .projects
            .keys()
            .map(|slug| slug.to_owned())
            .collect::<Vec<_>>();
        for slug in slugs {
            update_dependent_projects(&mut app_state, &slug).await?;
        }

        Ok(app_state)
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

    pub alerts: Option<Vec<Alert>>,
    pub data_source: Option<DataSource>,
    pub dependencies: Option<IndexMap<ProjectSlug, String>>,
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