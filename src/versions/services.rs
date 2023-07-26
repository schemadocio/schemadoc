use chrono::Utc;
use sha2::{Digest, Sha256};

use schemadoc_diff::checker::validate;
use schemadoc_diff::core::DiffResult;
use schemadoc_diff::schema_diff::HttpSchemaDiff;

use crate::app_state::AppState;
use crate::config::Config;
use crate::models::{ProjectSlug, Version};
use crate::storage::{Storage, Storer};
use crate::{alerts, dependencies, versions};

pub async fn compare_versions(
    app_state: &AppState,
    project: &ProjectSlug,
    src_id: u32,
    tgt_id: u32,
) -> anyhow::Result<DiffResult<HttpSchemaDiff>> {
    let src_version = versions::crud::get_version(app_state, project, src_id)
        .await
        .ok_or(anyhow::Error::msg("Source version not found"))?;
    let tgt_version = versions::crud::get_version(app_state, project, tgt_id)
        .await
        .ok_or(anyhow::Error::msg("Target version not found"))?;

    let storage = &app_state.storage;

    let src_schema_content = storage.read_file(&src_version.file_path).await?;
    let tgt_schema_content = storage.read_file(&tgt_version.file_path).await?;

    let diff = compare_versions_content(
        &String::from_utf8_lossy(&src_schema_content),
        &String::from_utf8_lossy(&tgt_schema_content),
    )?;

    Ok(diff)
}

pub async fn compare_version_with_schema_content(
    storage: &Storage,
    src_version: &Version,
    tgt_schema_content: &str,
) -> anyhow::Result<DiffResult<HttpSchemaDiff>> {
    let src_schema_content = storage.read_file(&src_version.file_path).await?;

    let diff = compare_versions_content(
        &String::from_utf8_lossy(&src_schema_content),
        tgt_schema_content,
    )?;

    Ok(diff)
}

fn compare_versions_content(
    src_schema_content: &str,
    tgt_schema_content: &str,
) -> Result<DiffResult<HttpSchemaDiff>, schemadoc_diff::error::Error> {
    let (src_schema, tgt_schema) =
        schemadoc_diff::try_deserialize_schema(src_schema_content, tgt_schema_content)?;

    let diff = schemadoc_diff::get_schema_diff(src_schema, tgt_schema);

    Ok(diff)
}

pub struct CreatedVersion {
    pub version: Option<Version>,
    pub diff: Option<HttpSchemaDiff>,
    pub src_version_id: Option<u32>,
}

async fn create_version_inner(
    app_state: &mut AppState,
    project_slug: &ProjectSlug,
    message: Option<String>,
    content: &str,
) -> anyhow::Result<CreatedVersion> {
    let Some(project) = app_state.projects.get_mut(project_slug) else {
        return Err(anyhow::Error::msg("Project versions not found"));
    };

    let versions = &mut project.versions;

    let latest_version = versions
        .as_ref()
        .and_then(|vs| vs.iter().max_by_key(|v| v.id));

    let src_version_id = latest_version.map(|lv| lv.id);

    let diff = match latest_version {
        Some(src_version) => {
            compare_version_with_schema_content(&app_state.storage, src_version, content).await?
        }
        None => {
            // For first version compare to itself
            compare_versions_content(content, content)?
        }
    };

    // Skip this version if it has no changes and there are any versions before it
    if diff.is_same_or_none() && latest_version.is_some() {
        return Ok(CreatedVersion {
            diff: None,
            version: None,
            src_version_id,
        });
    }

    let diff = diff.take().expect(
        "Root diff of two schemas must not be empty.\
             Probably two null schemas were provided.",
    );

    let hash = Sha256::digest(content);

    let file_path = format!("projects/{project_slug}/versions/{hash:x}.json");
    if !app_state.storage.exists(&file_path).await? {
        app_state
            .storage
            .put_file(&file_path, content.as_bytes())
            .await?;
    }

    let id = versions
        .as_ref()
        .and_then(|vs| vs.iter().max_by_key(|v| v.id).map(|v| &v.id + 1))
        .unwrap_or(0);

    let diff_file_path = format!("projects/{project_slug}/diffs/{id}.json");

    app_state
        .storage
        .put_file(&diff_file_path, &serde_json::to_vec(&diff)?)
        .await?;

    let diff_file_version = diff.get_diff_version();

    let version = diff
        .info
        .get()
        .and_then(|info| info.version.get().map(|v| v.to_owned()));

    let version = Version {
        id,
        version,
        message,
        file_path,
        diff_file_path,
        diff_file_version,
        created_at: Utc::now(),
    };

    match versions {
        None => *versions = Some(vec![version.clone()]),
        Some(versions) => versions.push(version.clone()),
    };

    project.persist_versions(&app_state.storage).await?;

    Ok(CreatedVersion {
        diff: Some(diff),
        version: Some(version),
        src_version_id,
    })
}

pub async fn create_version(
    config: &Config,
    app_state: &mut AppState,
    project_slug: &ProjectSlug,
    message: Option<String>,
    content: &str,
) -> anyhow::Result<Option<Version>> {
    let result = create_version_inner(app_state, project_slug, message, content).await?;

    let Some(version) = result.version else {
        return Ok(None);
    };

    let Some(diff) = result.diff else {
        return Ok(None);
    };

    let src_projects_slugs =
        dependencies::update_dependent_projects(app_state, project_slug).await?;

    //handle alerts
    let project = app_state
        .projects
        .get(project_slug)
        .expect("Project must not be removed during add version operation.");

    let validations = validate(&diff, &["*"]);

    // own alerts
    if project.alerts.is_some() {
        let alerts =
            alerts::get_own_alerts_info(config, project, version.id, &diff, &validations).await?;
        for alert in alerts {
            println!("Send own alert: {}", alert.service);
            alerts::send_alert(alert).await?;
        }
    };

    // deps alerts
    if let Some(src_version_id) = result.src_version_id {
        let dep_projects: Vec<_> = src_projects_slugs
            .iter()
            .filter_map(|slug| app_state.projects.get(slug))
            .collect();

        let alerts = alerts::get_deps_alerts_info(
            config,
            project,
            src_version_id,
            version.id,
            dep_projects,
            &diff,
            &validations,
        )
        .await?;
        for alert in alerts {
            println!("Send deps alert: {}", alert.service);
            alerts::send_alert(alert).await?;
        }
    }

    Ok(Some(version))
}
