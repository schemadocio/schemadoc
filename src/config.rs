use crate::models::ProjectSlug;

#[derive(Debug)]
pub struct Config {
    pub(crate) frontend_origin: String,
    pub(crate) frontend_static_files: String,
    pub(crate) pull_disable_after_attempt: u32,

    pub(crate) persistence: String,
    pub(crate) persistence_path: String,
}

impl Config {
    pub(crate) fn from_env() -> Self {
        let frontend_origin = std::env::var("SD_FRONTEND_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_owned());
        let frontend_static_files = std::env::var("SD_FRONTEND_STATIC_FILES")
            .unwrap_or_else(|_| "./static".to_owned());

        let persistence = std::env::var("SD_PERSISTENCE")
            .unwrap_or_else(|_| "local".to_owned());
        let persistence_path = std::env::var("SD_PERSISTENCE_PATH")
            .unwrap_or_else(|_| "/schemadoc".to_owned());

        let pull_disable_after_attempt: u32 = std::env::var("SD_PULL_DISABLE_AFTER_ATTEMPT")
            .unwrap_or_else(|_| "0".to_owned())
            .parse()
            .expect("SD_PULL_DISABLE_AFTER_ATTEMPT must be u32");

        Self {
            frontend_origin,
            frontend_static_files,
            pull_disable_after_attempt,

            persistence,
            persistence_path,
        }
    }

    pub(crate) fn url_to_version(&self, project_slug: &ProjectSlug, version_id: u32) -> String {
        format!(
            "{}/projects/{}/versions/{}",
            self.frontend_origin, project_slug, version_id
        )
    }
}
