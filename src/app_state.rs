use crate::models::{Project, ProjectSlug};
use crate::persistence::{load_data_file, Versioned};
use crate::storage::Storage;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

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

        Ok(Self {
            storage,
            projects: app_state.0,
        })
    }
}
