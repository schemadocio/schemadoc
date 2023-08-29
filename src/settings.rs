use crate::models::ProjectSlug;
use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug)]
pub struct Settings {
    pub(crate) frontend_origin: String,
    pub(crate) frontend_static_files: String,
    pub(crate) pull_disable_after_attempt: u32,

    pub(crate) persistence: PersistenceType,
    pub(crate) persistence_path: String,

    pub(crate) config_persistence: PersistenceType,
    pub(crate) config_persistence_path: String,
}

#[derive(PartialEq, Debug, Default)]
pub enum PersistenceType {
    #[default]
    Local,
}

impl PersistenceType {
    pub fn is_local(&self) -> bool {
        matches!(self, PersistenceType::Local)
    }
}

impl FromStr for PersistenceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(PersistenceType::Local),
            _ => Err(anyhow!("Value `{}` not supported.", s)),
        }
    }
}

impl Settings {
    pub(crate) fn from_env() -> anyhow::Result<Self> {
        let frontend_origin = std::env::var("SD_FRONTEND_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_owned());
        let frontend_static_files =
            std::env::var("SD_FRONTEND_STATIC_FILES").unwrap_or_else(|_| "./static".to_owned());

        let persistence = std::env::var("SD_PERSISTENCE")
            .ok()
            .map(|p| p.parse())
            .transpose()?
            .unwrap_or_default();
        let persistence_path =
            std::env::var("SD_PERSISTENCE_PATH").unwrap_or_else(|_| "./persistence".to_owned());

        let config_persistence = std::env::var("SD_CONFIG_PERSISTENCE")
            .ok()
            .map(|p| p.parse())
            .transpose()?
            .unwrap_or_default();
        let config_persistence_path = std::env::var("SD_CONFIG_PERSISTENCE_PATH")
            .unwrap_or_else(|_| "./persistence".to_owned());

        let pull_disable_after_attempt: u32 = std::env::var("SD_PULL_DISABLE_AFTER_ATTEMPT")
            .unwrap_or_else(|_| "0".to_owned())
            .parse()
            .expect("SD_PULL_DISABLE_AFTER_ATTEMPT must be u32");

        Ok(Self {
            frontend_origin,
            frontend_static_files,
            pull_disable_after_attempt,

            persistence,
            persistence_path,

            config_persistence,
            config_persistence_path,
        })
    }

    pub(crate) fn url_to_version(
        &self,
        project_slug: &ProjectSlug,
        branch_name: &str,
        version_id: u32,
    ) -> String {
        let branch_name = urlencoding::encode(branch_name);
        format!(
            "{}/projects/{}/branches/{}/versions/{}",
            self.frontend_origin, project_slug, branch_name, version_id
        )
    }

    pub(crate) fn url_to_dependency_compare(
        &self,
        project_slug: &ProjectSlug,
        dep: &ProjectSlug,
        src_branch_name: &str,
        src_version_id: u32,
        tgt_branch_name: &str,
        tgt_version_id: u32,
    ) -> String {
        format!(
            "{}/projects/{}/dependencies?dep={}&srcBranch={}&src={}&tgtBranch={}&tgt={}",
            self.frontend_origin,
            project_slug,
            dep,
            src_branch_name,
            src_version_id,
            tgt_branch_name,
            tgt_version_id
        )
    }
}
