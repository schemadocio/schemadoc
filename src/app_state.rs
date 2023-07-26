use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::dependencies::update_dependent_projects;
use crate::models::{Project, ProjectSlug};
use crate::persistence::{load_data_file, Versioned};
use crate::storage::Storage;

#[derive(Debug)]
pub struct AppState {
    pub storage: Storage,
    pub projects: IndexMap<ProjectSlug, Project>,
}

#[derive(Serialize, Deserialize, Default)]
struct AppStatePersistentData(pub IndexMap<ProjectSlug, Project>);

impl Versioned for AppStatePersistentData {
    fn latest() -> &'static str {
        "0.1"
    }
}

impl AppState {
    pub async fn read(storage: Storage) -> anyhow::Result<Self> {
        let mut app_state =
            load_data_file::<AppStatePersistentData, _, _>(&storage, "schemadoc.yaml").await?;

        for project in app_state.0.values_mut() {
            project.load_persistent_data(&storage).await?;
        }

        let mut app_state = Self {
            storage,
            projects: app_state.0,
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
