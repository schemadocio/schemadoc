use anyhow::bail;
use chrono::Utc;

use schemadoc_diff::checker::validate;
use schemadoc_diff::core::DiffResult;
use schemadoc_diff::schema_diff::HttpSchemaDiff;

use crate::app_state::AppState;
use crate::settings::Settings;
use crate::storage::Storer;

use crate::models::{ProjectSlug, Version};
use crate::{alerts, branches, dependencies, versions};

pub struct CreatedVersion {
    pub version: Version,
    pub diff: HttpSchemaDiff,
    pub src_version_id: u32,
    pub src_branch_name: String,
}

async fn create_version_inner(
    state: &mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    message: Option<String>,
    content: &str,
) -> anyhow::Result<Option<CreatedVersion>> {
    let (src_branch_name, src_version) = get_source_version(state, project_slug, branch_name)?;

    let Some(project) = state.projects.get_mut(project_slug) else {
        bail!("Project {project_slug} not found")
    };

    let Some(branch) = branches::get_branch(project, branch_name) else {
        bail!("Project branch {project_slug}/{branch_name} not found")
    };

    let diff = match src_version.as_ref() {
        Some(src_version) => {
            let src_schema_content = state.storage.read_file(&src_version.file_path).await?;
            compare_schemas_content(&String::from_utf8_lossy(&src_schema_content), content)?
        }
        None => {
            // For first version compare to itself
            compare_schemas_content(content, content)?
        }
    };

    // Skip this version if it has no changes and there are any versions before it
    if diff.is_same_or_none() && src_version.is_some() {
        return Ok(None);
    }

    let diff = diff.take().expect(
        "Root diff of two schemas must not be empty.\
             Probably two null schemas were provided.",
    );

    let next_id = branch
        .versions
        .iter()
        .max_by_key(|v| v.id)
        .map(|v| &v.id + 1)
        .unwrap_or(0);

    let diff_file_path = project
        .persist_version_diff(&state.storage, branch_name, next_id, &diff)
        .await?;

    let file_path = project.persist_version(&state.storage, content).await?;

    let diff_file_version = diff.get_diff_version();

    let version = diff.info.get().and_then(|info| info.version.get().cloned());

    let statistics = versions::statistics::get_diff_statistics(&diff);

    let version = Version {
        id: next_id,
        version,
        message,
        file_path,
        statistics,
        diff_file_path,
        diff_file_version,
        created_at: Utc::now(),
    };

    branches::get_branch_mut(project, branch_name)
        .expect("Must be created before this version")
        .versions
        .push(version.clone());

    project.persist_branches(&state.storage).await?;

    let src_version_id = src_version.as_ref().map(|v| v.id).unwrap_or(next_id);

    Ok(Some(CreatedVersion {
        diff,
        version,
        src_version_id,
        src_branch_name,
    }))
}

fn get_source_version(
    state: &AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
) -> anyhow::Result<(String, Option<Version>)> {
    let Some(project) = state.projects.get(project_slug) else {
        bail!("Project {project_slug} not found")
    };

    let Some(branch) = branches::get_branch(project, branch_name) else {
        bail!("Project branch {project_slug}/{branch_name} not found")
    };
    // Use latest version from branch as source version
    let src_version = branch.versions.iter().max_by_key(|v| v.id);
    if src_version.is_some() {
        return Ok((branch.name.clone(), src_version.cloned()));
    }
    // Use version specified as branch base version from base branch

    let Some(base) = &branch.base else {
        return Ok((branch.name.clone(), None));
    };

    let Some(base_branch) = branches::get_branch(project, &base.name) else {
        bail!(
            "Base branch {} not found for {}/{}",
            base.name,
            project_slug,
            branch_name
        )
    };

    let base_version = base_branch
        .versions
        .iter()
        .find(|v| v.id == base.version_id);

    Ok((base_branch.name.clone(), base_version.cloned()))
}

pub async fn create_version(
    settings: &Settings,
    state: &mut AppState,
    project_slug: &ProjectSlug,
    branch_name: &str,
    message: Option<String>,
    content: &str,
) -> anyhow::Result<Option<Version>> {
    let result = create_version_inner(state, project_slug, branch_name, message, content).await?;

    let Some(result) = result else {
        return Ok(None);
    };

    let src_projects_slugs =
        dependencies::update_dependent_projects(state, project_slug, Some(branch_name)).await?;

    //handle alerts
    let project = state
        .projects
        .get(project_slug)
        .expect("Project must not be removed during add version operation.");

    let validations = validate(&result.diff, &["*"]);

    // own alerts
    if !project.alerts.is_empty() {
        let alerts = alerts::get_own_alerts_info(
            settings,
            project,
            branch_name,
            result.version.id,
            &result.diff,
            &validations,
        )
        .await?;
        for alert in alerts {
            println!(
                "Send own alert: {}/{} - {}",
                project_slug, branch_name, alert.service
            );
            alerts::send_alert(alert).await?;
        }
    };

    // deps alerts
    let dep_projects: Vec<_> = src_projects_slugs
        .iter()
        .filter_map(|slug| state.projects.get(slug))
        .collect();
    if !dep_projects.is_empty() {
        let alerts = alerts::get_deps_alerts_info(
            settings,
            project,
            &result.src_branch_name,
            result.src_version_id,
            branch_name,
            result.version.id,
            dep_projects,
            &result.diff,
            &validations,
        )
        .await?;

        for alert in alerts {
            println!(
                "Send deps alert: {}/{} - {}",
                project_slug, branch_name, alert.service
            );
            alerts::send_alert(alert).await?;
        }
    }

    Ok(Some(result.version))
}

pub async fn compare_versions(
    state: &AppState,
    project_slug: &ProjectSlug,
    src_branch_name: &str,
    src_version_id: u32,
    tgt_branch_name: &str,
    tgt_version_id: u32,
) -> anyhow::Result<DiffResult<HttpSchemaDiff>> {
    let src_version =
        versions::crud::get_version(state, project_slug, src_branch_name, src_version_id).ok_or(
            anyhow::Error::msg(format!(
                "Source version {}/{}/{} not found",
                project_slug, src_branch_name, src_version_id
            )),
        )?;
    let tgt_version =
        versions::crud::get_version(state, project_slug, tgt_branch_name, tgt_version_id).ok_or(
            anyhow::Error::msg(format!(
                "Target version {}/{}/{} not found",
                project_slug, tgt_branch_name, tgt_version_id
            )),
        )?;

    let storage = &state.storage;

    let src_schema_content = storage.read_file(&src_version.file_path).await?;
    let tgt_schema_content = storage.read_file(&tgt_version.file_path).await?;

    let diff = compare_schemas_content(
        &String::from_utf8_lossy(&src_schema_content),
        &String::from_utf8_lossy(&tgt_schema_content),
    )?;

    Ok(diff)
}

pub fn compare_schemas_content(
    src_schema_content: &str,
    tgt_schema_content: &str,
) -> Result<DiffResult<HttpSchemaDiff>, schemadoc_diff::error::Error> {
    let (src_schema, tgt_schema) =
        schemadoc_diff::try_deserialize_schema(src_schema_content, tgt_schema_content)?;

    let diff = schemadoc_diff::get_schema_diff(src_schema, tgt_schema);

    Ok(diff)
}
